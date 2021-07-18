use crate::wrappers;
use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;

/// Generate bindings for a single function.
pub fn extendr_function(args: Vec<syn::NestedMeta>, func: ItemFn) -> TokenStream {
    let mut opts = wrappers::ExtendrOptions::default();

    for arg in &args {
        parse_options(&mut opts, arg);
    }

    let mut wrappers: Vec<ItemFn> = Vec::new();
    wrappers::make_function_wrappers(&opts, &mut wrappers, "", &func.attrs, &func.sig, None);

    TokenStream::from(quote! {
        #func

        # ( #wrappers )*
    })
}

/// Parse a set of attribute arguments for #[extendr(opts...)]
pub fn parse_options(opts: &mut wrappers::ExtendrOptions, arg: &syn::NestedMeta) {
    use syn::{Lit, LitBool, Meta, MetaNameValue, NestedMeta};

    match arg {
        NestedMeta::Meta(Meta::NameValue(MetaNameValue {
            ref path,
            eq_token: _,
            lit,
        })) => {
            if path.is_ident("use_try_from") {
                if let Lit::Bool(LitBool { value, .. }) = lit {
                    opts.use_try_from = *value;
                }
            } else {
                panic!("expected use_try_from");
            }
        }
        _ => panic!("expected #[extendr(opt = \"string\", ...)]"),
    }
}
