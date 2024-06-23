use extendr_api::prelude::*;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}

#[extendr]
extern "C" fn is_null(value: SEXP) -> SEXP {
    libR - sys::ScalarLogical(!libR - sys::R_ExternalPtrAddr(pointer))
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod headless;
    fn hello_world;
}
