//! R object handling.
//!
//! See. https://cran.r-project.org/doc/manuals/R-exts.html
//!
//! Fundamental principals:
//!
//! * Any function that can break the protection mechanism is unsafe.
//! * Users should be able to do almost everything without using libR_sys.
//! * The interface should be friendly to R users without Rust experience.
//!

use libR_sys::*;
use std::os::raw;

use crate::*;

use std::collections::HashMap;
use std::iter::IntoIterator;
use std::ops::{Range, RangeInclusive};

mod from_robj;
mod into_robj;
mod operators;
mod rinternals;

#[cfg(test)]
mod tests;

pub use from_robj::*;
pub use into_robj::*;
pub use iter::*;
pub use operators::*;
pub use rinternals::*;

/// Wrapper for an R S-expression pointer (SEXP).
///
/// Create R objects from rust types and iterators:
///
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     // Different ways of making integer scalar 1.
///     let non_na : Option<i32> = Some(1);
///     let a : Robj = vec![1].into();
///     let b = r!(1);
///     let c = r!(vec![1]);
///     let d = r!(non_na);
///     let e = r!([1]);
///     assert_eq!(a, b);
///     assert_eq!(a, c);
///     assert_eq!(a, d);
///     assert_eq!(a, e);
///
///     // Different ways of making boolean scalar TRUE.
///     let a : Robj = true.into();
///     let b = r!(TRUE);
///     assert_eq!(a, b);
///
///     // Create a named list
///     let a = list!(a = 1, b = "x");
///     assert_eq!(a.len(), 2);
///
///     // Use an iterator (like 1:10)
///     let a = r!(1 ..= 10);
///     assert_eq!(a, r!([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
///
///     // Use an iterator (like (1:10)[(1:10) %% 3 == 0])
///     let a = (1 ..= 10).filter(|v| v % 3 == 0).collect_robj();
///     assert_eq!(a, c!(3, 6, 9));
/// }
/// ```
///
/// Use iterators to get the contents of R objects.
///
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let a : Robj = c!(1, 2, 3, 4, 5);
///     let iter = a.as_integer_iter().unwrap();
///     let robj = iter.filter(|&x| x < 3).collect_robj();
///     assert_eq!(robj, c!(1, 2));
/// }
/// ```
///
/// Convert to/from Rust vectors.
///
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let a : Robj = r!(vec![1., 2., 3., 4.]);
///     let b : Vec<f64> = a.as_real_vector().unwrap();
///     assert_eq!(a.len(), 4);
///     assert_eq!(b, vec![1., 2., 3., 4.]);
/// }
/// ```
///
/// Iterate over names and values.
///
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let abc = list!(a = 1, b = "x", c = vec![1, 2]);
///     let names : Vec<_> = abc.names().unwrap().collect();
///     let names_and_values : Vec<_> = abc.as_named_list_iter().unwrap().collect();
///     assert_eq!(names, vec!["a", "b", "c"]);
///     assert_eq!(names_and_values, vec![("a", r!(1)), ("b", r!("x")), ("c", r!(vec![1, 2]))]);
/// }
/// ```
///
/// NOTE: as much as possible we wish to make this object safe (ie. no segfaults).
///
/// If you avoid using unsafe functions it is more likely that you will avoid
/// panics and segfaults. We will take great trouble to ensure that this
/// is true.
///
pub enum Robj {
    // This object owns the SEXP and must free it.
    #[doc(hidden)]
    Owned(SEXP),

    //  This object references a SEXP owned by libR.
    #[doc(hidden)]
    Sys(SEXP),
}

impl Clone for Robj {
    fn clone(&self) -> Self {
        unsafe {
            match *self {
                Robj::Owned(sexp) => new_owned(sexp),
                Robj::Sys(sexp) => new_sys(sexp),
            }
        }
    }
}

impl Default for Robj {
    fn default() -> Self {
        Robj::from(())
    }
}

impl Robj {
    /// Get a copy of the underlying SEXP.
    /// Note: this is unsafe.
    #[doc(hidden)]
    pub unsafe fn get(&self) -> SEXP {
        match self {
            Robj::Owned(sexp) => *sexp,
            Robj::Sys(sexp) => *sexp,
        }
    }

    /// Get a copy of the underlying SEXP for mutable types.
    /// This is valid only for owned objects as we are not
    /// permitted to modify parameters or system objects.
    #[doc(hidden)]
    pub unsafe fn get_mut(&mut self) -> Option<SEXP> {
        match self {
            Robj::Owned(sexp) => Some(*sexp),
            Robj::Sys(_) => None,
        }
    }

    #[doc(hidden)]
    /// Get the XXXSXP type of the object.
    pub fn sexptype(&self) -> u32 {
        unsafe { TYPEOF(self.get()) as u32 }
    }

