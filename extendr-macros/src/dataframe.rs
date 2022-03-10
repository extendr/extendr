use quote::quote;
use syn::{parse_macro_input, DataStruct, DeriveInput, Data};
use proc_macro::TokenStream;

fn parse_struct(input: &DeriveInput, _datastruct: &DataStruct) -> TokenStream {
    let structname = &input.ident;
    quote! {
        impl<I : ExactSizeIterator<Item=#structname> + Clone> I {
            fn into_dataframe(&self) -> extendr_api::wrapper::List
            where
                I : ExactSizeIterator<Item=#structname> + Clone,
            {
                let iter = input.into_iter();
                let len = iter.len();
                data_frame!(
                    x = Integers::from_values(iter.clone().map(|r| r.x)),
                    y = Strings::from_values(iter.map(|r| r.y))
                )
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