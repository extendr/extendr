use crate::wrappers;
use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;

/// Generate bindings for a single function.
pub fn extendr_function(args: Vec<syn::NestedMeta>, func: ItemFn) -> TokenStream {
    let mut opts = wrappers::ExtendrOptions {};

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
pub fn parse_options(_opts: &mut wrappers::ExtendrOptions, _arg: &syn::NestedMeta) {
    /*use syn::{Lit, Meta, MetaNameValue, NestedMeta};

    match arg {
        NestedMeta::Meta(Meta::NameValue(MetaNameValue {
            ref path,
            eq_token: _,
            lit: Lit::Str(ref _lit_str),
        })) => {
        }
        _ => panic!("expected #[extendr(opt = \"string\", ...)]"),
    }*/
}
