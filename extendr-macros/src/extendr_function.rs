use crate::{extendr_options::ExtendrOptions, wrappers};
use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;

/// Generate bindings for a single function.
pub(crate) fn extendr_function(mut func: ItemFn, opts: &ExtendrOptions) -> TokenStream {
    let wrappers = wrappers::make_function_wrappers(opts, "", &func.attrs, &mut func.sig, None);
    if let Err(e) = wrappers {
        return e.into_compile_error().into();
    };
    let wrappers = wrappers.unwrap();

    TokenStream::from(quote! {
        #func

        # ( #wrappers )*
    })
}
