//!
//!
//!
//!
use extendr_api::prelude::*;
use extendr_api::SEXP;

#[extendr]
extern "C" fn is_null(value: SEXP) -> SEXP {
    unsafe { libR_sys::Rf_ScalarLogical(libR_sys::R_ExternalPtrAddr(value).is_null() as _) }
}

extendr_module! {
    mod extendr_macro;

    extern "C" fn is_null;
}
