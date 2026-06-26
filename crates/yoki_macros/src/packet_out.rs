use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

use crate::attrs::{generate_field_encode, has_packet_flag, parse_packet_id};

pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_derive(&input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

fn expand_derive(input: &DeriveInput) -> syn::Result<TokenStream2> {
    let name = &input.ident;
    let id = parse_packet_id(input)?;

    let meta_impl = quote! {
        impl crate::packet::PacketMeta for #name {
            const ID: i32 = #id;
            const DIRECTION: crate::packet::PacketDirection = crate::packet::PacketDirection::Out;
        }
    };

    if has_packet_flag(input, "manual") {
        return Ok(meta_impl);
    }

    let encode_body = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                let field_writes = fields.named.iter().map(generate_field_encode);
                quote! {
                    #(
                        #field_writes
                    )*
                    Ok(())
                }
            }
            Fields::Unit => quote! { Ok(()) },
            Fields::Unnamed(_) => {
                return Err(syn::Error::new_spanned(
                    &input.ident,
                    "tuple structs are not supported by PacketOut",
                ));
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                &input.ident,
                "PacketOut can only be derived for structs",
            ));
        }
    };

    Ok(quote! {
        #meta_impl

        impl crate::packet::OutgoingPacket for #name {
            fn encode_payload(&self, writer: &mut yoki_binutils::writer::PacketWriter) -> Result<(), yoki_binutils::ProtocolError> {
                #encode_body
            }
        }
    })
}
