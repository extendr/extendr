use crate::wrappers;
use proc_macro::TokenStream;
use quote::quote;
use syn::{meta::ParseNestedMeta, ItemFn, Lit, LitBool};

/// Generate bindings for a single function.
pub fn extendr_function(mut func: ItemFn, opts: &wrappers::ExtendrOptions) -> TokenStream {
    let mut wrappers: Vec<ItemFn> = Vec::new();
    wrappers::make_function_wrappers(opts, &mut wrappers, "", &func.attrs, &mut func.sig, None);

    TokenStream::from(quote! {
        #func

        # ( #wrappers )*
    })
}

impl wrappers::ExtendrOptions {
    /// Parse a set of attribute arguments for #[extendr(opts...)]
    pub fn parse(&mut self, meta: ParseNestedMeta) -> syn::parse::Result<()> {
        fn help_message() -> ! {
            panic!("expected #[extendr(use_try_from=bool, r_name=\"name\")]");
        }

        let value = match meta.value() {
            Ok(value) => value,
            Err(_) => help_message(),
        };

        if meta.path.is_ident("use_try_from") {
            if let Ok(LitBool { value, .. }) = value.parse() {
                self.use_try_from = value;
                Ok(())
            } else {
                help_message();
            }
        } else if meta.path.is_ident("r_name") {
            if let Ok(Lit::Str(litstr)) = value.parse() {
                self.r_name = Some(litstr.value());
                Ok(())
            } else {
                help_message();
            }
        } else if meta.path.is_ident("mod_name") {
            if let Ok(Lit::Str(litstr)) = value.parse() {
                self.mod_name = Some(litstr.value());
                Ok(())
            } else {
                help_message();
            }
        } else {
            help_message();
        }
    }
}
