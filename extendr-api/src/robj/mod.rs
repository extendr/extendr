//! R object handling.
//!
//! See. [Writing R Extensions](https://cran.r-project.org/doc/manuals/R-exts.html)
//!
//! Fundamental principals:
//!
//! * Any function that can break the protection mechanism is unsafe.
//! * Users should be able to do almost everything without using `libR_sys`.
//! * The interface should be friendly to R users without Rust experience.
//!

use std::collections::HashMap;
use std::iter::IntoIterator;
use std::ops::{Range, RangeInclusive};
use std::os::raw;

use extendr_ffi::{
    dataptr, R_IsNA, R_NilValue, R_compute_identical, R_tryEval, Rboolean, Rcomplex, Rf_getAttrib,
    Rf_setAttrib, Rf_xlength, COMPLEX, INTEGER, LOGICAL, PRINTNAME, RAW, REAL, SEXPTYPE,
    SEXPTYPE::*, STRING_ELT, STRING_PTR_RO, TYPEOF, XLENGTH,
};

use crate::scalar::{RBool, RFloat, RInt};
use crate::*;
pub use into_robj::*;

#[deprecated(note = "Use IntoRObj instead", since = "0.9.0")]
pub use into_robj::IntoRObj as IntoRobj;
#[deprecated(note = "Use RObjIterTools instead", since = "0.9.0")]
pub use into_robj::RObjIterTools as RobjItertools;

pub use iter::*;
pub use operators::Operators;
use prelude::{c64, RCplx};
pub use rinternals::RInternals;

#[deprecated(note = "Use RInternals instead", since = "0.9.0")]
pub use rinternals::RInternals as Rinternals;

mod debug;
mod into_robj;
mod operators;
mod rinternals;
mod try_from_robj;

#[cfg(test)]
mod tests;

/// Wrapper for an R S-expression pointer (SEXP).
///
/// Create R objects from rust types and iterators:
///
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     // Different ways of making integer scalar 1.
///     let non_na : Option<i32> = Some(1);
///     let a : RObj = vec![1].into();
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
///     let a : RObj = true.into();
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
///     assert_eq!(a, r!([3, 6, 9]));
/// }
/// ```
///
/// Convert to/from Rust vectors.
///
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let a : RObj = r!(vec![1., 2., 3., 4.]);
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
///     let names_and_values : Vec<_> = abc.as_list().unwrap().iter().collect();
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
#[repr(transparent)]
pub struct RObj {
    inner: SEXP,
}

#[deprecated(note = "Use RObj instead", since = "0.9.0")]
pub type Robj = RObj;

impl Clone for RObj {
    fn clone(&self) -> Self {
        unsafe { RObj::from_sexp(self.get()) }
    }
}

impl Default for RObj {
    fn default() -> Self {
        RObj::from(())
    }
}

pub trait GetSexp {
    /// Get a copy of the underlying SEXP.
    ///
    /// # Safety
    ///
    /// Access to a raw SEXP pointer can cause undefined behaviour and is not thread safe.
    unsafe fn get(&self) -> SEXP;

    /// # Safety
    ///
    /// Access to a raw SEXP pointer can cause undefined behaviour and is not thread safe.
    unsafe fn get_mut(&mut self) -> SEXP;

    /// Get a reference to a RObj for this type.
    fn as_robj(&self) -> &RObj;

    /// Get a mutable reference to a RObj for this type.
    fn as_robj_mut(&mut self) -> &mut RObj;
}

impl GetSexp for RObj {
    unsafe fn get(&self) -> SEXP {
        self.inner
    }

    unsafe fn get_mut(&mut self) -> SEXP {
        self.inner
    }

    fn as_robj(&self) -> &RObj {
        unsafe { std::mem::transmute(&self.inner) }
    }

    fn as_robj_mut(&mut self) -> &mut RObj {
        unsafe { std::mem::transmute(&mut self.inner) }
    }
}

