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

use libR_sys::*;
use prelude::{c64, Rcplx};
use std::os::raw;

use crate::*;

use crate::scalar::{Rbool, Rfloat, Rint};
use std::collections::HashMap;
use std::iter::IntoIterator;
use std::ops::{Range, RangeInclusive};

// deprecated
mod from_robj;

mod debug;
mod into_robj;
mod operators;
mod rinternals;
mod try_from_robj;

#[cfg(test)]
mod tests;

pub use from_robj::*;
pub use into_robj::*;
pub use iter::*;
pub use operators::Operators;
pub use operators::*;
pub use rinternals::Rinternals;

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
///     assert_eq!(a, r!([3, 6, 9]));
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
pub struct Robj {
    inner: SEXP,
}

impl Clone for Robj {
    fn clone(&self) -> Self {
        unsafe { Robj::from_sexp(self.get()) }
    }
}

impl Default for Robj {
    fn default() -> Self {
        Robj::from(())
    }
}

pub trait GetSexp {
    /// Get a copy of the underlying SEXP.
    ///
    /// # Safety
    ///
    /// Access to a raw SEXP pointer can cause undefined behaviour and is not thread safe.
    unsafe fn get(&self) -> SEXP;

    unsafe fn get_mut(&mut self) -> SEXP;

    /// Get a reference to a Robj for this type.
    fn as_robj(&self) -> &Robj;

    /// Get a mutable reference to a Robj for this type.
    fn as_robj_mut(&mut self) -> &mut Robj;
}

impl GetSexp for Robj {
    unsafe fn get(&self) -> SEXP {
        self.inner
    }

    unsafe fn get_mut(&mut self) -> SEXP {
        self.inner
    }

    fn as_robj(&self) -> &Robj {
        unsafe { std::mem::transmute(&self.inner) }
    }

    fn as_robj_mut(&mut self) -> &mut Robj {
        unsafe { std::mem::transmute(&mut self.inner) }
    }
}

pub trait Slices: GetSexp {
    /// Get an immutable slice to this object's data.
    ///
    /// # Safety
    ///
    /// Unless the type is correct, this will cause undefined behaviour.
    /// Creating this slice will also instatiate and Altrep objects.
    unsafe fn as_typed_slice_raw<T>(&self) -> &[T] {
        let len = XLENGTH(self.get()) as usize;
        let data = DATAPTR_RO(self.get()) as *const T;
        std::slice::from_raw_parts(data, len)
    }

    /// Get a mutable slice to this object's data.
    ///
    /// # Safety
    ///
    /// Unless the type is correct, this will cause undefined behaviour.
    /// Creating this slice will also instatiate and Altrep objects.
    /// Not all obejects (especially not list and strings) support this.
    unsafe fn as_typed_slice_raw_mut<T>(&mut self) -> &mut [T] {
        let len = XLENGTH(self.get()) as usize;
        let data = DATAPTR(self.get_mut()) as *mut T;
        std::slice::from_raw_parts_mut(data, len)
    }
}

impl Slices for Robj {}

pub trait Length: GetSexp {
    /// Get the extended length of the object.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///
    /// let a : Robj = r!(vec![1., 2., 3., 4.]);
    /// assert_eq!(a.len(), 4);
    /// }
    /// ```
    fn len(&self) -> usize {
        unsafe { Rf_xlength(self.get()) as usize }
    }

    /// Returns `true` if the `Robj` contains no elements.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///
    /// let a : Robj = r!(vec![0.; 0]); // length zero of numeric vector
    /// assert_eq!(a.is_empty(), true);
    /// }
    /// ```
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Length for Robj {}

impl Robj {
    pub fn from_sexp(sexp: SEXP) -> Self {
        single_threaded(|| {
            unsafe { ownership::protect(sexp) };
            Robj { inner: sexp }
        })
    }

    /// A ref of an robj can be constructed from a ref to a SEXP
    /// as they have the same layout.
    pub fn from_sexp_ref(sexp: &SEXP) -> &Self {
        unsafe { std::mem::transmute(sexp) }
    }
}

pub trait Types: GetSexp {
    #[doc(hidden)]
    /// Get the XXXSXP type of the object.
    fn sexptype(&self) -> u32 {
        unsafe { TYPEOF(self.get()) as u32 }
    }

