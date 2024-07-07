use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, punctuated::Punctuated, Expr, Token};

#[allow(non_snake_case)]
pub fn R(item: TokenStream, expand_params: bool) -> TokenStream {
    // Check if the input is a string.
    let lit = match syn::parse2::<syn::LitStr>(item.clone()) {
        Ok(lit) => lit,
        Err(_) => {
            // If not a string, expand the tokens to make a string.
            let src = format!("{}", item);
            return quote!(extendr_api::functions::eval_string(#src));
        }
    };

    let mut src = lit.value();

    let mut expressions: Punctuated<Expr, Token!(,)> = Punctuated::new();
    if expand_params {
        // Replace rust expressions in {{..}} with _expr0, _expr1, ...
        while let Some(start) = src.find("{{") {
            if let Some(end) = src[start + 2..].find("}}") {
                if let Ok(param) = syn::parse_str::<Expr>(&src[start + 2..start + 2 + end]) {
                    src = format!(
                        "{} param.{} {}",
                        &src[0..start],
                        expressions.len(),
                        &src[start + 2 + end + 2..]
                    );
                    expressions.push(parse_quote!(&extendr_api::Robj::from(#param)));
                } else {
                    return quote!(compile_error!("Not a valid rust expression."));
                }
            } else {
                return quote!(compile_error!("Unterminated {{ block."));
            }
        }
    }

    if expressions.is_empty() {
        quote!(extendr_api::functions::eval_string(#src))
    } else {
        quote!(
            {
                let params = &[#expressions];
                extendr_api::functions::eval_string_with_params(#src, params)
            }
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_r_macro() {
        // Note: strip spaces to cover differences between compilers.

        // Naked R!
        assert_eq!(
            format!("{}", R(quote!(data.frame), true)),
            format!(
                "{}",
                quote!(extendr_api::functions::eval_string("data . frame"))
            )
        );

        // Quoted R!
        assert_eq!(
            format!("{}", R(quote!("data.frame"), true)),
            format!(
                "{}",
                quote!(extendr_api::functions::eval_string("data.frame"))
            )
        );

        // Param R!
        assert_eq!(
            format!("{}", R(quote!("a <- {{1}}"), true)),
            format!(
                "{}",
                quote!({
                    let params = &[&extendr_api::Robj::from(1)];
                    extendr_api::functions::eval_string_with_params("a <-  param.0 ", params)
                })
            )
        );

        // Unquoted R!
        assert_eq!(
            format!("{}", R(quote!(r#""hello""#), true)),
            format!(
                "{}",
                quote!(extendr_api::functions::eval_string("\"hello\""))
            )
        );

        // Rraw!
        assert_eq!(
            format!("{}", R(quote!("a <- {{1}}"), false)),
            format!(
                "{}",
                quote!(extendr_api::functions::eval_string("a <- {{1}}"))
            )
        );
    }
}