pub trait Slices: GetSexp {
    /// Get an immutable slice to this object's data.
    ///
    /// # Safety
    ///
    /// Unless the type is correct, this will cause undefined behaviour.
    /// Creating this slice will also instantiate an Altrep objects.
    unsafe fn as_typed_slice_raw<T>(&self) -> &[T] {
        let len = XLENGTH(self.get()) as usize;
        let data = dataptr(self.get()) as *const T;
        std::slice::from_raw_parts(data, len)
    }

    /// Get a mutable slice to this object's data.
    ///
    /// # Safety
    ///
    /// Unless the type is correct, this will cause undefined behaviour.
    /// Creating this slice will also instantiate Altrep objects.
    /// Not all objects (especially not list and strings) support this.
    unsafe fn as_typed_slice_raw_mut<T>(&mut self) -> &mut [T] {
        let len = XLENGTH(self.get()) as usize;
        let data = dataptr(self.get_mut()) as *mut T;
        std::slice::from_raw_parts_mut(data, len)
    }
}

impl Slices for RObj {}

pub trait Length: GetSexp {
    /// Get the extended length of the object.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///
    /// let a : RObj = r!(vec![1., 2., 3., 4.]);
    /// assert_eq!(a.len(), 4);
    /// }
    /// ```
    fn len(&self) -> usize {
        unsafe { Rf_xlength(self.get()) as usize }
    }

    /// Returns `true` if the `RObj` contains no elements.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///
    /// let a : RObj = r!(vec![0.; 0]); // length zero of numeric vector
    /// assert_eq!(a.is_empty(), true);
    /// }
    /// ```
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Length for RObj {}

impl RObj {
    /// # Safety
    ///
    /// This function dereferences a raw SEXP pointer.
    /// The caller must ensure that `sexp` is a valid SEXP pointer.
    pub unsafe fn from_sexp(sexp: SEXP) -> Self {
        single_threaded(|| {
            unsafe { ownership::protect(sexp) };
            RObj { inner: sexp }
        })
    }
}

pub trait Types: GetSexp {
    #[doc(hidden)]
    /// Get the XXXSXP type of the object.
    fn sexptype(&self) -> SEXPTYPE {
        unsafe { TYPEOF(self.get()) }
    }

    /// Get the type of an R object.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     assert_eq!(r!(NULL).rtype(), RType::Null);
    ///     assert_eq!(sym!(xyz).rtype(), RType::Symbol);
    ///     assert_eq!(r!(PairList::from_pairs(vec![("a", r!(1))])).rtype(), RType::PairList);
    ///     assert_eq!(R!("function() {}")?.rtype(), RType::Function);
    ///     assert_eq!(Environment::new_with_parent(Environment::global()).rtype(), RType::Environment);
    ///     assert_eq!(lang!("+", 1, 2).rtype(), RType::Language);
    ///     assert_eq!(RStr::from_string("hello").rtype(), RType::RStr);
    ///     assert_eq!(r!(TRUE).rtype(), RType::Logicals);
    ///     assert_eq!(r!(1).rtype(), RType::Integers);
    ///     assert_eq!(r!(1.0).rtype(), RType::Doubles);
    ///     assert_eq!(r!("1").rtype(), RType::Strings);
    ///     assert_eq!(r!(List::from_values(&[1, 2])).rtype(), RType::List);
    ///     assert_eq!(Expressions::from_str("x + y")?.rtype(), RType::Expressions);
    ///     assert_eq!(r!(Raw::from_bytes(&[1_u8, 2, 3])).rtype(), RType::Raw);
    /// }
    /// ```
    fn rtype(&self) -> RType {
        use SEXPTYPE::*;
        match self.sexptype() {
            NILSXP => RType::Null,
            SYMSXP => RType::Symbol,
            LISTSXP => RType::PairList,
            CLOSXP => RType::Function,
            ENVSXP => RType::Environment,
            PROMSXP => RType::Promise,
            LANGSXP => RType::Language,
            SPECIALSXP => RType::Special,
            BUILTINSXP => RType::Builtin,
            CHARSXP => RType::RStr,
            LGLSXP => RType::Logicals,
            INTSXP => RType::Integers,
            REALSXP => RType::Doubles,
            CPLXSXP => RType::Complexes,
            STRSXP => RType::Strings,
            DOTSXP => RType::Dot,
            ANYSXP => RType::Any,
            VECSXP => RType::List,
            EXPRSXP => RType::Expressions,
            BCODESXP => RType::Bytecode,
            EXTPTRSXP => RType::ExternalPtr,
            WEAKREFSXP => RType::WeakRef,
            RAWSXP => RType::Raw,
            #[cfg(not(use_objsxp))]
            S4SXP => RType::S4,
            #[cfg(use_objsxp)]
            OBJSXP => RType::S4,
            _ => RType::Unknown,
        }
    }

