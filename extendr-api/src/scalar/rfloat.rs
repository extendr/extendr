use crate::scalar::macros::*;
use crate::*;
use std::convert::TryFrom;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

/// Rfloat is a wrapper for f64 in the context of an R's integer vector.
///
/// Rfloat has a special NA value, obtained from R headers via R_NaReal.
///
/// Rfloat has the same footprint as an f64 value allowing us to use it in zero copy slices.
#[repr(C)]
pub struct Rfloat(pub f64);

impl Rfloat {
    gen_impl!(Rfloat, f64);

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
}

// `NA_real_` is a `NaN` with specific bit representation.
// Check that underlying `f64` is `NA_real_`.
gen_trait_impl!(Rfloat, f64, |x: &Rfloat| x.inner().is_na(), f64::na());
gen_from_primitive!(Rfloat, f64);
gen_from_scalar!(Rfloat, f64);
gen_sum_iter!(Rfloat);

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

impl TryFrom<&Robj> for Rfloat {
    type Error = Error;

    fn try_from(robj: &Robj) -> Result<Self> {
        // Check if the value is a scalar
        match robj.len() {
            0 => return Err(Error::ExpectedNonZeroLength(robj.clone())),
            1 => {}
            _ => return Err(Error::ExpectedScalar(robj.clone())),
        };

        // Check if the value is not a missing value.
        if robj.is_na() {
            return Ok(Rfloat::na());
        }

        // This should always work, NA is handled above.
        if let Some(v) = robj.as_real() {
            return Ok(Rfloat::from(v));
        }

        // Any integer (32 bit) can be represented as f64,
        // this always works.
        if let Some(v) = robj.as_integer() {
            return Ok(Rfloat::from(v as f64));
        }

        Err(Error::ExpectedNumeric(robj.clone()))
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
