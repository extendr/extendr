use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::ParseStream, punctuated::Punctuated, Token};
use syn::{parse_macro_input, parse_quote, Expr, ExprAssign, ExprPath, LitStr};

#[derive(Debug)]
struct Call {
    caller: LitStr,
    pairs: Punctuated<Expr, Token![,]>,
}

// Custom parser for a call eg. call!("xyz", a=1, b, c)
impl syn::parse::Parse for Call {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut res = Self {
            caller: input.parse::<LitStr>()?,
            pairs: Punctuated::new(),
        };

        while !input.is_empty() {
            input.parse::<Token![,]>()?;
            res.pairs.push(input.parse::<Expr>()?);
        }
        Ok(res)
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
        .map(|e| {
            if let Expr::Assign(ExprAssign { left, right, .. }) = e {
                if let Expr::Path(ExprPath { path, .. }) = &**left {
                    if let Some(ident) = path.get_ident() {
                        let s = ident.to_string();
                        return parse_quote!( (#s, extendr_api::Robj::from(#right)) );
                    }
                }
            }
            parse_quote!( ("", extendr_api::Robj::from(#e)) )
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