    fn as_any(&self) -> RAny<'_> {
        use SEXPTYPE::*;
        unsafe {
            match self.sexptype() {
                NILSXP => RAny::Null(self.as_robj()),
                SYMSXP => RAny::Symbol(std::mem::transmute::<&RObj, &Symbol>(self.as_robj())),
                LISTSXP => RAny::PairList(std::mem::transmute::<&RObj, &PairList>(self.as_robj())),
                CLOSXP => RAny::Function(std::mem::transmute::<&RObj, &Function>(self.as_robj())),
                ENVSXP => {
                    RAny::Environment(std::mem::transmute::<&RObj, &Environment>(self.as_robj()))
                }
                PROMSXP => RAny::Promise(std::mem::transmute::<&RObj, &Promise>(self.as_robj())),
                LANGSXP => RAny::Language(std::mem::transmute::<&RObj, &Language>(self.as_robj())),
                SPECIALSXP => {
                    RAny::Special(std::mem::transmute::<&RObj, &Primitive>(self.as_robj()))
                }
                BUILTINSXP => {
                    RAny::Builtin(std::mem::transmute::<&RObj, &Primitive>(self.as_robj()))
                }
                CHARSXP => RAny::RStr(std::mem::transmute::<&RObj, &RStr>(self.as_robj())),
                LGLSXP => RAny::Logicals(std::mem::transmute::<&RObj, &Logicals>(self.as_robj())),
                INTSXP => RAny::Integers(std::mem::transmute::<&RObj, &Integers>(self.as_robj())),
                REALSXP => RAny::Doubles(std::mem::transmute::<&RObj, &Doubles>(self.as_robj())),
                CPLXSXP => {
                    RAny::Complexes(std::mem::transmute::<&RObj, &Complexes>(self.as_robj()))
                }
                STRSXP => RAny::Strings(std::mem::transmute::<&RObj, &Strings>(self.as_robj())),
                DOTSXP => RAny::Dot(std::mem::transmute::<&RObj, &RObj>(self.as_robj())),
                ANYSXP => RAny::Any(std::mem::transmute::<&RObj, &RObj>(self.as_robj())),
                VECSXP => RAny::List(std::mem::transmute::<&RObj, &List>(self.as_robj())),
                EXPRSXP => {
                    RAny::Expressions(std::mem::transmute::<&RObj, &Expressions>(self.as_robj()))
                }
                BCODESXP => RAny::Bytecode(std::mem::transmute::<&RObj, &RObj>(self.as_robj())),
                EXTPTRSXP => RAny::ExternalPtr(std::mem::transmute::<&RObj, &RObj>(self.as_robj())),
                WEAKREFSXP => RAny::WeakRef(std::mem::transmute::<&RObj, &RObj>(self.as_robj())),
                RAWSXP => RAny::Raw(std::mem::transmute::<&RObj, &Raw>(self.as_robj())),
                #[cfg(not(use_objsxp))]
                S4SXP => RAny::S4(std::mem::transmute(self.as_robj())),
                #[cfg(use_objsxp)]
                OBJSXP => RAny::S4(std::mem::transmute::<&RObj, &S4>(self.as_robj())),
                _ => RAny::Unknown(std::mem::transmute::<&RObj, &RObj>(self.as_robj())),
            }
        }
    }
}

