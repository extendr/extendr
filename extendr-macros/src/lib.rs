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
// * An output conversion from that type to an owned external pointer object.
// * A finalizer for that type to free memory allocated.

#[allow(non_snake_case)]
mod R;
mod call;
mod dataframe;
mod extendr_function;
mod extendr_impl;
mod extendr_module;
mod extendr_options;
mod list;
mod list_struct;
mod pairlist;
mod pairs;
mod wrappers;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item};

/// The `#[extendr]`-macro may be placed on three items
///
/// - `fn` for wrapped rust-functions, see [`extendr-fn`]
/// - `impl`-blocks, see [`extendr-impl`]
///
/// [`extendr-fn`]: ./extendr_function/fn.extendr_function.html
/// [`extendr-impl`]: ./extendr_impl/fn.extendr_impl.html
///
/// There is also [`macro@extendr_module`], which is used for defining what rust
/// wrapped items should be visible to the surrounding R-package.
///
#[proc_macro_attribute]
pub fn extendr(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut opts = extendr_options::ExtendrOptions::default();

    let extendr_opts_parser = syn::meta::parser(|meta| opts.parse(meta));
    parse_macro_input!(attr with extendr_opts_parser);

    match parse_macro_input!(item as Item) {
        Item::Fn(func) => extendr_function::extendr_function(func, &opts),
        Item::Impl(item_impl) => match extendr_impl::extendr_impl(item_impl, &opts) {
            Ok(result) => result,
            Err(e) => e.into_compile_error().into(),
        },
        other_item => TokenStream::from(quote! {#other_item}),
    }
}

/// Define a module and export symbols to R
/// Example:
///```dont_run
/// extendr_module! {
///     mod name;
///     fn my_func1;
///     fn my_func2;
///     impl MyTrait;
/// }
/// ```
/// Outputs:
///
/// ```dont_run
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
/// some shortcuts for simple expressions, for example by caching symbols and constant values.
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

/// Derives an implementation of `TryFrom<Robj> for Struct` and `TryFrom<&Robj> for Struct` on this struct.
///
/// This allows any R object supporting the `$` operator (generally a list or an
/// environment) to be converted into that struct, as long as the corresponding fields on the R object are
/// of a compatible type to those on the Rust struct.
///
/// # Examples
/// In the below example, `foo_from_list` is an instance of the `Foo` struct, that has been converted
/// from an R list:
/// ```ignore
/// use extendr_api::prelude::*;
/// use extendr_macros::TryFromRobj;
/// # use extendr_api::test;
/// # test!{
///
/// #[derive(TryFromRobj, PartialEq, Debug)]
/// struct Foo {
///     a: u64,
///     b: String
/// }
/// let native_foo = Foo { a: 5, b: "bar".into() };
/// let foo_from_list: Foo = R!("list(a = 5, b = 'bar')")?.try_into()?;
/// assert_eq!(native_foo, foo_from_list);
/// # }
/// # Ok::<(), extendr_api::Error>(())
/// ```
///
/// See [`IntoRobj`] for converting arbitrary Rust types into R type by using
/// R's list / `List`.
///
#[proc_macro_derive(TryFromRobj)]
pub fn derive_try_from_robj(item: TokenStream) -> TokenStream {
    match list_struct::derive_try_from_robj(item) {
        Ok(result) => result,
        Err(e) => e.into_compile_error().into(),
    }
}

/// Derives an implementation of `From<Struct> for Robj` and `From<&Struct> for Robj` on this struct.
///
/// This allows the struct to be converted to a named list in R,
/// where the list names correspond to the field names of the Rust struct.
///
/// # Examples
/// In the below example, `converted` contains an R list object with the same fields as the
/// `Foo` struct.
/// ```ignore
/// use extendr_api::prelude::*;
/// use extendr_macros::IntoRobj;
///
/// # use extendr_api::test;
/// # test!{
/// #[derive(IntoRobj)]
/// struct Foo {
///     a: u32,
///     b: String
/// }
/// let converted: Robj = Foo {
///     a: 5,
///     b: String::from("bar")
/// }.into();
/// assert_eq!(converted, R!(r"list(a=5, b='bar')")?);
/// # }
/// # Ok::<(), extendr_api::Error>(())
/// ```
///
/// See [`TryFromRobj`] for a `derive`-macro in the other direction, i.e.
/// instantiation of a rust type, by an R list with fields corresponding to
/// said type.
///
/// # Details
///
/// Note, the `From<Struct> for Robj` behaviour is different from what is obtained by applying the standard `#[extendr]` macro
/// to an `impl` block. The `#[extendr]` behaviour returns to R a **pointer** to Rust memory, and generates wrapper functions for calling
/// Rust functions on that pointer. The implementation from `#[derive(IntoRobj)]` actually converts the Rust structure
/// into a native R list, which allows manipulation and access to internal fields, but it's a one-way conversion,
/// and converting it back to Rust will produce a copy of the original struct.
#[proc_macro_derive(IntoRobj)]
pub fn derive_into_robj(item: TokenStream) -> TokenStream {
    match list_struct::derive_into_robj(item) {
        Ok(result) => result,
        Err(e) => e.into_compile_error().into(),
    }
}

/// Enable the construction of dataframes from arrays of structures.
///
/// # Example
///
/// ```ignore
/// use extendr_api::prelude::*;
///
/// #[derive(Debug, IntoDataFrameRow)]
/// struct MyStruct {
///     x: i32,
///     y: String,
/// }
///
/// let v = vec![MyStruct { x: 0, y: "abc".into() }, MyStruct { x: 1, y: "xyz".into() }];
/// let df = v.into_dataframe()?;
///
/// assert!(df.inherits("data.frame"));
/// assert_eq!(df[0], r!([0, 1]));
/// assert_eq!(df[1], r!(["abc", "xyz"]));
/// ```
#[proc_macro_derive(IntoDataFrameRow)]
pub fn derive_into_dataframe(item: TokenStream) -> TokenStream {
    dataframe::derive_into_dataframe(item)
}