    /// Get the type of an R object.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     assert_eq!(r!(NULL).rtype(), Rtype::Null);
    ///     assert_eq!(sym!(xyz).rtype(), Rtype::Symbol);
    ///     assert_eq!(r!(Pairlist::from_pairs(vec![("a", r!(1))])).rtype(), Rtype::Pairlist);
    ///     assert_eq!(R!("function() {}")?.rtype(), Rtype::Function);
    ///     assert_eq!(Environment::new_with_parent(global_env()).rtype(), Rtype::Environment);
    ///     assert_eq!(lang!("+", 1, 2).rtype(), Rtype::Language);
    ///     assert_eq!(r!(Primitive::from_string("if")).rtype(), Rtype::Special);
    ///     assert_eq!(r!(Primitive::from_string("+")).rtype(), Rtype::Builtin);
    ///     assert_eq!(r!(Rstr::from_string("hello")).rtype(), Rtype::Rstr);
    ///     assert_eq!(r!(TRUE).rtype(), Rtype::Logicals);
    ///     assert_eq!(r!(1).rtype(), Rtype::Integers);
    ///     assert_eq!(r!(1.0).rtype(), Rtype::Doubles);
    ///     assert_eq!(r!("1").rtype(), Rtype::Strings);
    ///     assert_eq!(r!(List::from_values(&[1, 2])).rtype(), Rtype::List);
    ///     assert_eq!(parse("x + y")?.rtype(), Rtype::Expressions);
    ///     assert_eq!(r!(Raw::from_bytes(&[1_u8, 2, 3])).rtype(), Rtype::Raw);
    /// }
    /// ```
    fn rtype(&self) -> Rtype {
        match self.sexptype() {
            NILSXP => Rtype::Null,
            SYMSXP => Rtype::Symbol,
            LISTSXP => Rtype::Pairlist,
            CLOSXP => Rtype::Function,
            ENVSXP => Rtype::Environment,
            PROMSXP => Rtype::Promise,
            LANGSXP => Rtype::Language,
            SPECIALSXP => Rtype::Special,
            BUILTINSXP => Rtype::Builtin,
            CHARSXP => Rtype::Rstr,
            LGLSXP => Rtype::Logicals,
            INTSXP => Rtype::Integers,
            REALSXP => Rtype::Doubles,
            CPLXSXP => Rtype::Complexes,
            STRSXP => Rtype::Strings,
            DOTSXP => Rtype::Dot,
            ANYSXP => Rtype::Any,
            VECSXP => Rtype::List,
            EXPRSXP => Rtype::Expressions,
            BCODESXP => Rtype::Bytecode,
            EXTPTRSXP => Rtype::ExternalPtr,
            WEAKREFSXP => Rtype::WeakRef,
            RAWSXP => Rtype::Raw,
            S4SXP => Rtype::S4,
            _ => Rtype::Unknown,
        }
    }