impl Types for RObj {}

impl RObj {
    /// Is this object is an `NA` scalar?
    /// Works for character, integer and numeric types.
    ///
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
                use SEXPTYPE::*;
                match self.sexptype() {
                    STRSXP => STRING_ELT(sexp, 0) == extendr_ffi::R_NaString,
                    INTSXP => *(INTEGER(sexp)) == extendr_ffi::R_NaInt,
                    LGLSXP => *(LOGICAL(sexp)) == extendr_ffi::R_NaInt,
                    REALSXP => R_IsNA(*(REAL(sexp))) != 0,
                    CPLXSXP => R_IsNA((*COMPLEX(sexp)).r) != 0,
                    // a character vector contains `CHARSXP`, and thus you
                    // seldom have `RObj`'s that are `CHARSXP` themselves
                    CHARSXP => sexp == extendr_ffi::R_NaString,
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

    /// Convert an [`RObj`] into [`Integers`].
    pub fn as_integers(&self) -> Option<Integers> {
        self.clone().try_into().ok()
    }

    /// Get a `Vec<i32>` copied from the object.
    ///
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///
    /// let robj = r!([1, 2, 3]);
    /// assert_eq!(robj.as_integer_slice().unwrap(), vec![1, 2, 3]);
    /// }
    /// ```
    pub fn as_integer_vector(&self) -> Option<Vec<i32>> {
        self.as_integer_slice().map(|value| value.to_vec())
    }

    /// Get a read-only reference to the content of a logical vector
    /// using the tri-state [RBool]. Returns None if not a logical vector.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let robj = r!([TRUE, FALSE]);
    ///     assert_eq!(robj.as_logical_slice().unwrap(), [TRUE, FALSE]);
    /// }
    /// ```
    pub fn as_logical_slice(&self) -> Option<&[RBool]> {
        self.as_typed_slice()
    }

    /// Get a `Vec<RBool>` copied from the object
    /// using the tri-state [`RBool`].
    /// Returns `None` if not a logical vector.
    ///
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let robj = r!([TRUE, FALSE]);
    ///     assert_eq!(robj.as_logical_vector().unwrap(), vec![TRUE, FALSE]);
    /// }
    /// ```
    pub fn as_logical_vector(&self) -> Option<Vec<RBool>> {
        self.as_logical_slice().map(|value| value.to_vec())
    }

    /// Get an iterator over logical elements of this slice.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let robj = r!([TRUE, FALSE, NA_LOGICAL]);
    ///     let mut num_na = 0;
    ///     for val in robj.as_logical_iter().unwrap() {
    ///       if val.is_na() {
    ///           num_na += 1;
    ///       }
    ///     }
    ///     assert_eq!(num_na, 1);
    /// }
    /// ```
    pub fn as_logical_iter(&self) -> Option<impl Iterator<Item = &RBool>> {
        self.as_logical_slice().map(|slice| slice.iter())
    }

    /// Get a read-only reference to the content of a double vector.
    /// Note: the slice may contain NaN or NA values.
    /// We may introduce a "Real" type to handle this like the RBool type.
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
    ///
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
    pub fn as_real_iter(&self) -> Option<impl Iterator<Item = &f64>> {
        self.as_real_slice().map(|slice| slice.iter())
    }

    /// Get a `Vec<f64>` copied from the object.
    ///
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let robj = r!([1., 2., 3.]);
    ///     assert_eq!(robj.as_real_vector().unwrap(), vec![1., 2., 3.]);
    /// }
    /// ```
    pub fn as_real_vector(&self) -> Option<Vec<f64>> {
        self.as_real_slice().map(|value| value.to_vec())
    }

    /// Get a read-only reference to the content of an integer or logical vector.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let robj = r!(Raw::from_bytes(&[1, 2, 3]));
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
    ///     let mut robj = r!(Raw::from_bytes(&[1, 2, 3]));
    ///     let slice = robj.as_raw_slice_mut().unwrap();
    ///     slice[1] = 100;
    ///     assert_eq!(robj, r!(Raw::from_bytes(&[1, 100, 3])));
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
    ///    let robj1 = RObj::from("xyz");
    ///    assert_eq!(robj1.as_string_vector(), Some(vec!["xyz".to_string()]));
    ///    let robj2 = RObj::from(1);
    ///    assert_eq!(robj2.as_string_vector(), None);
    /// }
    /// ```
    pub fn as_string_vector(&self) -> Option<Vec<String>> {
        self.as_str_iter()
            .map(|iter| iter.map(str::to_string).collect())
    }

    /// Get a vector of string references.
    /// String references (&str) are faster, but have short lifetimes.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///    let robj1 = RObj::from("xyz");
    ///    assert_eq!(robj1.as_str_vector(), Some(vec!["xyz"]));
    ///    let robj2 = RObj::from(1);
    ///    assert_eq!(robj2.as_str_vector(), None);
    /// }
    /// ```
    pub fn as_str_vector(&self) -> Option<Vec<&str>> {
        self.as_str_iter().map(|iter| iter.collect())
    }

    /// Get a read-only reference to a scalar string type.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///    let robj1 = RObj::from("xyz");
    ///    let robj2 = RObj::from(1);
    ///    assert_eq!(robj1.as_str(), Some("xyz"));
    ///    assert_eq!(robj2.as_str(), None);
    /// }
    /// ```
    pub fn as_str<'a>(&self) -> Option<&'a str> {
        unsafe {
            let charsxp = match self.sexptype() {
                STRSXP => {
                    // only allows scalar strings
                    if self.len() != 1 {
                        return None;
                    }
                    STRING_ELT(self.get(), 0)
                }
                CHARSXP => self.get(),
                SYMSXP => PRINTNAME(self.get()),
                _ => return None,
            };
            rstr::charsxp_to_str(charsxp)
        }
    }

    /// Get a scalar integer.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///    let robj1 = RObj::from("xyz");
    ///    let robj2 = RObj::from(1);
    ///    let robj3 = RObj::from(NA_INTEGER);
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
    ///    let robj1 = RObj::from(1);
    ///    let robj2 = RObj::from(1.);
    ///    let robj3 = RObj::from(NA_REAL);
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
    ///    let robj1 = RObj::from(TRUE);
    ///    let robj2 = RObj::from(1.);
    ///    let robj3 = RObj::from(NA_LOGICAL);
    ///    assert_eq!(robj1.as_bool(), Some(true));
    ///    assert_eq!(robj2.as_bool(), None);
    ///    assert_eq!(robj3.as_bool(), None);
    /// }
    /// ```
    pub fn as_bool(&self) -> Option<bool> {
        match self.as_logical_slice() {
            Some(slice) if slice.len() == 1 && !slice[0].is_na() => Some(slice[0].is_true()),
            _ => None,
        }
    }

    /// Get a scalar boolean as a tri-boolean [RBool] value.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///    let robj1 = RObj::from(TRUE);
    ///    let robj2 = RObj::from([TRUE, FALSE]);
    ///    let robj3 = RObj::from(NA_LOGICAL);
    ///    assert_eq!(robj1.as_logical(), Some(TRUE));
    ///    assert_eq!(robj2.as_logical(), None);
    ///    assert_eq!(robj3.as_logical().unwrap().is_na(), true);
    /// }
    /// ```
    pub fn as_logical(&self) -> Option<RBool> {
        match self.as_logical_slice() {
            Some(slice) if slice.len() == 1 => Some(slice[0]),
            _ => None,
        }
    }
}

pub trait Eval: GetSexp {
    /// Evaluate the expression in R and return an error or an R object.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///
    ///    let add = lang!("+", 1, 2);
    ///    assert_eq!(add.eval().unwrap(), r!(3));
    /// }
    /// ```
    fn eval(&self) -> Result<RObj> {
        self.eval_with_env(&Environment::global())
    }

