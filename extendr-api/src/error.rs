//! Error handling in Rust called from R.

use libR_sys::*;
use std::os::raw;

static mut R_ERROR_BUF: Vec<u8> = Vec::new();

/// Throw an R error if a result is an error.
pub fn unwrap_or_throw<T>(r: Result<T, &'static str>) -> T {
    unsafe {
        match r {
            Err(e) => {
                R_ERROR_BUF.clear();
                R_ERROR_BUF.extend(e.bytes());
                R_ERROR_BUF.push(0);
                Rf_error(R_ERROR_BUF.as_slice().as_ptr() as *mut raw::c_char);
                unreachable!("");
            }
            Ok(v) => v,
        }
    }
}

