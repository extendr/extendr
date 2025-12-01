use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

pub(crate) fn rslice_native(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let ident = ast.ident.clone();
    let field_ty = match ast.data {
        Data::Struct(ref data) => match &data.fields {
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                fields.unnamed.first().unwrap().ty.clone()
            }
            Fields::Named(fields) if fields.named.len() == 1 => {
                fields.named.first().unwrap().ty.clone()
            }
            _ => {
                return syn::Error::new_spanned(
                    ast,
                    "`RSliceNative` derive only supports newtype structs with a single field",
                )
                .to_compile_error()
                .into()
            }
        },
        _ => {
            return syn::Error::new_spanned(
                ast,
                "`RSliceNative` derive is only available for structs",
            )
            .to_compile_error()
            .into()
        }
    };

    TokenStream::from(quote! {
        impl extendr_api::robj::into_robj::RNativeType for #ident {
            const SEXPTYPE: extendr_api::SEXPTYPE =
                <#field_ty as extendr_api::robj::into_robj::RNativeType>::SEXPTYPE;
            const PTR: unsafe extern "C" fn(extendr_api::SEXP) -> *mut Self = |sexp| {
                <#field_ty as extendr_api::robj::into_robj::RNativeType>::PTR(sexp).cast()
            };
        }

        impl extendr_api::robj::into_robj::RSliceNative for #ident {}
    })
}