    /// Evaluate the expression in R and return an error or an R object.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///
    ///    let add = lang!("+", 1, 2);
    ///    assert_eq!(add.eval_with_env(&Environment::global()).unwrap(), r!(3));
    /// }
    /// ```
    fn eval_with_env(&self, env: &Environment) -> Result<RObj> {
        single_threaded(|| unsafe {
            let mut error: raw::c_int = 0;
            let res = R_tryEval(self.get(), env.get(), &mut error as *mut raw::c_int);
            if error != 0 {
                Err(Error::EvalError(RObj::from_sexp(self.get())))
            } else {
                Ok(RObj::from_sexp(res))
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
    fn eval_blind(&self) -> RObj {
        let res = self.eval();
        if let Ok(robj) = res {
            robj
        } else {
            RObj::from(())
        }
    }
}

impl Eval for RObj {}

/// Generic access to typed slices in an RObj.
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
        impl<'a> AsTypedSlice<'a, $type> for RObj
        where
            Self : 'a,
        {
            fn as_typed_slice(&self) -> Option<&'a [$type]> {
                match self.sexptype() {
                    $( $sexp )|* => {
                        unsafe {
                            // if the vector is empty return an empty slice
                            if self.is_empty() {
                                return Some(&[])
                            }
                            // otherwise get the slice
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
                            if self.is_empty() {
                                return Some(&mut []);
                            }
                            let ptr = $fn(self.get_mut()) as *mut $type;

                            Some(std::slice::from_raw_parts_mut(ptr, self.len()))

                        }
                    }
                    _ => None
                }
            }
        }
    }
}

make_typed_slice!(RBool, INTEGER, LGLSXP);
make_typed_slice!(i32, INTEGER, INTSXP);
make_typed_slice!(RInt, INTEGER, INTSXP);
make_typed_slice!(f64, REAL, REALSXP);
make_typed_slice!(RFloat, REAL, REALSXP);
make_typed_slice!(u8, RAW, RAWSXP);
make_typed_slice!(RStr, STRING_PTR_RO, STRSXP);
make_typed_slice!(c64, COMPLEX, CPLXSXP);
make_typed_slice!(RCplx, COMPLEX, CPLXSXP);
make_typed_slice!(Rcomplex, COMPLEX, CPLXSXP);

/// Provides access to the attributes of an R object.
///
/// The `Attribute` trait provides a consistent interface to getting, setting, and checking for the presence of attributes in an R object.
///
#[allow(non_snake_case)]
pub trait Attributes: Types + Length {
    /// Get a specific attribute as a borrowed `RObj` if it exists.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///    let mut robj = r!("hello");
    ///    robj.set_attrib(sym!(xyz), 1);
    ///    assert_eq!(robj.get_attrib(sym!(xyz)), Some(r!(1)));
    /// }
    /// ```
    fn get_attrib<'a, N>(&self, name: N) -> Option<RObj>
    where
        Self: 'a,
        RObj: From<N> + 'a,
    {
        let name = RObj::from(name);
        if self.sexptype() == SEXPTYPE::CHARSXP {
            None
        } else {
            // FIXME: this attribute does not need protection
            let res = unsafe { RObj::from_sexp(Rf_getAttrib(self.get(), name.get())) };
            if res.is_null() {
                None
            } else {
                Some(res)
            }
        }
    }