    fn as_any(&self) -> Rany {
        unsafe {
            match self.sexptype() {
                NILSXP => Rany::Null(std::mem::transmute(self.as_robj())),
                SYMSXP => Rany::Symbol(std::mem::transmute(self.as_robj())),
                LISTSXP => Rany::Pairlist(std::mem::transmute(self.as_robj())),
                CLOSXP => Rany::Function(std::mem::transmute(self.as_robj())),
                ENVSXP => Rany::Environment(std::mem::transmute(self.as_robj())),
                PROMSXP => Rany::Promise(std::mem::transmute(self.as_robj())),
                LANGSXP => Rany::Language(std::mem::transmute(self.as_robj())),
                SPECIALSXP => Rany::Special(std::mem::transmute(self.as_robj())),
                BUILTINSXP => Rany::Builtin(std::mem::transmute(self.as_robj())),
                CHARSXP => Rany::Rstr(std::mem::transmute(self.as_robj())),
                LGLSXP => Rany::Logicals(std::mem::transmute(self.as_robj())),
                INTSXP => Rany::Integers(std::mem::transmute(self.as_robj())),
                REALSXP => Rany::Doubles(std::mem::transmute(self.as_robj())),
                CPLXSXP => Rany::Complexes(std::mem::transmute(self.as_robj())),
                STRSXP => Rany::Strings(std::mem::transmute(self.as_robj())),
                DOTSXP => Rany::Dot(std::mem::transmute(self.as_robj())),
                ANYSXP => Rany::Any(std::mem::transmute(self.as_robj())),
                VECSXP => Rany::List(std::mem::transmute(self.as_robj())),
                EXPRSXP => Rany::Expressions(std::mem::transmute(self.as_robj())),
                BCODESXP => Rany::Bytecode(std::mem::transmute(self.as_robj())),
                EXTPTRSXP => Rany::ExternalPtr(std::mem::transmute(self.as_robj())),
                WEAKREFSXP => Rany::WeakRef(std::mem::transmute(self.as_robj())),
                RAWSXP => Rany::Raw(std::mem::transmute(self.as_robj())),
                S4SXP => Rany::S4(std::mem::transmute(self.as_robj())),
                _ => Rany::Unknown(std::mem::transmute(self.as_robj())),
            }
        }
    }
}

impl Types for Robj {}

impl Robj {
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
                match self.sexptype() {
                    STRSXP => STRING_ELT(sexp, 0) == libR_sys::R_NaString,
                    INTSXP => *(INTEGER(sexp)) == libR_sys::R_NaInt,
                    LGLSXP => *(LOGICAL(sexp)) == libR_sys::R_NaInt,
                    REALSXP => R_IsNA(*(REAL(sexp))) != 0,
                    CPLXSXP => R_IsNA((*COMPLEX(sexp)).r) != 0,
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

    /// Convert an [`Robj`] into [`Integers`].
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
    /// using the tri-state [Rbool]. Returns None if not a logical vector.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let robj = r!([TRUE, FALSE]);
    ///     assert_eq!(robj.as_logical_slice().unwrap(), [TRUE, FALSE]);
    /// }
    /// ```
    pub fn as_logical_slice(&self) -> Option<&[Rbool]> {
        self.as_typed_slice()
    }

    /// Get a `Vec<Rbool>` copied from the object
    /// using the tri-state [`Rbool`].
    /// Returns `None` if not a logical vector.
    ///
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let robj = r!([TRUE, FALSE]);
    ///     assert_eq!(robj.as_logical_vector().unwrap(), vec![TRUE, FALSE]);
    /// }
    /// ```
    pub fn as_logical_vector(&self) -> Option<Vec<Rbool>> {
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
    pub fn as_logical_iter(&self) -> Option<impl Iterator<Item = &Rbool>> {
        self.as_logical_slice().map(|slice| slice.iter())
    }

    /// Get a read-only reference to the content of a double vector.
    /// Note: the slice may contain NaN or NA values.
    /// We may introduce a "Real" type to handle this like the Rbool type.
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
    ///    let robj1 = Robj::from("xyz");
    ///    assert_eq!(robj1.as_string_vector(), Some(vec!["xyz".to_string()]));
    ///    let robj2 = Robj::from(1);
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
    ///    let robj1 = Robj::from("xyz");
    ///    assert_eq!(robj1.as_str_vector(), Some(vec!["xyz"]));
    ///    let robj2 = Robj::from(1);
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
            Some(slice) if slice.len() == 1 && !slice[0].is_na() => Some(slice[0].is_true()),
            _ => None,
        }
    }

