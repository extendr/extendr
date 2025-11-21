use proc_macro2::Ident;
use quote::quote;
use syn::{Attribute, Expr, ExprLit, Type};

/// Collect a single doc string from a list of attributes by concatenating
/// the individual `///` lines together separated by newlines.
pub(crate) fn doc_string(attrs: &[Attribute]) -> String {
    let mut res = String::new();
    for attr in attrs {
        if !attr.path().is_ident("doc") {
            continue;
        }

        if let syn::Meta::NameValue(ref nv) = attr.meta {
            if let Expr::Lit(ExprLit {
                lit: syn::Lit::Str(ref litstr),
                ..
            }) = nv.value
            {
                if !res.is_empty() {
                    res.push('\n');
                }
                res.push_str(&litstr.value());
            }
        }
    }
    res
}

/// Produce a simplified type name that is meaningful to R. If the type path
/// is too complex, fall back to a mangled representation.
pub(crate) fn type_name(type_: &Type) -> String {
    match type_ {
        Type::Path(syn::TypePath { path, .. }) => {
            if let Some(ident) = path.get_ident() {
                ident.to_string()
            } else if path.segments.len() == 1 {
                let seg = path.segments.clone().into_iter().next().unwrap();
                seg.ident.to_string()
            } else {
                mangled_type_name(type_)
            }
        }
        Type::Group(syn::TypeGroup { elem, .. }) => type_name(elem),
        Type::Reference(syn::TypeReference { elem, .. }) => type_name(elem),
        Type::Paren(syn::TypeParen { elem, .. }) => type_name(elem),
        Type::Ptr(syn::TypePtr { elem, .. }) => type_name(elem),
        _ => mangled_type_name(type_),
    }
}

pub(crate) fn mangled_type_name(type_: &Type) -> String {
    let src = quote!( #type_ ).to_string();
    let mut res = String::new();
    for c in src.chars() {
        if c != ' ' {
            if c.is_alphanumeric() {
                res.push(c)
            } else {
                let f = format!("_{:02x}", c as u32);
                res.push_str(&f);
            }
        }
    }
    res
}

/// Remove the raw identifier prefix (`r#`) from an [`Ident`].
pub(crate) fn sanitize_identifier(ident: Ident) -> Ident {
    static PREFIX: &str = "r#";
    let (ident, span) = (ident.to_string(), ident.span());
    let ident = match ident.strip_prefix(PREFIX) {
        Some(ident) => ident.into(),
        None => ident,
    };

    Ident::new(&ident, span)
}

/// Extract a named string literal attribute (e.g. `#[default = "foo"]`) and
/// remove it from the attribute list in place.
pub(crate) fn take_string_literal_attr(attrs: &mut Vec<Attribute>, name: &str) -> Option<String> {
    let mut new_attrs = Vec::new();
    let mut res = None;
    for a in attrs.drain(..) {
        if let syn::Meta::NameValue(ref nv) = a.meta {
            if nv.path.is_ident(name) {
                if let Expr::Lit(ExprLit {
                    lit: syn::Lit::Str(ref litstr),
                    ..
                }) = nv.value
                {
                    res = Some(litstr.value());
                    continue;
                }
            }
        }

        new_attrs.push(a);
    }
    *attrs = new_attrs;
    res
}