    /// Return true if an attribute exists.
    fn has_attrib<'a, N>(&self, name: N) -> bool
    where
        Self: 'a,
        RObj: From<N> + 'a,
    {
        let name = RObj::from(name);
        if self.sexptype() == SEXPTYPE::CHARSXP {
            false
        } else {
            unsafe { Rf_getAttrib(self.get(), name.get()) != R_NilValue }
        }
    }

    /// Set a specific attribute in-place and return the object.
    ///
    /// Note that some combinations of attributes are illegal and this will
    /// return an error.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///    let mut robj = r!("hello");
    ///    robj.set_attrib(sym!(xyz), 1)?;
    ///    assert_eq!(robj.get_attrib(sym!(xyz)), Some(r!(1)));
    /// }
    /// ```
    fn set_attrib<N, V>(&mut self, name: N, value: V) -> Result<&mut Self>
    where
        N: Into<RObj>,
        V: Into<RObj>,
    {
        let name = name.into();
        let value = value.into();
        unsafe {
            let sexp = self.get_mut();
            let result =
                single_threaded(|| catch_r_error(|| Rf_setAttrib(sexp, name.get(), value.get())));
            result.map(|_| self)
        }
    }

    /// Get the `names` attribute as a string iterator if one exists.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///    let list = list!(a = 1, b = 2, c = 3);
    ///    let names : Vec<_> = list.names().unwrap().collect();
    ///    assert_eq!(names, vec!["a", "b", "c"]);
    /// }
    /// ```
    fn names(&self) -> Option<StrIter> {
        if let Some(names) = self.get_attrib(wrapper::symbol::names_symbol()) {
            names.as_str_iter()
        } else {
            None
        }
    }

