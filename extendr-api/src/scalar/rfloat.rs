use crate::prelude::{Rint, Scalar};
use crate::scalar::macros::*;
use crate::*;
use std::cmp::Ordering::*;
use std::convert::TryFrom;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

/// `Rfloat` is a wrapper for `f64` in the context of an R's integer vector.
///
/// `Rfloat` has a special `NA` value, obtained from R headers via `R_NaReal`.
///
/// `Rfloat` has the same footprint as an `f64` value allowing us to use it in zero copy slices.
#[repr(transparent)]
pub struct Rfloat(f64);

impl Scalar<f64> for Rfloat {
    fn inner(&self) -> f64 {
        self.0
    }

    fn new(val: f64) -> Self {
        Rfloat(val)
    }
}

impl Rfloat {
    pub fn is_nan(&self) -> bool {
        self.0.is_nan()
    }
    pub fn is_sign_positive(&self) -> bool {
        self.0.is_sign_positive()
    }
    pub fn is_sign_negative(&self) -> bool {
        self.0.is_sign_negative()
    }
    pub fn is_infinite(&self) -> bool {
        self.0.is_infinite()
    }
    pub fn is_subnormal(&self) -> bool {
        self.0.is_subnormal()
    }
    pub fn abs(&self) -> Rfloat {
        self.0.abs().into()
    }
    pub fn sqrt(&self) -> Rfloat {
        self.0.sqrt().into()
    }

    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     assert!(Rfloat::na().min(Rfloat::default()).is_na());    
    ///     assert!(Rfloat::default().min(Rfloat::na()).is_na());
    ///     assert_eq!(Rfloat::default().min(Rfloat::default()), Rfloat::default());
    ///     assert_eq!(Rfloat::from(1).min(Rfloat::from(2)), Rfloat::from(1));    
    ///     assert_eq!(Rfloat::from(2).min(Rfloat::from(1)), Rfloat::from(1));    
    /// }
    /// ```
    pub fn min(&self, other: Self) -> Self {
        match self.partial_cmp(&other) {
            Some(Less | Equal) => *self,
            Some(Greater) => other,
            _ => Self::na(),
        }
    }

    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     assert!(Rfloat::na().max(Rfloat::default()).is_na());    
    ///     assert!(Rfloat::default().max(Rfloat::na()).is_na());
    ///     assert_eq!(Rfloat::default().max(Rfloat::default()), Rfloat::default());
    ///     assert_eq!(Rfloat::from(1).max(Rfloat::from(2)), Rfloat::from(2));    
    ///     assert_eq!(Rfloat::from(2).max(Rfloat::from(1)), Rfloat::from(2));    
    /// }
    /// ```
    pub fn max(&self, other: Self) -> Self {
        match self.partial_cmp(&other) {
            Some(Less) => other,
            Some(Greater | Equal) => *self,
            _ => Self::na(),
        }
    }
}

// `NA_real_` is a `NaN` with specific bit representation.
// Check that underlying `f64` is `NA_real_`.
gen_trait_impl!(Rfloat, f64, |x: &Rfloat| x.inner().is_na(), f64::na());
gen_from_primitive!(Rfloat, f64);

impl From<Rfloat> for Option<f64> {
    fn from(v: Rfloat) -> Self {
        if v.is_na() {
            None
        } else {
            Some(v.0)
        }
    }
}

gen_sum_iter!(Rfloat);
gen_partial_ord!(Rfloat, f64);

// Generate binary ops for +, -, * and /
gen_binop!(
    Rfloat,
    f64,
    Add,
    |lhs: f64, rhs: f64| Some(lhs + rhs),
    "Add two Rfloat values or an option of f64."
);
gen_binop!(
    Rfloat,
    f64,
    Sub,
    |lhs: f64, rhs: f64| Some(lhs - rhs),
    "Subtract two Rfloat values or an option of f64."
);
gen_binop!(
    Rfloat,
    f64,
    Mul,
    |lhs: f64, rhs: f64| Some(lhs * rhs),
    "Multiply two Rfloat values or an option of f64."
);
gen_binop!(
    Rfloat,
    f64,
    Div,
    |lhs: f64, rhs: f64| Some(lhs / rhs),
    "Divide two Rfloat values or an option of f64."
);
gen_binopassign!(
    Rfloat,
    f64,
    AddAssign,
    |lhs: f64, rhs: f64| Some(lhs + rhs),
    "Add two Rfloat values or an option of f64, modifying the left-hand side in place. Overflows to NA."
);
gen_binopassign!(
    Rfloat,
    f64,
    SubAssign,
    |lhs: f64, rhs: f64| Some(lhs - rhs),
    "Subtract two Rfloat values or an option of f64, modifying the left-hand side in place. Overflows to NA."
);
gen_binopassign!(
    Rfloat,
    f64,
    MulAssign,
    |lhs: f64, rhs: f64| Some(lhs * rhs),
    "Multiply two Rfloat values or an option of f64, modifying the left-hand side in place. Overflows to NA."
);
gen_binopassign!(
    Rfloat,
    f64,
    DivAssign,
    |lhs: f64, rhs: f64| Some(lhs / rhs),
    "Divide two Rfloat values or an option of f64, modifying the left-hand side in place. Overflows to NA."
);

// Generate unary ops for -, !
gen_unop!(Rfloat, Neg, |lhs: f64| Some(-lhs), "Negate a Rfloat value.");

impl From<i32> for Rfloat {
    fn from(value: i32) -> Self {
        Rfloat::from(value as f64)
    }
}

impl From<Rint> for Rfloat {
    fn from(value: Rint) -> Self {
        if value.is_na() {
            Rfloat::na()
        } else {
            Rfloat::from(value.inner())
        }
    }
}

impl TryFrom<&Robj> for Rfloat {
    type Error = Error;

    fn try_from(robj: &Robj) -> Result<Self> {
        let f64_val: Result<f64> = robj.try_into();
        match f64_val {
            Ok(val) => Ok(Rfloat::from(val)),
            // TODO: Currently this results in an extra protection of robj
            Err(Error::MustNotBeNA(_)) => Ok(Rfloat::na()),
            Err(e) => Err(e),
        }
    }
}

impl std::fmt::Debug for Rfloat {
    /// Debug format.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_na() {
            write!(f, "NA_REAL")
        } else {
            self.inner().fmt(f)
        }
    }
}
