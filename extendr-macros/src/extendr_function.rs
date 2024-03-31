use crate::wrappers;
use proc_macro::TokenStream;
use quote::quote;
use syn::{meta::ParseNestedMeta, ItemFn, Lit, LitBool};

/// Generate bindings for a single function.
pub fn extendr_function(mut func: ItemFn, opts: &wrappers::ExtendrOptions) -> TokenStream {
    let mut wrappers: Vec<ItemFn> = Vec::new();

    let res =
        wrappers::make_function_wrappers(opts, &mut wrappers, "", &func.attrs, &mut func.sig, None);
    if let Err(e) = res {
        return e.into_compile_error().into();
    };

    TokenStream::from(quote! {
        #func

        # ( #wrappers )*
    })
}

impl wrappers::ExtendrOptions {
    /// Parse a set of attribute arguments for `#[extendr(opts...)]`
    ///
    /// Supported options:
    ///
    /// - `use_try_from = bool` which uses `TryFrom<Robj>` for argument conversions.
    /// - `r_name = "name"` which specifies the name of the wrapper on the R-side.
    /// - `use_rng = bool` ensures the RNG-state is pulled and pushed
    ///
    pub fn parse(&mut self, meta: ParseNestedMeta) -> syn::parse::Result<()> {
        let value = meta.value()?;
        let path = meta
            .path
            .get_ident()
            .ok_or(meta.error("Unexpected syntax"))?;

        match path.to_string().as_str() {
            "use_try_from" => {
                if let Ok(LitBool { value, .. }) = value.parse() {
                    self.use_try_from = value;
                    Ok(())
                } else {
                    Err(value.error("`use_try_from` must be `true` or `false`"))
                }
            }
            "r_name" => {
                if let Ok(Lit::Str(litstr)) = value.parse() {
                    self.r_name = Some(litstr.value());
                    Ok(())
                } else {
                    Err(value.error("`r_name` must be a string literal"))
                }
            }
            "mod_name" => {
                if let Ok(Lit::Str(litstr)) = value.parse() {
                    self.mod_name = Some(litstr.value());
                    Ok(())
                } else {
                    Err(value.error("`mod_name` must be a string literal"))
                }
            }
            "use_rng" => {
                if let Ok(LitBool { value, .. }) = value.parse() {
                    self.use_rng = value;
                    Ok(())
                } else {
                    Err(value.error("`use_rng` must be `true` or `false`"))
                }
            }
            _ => Err(syn::Error::new_spanned(meta.path, "Unexpected key")),
        }
    }
}
