use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

use crate::pairs::Pairs;

pub fn list(item: TokenStream) -> TokenStream {
    let list = parse_macro_input!(item as Pairs);

    let nv = list.names_and_values();

    if nv.is_empty() {
        TokenStream::from(quote!(extendr_api::wrapper::List::default()))
    } else {
        let values: Vec<proc_macro2::TokenStream> = nv
            .iter()
            .map(|(_n, v)| quote!( extendr_api::Robj::from(#v) ))
            .collect();
        if nv.iter().any(|(n, _v)| !n.is_empty()) {
            let names: Vec<proc_macro2::TokenStream> =
                nv.iter().map(|(n, _v)| quote!( #n )).collect();
            // Note that this unwrap should not fail.
            TokenStream::from(quote!(
                extendr_api::List::from_names_and_values(&[# ( #names ),*], &[# ( #values ),*]).unwrap()
            ))
        } else {
            TokenStream::from(quote!(
                extendr_api::List::from_values(&[# ( #values ),*])
            ))
        }
    }
}
