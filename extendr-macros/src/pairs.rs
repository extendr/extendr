//! Internal module for parsing R-like variadic arguments.

use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Expr, ExprAssign, ExprPath, Token,
};

#[derive(Debug)]
pub(crate) struct Pairs {
    pub(crate) pairs: Punctuated<Expr, Token![,]>,
}

#[derive(Debug)]
pub(crate) enum Pair<'a> {
    Named { name: String, value: &'a Expr },
    Unnamed(&'a Expr),
}

impl Parse for Pairs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            pairs: input.parse_terminated(Expr::parse, Token![,])?,
        })
    }
}

impl Pairs {
    pub(crate) fn iter(&self) -> impl Iterator<Item = Pair<'_>> {
        self.pairs.iter().map(|expr| {
            if let Expr::Assign(ExprAssign { left, right, .. }) = expr {
                if let Expr::Path(ExprPath { path, .. }) = &**left {
                    if let Some(ident) = path.get_ident() {
                        return Pair::Named {
                            name: ident.to_string(),
                            value: right.as_ref(),
                        };
                    }
                }
            }
            Pair::Unnamed(expr)
        })
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }
}
