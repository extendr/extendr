use extendr_api::prelude::*;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}

use libR_sys::SEXP;

#[extendr]
extern "C" fn is_null(value: SEXP) -> SEXP {
    unsafe { libR_sys::Rf_ScalarLogical(libR_sys::R_ExternalPtrAddr(value).is_null() as _) }
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod headless;
    extern "C" fn is_null;
}
