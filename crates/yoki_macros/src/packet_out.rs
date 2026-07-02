use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

use crate::attrs::generate_field_encode;

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
            const DIRECTION: crate::packet::PacketDirection = crate::packet::PacketDirection::Out;
        }
    };

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
            fn encode_payload(&self, writer: &mut yoki_binutils::BinaryWriter) -> Result<(), yoki_binutils::ProtocolError> {
                #encode_body
            }
        }
    })
}
