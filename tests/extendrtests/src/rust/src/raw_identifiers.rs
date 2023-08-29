use extendr_api::prelude::*;

/// Test raw identifiers (`r#`) in function arguments are parsed correctly.
/// See [Issue #582](https://github.com/extendr/extendr/issues/528) for details.
/// @param type : i32 or `NULL`
/// @export
#[extendr(use_try_from = true)]
fn raw_identifier_in_fn_args(#[default = "NULL"] r#type: Nullable<i32>) -> Nullable<i32> {
    r#type
}

/// Test raw identifiers (`r#`) as function names are parsed correctly.
/// See [Issue #582](https://github.com/extendr/extendr/issues/528) for details.
/// @export
#[extendr(use_try_from = true)]
fn r#true() -> bool {
    true
}

/// Combine raw identifiers (`r#`) as a function name and in arguments are parsed correctly.
/// See [Issue #582](https://github.com/extendr/extendr/issues/528) for details.
/// @param type : i32 or `NULL`
/// @export
#[extendr(use_try_from = true)]
fn r#false(r#type: bool) -> bool {
    !r#type
}

// Macro to generate exports
extendr_module! {
    fn raw_identifier_in_fn_args;
    fn r#true;
    fn r#false;

    mod raw_identifiers;
}
