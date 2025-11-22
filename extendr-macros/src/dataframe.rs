use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput};

fn parse_struct(input: &DeriveInput, datastruct: &DataStruct) -> TokenStream {
    let structname = &input.ident;
    let mut a: Vec<syn::Ident> = Vec::new();
    let mut a_names = Vec::new();
    for f in &datastruct.fields {
        let ident = f
            .ident
            .clone()
            .expect("struct fields must be named for IntoDataFrameRow");
        a_names.push(syn::LitStr::new(&ident.to_string(), ident.span()));
        a.push(ident);
    }
    quote! {
        impl extendr_api::wrapper::IntoDataFrameRow<#structname> for Vec<#structname>
        {
            fn into_dataframe(self) -> extendr_api::Result<extendr_api::wrapper::Dataframe<#structname>> {
                #(let mut #a = Vec::with_capacity(self.len());)*
                for val in self {
                    #(#a.push(val.#a);)*
                }
                let caller = extendr_api::functions::eval_string("data.frame")?;
                let res = extendr_api::wrapper::Pairlist::from_pairs(&[
                    #((#a_names, extendr_api::robj::Robj::from(#a))),*
                ]);
                caller.call(res)?.try_into()
            }
        }

        impl<I> extendr_api::wrapper::IntoDataFrameRow<#structname> for (I,)
        where
            I : ExactSizeIterator<Item=#structname>,
        {
            /// Thanks to RFC 2451, we need to wrap a generic iterator in a tuple!
            fn into_dataframe(self) -> extendr_api::Result<extendr_api::wrapper::Dataframe<#structname>> {
                #(let mut #a = Vec::with_capacity(self.0.len());)*
                for val in self.0 {
                    #(#a.push(val.#a);)*
                }
                let caller = extendr_api::functions::eval_string("data.frame")?;
                let res = caller.call(extendr_api::wrapper::Pairlist::from_pairs(&[
                    #((#a_names, extendr_api::robj::Robj::from(#a))),*
                ]))?;
                res.try_into()
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