    /// Get the type of an R object.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     assert_eq!(r!(NULL).rtype(), RType::Null);
    ///     assert_eq!(sym!(xyz).rtype(), RType::Symbol);
    ///     assert_eq!(r!(Pairlist{names_and_values: vec![("a", r!(1))]}).rtype(), RType::Pairlist);
    ///     assert_eq!(R!(function() {})?.rtype(), RType::Function);
    ///     assert_eq!(new_env().rtype(), RType::Enviroment);
    ///     assert_eq!(lang!("+", 1, 2).rtype(), RType::Language);
    ///     assert_eq!(r!(Primitive("if")).rtype(), RType::Special);
    ///     assert_eq!(r!(Primitive("+")).rtype(), RType::Builtin);
    ///     assert_eq!(r!(Character("hello")).rtype(), RType::Character);
    ///     assert_eq!(r!(TRUE).rtype(), RType::Logical);
    ///     assert_eq!(r!(1).rtype(), RType::Integer);
    ///     assert_eq!(r!(1.0).rtype(), RType::Real);
    ///     assert_eq!(r!("1").rtype(), RType::String);
    ///     assert_eq!(r!(List(&[1, 2])).rtype(), RType::List);
    ///     assert_eq!(parse("x + y")?.rtype(), RType::Expression);
    ///     assert_eq!(r!(Raw(&[1_u8, 2, 3])).rtype(), RType::Raw);
    /// }
    /// ```
    pub fn rtype(&self) -> RType {
        match self.sexptype() {
            NILSXP => RType::Null,
            SYMSXP => RType::Symbol,
            LISTSXP => RType::Pairlist,
            CLOSXP => RType::Function,
            ENVSXP => RType::Enviroment,
            PROMSXP => RType::Promise,
            LANGSXP => RType::Language,
            SPECIALSXP => RType::Special,
            BUILTINSXP => RType::Builtin,
            CHARSXP => RType::Character,
            LGLSXP => RType::Logical,
            INTSXP => RType::Integer,
            REALSXP => RType::Real,
            CPLXSXP => RType::Complex,
            STRSXP => RType::String,
            DOTSXP => RType::Dot,
            ANYSXP => RType::Any,
            VECSXP => RType::List,
            EXPRSXP => RType::Expression,
            BCODESXP => RType::Bytecode,
            EXTPTRSXP => RType::ExternalPtr,
            WEAKREFSXP => RType::WeakRef,
            RAWSXP => RType::Raw,
            S4SXP => RType::S4,
            _ => RType::Unknown,
        }
    }

    /// Get the extended length of the object.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///
    /// let a : Robj = r!(vec![1., 2., 3., 4.]);
    /// assert_eq!(a.len(), 4);
    /// }
    /// ```
    pub fn len(&self) -> usize {
        unsafe { Rf_xlength(self.get()) as usize }
    }

    /// Get a variable from an enviroment, but not its ancestors.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///
    /// let env = new_env();
    /// env.set_local(sym!(x), "fred");
    /// assert_eq!(env.local(sym!(x)), Some(r!("fred")));
    /// }
    /// ```
    pub fn local<K: Into<Robj>>(&self, key: K) -> Option<Robj> {
        let key = key.into();
        if self.is_environment() && key.is_symbol() {
            unsafe { Some(new_owned(Rf_findVarInFrame3(self.get(), key.get(), 1))) }
        } else {
            None
        }
    }

    /// Set or define a variable in an enviroment.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///
    /// let env = new_env();
    /// env.set_local(sym!(x), "harry");
    /// env.set_local(sym!(x), "fred");
    /// assert_eq!(env.local(sym!(x)), Some(r!("fred")));
    /// }
    /// ```
    pub fn set_local<K: Into<Robj>, V: Into<Robj>>(&self, key: K, value: V) {
        let key = key.into();
        let value = value.into();
        if self.is_environment() && key.is_symbol() {
            single_threaded(|| unsafe {
                Rf_defineVar(key.get(), value.get(), self.get());
            })
        }
    }

    /// Get the parent of an environment.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///
    /// let global_parent = global_env().parent().unwrap();
    /// assert_eq!(global_parent.is_environment(), true);
    /// assert_eq!(base_env().parent(), None);
    /// assert_eq!(r!(1).parent(), None);
    /// }
    /// ```
    pub fn parent(&self) -> Option<Robj> {
        unsafe {
            if self.is_environment() {
                let parent = ENCLOS(self.get());
                if Rf_isEnvironment(parent) != 0 && parent != R_EmptyEnv {
                    return Some(new_owned(parent));
                }
            }
            None
        }
    }

    /// Is this object is an NA scalar?
    /// Works for character, integer and numeric types.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///
    /// assert_eq!(r!(NA_INTEGER).is_na(), true);
    /// assert_eq!(r!(NA_REAL).is_na(), true);
    /// assert_eq!(r!(NA_STRING).is_na(), true);
    /// }
    /// ```
    pub fn is_na(&self) -> bool {
        if self.len() != 1 {
            false
        } else {
            unsafe {
                let sexp = self.get();
                match self.sexptype() {
                    STRSXP => STRING_ELT(sexp, 0) == libR_sys::R_NaString,
                    INTSXP => *(INTEGER(sexp)) == libR_sys::R_NaInt,
                    LGLSXP => *(LOGICAL(sexp)) == libR_sys::R_NaInt,
                    REALSXP => R_IsNA(*(REAL(sexp))) != 0,
                    _ => false,
                }
            }
        }
    }

