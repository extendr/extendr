//! Internal module for parsing R-like variadic arguments.

use syn::{parse::ParseStream, punctuated::Punctuated, Expr, ExprAssign, ExprPath, Token};

#[derive(Debug)]
pub(crate) struct Pairs {
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

impl Pairs {
    // Having parsed a variadic expression, extract the names and values.
    pub(crate) fn names_and_values(&self) -> Vec<(String, &Expr)> {
        self.pairs
            .iter()
            .map(|e| -> (String, &Expr) {
                if let Expr::Assign(ExprAssign { left, right, .. }) = e {
                    if let Expr::Path(ExprPath { path, .. }) = &**left {
                        let s = path.get_ident().unwrap().to_string();
                        return (s, right.as_ref());
                    }
                }
                ("".to_owned(), e)
            })
            .collect()
    }
}
