use quote::quote;
use syn::{parse_macro_input, DataStruct, DeriveInput, Data};
use proc_macro::TokenStream;

fn parse_struct(input: &DeriveInput, datastruct: &DataStruct) -> TokenStream {
    #![allow(non_snake_case)]
    let structname = &input.ident;
    let mut A = Vec::new();
    let mut a = Vec::new();
    for f in &datastruct.fields {
        A.push(f.ty.clone());
        a.push(f.ident.clone());
    }
    quote! {
        impl IntoDataframe<#structname> for Vec<#structname>
        {
            fn into_dataframe(self) -> Result<Dataframe<#structname>> {
                #(let mut #a = Vec::new();)*
                for val in self {
                    #(#a.push(val.#a);)*
                }
                let caller = eval_string("data.frame")?;
                let res = caller.call(Pairlist::from_pairs(&[
                    #((stringify!(#a), extendr_api::robj::Robj::from(#a))),*
                ]))?;
                res.try_into()
            }
        }
    }.into()
}

pub fn derive_into_dataframe(item: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(item as DeriveInput);

    match &input.data {
        Data::Struct(datastruct) => parse_struct(&input, datastruct),
        _ => quote!(compile_error("IntoDataframe expected a struct.")).into(),
    }
}