    /// Get a read-only reference to the content of an integer vector.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///
    /// let robj = r!([1, 2, 3]);
    /// assert_eq!(robj.as_integer_slice().unwrap(), [1, 2, 3]);
    /// }
    /// ```
    pub fn as_integer_slice<'a>(&self) -> Option<&'a [i32]> {
        self.as_typed_slice()
    }

    /// Get an iterator over integer elements of this slice.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///
    /// let robj = r!([1, 2, 3]);
    /// let mut tot = 0;
    /// for val in robj.as_integer_iter().unwrap() {
    ///   tot += val;
    /// }
    /// assert_eq!(tot, 6);
    /// }
    /// ```
    pub fn as_integer_iter(&self) -> Option<Int> {
        if let Some(slice) = self.as_integer_slice() {
            Some(Int::from_slice(self.to_owned(), slice))
        } else {
            None
        }
    }

    /// Get a Vec<i32> copied from the object.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///
    /// let robj = r!([1, 2, 3]);
    /// assert_eq!(robj.as_integer_slice().unwrap(), vec![1, 2, 3]);
    /// }
    /// ```
    pub fn as_integer_vector(&self) -> Option<Vec<i32>> {
        if let Some(value) = self.as_integer_slice() {
            Some(value.iter().cloned().collect::<Vec<_>>())
        } else {
            None
        }
    }

    /// Get a read-only reference to the content of a logical vector
    /// using the tri-state [Bool]. Returns None if not a logical vector.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let robj = r!([TRUE, FALSE, NA_LOGICAL]);
    ///     assert_eq!(robj.as_logical_slice().unwrap(), [TRUE, FALSE, NA_LOGICAL]);
    /// }
    /// ```
    pub fn as_logical_slice(&self) -> Option<&[Bool]> {
        self.as_typed_slice()
    }

    /// Get a Vec<Bool> copied from the object
    /// using the tri-state [Bool].
    /// Returns None if not a logical vector.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let robj = r!([TRUE, FALSE, NA_LOGICAL]);
    ///     assert_eq!(robj.as_logical_vector().unwrap(), vec![TRUE, FALSE, NA_LOGICAL]);
    /// }
    /// ```
    pub fn as_logical_vector(&self) -> Option<Vec<Bool>> {
        if let Some(value) = self.as_logical_slice() {
            Some(value.iter().cloned().collect::<Vec<_>>())
        } else {
            None
        }
    }

    /// Get an iterator over logical elements of this slice.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let robj = r!([TRUE, FALSE, NA_LOGICAL]);
    ///     let (mut nt, mut nf, mut nna) = (0, 0, 0);
    ///     for val in robj.as_logical_iter().unwrap() {
    ///       match val {
    ///         TRUE => nt += 1,
    ///         FALSE => nf += 1,
    ///         NA_LOGICAL => nna += 1,
    ///         _ => ()
    ///       }
    ///     }
    ///     assert_eq!((nt, nf, nna), (1, 1, 1));
    /// }
    /// ```
    pub fn as_logical_iter(&self) -> Option<Logical> {
        if let Some(slice) = self.as_logical_slice() {
            Some(Logical::from_slice(self.to_owned(), slice))
        } else {
            None
        }
    }

    /// Get a read-only reference to the content of a double vector.
    /// Note: the slice may contain NaN or NA values.
    /// We may introduce a "Real" type to handle this like the Bool type.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let robj = r!([Some(1.), None, Some(3.)]);
    ///     let mut tot = 0.;
    ///     for val in robj.as_real_slice().unwrap() {
    ///       if !val.is_na() {
    ///         tot += val;
    ///       }
    ///     }
    ///     assert_eq!(tot, 4.);
    /// }
    /// ```
    pub fn as_real_slice(&self) -> Option<&[f64]> {
        self.as_typed_slice()
    }

    /// Get an iterator over real elements of this slice.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let robj = r!([1., 2., 3.]);
    ///     let mut tot = 0.;
    ///     for val in robj.as_real_iter().unwrap() {
    ///       if !val.is_na() {
    ///         tot += val;
    ///       }
    ///     }
    ///     assert_eq!(tot, 6.);
    /// }
    /// ```
    pub fn as_real_iter(&self) -> Option<Real> {
        if let Some(slice) = self.as_real_slice() {
            Some(Real::from_slice(self.to_owned(), slice))
        } else {
            None
        }
    }

