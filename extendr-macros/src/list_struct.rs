use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

/// Implementation of the TryFromList macro. Refer to the documentation there
pub fn derive_try_from_list(item: TokenStream) -> syn::parse::Result<TokenStream> {
    // Parse the tokens into a Struct
    let ast = syn::parse::<DeriveInput>(item)?;
    let inside = if let Data::Struct(inner) = ast.data {
        inner
    } else {
        return Err(syn::Error::new_spanned(ast, "Only struct is supported"));
    };
    let struct_name = ast.ident;
    let struct_name_str = struct_name.to_string();

    // Iterate each struct field and capture a conversion from Robj for each field
    let mut tokens = Vec::<_>::with_capacity(inside.fields.len());
    let is_tuple_struct = inside
        .fields
        .iter()
        .next()
        .map(|x| x.ident.is_none())
        .unwrap_or(false);
    for (id_field, field) in inside.fields.iter().enumerate() {
        if is_tuple_struct {
            let field_name = syn::Index::from(id_field);
            let field_str = format!(".{id_field}");
            // This is like `value[[id_field]]` in R
            tokens.push(quote!(
                #field_name: value
                    .elt(#id_field)?
                    .try_into()
                    .map_err(|error| extendr_api::error::Error::Other(format!(
                        "failed to convert tuple field `{}` on `{}`: {}",
                        #field_str,
                        #struct_name_str,
                        error
                    )))?
            ));
        } else {
            let field_name = field.ident.as_ref().unwrap();
            let field_str = field_name.to_string();
            // This is like `value$foo` in R
            tokens.push(quote!(
                #field_name: value
                    .dollar(#field_str)?
                    .try_into()
                    .map_err(|error| extendr_api::error::Error::Other(format!(
                        "failed to convert field `{}` on `{}`: {}",
                        #field_str,
                        #struct_name_str,
                        error
                    )))?
            ));
        }
    }

    // Emit the conversion trait impl
    Ok(TokenStream::from(quote!(
        impl std::convert::TryFrom<&extendr_api::Robj> for #struct_name {
            type Error = extendr_api::Error;

            fn try_from(value: &extendr_api::Robj) -> extendr_api::Result<Self> {
                let value: List = value.try_into()?;
                Ok(#struct_name {
                    #(#tokens),*
                })
            }
        }

        impl std::convert::TryFrom<extendr_api::Robj> for #struct_name {
            type Error = extendr_api::Error;

            fn try_from(value: extendr_api::Robj) -> extendr_api::Result<Self> {
                let value: List = value.try_into()?;
                Ok(#struct_name {
                    #(#tokens),*
                })
            }
        }
    )))
}

/// Implementation of the `IntoList` macro. Refer to the documentation there
pub fn derive_into_list(item: TokenStream) -> syn::parse::Result<TokenStream> {
    // Parse the tokens into a Struct
    let ast = syn::parse::<DeriveInput>(item)?;
    let inside = if let Data::Struct(inner) = ast.data {
        inner
    } else {
        return Err(syn::Error::new_spanned(ast, "Only `struct` is supported"));
    };
    let struct_name = ast.ident;

    // Iterate each struct field and capture a token that creates a KeyValue pair (tuple) for
    // each field
    let mut tokens = Vec::<_>::with_capacity(inside.fields.len());

    for (id_field, field) in inside.fields.iter().enumerate() {
        let mut ignore = false;

        let field_attributes = &field.attrs;
        for attrib in field_attributes {
            if !attrib.path().is_ident("into_list") {
                continue;
            }
            let ignore_flag: syn::Meta = attrib.parse_args()?;
            match ignore_flag {
                syn::Meta::Path(path) => {
                    if path.is_ident("ignore") {
                        ignore = true;
                    }
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        ignore_flag,
                        "unrecognized attribute for `IntoList`",
                    ))
                }
            }
        }

        if ignore {
            continue;
        }

        let is_tuple_struct = field.colon_token.is_none();
        if is_tuple_struct {
            let dot_field_name = format!(".{id_field}");
            let id_field_index = syn::Index::from(id_field);
            tokens.push(quote!(
                (#dot_field_name, (&value.#id_field_index).into())
            ));
        } else {
            let field_name = field.ident.as_ref().unwrap();
            let field_str = field_name.to_string();
            tokens.push(quote!(
                (#field_str, (&value.#field_name).into())
            ));
        }
    }

    // The only thing we emit from this macro is the conversion trait impl
    Ok(TokenStream::from(quote!(
        impl std::convert::From<&#struct_name> for extendr_api::Robj {
            fn from(value: &#struct_name) -> Self {
                extendr_api::List::from_pairs([#(#tokens),*]).into()
            }
        }
        impl std::convert::From<#struct_name> for extendr_api::Robj {
            fn from(value: #struct_name) -> Self {
                (&value).into()
            }
        }
    )))
}
