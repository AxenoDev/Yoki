use proc_macro2::TokenStream as TokenStream2;
use syn::spanned::Spanned;
use syn::{Field, Ident, LitInt, LitStr, Type};

pub fn parse_packet_id(input: &syn::DeriveInput) -> syn::Result<i32> {
    for attr in &input.attrs {
        if !attr.path().is_ident("packet") {
            continue;
        }

        let mut id = None;

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("id") {
                let value: LitInt = meta.value()?.parse()?;
                id = Some(lit_int_to_i32(&value)?);
            }
            Ok(())
        })?;

        if let Some(id) = id {
            return Ok(id);
        }
    }

    Err(syn::Error::new(
        input.ident.span(),
        "missing #[packet(id = ...)] attribute",
    ))
}

pub fn has_packet_flag(input: &syn::DeriveInput, flag: &str) -> bool {
    input.attrs.iter().any(|attr| {
        if !attr.path().is_ident("packet") {
            return false;
        }

        let mut found = false;
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident(flag) {
                found = true;
            }
            Ok(())
        });
        found
    })
}

pub fn lit_int_to_i32(value: &LitInt) -> syn::Result<i32> {
    let s = value.to_string();
    if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        i32::from_str_radix(hex, 16)
            .map_err(|_| syn::Error::new(value.span(), "invalid hex packet id"))
    } else {
        s.parse()
            .map_err(|_| syn::Error::new(value.span(), "invalid packet id"))
    }
}

pub fn field_has_protocol_flag(field: &Field, flag: &str) -> bool {
    field.attrs.iter().any(|attr| {
        if !attr.path().is_ident("protocol") {
            return false;
        }

        let mut found = false;
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident(flag) {
                found = true;
            }
            Ok(())
        });
        found
    })
}

pub fn field_present_if(field: &Field) -> Option<Ident> {
    for attr in &field.attrs {
        if attr.path().is_ident("present_if") {
            if let Ok(value) = attr.parse_args::<LitStr>() {
                return Some(Ident::new(&value.value(), value.span()));
            }
        }
    }

    None
}

pub fn is_vec_u8(ty: &Type) -> bool {
    let Type::Path(path) = ty else {
        return false;
    };

    let Some(segment) = path.path.segments.last() else {
        return false;
    };

    if segment.ident != "Vec" {
        return false;
    }

    let syn::PathArguments::AngleBracketed(args) = &segment.arguments else {
        return false;
    };

    matches!(
        args.args.first(),
        Some(syn::GenericArgument::Type(Type::Path(inner))) if inner.path.is_ident("u8")
    )
}

pub fn generate_field_decode(field: &Field) -> TokenStream2 {
    let name = &field.ident;
    let ty = &field.ty;

    if field_has_protocol_flag(field, "skip") {
        return quote::quote! { #name: ::core::default::Default::default(), };
    }

    if field_has_protocol_flag(field, "remaining") {
        return quote::quote! { #name: reader.read_remaining_bytes(), };
    }

    if field_has_protocol_flag(field, "remaining_option") {
        return quote::quote! { #name: Some(reader.read_remaining_bytes()), };
    }

    if let Some(present_field) = field_present_if(field) {
        return quote::quote! {
            #name: if #present_field {
                reader.read_remaining_bytes()
            } else {
                ::std::vec::Vec::new()
            },
        };
    }

    if is_vec_u8(ty) {
        return quote::quote! { #name: reader.read_byte_array()?, };
    }

    quote::quote! {
        #name: <#ty as yoki_binutils::ProtocolRead>::read_from(reader)?,
    }
}

pub fn generate_field_encode(field: &Field) -> TokenStream2 {
    let name = &field.ident;
    let ty = &field.ty;

    if field_has_protocol_flag(field, "skip") {
        return TokenStream2::new();
    }

    if field_has_protocol_flag(field, "remaining") {
        return quote::quote! { writer.extend(&self.#name); };
    }

    if field_has_protocol_flag(field, "remaining_option") {
        return quote::quote! {
            if let Some(ref bytes) = self.#name {
                writer.extend(bytes);
            }
        };
    }

    if let Some(present_field) = field_present_if(field) {
        return quote::quote! {
            if #present_field {
                writer.extend(&self.#name);
            }
        };
    }

    if is_vec_u8(ty) {
        return quote::quote! { writer.write_byte_array(&self.#name); };
    }

    quote::quote! { yoki_binutils::ProtocolWrite::write_to(&self.#name, writer)?; }
}

pub fn parse_protocol_id_attr(attrs: &[syn::Attribute]) -> syn::Result<ProtocolIdAttr> {
    let attr = attrs
        .iter()
        .find(|a| a.path().is_ident("protocol_id"))
        .ok_or_else(|| {
            syn::Error::new(
                proc_macro2::Span::call_site(),
                "missing #[protocol_id] attribute",
            )
        })?;

    let mut state = None;
    let mut bound = None;
    let mut id = None;

    attr.parse_nested_meta(|meta| {
        if meta.path.is_ident("state") {
            state = Some(meta.value()?.parse::<LitStr>()?);
        } else if meta.path.is_ident("bound") {
            bound = Some(meta.value()?.parse::<LitStr>()?);
        } else if meta.path.is_ident("id") {
            id = Some(lit_int_to_i32(&meta.value()?.parse::<LitInt>()?)?);
        }
        Ok(())
    })?;

    Ok(ProtocolIdAttr {
        state: state.ok_or_else(|| syn::Error::new(attr.span(), "missing state in protocol_id"))?,
        bound: bound.ok_or_else(|| syn::Error::new(attr.span(), "missing bound in protocol_id"))?,
        id: id.ok_or_else(|| syn::Error::new(attr.span(), "missing id in protocol_id"))?,
    })
}

pub struct ProtocolIdAttr {
    pub state: LitStr,
    pub bound: LitStr,
    pub id: i32,
}

pub fn state_str_to_ident(state: &str) -> syn::Result<Ident> {
    let ident = match state {
        "handshake" | "handshaking" => "Handshaking",
        "status" => "Status",
        "login" => "Login",
        "configuration" => "Configuration",
        "play" => "Play",
        "transfer" => "Transfer",
        other => {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("unknown state: {other}"),
            ));
        }
    };

    Ok(Ident::new(ident, proc_macro2::Span::call_site()))
}

pub fn packet_type_from_variant_field(ty: &Type) -> syn::Result<syn::Path> {
    match ty {
        Type::Path(type_path) => Ok(type_path.path.clone()),
        _ => Err(syn::Error::new_spanned(
            ty,
            "expected a path type for the packet struct",
        )),
    }
}