    /// Get a Vec<f64> copied from the object.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let robj = r!([1., 2., 3.]);
    ///     assert_eq!(robj.as_real_vector().unwrap(), vec![1., 2., 3.]);
    /// }
    /// ```
    pub fn as_real_vector(&self) -> Option<Vec<f64>> {
        if let Some(value) = self.as_real_slice() {
            Some(value.iter().cloned().collect::<Vec<_>>())
        } else {
            None
        }
    }

    /// Get a read-only reference to the content of an integer or logical vector.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let robj = r!(Raw(&[1, 2, 3]));
    ///     assert_eq!(robj.as_raw_slice().unwrap(), &[1, 2, 3]);
    /// }
    /// ```
    pub fn as_raw_slice(&self) -> Option<&[u8]> {
        self.as_typed_slice()
    }

    /// Get a read-write reference to the content of an integer or logical vector.
    /// Note that rust slices are 0-based so `slice[1]` is the middle value.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let mut robj = r!([1, 2, 3]);
    ///     let slice : & mut [i32] = robj.as_integer_slice_mut().unwrap();
    ///     slice[1] = 100;
    ///     assert_eq!(robj, r!([1, 100, 3]));
    /// }
    /// ```
    pub fn as_integer_slice_mut(&mut self) -> Option<&mut [i32]> {
        self.as_typed_slice_mut()
    }

    /// Get a read-write reference to the content of a double vector.
    /// Note that rust slices are 0-based so `slice[1]` is the middle value.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let mut robj = r!([1.0, 2.0, 3.0]);
    ///     let slice = robj.as_real_slice_mut().unwrap();
    ///     slice[1] = 100.0;
    ///     assert_eq!(robj, r!([1.0, 100.0, 3.0]));
    /// }
    /// ```
    pub fn as_real_slice_mut(&mut self) -> Option<&mut [f64]> {
        self.as_typed_slice_mut()
    }

    /// Get a read-write reference to the content of a raw vector.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let mut robj = r!(Raw(&[1, 2, 3]));
    ///     let slice = robj.as_raw_slice_mut().unwrap();
    ///     slice[1] = 100;
    ///     assert_eq!(robj, r!(Raw(&[1, 100, 3])));
    /// }
    /// ```
    pub fn as_raw_slice_mut(&mut self) -> Option<&mut [u8]> {
        self.as_typed_slice_mut()
    }

    /// Get a vector of owned strings.
    /// Owned strings have long lifetimes, but are much slower than references.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///    let robj1 = Robj::from("xyz");
    ///    assert_eq!(robj1.as_string_vector(), Some(vec!["xyz".to_string()]));
    ///    let robj2 = Robj::from(1);
    ///    assert_eq!(robj2.as_string_vector(), None);
    /// }
    /// ```
    pub fn as_string_vector(&self) -> Option<Vec<String>> {
        if let Some(iter) = self.as_str_iter() {
            Some(iter.map(str::to_string).collect())
        } else {
            None
        }
    }

    /// Get a vector of string references.
    /// String references (&str) are faster, but have short lifetimes.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///    let robj1 = Robj::from("xyz");
    ///    assert_eq!(robj1.as_str_vector(), Some(vec!["xyz"]));
    ///    let robj2 = Robj::from(1);
    ///    assert_eq!(robj2.as_str_vector(), None);
    /// }
    /// ```
    pub fn as_str_vector(&self) -> Option<Vec<&str>> {
        if let Some(iter) = self.as_str_iter() {
            Some(iter.collect())
        } else {
            None
        }
    }

    /// Get a read-only reference to a scalar string type.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///    let robj1 = Robj::from("xyz");
    ///    let robj2 = Robj::from(1);
    ///    assert_eq!(robj1.as_str(), Some("xyz"));
    ///    assert_eq!(robj2.as_str(), None);
    /// }
    /// ```
    pub fn as_str<'a>(&self) -> Option<&'a str> {
        unsafe {
            match self.sexptype() {
                STRSXP => {
                    if self.len() != 1 {
                        None
                    } else {
                        Some(to_str(R_CHAR(STRING_ELT(self.get(), 0)) as *const u8))
                    }
                }
                // CHARSXP => Some(to_str(R_CHAR(self.get()) as *const u8)),
                // SYMSXP => Some(to_str(R_CHAR(PRINTNAME(self.get())) as *const u8)),
                _ => None,
            }
        }
    }

    /// Get a scalar integer.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///    let robj1 = Robj::from("xyz");
    ///    let robj2 = Robj::from(1);
    ///    let robj3 = Robj::from(NA_INTEGER);
    ///    assert_eq!(robj1.as_integer(), None);
    ///    assert_eq!(robj2.as_integer(), Some(1));
    ///    assert_eq!(robj3.as_integer(), None);
    /// }
    /// ```
    pub fn as_integer(&self) -> Option<i32> {
        match self.as_integer_slice() {
            Some(slice) if slice.len() == 1 && !slice[0].is_na() => Some(slice[0]),
            _ => None,
        }
    }

