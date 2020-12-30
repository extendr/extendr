//! Wrappers are lightweight proxies for references to R datatypes.
//! They do not contain an Robj (see array.rs for an example of this).

use crate::robj::*;
use crate::single_threaded;
#[doc(hidden)]
use libR_sys::*;
#[doc(hidden)]
use std::ffi::CString;

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

/// Wrapper for creating raw (byte) objects.
///
/// Example:
/// ```
/// use extendr_api::*;
/// extendr_engine::start_r();
/// let bytes = r!(Raw(&[1, 2, 3]));
/// assert_eq!(bytes.len(), 3);
/// assert_eq!(bytes, r!(Raw(&[1, 2, 3])));
/// ```
///
#[derive(Debug, PartialEq)]
pub struct Raw<'a>(pub &'a [u8]);

/// Wrapper for creating language objects.
/// Example:
/// ```
/// use extendr_api::*;
/// extendr_engine::start_r();
/// let call_to_xyz = r!(Lang(&[r!(Symbol("xyz")), r!(1), r!(2)]));
/// assert_eq!(call_to_xyz.is_language(), true);
/// assert_eq!(call_to_xyz.len(), 3);
/// assert_eq!(format!("{:?}", call_to_xyz), r#"r!(Lang(&[r!(Symbol("xyz")), r!(1), r!(2)]))"#);
///
/// ```
///
/// Note: You can use the [lang!] macro for this.
#[derive(Debug, PartialEq)]
pub struct Lang<T>(pub T);

/// Wrapper for creating pair list (LISTSXP) objects.
/// Example:
/// ```
/// use extendr_api::*;
/// extendr_engine::start_r();
/// let call_to_xyz = r!(Pairlist(&[r!(NULL), r!(1), r!(2)]));
/// assert_eq!(call_to_xyz.is_pair_list(), true);
/// assert_eq!(call_to_xyz.len(), 3);
/// assert_eq!(format!("{:?}", call_to_xyz), r#"r!(Pairlist(&[r!(NULL), r!(1), r!(2)]))"#);
///
/// ```
///
/// Note: You can use the [lang!] macro for this.
#[derive(Debug, PartialEq)]
pub struct Pairlist<T>(pub T);

/// Wrapper for creating list objects.
/// Example:
/// ```
/// use extendr_api::*;
/// extendr_engine::start_r();
/// let mixed_list = r!(List(&[r!(1.), r!("xyz")]));
/// assert_eq!(mixed_list.len(), 2);
/// ```
///
/// Note: prefer to use the [list!] macro for named lists.
#[derive(Debug, PartialEq)]
pub struct List<T>(pub T);

/// Wrapper for creating expression objects.
/// Example:
/// ```
/// use extendr_api::*;
/// extendr_engine::start_r();
/// let expr = r!(Expr(&[r!(1.), r!("xyz")]));
/// assert_eq!(expr.len(), 2);
/// ```
#[derive(Debug, PartialEq)]
pub struct Expr<'a>(pub &'a [Robj]);

impl<T> From<List<T>> for Robj
where
    T: IntoIterator,
    Robj: From<T::Item>,
{
    /// Make a list object from an array of Robjs.
    fn from(val: List<T>) -> Self {
        make_vector(VECSXP, val.0)
    }
}

impl<'a> From<Expr<'a>> for Robj {
    /// Make an expression object from a collection Robjs.
    fn from(val: Expr<'a>) -> Self {
        make_vector(EXPRSXP, val.0)
    }
}

impl<'a> From<Raw<'a>> for Robj {
    /// Make a raw object from bytes.
    fn from(val: Raw<'a>) -> Self {
        single_threaded(|| unsafe {
            let val = val.0;
            let sexp = Rf_allocVector(RAWSXP, val.len() as R_xlen_t);
            R_PreserveObject(sexp);
            let ptr = RAW(sexp);
            for (i, &v) in val.iter().enumerate() {
                *ptr.offset(i as isize) = v;
            }
            Robj::Owned(sexp)
        })
    }
}

impl<'a> From<Symbol<'a>> for Robj {
    /// Make a symbol object.
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

impl<T> From<Lang<T>> for Robj
where
    T: IntoIterator,
    Robj: From<T::Item>,
{
    /// Convert a wrapper to an R language object.
    fn from(val: Lang<T>) -> Self {
        single_threaded(|| unsafe {
            let values = get_protected_values(val.0);
            let mut res = R_NilValue;
            let len = values.len();
            for val in values.into_iter().rev() {
                res = Rf_lcons(val, res);
            }
            Rf_unprotect(len as i32);
            new_owned(res)
        })
    }
}

impl<T> From<Pairlist<T>> for Robj
where
    T: IntoIterator,
    Robj: From<T::Item>,
{
    /// Convert a wrapper to a LISTSXP object.
    fn from(val: Pairlist<T>) -> Self {
        single_threaded(|| unsafe {
            let values = get_protected_values(val.0);
            let mut res = R_NilValue;
            let len = values.len();
            for val in values.into_iter().rev() {
                res = Rf_cons(val, res);
            }
            Rf_unprotect(len as i32);
            new_owned(res)
        })
    }
}

unsafe fn get_protected_values<T>(values: T) -> Vec<SEXP>
where
    T: IntoIterator,
    Robj: From<T::Item>,
{
    values
        .into_iter()
        .map(|item| Rf_protect(Robj::from(item).get()))
        .collect()
}

fn make_vector<T>(sexptype: u32, val: T) -> Robj
where
    T: IntoIterator,
    Robj: From<T::Item>,
{
    single_threaded(|| unsafe {
        let values = get_protected_values(val);
        let sexp = Rf_allocVector(sexptype, values.len() as R_xlen_t);
        R_PreserveObject(sexp);
        for i in 0..values.len() {
            SET_VECTOR_ELT(sexp, i as R_xlen_t, values[i]);
        }
        Rf_unprotect(values.len() as i32);
        Robj::Owned(sexp)
    })
}
