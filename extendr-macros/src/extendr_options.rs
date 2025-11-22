use syn::{meta::ParseNestedMeta, Lit, LitBool};

#[derive(Debug, Default)]
pub(crate) struct ExtendrOptions {
    pub r_name: Option<String>,
    pub mod_name: Option<String>,
    pub use_rng: bool,
    pub invisible: Option<bool>,
    pub result_mode: Option<ResultMode>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ResultMode {
    Default,
    List,
    Condition,
}

impl ExtendrOptions {
    /// Parse a set of attribute arguments for `#[extendr(opts...)]`
    ///
    /// Supported options:
    ///
    /// - `r_name = "name"` which specifies the name of the wrapper on the R-side.
    /// - `use_rng = bool` ensures the RNG-state is pulled and pushed
    /// - `result = "list"|"condition"|"default"` overrides how `Result<T,E>` is sent over the FFI boundary
    ///
    pub fn parse(&mut self, meta: ParseNestedMeta) -> syn::parse::Result<()> {
        let path = meta
            .path
            .get_ident()
            .ok_or(meta.error("Unexpected syntax"))?;

        match path.to_string().as_str() {
            "invisible" => {
                self.invisible = Some(true);
                Ok(())
            }
            _ => {
                let value = meta.value()?;
                match path.to_string().as_str() {
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
                    "result" => {
                        if let Ok(Lit::Str(litstr)) = value.parse() {
                            let mode = match litstr.value().as_str() {
                                "list" => ResultMode::List,
                                "condition" => ResultMode::Condition,
                                "default" => ResultMode::Default,
                                other => {
                                    return Err(syn::Error::new_spanned(
                                        litstr,
                                        format!(
                                            "invalid result mode `{other}`; expected \"list\", \"condition\", or \"default\""
                                        ),
                                    ))
                                }
                            };
                            self.result_mode = Some(mode);
                            Ok(())
                        } else {
                            Err(value.error("`result` must be a string literal"))
                        }
                    }
                    _ => Err(syn::Error::new_spanned(meta.path, "Unexpected key")),
                }
            }
        }
    }
}