    /// Get a scalar real.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///    let robj1 = Robj::from(1);
    ///    let robj2 = Robj::from(1.);
    ///    let robj3 = Robj::from(NA_REAL);
    ///    assert_eq!(robj1.as_real(), None);
    ///    assert_eq!(robj2.as_real(), Some(1.));
    ///    assert_eq!(robj3.as_real(), None);
    /// }
    /// ```
    pub fn as_real(&self) -> Option<f64> {
        match self.as_real_slice() {
            Some(slice) if slice.len() == 1 && !slice[0].is_na() => Some(slice[0]),
            _ => None,
        }
    }

    /// Get a scalar rust boolean.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///    let robj1 = Robj::from(TRUE);
    ///    let robj2 = Robj::from(1.);
    ///    let robj3 = Robj::from(NA_LOGICAL);
    ///    assert_eq!(robj1.as_bool(), Some(true));
    ///    assert_eq!(robj2.as_bool(), None);
    ///    assert_eq!(robj3.as_bool(), None);
    /// }
    /// ```
    pub fn as_bool(&self) -> Option<bool> {
        match self.as_logical_slice() {
            Some(slice) if slice.len() == 1 && !slice[0].is_na() => Some(slice[0].into()),
            _ => None,
        }
    }

    /// Get a scalar boolean as a tri-boolean [Bool] value.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///    let robj1 = Robj::from(TRUE);
    ///    let robj2 = Robj::from([TRUE, FALSE]);
    ///    let robj3 = Robj::from(NA_LOGICAL);
    ///    assert_eq!(robj1.as_logical(), Some(TRUE));
    ///    assert_eq!(robj2.as_logical(), None);
    ///    assert_eq!(robj3.as_logical(), Some(NA_LOGICAL));
    /// }
    /// ```
    pub fn as_logical(&self) -> Option<Bool> {
        match self.as_logical_slice() {
            Some(slice) if slice.len() == 1 => Some(slice[0]),
            _ => None,
        }
    }

    /// Evaluate the expression in R and return an error or an R object.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///
    ///    let add = lang!("+", 1, 2);
    ///    assert_eq!(add.eval().unwrap(), r!(3));
    /// }
    /// ```
    pub fn eval(&self) -> Result<Robj> {
        single_threaded(|| unsafe {
            let mut error: raw::c_int = 0;
            let res = R_tryEval(self.get(), R_GlobalEnv, &mut error as *mut raw::c_int);
            if error != 0 {
                Err(Error::EvalError {
                    code: r!(self),
                    error,
                })
            } else {
                Ok(new_owned(res))
            }
        })
    }

    /// Evaluate the expression and return NULL or an R object.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///    let bad = lang!("imnotavalidfunctioninR", 1, 2);
    ///    assert_eq!(bad.eval_blind(), r!(NULL));
    /// }
    /// ```
    pub fn eval_blind(&self) -> Robj {
        let res = self.eval();
        if res.is_err() {
            Robj::from(())
        } else {
            Robj::from(res.unwrap())
        }
    }

    /// Return true if the object is owned by this wrapper.
    /// If so, it will be released when the wrapper drops.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///    let owned = r!(1);      // Allocated vector.
    ///    let borrowed = r!(());  // R_NilValue
    ///    assert_eq!(owned.is_owned(), true);
    ///    assert_eq!(borrowed.is_owned(), false);
    /// }
    /// ```
    pub fn is_owned(&self) -> bool {
        match self {
            Robj::Owned(_) => true,
            _ => false,
        }
    }

    // Convert the Robj to an owned one.
    #[doc(hidden)]
    pub fn to_owned(self) -> Robj {
        match self {
            Robj::Owned(_) => self,
            _ => unsafe { new_owned(self.get()) },
        }
    }
}

/// Generic access to typed slices in an Robj.
pub trait AsTypedSlice<'a, T>
where
    Self: 'a,
{
    fn as_typed_slice(&self) -> Option<&'a [T]>
    where
        Self: 'a,
    {
        None
    }

    fn as_typed_slice_mut(&mut self) -> Option<&'a mut [T]>
    where
        Self: 'a,
    {
        None
    }
}

macro_rules! make_typed_slice {
    ($type: ty, $fn: tt, $($sexp: tt),* ) => {
        impl<'a> AsTypedSlice<'a, $type> for Robj
        where
            Self : 'a,
        {
            fn as_typed_slice(&self) -> Option<&'a [$type]> {
                match self.sexptype() {
                    $( $sexp )|* => {
                        unsafe {
                            let ptr = $fn(self.get()) as *const $type;
                            Some(std::slice::from_raw_parts(ptr, self.len()))
                        }
                    }
                    _ => None
                }
            }

            fn as_typed_slice_mut(&mut self) -> Option<&'a mut [$type]> {
                match self.sexptype() {
                    $( $sexp )|* => {
                        unsafe {
                            let ptr = $fn(self.get()) as *mut $type;
                            Some(std::slice::from_raw_parts_mut(ptr, self.len()))
                        }
                    }
                    _ => None
                }
            }
        }
    }
}

