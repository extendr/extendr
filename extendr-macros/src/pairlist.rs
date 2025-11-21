use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, Expr};

use crate::pairs::{Pair, Pairs};

pub fn pairlist(item: TokenStream) -> TokenStream {
    let pairlist = parse_macro_input!(item as Pairs);
    let pairs = pairlist
        .iter()
        .map(|pair| match pair {
            Pair::Named { name, value } => parse_quote!((#name, extendr_api::Robj::from(#value))),
            Pair::Unnamed(expr) => parse_quote!(("", extendr_api::Robj::from(#expr))),
        })
        .collect::<Vec<Expr>>();

    match pairs.is_empty() {
        true => TokenStream::from(quote!(extendr_api::wrapper::Pairlist::from(()))),
        false => TokenStream::from(quote!(
            extendr_api::wrapper::Pairlist::from_pairs(&[# ( #pairs ),*])
        )),
    }
}
