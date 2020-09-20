//! Wrappers are lightweight proxies for references to R datatypes.
//! They do not contain an Robj (see array.rs for an example of this).

use crate::logical::*;
use crate::robj::*;
use libR_sys::*;
use std::ffi::CString;

/// Wrapper for creating symbols.
#[derive(Debug, PartialEq)]
pub struct Symbol<'a>(pub &'a str);

/// Wrapper for creating character objects.
#[derive(Debug, PartialEq)]
pub struct Character<'a>(pub &'a str);

/// Wrapper for creating language objects.
#[derive(Debug, PartialEq)]
pub struct Lang<'a>(pub &'a str);

/// Wrapper for creating list objects.
#[derive(Debug, PartialEq)]
pub struct List<'a>(pub &'a [Robj]);

impl<'a> PartialEq<List<'a>> for Robj {
    fn eq(&self, rhs: &List) -> bool {
        match self.sexptype() {
            VECSXP if self.len() == rhs.0.len() => {
                for (l, r) in self.list_iter().unwrap().zip(rhs.0.iter()) {
                    if !l.eq(r) {
                        return false;
                    }
                }
                true
            }
            _ => false,
        }
    }
}

/// Make a list object from an array of Robjs.
impl<'a> From<List<'a>> for Robj {
    fn from(val: List<'a>) -> Self {
        unsafe {
            let sexp = Rf_allocVector(VECSXP, val.0.len() as R_xlen_t);
            R_PreserveObject(sexp);
            for i in 0..val.0.len() {
                SET_VECTOR_ELT(sexp, i as R_xlen_t, val.0[i].get());
            }
            Robj::Owned(sexp)
        }
    }
}

/// Convert an integer slice to a logical object.
impl<'a> From<&'a [Bool]> for Robj {
    fn from(vals: &[Bool]) -> Self {
        unsafe {
            let len = vals.len();
            let sexp = Rf_allocVector(LGLSXP, len as R_xlen_t);
            R_PreserveObject(sexp);
            let ptr = LOGICAL(sexp);
            let slice = std::slice::from_raw_parts_mut(ptr, len);
            for (i, &v) in vals.iter().enumerate() {
                slice[i] = v.0;
            }
            Robj::Owned(sexp)
        }
    }
}

/// Convert a string to a symbol.
impl<'a> From<Symbol<'a>> for Robj {
    fn from(name: Symbol) -> Self {
        unsafe {
            if let Ok(name) = CString::new(name.0) {
                new_owned(Rf_install(name.as_ptr()))
            } else {
                Robj::from(())
            }
        }
    }
}
