use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

pub(crate) fn canbena(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let ident = ast.ident.clone();
    let (field_ident, construct_na) = match ast.data {
        Data::Struct(ref data) => match &data.fields {
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => (
                quote! { self.0 },
                quote! { Self(extendr_api::CanBeNA::na()) },
            ),
            Fields::Named(fields) if fields.named.len() == 1 => {
                let name = fields.named.first().unwrap().ident.as_ref().unwrap();
                (
                    quote! { self.#name },
                    quote! { Self { #name: extendr_api::CanBeNA::na() } },
                )
            }
            _ => {
                return syn::Error::new_spanned(
                    ast,
                    "`CanBeNA` derive only supports newtype structs with a single field",
                )
                .to_compile_error()
                .into()
            }
        },
        _ => {
            return syn::Error::new_spanned(ast, "`CanBeNA` derive is only available for structs")
                .to_compile_error()
                .into()
        }
    };

    TokenStream::from(quote! {
        impl extendr_api::CanBeNA for #ident {
            fn is_na(&self) -> bool {
                extendr_api::CanBeNA::is_na(&#field_ident)
            }

            fn na() -> Self {
                #construct_na
            }
        }
    })
}
