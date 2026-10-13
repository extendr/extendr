use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Ident};

fn parse_struct(input: &DeriveInput, datastruct: &DataStruct) -> TokenStream {
    let structname = &input.ident;
    let mut a = Vec::new();
    for f in &datastruct.fields {
        a.push(f.ident.clone());
    }
    let row = Ident::new("row", Span::mixed_site());
    let caller = Ident::new("caller", Span::mixed_site());
    let res = Ident::new("res", Span::mixed_site());
    quote! {
        impl extendr_api::wrapper::IntoDataFrameRow<#structname> for Vec<#structname>
        {
            fn into_dataframe(self) -> extendr_api::Result<extendr_api::wrapper::Dataframe<#structname>> {
                #(let mut #a = Vec::with_capacity(self.len());)*
                for #row in self {
                    #(#a.push(#row.#a);)*
                }
                let #caller = extendr_api::functions::eval_string("data.frame")?;
                let #res = #caller.call(extendr_api::wrapper::Pairlist::from_pairs(&[
                    #((stringify!(#a), extendr_api::robj::Robj::from(#a))),*
                ]))?;
                #res.try_into()
            }
        }

        impl<I> extendr_api::wrapper::IntoDataFrameRow<#structname> for (I,)
        where
            I : ExactSizeIterator<Item=#structname>,
        {
            /// Thanks to RFC 2451, we need to wrap a generic iterator in a tuple!
            fn into_dataframe(self) -> extendr_api::Result<extendr_api::wrapper::Dataframe<#structname>> {
                #(let mut #a = Vec::with_capacity(self.0.len());)*
                for #row in self.0 {
                    #(#a.push(#row.#a);)*
                }
                let #caller = extendr_api::functions::eval_string("data.frame")?;
                let #res = #caller.call(extendr_api::wrapper::Pairlist::from_pairs(&[
                    #((stringify!(#a), extendr_api::robj::Robj::from(#a))),*
                ]))?;
                #res.try_into()
            }
        }
    }
    .into()
}

pub fn derive_into_dataframe(item: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(item as DeriveInput);

    match &input.data {
        Data::Struct(datastruct) => parse_struct(&input, datastruct),
        _ => quote!(compile_error("IntoDataFrameRow expected a struct.")).into(),
    }
}
