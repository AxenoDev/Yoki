use std::collections::HashMap;
use std::sync::OnceLock;
use proc_macro::TokenStream;
use std::path::{Path, PathBuf};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};
use serde::Deserialize;
use crate::attrs::{packet_type_from_variant_field, parse_protocol_id_attr, state_str_to_ident};

pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_derive(&input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

struct VariantInfo {
    variant_ident: syn::Ident,
    packet_type: syn::Path,
    state_ident: syn::Ident,
    state_str: String,
    bound: String,
    name_str: String,
}

#[derive(Deserialize)]
struct PacketEntry {
    protocol_id: i32,
}

type Registry = HashMap<String, HashMap<String, HashMap<String, PacketEntry>>>;

static PROTOCOL_VERSION_REGISTRIES: OnceLock<Vec<(String, Registry)>> = OnceLock::new();

fn find_data_generated_dir() -> PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut current = manifest_dir;

    loop {
        let candidate = current.join("data").join("generated");
        if candidate.is_dir() {
            return candidate;
        }

        match current.parent() {
            Some(parent) => current = parent,
            None => panic!("Couldn't find parent: {} ", current.display()),
        }
    }
}

fn load_registry() -> &'static Vec<(String, Registry)> {
    PROTOCOL_VERSION_REGISTRIES.get_or_init(|| {
        let data_generated_dir = find_data_generated_dir();

        let mut registries = Vec::new();
        let mut skipped_dirs = Vec::new();

        let entries = std::fs::read_dir(&data_generated_dir).unwrap_or_else(|err| {
            panic!(
                "impossible de lire le dossier {}: {err}",
                data_generated_dir.display()
            )
        });

        for entry in entries {
            let entry = entry.expect("entrée de dossier invalide dans data/generated");
            if !entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                continue;
            }

            let folder_name = entry.file_name().to_string_lossy().into_owned();

            let mut chars = folder_name.chars();
            let is_valid_ident =
                matches!(chars.next(), Some(c) if c.is_ascii_alphabetic() || c == '_')
                    && chars.all(|c| c.is_ascii_alphanumeric() || c == '_');
            if !is_valid_ident {
                skipped_dirs.push(format!("{folder_name} (nom invalide)"));
                continue;
            }

            let packets_path = entry.path().join("reports").join("packets.json");
            if !packets_path.exists() {
                skipped_dirs.push(format!("{folder_name} (pas de reports/packets.json)"));
                continue;
            }

            let raw = std::fs::read_to_string(&packets_path).unwrap_or_else(|err| {
                panic!("impossible de lire {}: {err}", packets_path.display())
            });

            let registry: Registry = serde_json::from_str(&raw).unwrap_or_else(|err| {
                panic!("packets.json invalide dans {}: {err}", packets_path.display())
            });

            registries.push((folder_name, registry));
        }

        if registries.is_empty() {
            panic!(
                "aucune version de protocole trouvée sous {}. Dossiers ignorés: [{}]",
                data_generated_dir.display(),
                skipped_dirs.join(", ")
            );
        }

        registries.sort_by(|a, b| a.0.cmp(&b.0));
        registries
    })
}

fn resolve_packet_id(registry: &Registry, state: &str, bound: &str, name: &str) -> Option<i32> {
    registry.get(state)?.get(bound)?.get(name).map(|e| e.protocol_id)
}