    /// Return true if this object has an attribute called `names`.
    fn has_names(&self) -> bool {
        self.has_attrib(wrapper::symbol::names_symbol())
    }

    /// Set the `names` attribute from a string iterator.
    ///
    /// Returns `Error::NamesLengthMismatch` if the length of the names does
    /// not match the length of the object.
    ///
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let mut obj = r!([1, 2, 3]);
    ///     obj.set_names(&["a", "b", "c"]).unwrap();
    ///     assert_eq!(obj.names().unwrap().collect::<Vec<_>>(), vec!["a", "b", "c"]);
    ///     assert_eq!(r!([1, 2, 3]).set_names(&["a", "b"]), Err(Error::NamesLengthMismatch(r!(["a", "b"]))));
    /// }
    /// ```
    fn set_names<T>(&mut self, names: T) -> Result<&mut Self>
    where
        T: IntoIterator,
        T::IntoIter: ExactSizeIterator,
        T::Item: ToVectorValue + AsRef<str>,
    {
        let iter = names.into_iter();
        let robj = iter.collect_robj();
        if !robj.is_vector() && !robj.is_pairlist() {
            Err(Error::ExpectedVector(robj))
        } else if robj.len() != self.len() {
            Err(Error::NamesLengthMismatch(robj))
        } else {
            self.set_attrib(wrapper::symbol::names_symbol(), robj)
        }
    }

    /// Get the `dim` attribute as an integer iterator if one exists.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///
    ///    let array = R!(r#"array(data = c(1, 2, 3, 4), dim = c(2, 2), dimnames = list(c("x", "y"), c("a","b")))"#).unwrap();
    ///    let dim : Vec<_> = array.dim().unwrap().iter().collect();
    ///    assert_eq!(dim, vec![2, 2]);
    /// }
    /// ```
    fn dim(&self) -> Option<Integers> {
        if let Some(dim) = self.get_attrib(wrapper::symbol::dim_symbol()) {
            dim.as_integers()
        } else {
            None
        }
    }

