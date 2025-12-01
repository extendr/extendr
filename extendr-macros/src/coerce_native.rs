use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

pub(crate) fn coerce_native(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let ident = ast.ident.clone();

    let (field_access, field_ty) = match ast.data {
        Data::Struct(ref data) => match &data.fields {
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => (
                quote! { self.0 },
                fields.unnamed.first().unwrap().ty.clone(),
            ),
            Fields::Named(fields) if fields.named.len() == 1 => {
                let name = fields.named.first().unwrap().ident.as_ref().unwrap();
                (
                    quote! { self.#name },
                    fields.named.first().unwrap().ty.clone(),
                )
            }
            _ => {
                return syn::Error::new_spanned(
                    ast,
                    "`CoerceNative` derive only supports newtype structs with a single field",
                )
                .to_compile_error()
                .into()
            }
        },
        _ => {
            return syn::Error::new_spanned(
                ast,
                "`CoerceNative` derive is only available for structs",
            )
            .to_compile_error()
            .into()
        }
    };

    TokenStream::from(quote! {
        impl extendr_api::robj::into_robj::CoerceNative for #ident {
            type Target =
                <#field_ty as extendr_api::robj::into_robj::CoerceNative>::Target;

            fn coerce(&self) -> Self::Target {
                extendr_api::robj::into_robj::CoerceNative::coerce(&#field_access)
            }
        }
    })
}
