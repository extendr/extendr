use syn::{meta::ParseNestedMeta, Lit, LitBool};

#[derive(Debug, Default)]
pub(crate) struct ExtendrOptions {
    pub r_name: Option<String>,
    pub mod_name: Option<String>,
    pub use_rng: bool,
}

impl ExtendrOptions {
    /// Parse a set of attribute arguments for `#[extendr(opts...)]`
    ///
    /// Supported options:
    ///
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
