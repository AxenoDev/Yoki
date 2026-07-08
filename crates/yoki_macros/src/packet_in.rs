use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

use crate::attrs::generate_field_decode;

pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_derive(&input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

fn expand_derive(input: &DeriveInput) -> syn::Result<TokenStream2> {
    let name = &input.ident;

    let meta_impl = quote! {
        impl crate::packet::PacketMeta for #name {
            const DIRECTION: crate::packet::PacketDirection = crate::packet::PacketDirection::In;
        }
    };

    let decode_body = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                let field_reads = fields.named.iter().map(generate_field_decode);
                quote! {
                    Ok(Self {
                        #(#field_reads)*
                    })
                }
            }
            Fields::Unit => quote! { Ok(Self) },
            Fields::Unnamed(_) => {
                return Err(syn::Error::new_spanned(
                    &input.ident,
                    "tuple structs are not supported by PacketIn",
                ));
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                &input.ident,
                "PacketIn can only be derived for structs",
            ));
        }
    };

    Ok(quote! {
        #meta_impl

        impl crate::packet::IncomingPacket for #name {
            fn decode_payload(reader: &mut yoki_binutils::BinaryReader<'_>) -> Result<Self, yoki_binutils::ProtocolError> {
                #decode_body
            }
        }
    })
}