make_typed_slice!(Bool, INTEGER, LGLSXP);
make_typed_slice!(i32, INTEGER, INTSXP);
make_typed_slice!(f64, REAL, REALSXP);
make_typed_slice!(u8, RAW, RAWSXP);

/// These are helper functions which give access to common properties of R objects.
#[allow(non_snake_case)]
impl Robj {
    /// Get a specific attribute as a borrowed robj if it exists.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///
    ///    let mut robj = r!("hello");
    ///    robj.set_attrib(Symbol("xyz"), 1);
    ///    assert_eq!(robj.get_attrib(Symbol("xyz")), Some(r!(1)));
    /// }
    /// ```
    pub fn get_attrib<'a, N>(&self, name: N) -> Option<Robj>
    where
        Self: 'a,
        Robj: From<N> + 'a,
    {
        let name = Robj::from(name);
        if self.sexptype() == CHARSXP {
            None
        } else {
            let res = unsafe { new_owned(Rf_getAttrib(self.get(), name.get())) };
            if res.is_null() {
                None
            } else {
                Some(res)
            }
        }
    }

    /// Set a specific attribute and return the object.
    ///
    /// Note that some combinations of attributes are illegal and this will
    /// return an error.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///
    ///    let mut robj = r!("hello").set_attrib(Symbol("xyz"), 1)?;
    ///    assert_eq!(robj.get_attrib(Symbol("xyz")), Some(r!(1)));
    /// }
    /// ```
    pub fn set_attrib<N, V>(&self, name: N, value: V) -> Result<Robj>
    where
        N: Into<Robj>,
        V: Into<Robj>,
    {
        let name = name.into();
        let value = value.into();
        unsafe {
            single_threaded(|| {
                catch_r_error(|| Rf_setAttrib(self.get(), name.get(), value.get()))
                    .map(|_| self.clone())
            })
        }
    }

    /// Get the names attribute as a string iterator if one exists.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///    let list = list!(a = 1, b = 2, c = 3);
    ///    let names : Vec<_> = list.names().unwrap().collect();
    ///    assert_eq!(names, vec!["a", "b", "c"]);
    /// }
    /// ```
    pub fn names(&self) -> Option<StrIter> {
        if let Some(names) = self.get_attrib(names_symbol()) {
            names.as_str_iter()
        } else {
            None
        }
    }

    /// Set the names attribute from a string iterator.
    ///
    /// Returns Error::NamesLengthMismatch if the length of the names does
    /// not match the length of the object.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let mut obj = r!([1, 2, 3]).set_names(&["a", "b", "c"]).unwrap();
    ///     assert_eq!(obj.names().unwrap().collect::<Vec<_>>(), vec!["a", "b", "c"]);
    ///     assert_eq!(r!([1, 2, 3]).set_names(&["a", "b"]), Err(Error::NamesLengthMismatch));
    /// }
    /// ```
    pub fn set_names<T>(&self, names: T) -> Result<Robj>
    where
        T: IntoIterator,
        T::IntoIter: Iterator,
        T::Item: ToVectorValue + AsRef<str>,
    {
        let iter = names.into_iter();
        let robj = iter.collect_robj();
        if robj.len() == self.len() {
            self.set_attrib(names_symbol(), robj)
        } else {
            Err(Error::NamesLengthMismatch)
        }
    }

    /// Get the dim attribute as an integer iterator if one exists.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///
    ///    let array = R!(array(data = c(1, 2, 3, 4), dim = c(2, 2), dimnames = list(c("x", "y"), c("a","b")))).unwrap();
    ///    let dim : Vec<_> = array.dim().unwrap().collect();
    ///    assert_eq!(dim, vec![2, 2]);
    /// }
    /// ```
    pub fn dim(&self) -> Option<Int> {
        if let Some(dim) = self.get_attrib(dim_symbol()) {
            dim.as_integer_iter()
        } else {
            None
        }
    }

    /// Get the dimnames attribute as a list iterator if one exists.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///    let array = R!(array(data = c(1, 2, 3, 4), dim = c(2, 2), dimnames = list(c("x", "y"), c("a","b")))).unwrap();
    ///    let names : Vec<_> = array.dimnames().unwrap().collect();
    ///    assert_eq!(names, vec![r!(["x", "y"]), r!(["a", "b"])]);
    /// }
    /// ```
    pub fn dimnames(&self) -> Option<ListIter> {
        if let Some(names) = self.get_attrib(dimnames_symbol()) {
            names.as_list_iter()
        } else {
            None
        }
    }

    /// Return an iterator over names and values of a list if they exist.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///    let list = list!(a = 1, b = 2, c = 3);
    ///    let names_and_values : Vec<_> = list.as_named_list_iter().unwrap().collect();
    ///    assert_eq!(names_and_values, vec![("a", r!(1)), ("b", r!(2)), ("c", r!(3))]);
    /// }
    /// ```
    pub fn as_named_list_iter(&self) -> Option<NamedListIter> {
        if let Some(names) = self.names() {
            if let Some(values) = self.as_list_iter() {
                return Some(names.zip(values));
            }
        }
        None
    }

