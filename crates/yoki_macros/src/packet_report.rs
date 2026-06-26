use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

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
    bound: String,
    id: i32,
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
        let state_ident = state_str_to_ident(&protocol_id.state.value())?;

        variant_infos.push(VariantInfo {
            variant_ident: variant.ident.clone(),
            packet_type,
            state_ident,
            bound: protocol_id.bound.value(),
            id: protocol_id.id,
        });
    }

    let decode_arms = variant_infos
        .iter()
        .filter(|v| v.bound == "serverbound")
        .map(|v| {
            let state_ident = &v.state_ident;
            let id = v.id;
            let variant_ident = &v.variant_ident;
            let packet_type = &v.packet_type;
            quote! {
                (minecraft_protocol::State::#state_ident, #id) => {
                    let packet = raw.decode::<#packet_type>()?;
                    return Ok(Self::#variant_ident(packet));
                }
            }
        });

    let encode_arms = variant_infos
        .iter()
        .filter(|v| v.bound == "clientbound")
        .map(|v| {
            let variant_ident = &v.variant_ident;
            let id = v.id;
            quote! {
                Self::#variant_ident(packet) => {
                    let mut writer = yoki_binutils::writer::PacketWriter::new();
                    minecraft_packet::OutgoingPacket::encode_payload(&packet, &mut writer)?;
                    return Ok(minecraft_packet::RawPacket {
                        id: #id,
                        payload: writer.into_inner(),
                    });
                }
            }
        });

    let id_arms = variant_infos.iter().map(|v| {
        let variant_ident = &v.variant_ident;
        let state_ident = &v.state_ident;
        let id = v.id;
        quote! {
            Self::#variant_ident(_) => (minecraft_protocol::State::#state_ident, #id),
        }
    });

    Ok(quote! {
        impl #enum_ident {
            pub fn decode_serverbound(
                state: minecraft_protocol::State,
                raw: &minecraft_packet::RawPacket,
            ) -> Result<Self, yoki_binutils::ProtocolError> {
                match (state, raw.id) {
                    #(#decode_arms)*
                    (state, id) => Err(yoki_binutils::ProtocolError::UnknownPacket {
                        id,
                        conn: Some(state),
                    }),
                }
            }

            pub fn encode_clientbound(self) -> Result<minecraft_packet::RawPacket, yoki_binutils::ProtocolError> {
                match self {
                    #(#encode_arms)*
                    _ => Err(yoki_binutils::ProtocolError::UnknownPacket {
                        id: -1,
                        conn: None,
                    }),
                }
            }

            pub fn state_and_id(&self) -> (minecraft_protocol::State, i32) {
                match self {
                    #(#id_arms)*
                }
            }
        }
    })
}