    /// Get a scalar boolean as a tri-boolean [Rbool] value.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///    let robj1 = Robj::from(TRUE);
    ///    let robj2 = Robj::from([TRUE, FALSE]);
    ///    let robj3 = Robj::from(NA_LOGICAL);
    ///    assert_eq!(robj1.as_logical(), Some(TRUE));
    ///    assert_eq!(robj2.as_logical(), None);
    ///    assert_eq!(robj3.as_logical().unwrap().is_na(), true);
    /// }
    /// ```
    pub fn as_logical(&self) -> Option<Rbool> {
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
    fn eval(&self) -> Result<Robj> {
        self.eval_with_env(&global_env())
    }

    /// Evaluate the expression in R and return an error or an R object.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///
    ///    let add = lang!("+", 1, 2);
    ///    assert_eq!(add.eval_with_env(&global_env()).unwrap(), r!(3));
    /// }
    /// ```
    fn eval_with_env(&self, env: &Environment) -> Result<Robj> {
        single_threaded(|| unsafe {
            let mut error: raw::c_int = 0;
            let res = R_tryEval(self.get(), env.get(), &mut error as *mut raw::c_int);
            if error != 0 {
                Err(Error::EvalError(Robj::from_sexp(self.get())))
            } else {
                Ok(Robj::from_sexp(res))
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
    fn eval_blind(&self) -> Robj {
        let res = self.eval();
        if let Ok(robj) = res {
            robj
        } else {
            Robj::from(())
        }
    }
}

impl Eval for Robj {}

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

make_typed_slice!(Rbool, INTEGER, LGLSXP);
make_typed_slice!(i32, INTEGER, INTSXP);
make_typed_slice!(u32, INTEGER, INTSXP);
make_typed_slice!(Rint, INTEGER, INTSXP);
make_typed_slice!(f64, REAL, REALSXP);
make_typed_slice!(Rfloat, REAL, REALSXP);
make_typed_slice!(u8, RAW, RAWSXP);
make_typed_slice!(Rstr, STRING_PTR, STRSXP);
make_typed_slice!(c64, COMPLEX, CPLXSXP);
make_typed_slice!(Rcplx, COMPLEX, CPLXSXP);
make_typed_slice!(Rcomplex, COMPLEX, CPLXSXP);

/// These are helper functions which give access to common properties of R objects.
#[allow(non_snake_case)]
pub trait Attributes: Types + Length {
    /// Get a specific attribute as a borrowed `Robj` if it exists.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///
    ///    let mut robj = r!("hello");
    ///    robj.set_attrib(sym!(xyz), 1);
    ///    assert_eq!(robj.get_attrib(sym!(xyz)), Some(r!(1)));
    /// }
    /// ```
    fn get_attrib<'a, N>(&self, name: N) -> Option<Robj>
    where
        Self: 'a,
        Robj: From<N> + 'a,
    {
        let name = Robj::from(name);
        if self.sexptype() == CHARSXP {
            None
        } else {
            // FIXME: this attribute does not need protection
            let res = unsafe { Robj::from_sexp(Rf_getAttrib(self.get(), name.get())) };
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
        Robj: From<N> + 'a,
    {
        let name = Robj::from(name);
        if self.sexptype() == CHARSXP {
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
    ///    let mut robj = r!("hello").set_attrib(sym!(xyz), 1)?;
    ///    assert_eq!(robj.get_attrib(sym!(xyz)), Some(r!(1)));
    /// }
    /// ```
    fn set_attrib<N, V>(&mut self, name: N, value: V) -> Result<Robj>
    where
        N: Into<Robj>,
        V: Into<Robj>,
    {
        let name = name.into();
        let value = value.into();
        unsafe {
            let sexp = self.get_mut();
            single_threaded(|| {
                catch_r_error(|| Rf_setAttrib(sexp, name.get(), value.get()))
                    // FIXME: there is no reason to re-wrap this, as this mutates
                    // the input `self`, and returns another pointer to the same
                    // object
                    .map(|_| Robj::from_sexp(sexp))
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
    fn names(&self) -> Option<StrIter> {
        if let Some(names) = self.get_attrib(wrapper::symbol::names_symbol()) {
            names.as_str_iter()
        } else {
            None
        }
    }

    /// Return true if this object has names.
    fn has_names(&self) -> bool {
        self.has_attrib(wrapper::symbol::names_symbol())
    }

    /// Set the names attribute from a string iterator.
    ///
    /// Returns `Error::NamesLengthMismatch` if the length of the names does
    /// not match the length of the object.
    ///
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let mut obj = r!([1, 2, 3]).set_names(&["a", "b", "c"]).unwrap();
    ///     assert_eq!(obj.names().unwrap().collect::<Vec<_>>(), vec!["a", "b", "c"]);
    ///     assert_eq!(r!([1, 2, 3]).set_names(&["a", "b"]), Err(Error::NamesLengthMismatch(r!(["a", "b"]))));
    /// }
    /// ```
    fn set_names<T>(&mut self, names: T) -> Result<Robj>
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

    /// Get the dim attribute as an integer iterator if one exists.
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

    /// Get the dimnames attribute as a list iterator if one exists.
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

    /// Get the class attribute as a string iterator if one exists.
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

    /// Set the class attribute from a string iterator, and returns the same
    /// object.
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
    fn set_class<T>(&mut self, class: T) -> Result<Robj>
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

    /// Get the levels attribute as a string iterator if one exists.
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

impl Attributes for Robj {}

/// Compare equality with integer slices.
impl PartialEq<[i32]> for Robj {
    fn eq(&self, rhs: &[i32]) -> bool {
        self.as_integer_slice() == Some(rhs)
    }
}

/// Compare equality with slices of double.
impl PartialEq<[f64]> for Robj {
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

            // see https://github.com/hadley/r-internals/blob/master/misc.md
            R_compute_identical(self.get(), rhs.get(), 16) != 0
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
            ownership::unprotect(self.inner);
        }
    }
}
