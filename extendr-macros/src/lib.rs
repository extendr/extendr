//!
//! Macros for generating wrappers for rust functions.

//
// We can invoke the #[extendr] macro on functions or struct impls.
//
// eg.
//
// ```ignore
// #[extendr]
// fn hello() -> &'static str {
//     "hello"
// }
// ```
//
// These macros add additional functions which you can see using the
// `cargo expand` extension.
//
// Invoking the #[extendr_module] macro generates an entrypoint for the
// library that will be called by R. Note that we add a postfix
// `_extendr` to the init function because we need to forward routine
// registration from C to Rust, and the C function will be called
// `R_init_hello()`.
//
// ```ignore
// #[no_mangle]
// #[allow(non_snake_case)]
// pub extern "C" fn R_init_hello_extendr(info: *mut extendr_api::DllInfo) {
//     let mut call_methods = Vec::new();
//     init__hello(info, &mut call_methods);
//     unsafe { extendr_api::register_call_methods(info, call_methods.as_ref()) };
// }
// ```
//
// The module also generates the `init__` functions that provide metadata
// to R to register the wrappers.
//
// ```ignore
// #[allow(non_snake_case)]
// fn init__hello(info: *mut extendr_api::DllInfo, call_methods: &mut Vec<extendr_api::CallMethod>) {
//     call_methods.push(extendr_api::CallMethod {
//         call_symbol: std::ffi::CString::new("wrap__hello").unwrap(),
//         func_ptr: wrap__hello as *const u8,
//         num_args: 0i32,
//     })
// }
// ```
//
// In the case of struct impls we also generate the following:
//
// * Wrappers and init functions for all methods.
// * A single init function that calls the other init functions for the methods.
// * An input conversion from an external pointer to a reference and a move of that type.
// * An output converstion from that type to an owned external pointer object.
// * A finalizer for that type to free memory allocated.

#[allow(non_snake_case)]
mod R;
mod call;
mod extendr_function;
mod extendr_impl;
mod extendr_module;
mod list;
mod pairlist;
mod pairs;
mod wrappers;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item};

#[proc_macro_attribute]
pub fn extendr(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as syn::AttributeArgs);
    match parse_macro_input!(item as Item) {
        Item::Fn(func) => extendr_function::extendr_function(args, func),
        Item::Impl(item_impl) => extendr_impl::extendr_impl(item_impl),
        other_item => TokenStream::from(quote! {#other_item}),
    }
}

/// Define a module and export symbols to R
/// Example:
///```ignore
/// extendr_module! {
///     mod name;
///     fn my_func1;
///     fn my_func2;
///     impl MyTrait;
/// }
/// ```
/// Outputs:
///
/// ```ignore
/// #[no_mangle]
/// #[allow(non_snake_case)]
/// pub extern "C" fn R_init_hello_extendr(info: *mut extendr_api::DllInfo) {
///     let mut call_methods = Vec::new();
///     init__hello(info, &mut call_methods);
///     unsafe { extendr_api::register_call_methods(info, call_methods.as_ref()) };
/// }
/// ```
#[proc_macro]
pub fn extendr_module(item: TokenStream) -> TokenStream {
    extendr_module::extendr_module(item)
}

/// Create a Pairlist R object from a list of name-value pairs.
/// ```ignore
///     assert_eq!(pairlist!(a=1, 2, 3), Pairlist::from_pairs(&[("a", 1), ("", 2), ("", 3)]));
/// ```
#[proc_macro]
pub fn pairlist(item: TokenStream) -> TokenStream {
    pairlist::pairlist(item)
}

/// Create a List R object from a list of name-value pairs.
/// ```ignore
///     assert_eq!(list!(a=1, 2, 3), List::from_pairs(&[("a", 1), ("", 2), ("", 3)]));
/// ```
#[proc_macro]
pub fn list(item: TokenStream) -> TokenStream {
    list::list(item)
}

/// Call a function or primitive defined by a text expression with arbitrary parameters.
/// This currently works by parsing and evaluating the string in R, but will probably acquire
/// some shortcuts for simple expessions, for example by caching symbols and constant values.
///
/// ```ignore
///     assert_eq!(call!("`+`", 1, 2), r!(3));
///     assert_eq!(call!("list", 1, 2), r!([r!(1), r!(2)]));
/// ```
#[proc_macro]
pub fn call(item: TokenStream) -> TokenStream {
    call::call(item)
}

/// Execute R code by parsing and evaluating tokens.
///
/// ```ignore
///     R!("c(1, 2, 3)");
///     R!("{{(0..3).collect_robj()}} + 1");
///     R!(r#"
///       print("hello")
///     "#);
/// ```
#[proc_macro]
#[allow(non_snake_case)]
pub fn R(item: TokenStream) -> TokenStream {
    R::R(item.into(), true).into()
}

/// Execute R code by parsing and evaluating tokens
/// but without expanding parameters.
///
/// ```ignore
/// // c.f. https://dplyr.tidyverse.org/articles/programming.html
/// Rraw!(r#"
/// var_summary <- function(data, var) {
///   data %>%
///     summarise(n = n(), min = min({{ var }}), max = max({{ var }}))
/// }
/// "#)
/// ```
#[proc_macro]
#[allow(non_snake_case)]
pub fn Rraw(item: TokenStream) -> TokenStream {
    R::R(item.into(), false).into()
}
