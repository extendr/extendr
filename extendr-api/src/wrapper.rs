//! Wrappers are lightweight proxies for references to R datatypes.
//! They do not contain an Robj (see array.rs for an example of this).

use crate::robj::*;
#[doc(hidden)]
use libR_sys::*;
#[doc(hidden)]
use std::ffi::CString;
use crate::single_threaded;

/// Wrapper for creating symbols.
///
/// Example:
/// ```
/// use extendr_api::*;
/// extendr_engine::start_r();
/// let symbol = r!(Symbol("xyz"));
/// assert_eq!(symbol.as_str(), Some("xyz"));
/// assert!(symbol.is_symbol());
/// ```
/// Note that creating a symbol from a string is expensive
/// and so you may want to cache them.
///
#[derive(Debug, PartialEq)]
pub struct Symbol<'a>(pub &'a str);

/// Wrapper for creating character objects.
/// These are used only as the contents of a character
/// vector.
///
/// Example:
/// ```
/// use extendr_api::*;
/// extendr_engine::start_r();
/// let chr = r!(Character("xyz"));
/// assert_eq!(chr.as_str(), Some("xyz"));
/// ```
///
#[derive(Debug, PartialEq)]
pub struct Character<'a>(pub &'a str);

/// Wrapper for creating language objects.
/// Example:
/// ```
/// use extendr_api::*;
/// extendr_engine::start_r();
/// let _call_to_xyz = r!(Lang("xyz"));
/// ```
///
/// Note: prefer to use the [lang!] macro for this.
#[derive(Debug, PartialEq)]
pub struct Lang<'a>(pub &'a str);

/// Wrapper for creating list objects.
/// Example:
/// ```
/// use extendr_api::*;
/// extendr_engine::start_r();
/// let mixed_list = r!(List(&[r!(1.), r!("xyz")]));
/// assert_eq!(mixed_list.len(), 2);
/// ```
///
/// Note: prefer to use the [list!] macro for this.
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

impl<'a> From<List<'a>> for Robj {
    /// Make a list object from an array of Robjs.
    fn from(val: List<'a>) -> Self {
        single_threaded(|| unsafe {
            let sexp = Rf_allocVector(VECSXP, val.0.len() as R_xlen_t);
            R_PreserveObject(sexp);
            for i in 0..val.0.len() {
                SET_VECTOR_ELT(sexp, i as R_xlen_t, val.0[i].get());
            }
            Robj::Owned(sexp)
        })
    }
}

impl<'a> From<Symbol<'a>> for Robj {
    /// Convert a string to a symbol.
    fn from(name: Symbol) -> Self {
        single_threaded(|| unsafe {
            if let Ok(name) = CString::new(name.0) {
                new_owned(Rf_install(name.as_ptr()))
            } else {
                Robj::from(())
            }
        })
    }
}

impl<'a> From<Lang<'a>> for Robj {
    /// Convert a wrapped string ref to an Robj language object.
    fn from(val: Lang<'a>) -> Self {
        single_threaded(|| unsafe {
            let name = Robj::from(Symbol(val.0));
            new_owned(Rf_lang1(name.get()))
        })
    }
}
