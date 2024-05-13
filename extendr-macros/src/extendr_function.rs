use crate::{extendr_options::ExtendrOptions, wrappers};
use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;

/// Generate bindings for a single function.
pub fn extendr_function(mut func: ItemFn, opts: &ExtendrOptions) -> TokenStream {
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
