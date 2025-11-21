use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::ParseStream, punctuated::Punctuated, Token};
use syn::{parse_macro_input, parse_quote, Expr, LitStr};

use crate::pairs::{Pair, Pairs};

#[derive(Debug)]
struct Call {
    caller: LitStr,
    pairs: Pairs,
}

// Custom parser for a call eg. call!("xyz", a=1, b, c)
impl syn::parse::Parse for Call {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let caller = input.parse::<LitStr>()?;
        let pairs = if input.is_empty() {
            Pairs {
                pairs: Punctuated::new(),
            }
        } else {
            input.parse::<Token![,]>()?;
            Pairs {
                pairs: input.parse_terminated(Expr::parse, Token![,])?,
            }
        };

        Ok(Self { caller, pairs })
    }
}

pub fn call(item: TokenStream) -> TokenStream {
    // Get a [Call] object from the input token stream.
    // This consists of a literal string followed by named or unnamed arguments
    // as in the pairlist macro.
    let call = parse_macro_input!(item as Call);

    // Convert the pairs into tuples of ("name", Robj::from(value))
    let pairs = call
        .pairs
        .iter()
        .map(|pair| match pair {
            Pair::Named { name, value } => parse_quote!((#name, extendr_api::Robj::from(#value))),
            Pair::Unnamed(expr) => parse_quote!(("", extendr_api::Robj::from(#expr))),
        })
        .collect::<Vec<Expr>>();

    // Use eval_string to convert the literal string into a callable object.
    let caller = &call.caller;
    let caller = quote!(extendr_api::functions::eval_string(#caller));

    // Use the "call" method of Robj to call the function or primitive.
    // This will error if the object is not callable.
    let res = if pairs.is_empty() {
        quote!(
            (#caller).and_then(|caller| caller.call(extendr_api::wrapper::Pairlist::new()))
        )
    } else {
        quote!(
            (#caller).and_then(|caller| caller.call(extendr_api::wrapper::Pairlist::from_pairs(&[# ( #pairs ),*])))
        )
    };

    TokenStream::from(res)
}
