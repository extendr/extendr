use crate::wrappers;
use proc_macro::TokenStream;
use quote::quote;
use syn::{meta::ParseNestedMeta, ItemFn, Lit, LitBool};

/// Generate bindings for a single function.
pub fn extendr_function(args: Vec<syn::NestedMeta>, mut func: ItemFn) -> TokenStream {
    let mut opts = wrappers::ExtendrOptions::default();

    for arg in &args {
        parse_options(&mut opts, arg);
    }
    let mut wrappers: Vec<ItemFn> = Vec::new();
    wrappers::make_function_wrappers(opts, &mut wrappers, "", &func.attrs, &mut func.sig, None);

    TokenStream::from(quote! {
        #func

        # ( #wrappers )*
    })
}

/// Parse a set of attribute arguments for `#[extendr(opts...)]`
///
/// Supported options:
///
/// - `use_try_from = bool` which uses `TryFrom<Robj>` for argument conversions.
/// - `r_name = "name"` which specifies the name of the wrapper on the R-side.
/// - `use_rng = bool` ensures the RNG-state is pulled and pushed
///
pub fn parse_options(opts: &mut wrappers::ExtendrOptions, arg: &syn::NestedMeta) {
    use syn::{Lit, LitBool, Meta, MetaNameValue, NestedMeta};

    fn help_message() -> ! {
        panic!("expected #[extendr(use_try_from = bool, r_name = \"name\", mod_name = \"r_mod_name\", use_rng = bool)]");
    }

    match arg {
        NestedMeta::Meta(Meta::NameValue(MetaNameValue {
            ref path,
            eq_token: _,
            lit,
        })) => {
            if path.is_ident("use_try_from") {
                if let Lit::Bool(LitBool { value, .. }) = lit {
                    opts.use_try_from = *value;
                } else {
                    help_message();
                }
            } else if path.is_ident("r_name") {
                if let Lit::Str(litstr) = lit {
                    opts.r_name = Some(litstr.value());
                } else {
                    help_message();
                }
            } else if path.is_ident("mod_name") {
                if let Lit::Str(litstr) = lit {
                    opts.mod_name = Some(litstr.value());
                } else {
                    help_message();
                }
            } else if path.is_ident("use_rng") {
                if let Lit::Bool(LitBool { value, .. }) = lit {
                    opts.use_rng = *value;
                } else {
                    help_message();
                }
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