fn expand_derive(input: &DeriveInput) -> syn::Result<TokenStream2> {
    let enum_ident = &input.ident;

    let variants = match &input.data {
        Data::Enum(data_enum) => &data_enum.variants,
        _ => {
            return Err(syn::Error::new_spanned(
                enum_ident,
                "PacketReport can only be derived for enums",
            ));
        }
    };

    let mut variant_infos = Vec::new();

    for variant in variants {
        let fields = match &variant.fields {
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => &fields.unnamed,
            _ => {
                return Err(syn::Error::new_spanned(
                    variant,
                    "enum variants must have exactly one unnamed field (the packet type)",
                ));
            }
        };

        let packet_type = packet_type_from_variant_field(&fields.first().unwrap().ty)?;
        let protocol_id = parse_protocol_id_attr(&variant.attrs)?;

        let state_str = protocol_id.state.value();
        let bound_str = protocol_id.bound.value();
        let name_str = protocol_id.name.value();
        let state_ident = state_str_to_ident(&state_str)?;

        let known_in_any_version = load_registry().iter().any(|(_, registry)| {
            resolve_packet_id(registry, &state_str, &bound_str, &name_str).is_some()
        });

        if !known_in_any_version {
            return Err(syn::Error::new_spanned(
                &protocol_id.name,
                format!(
                    "paquet `{name_str}` introuvable pour state=`{state_str}` bound=`{bound_str}` dans aucune version connue de data/generated"
                ),
            ));
        }

        variant_infos.push(VariantInfo {
            variant_ident: variant.ident.clone(),
            packet_type,
            state_ident,
            state_str,
            bound: bound_str,
            name_str,
        });
    }

    let registries = load_registry();

    let mut decode_version_arms = Vec::new();
    let mut encode_version_arms = Vec::new();
    let mut id_version_arms = Vec::new();

    for (folder_name, registry) in registries {
        let version_ident = syn::Ident::new(folder_name, proc_macro2::Span::call_site());

        let mut decode_arms = Vec::new();
        let mut encode_arms = Vec::new();
        let mut id_arms = Vec::new();

        for v in &variant_infos {
            let Some(id) = resolve_packet_id(registry, &v.state_str, &v.bound, &v.name_str) else {
                continue;
            };

            let variant_ident = &v.variant_ident;
            let state_ident = &v.state_ident;
            let packet_type = &v.packet_type;

            if v.bound == "serverbound" {
                decode_arms.push(quote! {
                    (minecraft_protocol::State::#state_ident, #id) => {
                        let packet = raw.decode::<#packet_type>()?;
                        return Ok(Self::#variant_ident(packet));
                    }
                });
            }

            if v.bound == "clientbound" {
                encode_arms.push(quote! {
                    Self::#variant_ident(packet) => {
                        let mut writer = yoki_binutils::BinaryWriter::new();
                        minecraft_packet::OutgoingPacket::encode_payload(&packet, &mut writer)?;
                        return Ok(minecraft_packet::RawPacket {
                            id: #id,
                            payload: writer.into_inner(),
                        });
                    }
                });
            }

            id_arms.push(quote! {
                Self::#variant_ident(_) => Ok((minecraft_protocol::State::#state_ident, #id)),
            });
        }

        decode_version_arms.push(quote! {
            protocol_version::protocol_version::ProtocolVersion::#version_ident => {
                match (state, raw.id) {
                    #(#decode_arms)*
                    (state, id) => Err(yoki_binutils::ProtocolError::UnknownPacket {
                        id,
                        conn: Some(state),
                    }),
                }
            }
        });

        encode_version_arms.push(quote! {
            protocol_version::protocol_version::ProtocolVersion::#version_ident => {
                match self {
                    #(#encode_arms)*
                    _ => Err(yoki_binutils::ProtocolError::UnknownPacket {
                        id: -1,
                        conn: None,
                    }),
                }
            }
        });

        id_version_arms.push(quote! {
            protocol_version::protocol_version::ProtocolVersion::#version_ident => match self {
                #(#id_arms)*
                _ => Err(yoki_binutils::ProtocolError::UnknownPacket {
                    id: -1,
                    conn: None,
                }),
            }
        });
    }

    Ok(quote! {
        impl #enum_ident {
            pub fn decode_serverbound(
                protocol_version: protocol_version::protocol_version::ProtocolVersion,
                state: minecraft_protocol::State,
                raw: &minecraft_packet::RawPacket,
            ) -> Result<Self, yoki_binutils::ProtocolError> {
                match protocol_version {
                    #(#decode_version_arms)*
                    _ => Err(yoki_binutils::ProtocolError::UnknownPacket {
                        id: raw.id,
                        conn: Some(state),
                    }),
                }
            }

            pub fn encode_clientbound(
                self,
                protocol_version: protocol_version::protocol_version::ProtocolVersion,
            ) -> Result<minecraft_packet::RawPacket, yoki_binutils::ProtocolError> {
                match protocol_version {
                    #(#encode_version_arms)*
                    _ => Err(yoki_binutils::ProtocolError::UnknownPacket {
                        id: -1,
                        conn: None,
                    }),
                }
            }

            pub fn state_and_id(
                &self,
                protocol_version: protocol_version::protocol_version::ProtocolVersion,
            ) -> Result<(minecraft_protocol::State, i32), yoki_binutils::ProtocolError> {
                match protocol_version {
                    #(#id_version_arms)*
                    _ => Err(yoki_binutils::ProtocolError::UnknownPacket {
                        id: -1,
                        conn: None,
                    }),
                }
            }
        }
    })
}
