use extendr_api::prelude::*;

/// Test raw identifiers (`r#`) in function arguments are parsed correctly.
/// See (Issue #582)[https://github.com/extendr/extendr/issues/528] for details.
/// @export
#[extendr(use_try_from = true)]
fn raw_identifier_in_fn_args(#[default = "NULL"] r#type: Nullable<i32>) -> Nullable<i32> {
    r#type
}

// Macro to generate exports
extendr_module! {
    fn raw_identifier_in_fn_args;

    mod raw_identifiers;
}