    /// Get the `dimnames` attribute as a list iterator if one exists.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///    let array = R!(r#"array(data = c(1, 2, 3, 4), dim = c(2, 2), dimnames = list(c("x", "y"), c("a","b")))"#).unwrap();
    ///    let names : Vec<_> = array.dimnames().unwrap().collect();
    ///    assert_eq!(names, vec![r!(["x", "y"]), r!(["a", "b"])]);
    /// }
    /// ```
    fn dimnames(&self) -> Option<ListIter> {
        if let Some(names) = self.get_attrib(wrapper::symbol::dimnames_symbol()) {
            names.as_list().map(|v| v.values())
        } else {
            None
        }
    }

    /// Get the `class` attribute as a string iterator if one exists.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///    let formula = R!("y ~ A * x + b").unwrap();
    ///    let class : Vec<_> = formula.class().unwrap().collect();
    ///    assert_eq!(class, ["formula"]);
    /// }
    /// ```
    fn class(&self) -> Option<StrIter> {
        if let Some(class) = self.get_attrib(wrapper::symbol::class_symbol()) {
            class.as_str_iter()
        } else {
            None
        }
    }

    /// Set the `class` attribute from a string iterator, and return the same
    /// object.
    ///
    /// May return an error for some class names.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let mut obj = r!([1, 2, 3]);
    ///     obj.set_class(&["a", "b", "c"])?;
    ///     assert_eq!(obj.class().unwrap().collect::<Vec<_>>(), vec!["a", "b", "c"]);
    ///     assert_eq!(obj.inherits("a"), true);
    /// }
    /// ```
    fn set_class<T>(&mut self, class: T) -> Result<&mut Self>
    where
        T: IntoIterator,
        T::IntoIter: ExactSizeIterator,
        T::Item: ToVectorValue + AsRef<str>,
    {
        let iter = class.into_iter();
        self.set_attrib(wrapper::symbol::class_symbol(), iter.collect_robj())
    }

    /// Return true if this object has this class attribute.
    /// Implicit classes are not supported.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///    let formula = R!("y ~ A * x + b").unwrap();
    ///    assert_eq!(formula.inherits("formula"), true);
    /// }
    /// ```
    fn inherits(&self, classname: &str) -> bool {
        if let Some(mut iter) = self.class() {
            iter.any(|n| n == classname)
        } else {
            false
        }
    }

    /// Get the `levels` attribute as a string iterator if one exists.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///    let factor = factor!(vec!["abcd", "def", "fg", "fg"]);
    ///    let levels : Vec<_> = factor.levels().unwrap().collect();
    ///    assert_eq!(levels, vec!["abcd", "def", "fg"]);
    /// }
    /// ```
    fn levels(&self) -> Option<StrIter> {
        if let Some(levels) = self.get_attrib(wrapper::symbol::levels_symbol()) {
            levels.as_str_iter()
        } else {
            None
        }
    }
}

impl Attributes for RObj {}

/// Compare equality with integer slices.
impl PartialEq<[i32]> for RObj {
    fn eq(&self, rhs: &[i32]) -> bool {
        self.as_integer_slice() == Some(rhs)
    }
}

/// Compare equality with slices of double.
impl PartialEq<[f64]> for RObj {
    fn eq(&self, rhs: &[f64]) -> bool {
        self.as_real_slice() == Some(rhs)
    }
}

/// Compare equality with strings.
impl PartialEq<str> for RObj {
    fn eq(&self, rhs: &str) -> bool {
        self.as_str() == Some(rhs)
    }
}

/// Compare equality with two RObjs.
impl PartialEq<RObj> for RObj {
    fn eq(&self, rhs: &RObj) -> bool {
        unsafe {
            if self.get() == rhs.get() {
                return true;
            }

            // see https://github.com/hadley/r-internals/blob/master/misc.md
            R_compute_identical(self.get(), rhs.get(), 16) != Rboolean::FALSE
        }
    }
}

/// Release any owned objects.
impl Drop for RObj {
    fn drop(&mut self) {
        unsafe {
            ownership::unprotect(self.inner);
        }
    }
}