    /// Get the class attribute as a string iterator if one exists.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///    let formula = R!(y ~ A * x + b).unwrap();
    ///    let class : Vec<_> = formula.class().unwrap().collect();
    ///    assert_eq!(class, ["formula"]);
    /// }
    /// ```
    pub fn class(&self) -> Option<StrIter> {
        if let Some(class) = self.get_attrib(class_symbol()) {
            class.as_str_iter()
        } else {
            None
        }
    }

    /// Set the class attribute from a string iterator.
    ///
    /// May return an error for some class names.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let mut obj = r!([1, 2, 3]).set_class(&["a", "b", "c"])?;
    ///     assert_eq!(obj.class().unwrap().collect::<Vec<_>>(), vec!["a", "b", "c"]);
    ///     assert_eq!(obj.inherits("a"), true);
    /// }
    /// ```
    pub fn set_class<T>(&self, class: T) -> Result<Robj>
    where
        T: IntoIterator,
        T::IntoIter: Iterator,
        T::Item: ToVectorValue + AsRef<str>,
    {
        let iter = class.into_iter();
        self.set_attrib(class_symbol(), iter.collect_robj())
    }

    /// Return true if this class inherits this class.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///    let formula = R!(y ~ A * x + b).unwrap();
    ///    assert_eq!(formula.inherits("formula"), true);
    /// }
    /// ```
    pub fn inherits(&self, classname: &str) -> bool {
        if let Some(mut iter) = self.class() {
            iter.find(|&n| n == classname).is_some()
        } else {
            false
        }
    }

    /// Get the levels attribute as a string iterator if one exists.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///    let factor = factor!(vec!["abcd", "def", "fg", "fg"]);
    ///    let levels : Vec<_> = factor.levels().unwrap().collect();
    ///    assert_eq!(levels, vec!["abcd", "def", "fg"]);
    /// }
    /// ```
    pub fn levels(&self) -> Option<StrIter> {
        if let Some(levels) = self.get_attrib(levels_symbol()) {
            levels.as_str_iter()
        } else {
            None
        }
    }

    /// Get the names in an environment.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///    let names_and_values : std::collections::HashMap<_, _> = (0..4).map(|i| (format!("n{}", i), r!(i))).collect();
    ///    let env = r!(Env{parent: global_env(), names_and_values});
    ///    assert_eq!(env.ls().unwrap(), vec!["n0", "n1", "n2", "n3"]);
    /// }
    /// ```
    pub fn ls(&self) -> Option<Vec<&str>> {
        self.as_env_iter()
            .map(|iter| iter.map(|(k, _)| k).collect::<Vec<_>>())
    }
}

#[doc(hidden)]
pub unsafe fn new_owned(sexp: SEXP) -> Robj {
    single_threaded(|| ownership::protect(sexp));
    Robj::Owned(sexp)
}

#[doc(hidden)]
pub unsafe fn new_sys(sexp: SEXP) -> Robj {
    Robj::Sys(sexp)
}

/// Compare equality with integer slices.
impl<'a> PartialEq<[i32]> for Robj {
    fn eq(&self, rhs: &[i32]) -> bool {
        self.as_integer_slice() == Some(rhs)
    }
}

/// Compare equality with slices of double.
impl<'a> PartialEq<[f64]> for Robj {
    fn eq(&self, rhs: &[f64]) -> bool {
        self.as_real_slice() == Some(rhs)
    }
}

/// Compare equality with strings.
impl PartialEq<str> for Robj {
    fn eq(&self, rhs: &str) -> bool {
        self.as_str() == Some(rhs)
    }
}

/// Compare equality with two Robjs.
impl PartialEq<Robj> for Robj {
    fn eq(&self, rhs: &Robj) -> bool {
        unsafe {
            if self.get() == rhs.get() {
                return true;
            }
            if self.sexptype() == rhs.sexptype() && self.len() == rhs.len() {
                let lsexp = self.get();
                let rsexp = rhs.get();
                match self.sexptype() {
                    NILSXP => true,
                    SYMSXP => PRINTNAME(lsexp) == PRINTNAME(rsexp),
                    LISTSXP | LANGSXP | DOTSXP => self
                        .as_pairlist_iter()
                        .unwrap()
                        .eq(rhs.as_pairlist_iter().unwrap()),
                    CLOSXP => false,
                    ENVSXP => false, // objects must match.
                    PROMSXP => false,
                    SPECIALSXP => false,
                    BUILTINSXP => false,
                    CHARSXP => self.as_character() == rhs.as_character(),
                    LGLSXP => self.as_logical_slice() == rhs.as_logical_slice(),
                    INTSXP => self.as_integer_slice() == rhs.as_integer_slice(),
                    REALSXP => self.as_real_slice() == rhs.as_real_slice(),
                    CPLXSXP => false,
                    ANYSXP => false,
                    VECSXP | EXPRSXP | WEAKREFSXP => {
                        self.as_list_iter().unwrap().eq(rhs.as_list_iter().unwrap())
                    }
                    STRSXP => self.as_str_iter().unwrap().eq(rhs.as_str_iter().unwrap()),
                    BCODESXP => false,
                    EXTPTRSXP => false,
                    RAWSXP => self.as_raw_slice() == rhs.as_raw_slice(),
                    S4SXP => false,
                    NEWSXP => false,
                    FREESXP => false,
                    _ => false,
                }
            } else {
                false
            }
        }
    }
}

