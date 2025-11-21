use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

use crate::pairs::{Pair, Pairs};

pub fn list(item: TokenStream) -> TokenStream {
    let list = parse_macro_input!(item as Pairs);

    if list.is_empty() {
        return TokenStream::from(quote!(extendr_api::wrapper::List::default()));
    }

    let (names, values): (Vec<_>, Vec<_>) = list
        .iter()
        .map(|pair| match pair {
            Pair::Named { name, value } => {
                (Some(quote!(#name)), quote!(extendr_api::Robj::from(#value)))
            }
            Pair::Unnamed(value) => (None, quote!(extendr_api::Robj::from(#value))),
        })
        .unzip();

    let any_names = names.iter().any(|name| name.is_some());
    let values = values;

    if any_names {
        let names = names
            .into_iter()
            .map(|name| name.unwrap_or_else(|| quote!("")))
            .collect::<Vec<_>>();
        TokenStream::from(quote!(
            extendr_api::List::from_names_and_values(&[#(#names),*], &[#(#values),*]).unwrap()
        ))
    } else {
        TokenStream::from(quote!(
            extendr_api::List::from_values(&[#(#values),*])
        ))
    }
}
