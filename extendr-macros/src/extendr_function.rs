use crate::wrappers;
use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;

/// Generate bindings for a single function.
pub fn extendr_function(args: Vec<syn::NestedMeta>, mut func: ItemFn) -> TokenStream {
    let mut opts = wrappers::ExtendrOptions::default();

    for arg in &args {
        parse_options(&mut opts, arg);
    }

    let mut wrappers: Vec<ItemFn> = Vec::new();
    wrappers::make_function_wrappers(&opts, &mut wrappers, "", &func.attrs, &mut func.sig, None);

    TokenStream::from(quote! {
        #func

        # ( #wrappers )*
    })
}

/// Parse a set of attribute arguments for #[extendr(opts...)]
pub fn parse_options(opts: &mut wrappers::ExtendrOptions, arg: &syn::NestedMeta) {
    use syn::{Lit, LitBool, Meta, MetaNameValue, NestedMeta};

    fn help_message() -> ! {
        panic!("expected #[extendr(use_try_from=bool, r_name=\"name\", r_class_name=\"AnyChosenName\")]");
    }

    match arg {
        NestedMeta::Meta(Meta::NameValue(MetaNameValue {
            ref path,
            eq_token: _,
            lit,
        })) => match lit {
            Lit::Bool(LitBool { value, .. }) if path.is_ident("use_try_from") => {
                opts.use_try_from = *value;
            }
            Lit::Str(litstr) if path.is_ident("r_name") => opts.r_name = Some(litstr.value()),
            Lit::Str(litstr) if path.is_ident("mod_name") => opts.mod_name = Some(litstr.value()),
            Lit::Str(litstr) if path.is_ident("r_class_name") => {
                opts.r_class_name = Some(litstr.value())
            }
            _ => help_message(),
        },
        _ => help_message(),
    }
}