/// Implement {:?} formatting.
impl std::fmt::Debug for Robj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.sexptype() {
            NILSXP => write!(f, "r!(NULL)"),
            SYMSXP => {
                if self.is_missing_arg() {
                    write!(f, "missing_arg()")
                } else if self.is_unbound_value() {
                    write!(f, "unbound_value()")
                } else {
                    write!(f, "sym!({})", self.as_symbol().unwrap().0)
                }
            }
            LISTSXP => write!(f, "r!({:?})", self.as_pairlist().unwrap()),
            CLOSXP => write!(f, "r!(Function())"),
            ENVSXP => unsafe {
                let sexp = self.get();
                if sexp == R_GlobalEnv {
                    write!(f, "global_env()")
                } else if sexp == R_BaseEnv {
                    write!(f, "base_env()")
                } else if sexp == R_EmptyEnv {
                    write!(f, "empty_env()")
                } else {
                    write!(f, "r!({:?})", self.as_environment().unwrap())
                }
            },
            PROMSXP => write!(f, "r!(Promise())"),
            LANGSXP => write!(f, "r!({:?})", self.as_lang().unwrap()),
            SPECIALSXP => write!(f, "r!(Special())"),
            BUILTINSXP => write!(f, "r!(Builtin())"),
            CHARSXP => write!(f, "r!({:?})", self.as_character().unwrap()),
            LGLSXP => {
                let slice = self.as_logical_slice().unwrap();
                if slice.len() == 1 {
                    write!(
                        f,
                        "{}",
                        if slice[0].0 == 0 {
                            "r!(FALSE)"
                        } else {
                            "r!(TRUE)"
                        }
                    )
                } else {
                    write!(f, "r!({:?})", slice)
                }
            }
            INTSXP => {
                let slice = self.as_integer_slice().unwrap();
                if slice.len() == 1 {
                    write!(f, "r!({:?})", slice[0])
                } else {
                    write!(f, "r!({:?})", self.as_integer_slice().unwrap())
                }
            }
            REALSXP => {
                let slice = self.as_real_slice().unwrap();
                if slice.len() == 1 {
                    write!(f, "r!({:?})", slice[0])
                } else {
                    write!(f, "r!({:?})", slice)
                }
            }
            VECSXP => write!(f, "r!({:?})", self.as_list().unwrap()),
            EXPRSXP => write!(f, "r!({:?})", self.as_expr().unwrap()),
            WEAKREFSXP => write!(
                f,
                "r!(Weakref({:?}))",
                self.as_list_iter().unwrap().collect::<Vec<_>>()
            ),
            // CPLXSXP => false,
            STRSXP => {
                write!(f, "r!([")?;
                let mut sep = "";
                for s in self.as_str_iter().unwrap() {
                    // if s.is_na() {
                    //     write!(f, "{}na_str()", sep)?;
                    // } else {
                    write!(f, "{}{:?}", sep, s)?;
                    // }
                    sep = ", ";
                }
                write!(f, "])")
            }
            DOTSXP => write!(f, "r!(Dot())"),
            ANYSXP => write!(f, "r!(Any())"),
            BCODESXP => write!(f, "r!(Bcode())"),
            EXTPTRSXP => write!(f, "r!(Extptr())"),
            RAWSXP => {
                write!(f, "r!({:?})", self.as_raw().unwrap())
            }
            S4SXP => write!(f, "r!(S4())"),
            NEWSXP => write!(f, "r!(New())"),
            FREESXP => write!(f, "r!(Free())"),
            _ => write!(f, "??"),
        }
    }
}

// Internal utf8 to str conversion.
// Lets not worry about non-ascii/unicode strings for now (or ever).
pub(crate) unsafe fn to_str<'a>(ptr: *const u8) -> &'a str {
    let mut len = 0;
    loop {
        if *ptr.offset(len) == 0 {
            break;
        }
        len += 1;
    }
    let slice = std::slice::from_raw_parts(ptr, len as usize);
    std::str::from_utf8_unchecked(slice)
}

/// Release any owned objects.
impl Drop for Robj {
    fn drop(&mut self) {
        unsafe {
            match self {
                Robj::Owned(sexp) => ownership::unprotect(*sexp),
                Robj::Sys(_) => (),
            }
        }
    }
}
