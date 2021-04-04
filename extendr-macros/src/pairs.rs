use syn::{parse::ParseStream, punctuated::Punctuated, Expr, Token};

#[derive(Debug)]
pub struct Pairs {
    pub pairs: Punctuated<Expr, Token![,]>,
}

// Custom parser for a pairlist (a=1, 2, 3) or (1, 2, 3) or (1, b=2, c=3) etc.
impl syn::parse::Parse for Pairs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut res = Self {
            pairs: Punctuated::new(),
        };
        while !input.is_empty() {
            res.pairs.push(input.parse::<Expr>()?);
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
        }
        Ok(res)
    }
}
