use crate::prelude::RInt;
use crate::scalar::macros::*;
use crate::*;
use std::cmp::Ordering::*;
use std::convert::TryFrom;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

/// `RFloat` is a wrapper for `f64` in the context of an R's integer vector.
///
/// `RFloat` has a special `NA` value, obtained from R headers via `R_NaReal`.
///
/// `RFloat` has the same footprint as an `f64` value allowing us to use it in zero copy slices.
#[repr(transparent)]
#[readonly::make]
pub struct RFloat(pub f64);

impl RFloat {
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
    pub fn abs(&self) -> RFloat {
        self.0.abs().into()
    }
    pub fn sqrt(&self) -> RFloat {
        self.0.sqrt().into()
    }

    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     assert!(RFloat::na().min(RFloat::default()).is_na());    
    ///     assert!(RFloat::default().min(RFloat::na()).is_na());
    ///     assert_eq!(RFloat::default().min(RFloat::default()), RFloat::default());
    ///     assert_eq!(RFloat::from(1).min(RFloat::from(2)), RFloat::from(1));    
    ///     assert_eq!(RFloat::from(2).min(RFloat::from(1)), RFloat::from(1));    
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
    ///     assert!(RFloat::na().max(RFloat::default()).is_na());    
    ///     assert!(RFloat::default().max(RFloat::na()).is_na());
    ///     assert_eq!(RFloat::default().max(RFloat::default()), RFloat::default());
    ///     assert_eq!(RFloat::from(1).max(RFloat::from(2)), RFloat::from(2));    
    ///     assert_eq!(RFloat::from(2).max(RFloat::from(1)), RFloat::from(2));    
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
gen_trait_impl!(RFloat, f64, |x: &RFloat| x.0.is_na(), f64::na());
gen_from_primitive!(RFloat, f64);

impl From<RFloat> for Option<f64> {
    fn from(v: RFloat) -> Self {
        if v.is_na() {
            None
        } else {
            Some(v.0)
        }
    }
}

impl From<RFloat> for f64 {
    fn from(v: RFloat) -> Self {
        v.0
    }
}

gen_sum_iter!(RFloat);
gen_partial_ord!(RFloat, f64);

// Generate binary ops for +, -, * and /
gen_binop!(
    RFloat,
    f64,
    Add,
    |lhs: f64, rhs: f64| Some(lhs + rhs),
    "Add two RFloat values or an option of f64."
);
gen_binop!(
    RFloat,
    f64,
    Sub,
    |lhs: f64, rhs: f64| Some(lhs - rhs),
    "Subtract two RFloat values or an option of f64."
);
gen_binop!(
    RFloat,
    f64,
    Mul,
    |lhs: f64, rhs: f64| Some(lhs * rhs),
    "Multiply two RFloat values or an option of f64."
);
gen_binop!(
    RFloat,
    f64,
    Div,
    |lhs: f64, rhs: f64| Some(lhs / rhs),
    "Divide two RFloat values or an option of f64."
);
gen_binopassign!(
    RFloat,
    f64,
    AddAssign,
    |lhs: f64, rhs: f64| Some(lhs + rhs),
    "Add two RFloat values or an option of f64, modifying the left-hand side in place. Overflows to NA."
);
gen_binopassign!(
    RFloat,
    f64,
    SubAssign,
    |lhs: f64, rhs: f64| Some(lhs - rhs),
    "Subtract two RFloat values or an option of f64, modifying the left-hand side in place. Overflows to NA."
);
gen_binopassign!(
    RFloat,
    f64,
    MulAssign,
    |lhs: f64, rhs: f64| Some(lhs * rhs),
    "Multiply two RFloat values or an option of f64, modifying the left-hand side in place. Overflows to NA."
);
gen_binopassign!(
    RFloat,
    f64,
    DivAssign,
    |lhs: f64, rhs: f64| Some(lhs / rhs),
    "Divide two RFloat values or an option of f64, modifying the left-hand side in place. Overflows to NA."
);

// Generate unary ops for -, !
gen_unop!(RFloat, Neg, |lhs: f64| Some(-lhs), "Negate a RFloat value.");

impl From<i32> for RFloat {
    fn from(value: i32) -> Self {
        RFloat::from(value as f64)
    }
}

impl From<RInt> for RFloat {
    fn from(value: RInt) -> Self {
        if value.is_na() {
            RFloat::na()
        } else {
            RFloat::from(value.0)
        }
    }
}

impl TryFrom<&RObj> for RFloat {
    type Error = Error;

    fn try_from(robj: &RObj) -> Result<Self> {
        let f64_val: Result<f64> = robj.try_into();
        match f64_val {
            Ok(val) => Ok(RFloat::from(val)),
            // TODO: Currently this results in an extra protection of robj
            Err(Error::MustNotBeNA(_)) => Ok(RFloat::na()),
            Err(e) => Err(e),
        }
    }
}

impl std::fmt::Debug for RFloat {
    /// Debug format.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_na() {
            write!(f, "NA_REAL")
        } else {
            self.0.fmt(f)
        }
    }
}

#[deprecated(note = "Use RFloat instead", since = "0.9.0")]
pub type Rfloat = RFloat;
