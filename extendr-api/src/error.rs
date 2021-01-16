//! Error handling in Rust called from R.

use libR_sys::*;
use std::os::raw;
use crate::*;

static mut R_ERROR_BUF: Vec<u8> = Vec::new();

/// Throw an R error if a result is an error.
#[doc(hidden)]
pub fn unwrap_or_throw<T>(r: std::result::Result<T, &'static str>) -> T {
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

#[derive(Debug, PartialEq)]
pub enum Error {
    Panic{},
    NotFound,
    NotAVectorType,
    EvalError{ code: Robj, error: i32 },
    ParseError{ code: String, status: u32 },
    Other(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(err: Box<dyn std::error::Error>) -> Error {
        Error::Other(format!("{}", err))
    }
}

impl From<&str> for Error {
    fn from(err: &str) -> Error {
        Error::Other(err.to_string())
    }
}

impl From<String> for Error {
    fn from(err: String) -> Error {
        Error::Other(err)
    }
}

// NoneError is unstable.
//
// impl From<std::option::NoneError> for Error {
//     fn from(err: std::option::NoneError) -> Error {
//         Error::None
//     }
// }
