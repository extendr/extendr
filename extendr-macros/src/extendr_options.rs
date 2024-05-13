use syn::{meta::ParseNestedMeta, Lit, LitBool};

#[derive(Debug)]
pub(crate) struct ExtendrOptions {
    #[deprecated]
    pub use_try_from: bool,
    pub r_name: Option<String>,
    pub mod_name: Option<String>,
    pub use_rng: bool,
}

impl Default for ExtendrOptions {
    fn default() -> Self {
        let use_try_from = true;
        Self {
            use_try_from,
            r_name: Default::default(),
            mod_name: Default::default(),
            use_rng: Default::default(),
        }
    }
}

impl ExtendrOptions {
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
