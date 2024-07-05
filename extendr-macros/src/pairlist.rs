use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, Expr, ExprAssign, ExprPath};

use crate::pairs::Pairs;

pub fn pairlist(item: TokenStream) -> TokenStream {
    let pairlist = parse_macro_input!(item as Pairs);
    let pairs = pairlist
        .pairs
        .iter()
        .map(|e| {
            if let Expr::Assign(ExprAssign { left, right, .. }) = e {
                if let Expr::Path(ExprPath { path, .. }) = &**left {
                    let s = path.get_ident().unwrap().to_string();
                    return parse_quote!( (#s, extendr_api::Robj::from(#right)) );
                }
            }
            parse_quote!( ("", extendr_api::Robj::from(#e)) )
        })
        .collect::<Vec<Expr>>();

    if pairs.is_empty() {
        TokenStream::from(quote!(extendr_api::wrapper::Pairlist::from(())))
    } else {
        TokenStream::from(quote!(
            extendr_api::wrapper::Pairlist::from_pairs(&[# ( #pairs ),*])
        ))
    }
}
