extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn};
use quote::quote;

#[proc_macro_attribute]
pub fn export_function(_attr: TokenStream, item: TokenStream) -> TokenStream {
    //println!("export_function attr: {:?}", attr);
    //println!("export_function item: {:?}", item);
    let ast = parse_macro_input!(item as ItemFn);
    //let ident = ast.sig.ident.ident;
    println!("export_function ast: {:#?}", ast);
    let expanded = quote! {
        #[allow(dead_code)]
        #ast
    };
    println!("res: {}", expanded);
    TokenStream::from(expanded)
}

