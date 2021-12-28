use crate::scalar::macros::*;
use crate::*;
use std::convert::TryFrom;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};
use crate::scalar::Rfloat;

// #[cfg(feature="num-complex")]
type C64 = num_complex::Complex<f64>;

impl CanBeNA for C64 {
    fn is_na(&self) -> bool {
        unsafe { R_IsNA(self.re) != 0 }
    }

    fn na() -> C64 {
        unsafe { C64::from(R_NaReal) }
    }
}

/// Rcplx is a wrapper for f64 in the context of an R's complex vector.
///
/// Rcplx has a special NA value, obtained from R headers via R_NaReal.
///
/// Rcplx has the same footprint as R's complex value allowing us to use it in zero copy slices.
pub struct Rcplx(C64);

impl Rcplx {
    gen_impl!(Rcplx, C64);

    pub fn is_nan(&self) -> bool {
        self.0.is_nan()
    }

    pub fn is_infinite(&self) -> bool {
        self.0.is_infinite()
    }

    pub fn re(&self) -> Rfloat {
        Rfloat::from(self.0.re)
    }

    pub fn im(&self) -> Rfloat {
        Rfloat::from(self.0.im)
    }
}

impl From<f64> for Rcplx {
    fn from(val: f64) -> Self {
        Rcplx(C64::from(val))
    }
}

impl From<Rfloat> for Rcplx {
    fn from(val: Rfloat) -> Self {
        Rcplx(C64::from(val.inner()))
    }
}

// `NA_real_` is a `NaN` with specific bit representation.
// Check that underlying `f64` is `NA_real_`.
gen_trait_impl!(Rcplx, C64, |x: &Rcplx| x.inner().re.is_na(), C64::na());
gen_from_primitive!(Rcplx, C64);
gen_from_scalar!(Rcplx, C64);
gen_sum_iter!(Rcplx, 0f64);

// Generate binary ops for +, -, * and /
gen_binop!(
    Rcplx,
    C64,
    Add,
    |lhs: C64, rhs: C64| Some(lhs + rhs),
    "Add two Rcplx values or an option of C64."
);
gen_binop!(
    Rcplx,
    C64,
    Sub,
    |lhs: C64, rhs: C64| Some(lhs - rhs),
    "Subtract two Rcplx values or an option of C64."
);
gen_binop!(
    Rcplx,
    C64,
    Mul,
    |lhs: C64, rhs: C64| Some(lhs * rhs),
    "Multiply two Rcplx values or an option of C64."
);
gen_binop!(
    Rcplx,
    C64,
    Div,
    |lhs: C64, rhs: C64| Some(lhs / rhs),
    "Divide two Rcplx values or an option of C64."
);
gen_binopassign!(
    Rcplx,
    C64,
    AddAssign,
    |lhs: C64, rhs: C64| Some(lhs + rhs),
    "Add two Rcplx values or an option of C64, modifying the left-hand side in place. Overflows to NA."
);
gen_binopassign!(
    Rcplx,
    C64,
    SubAssign,
    |lhs: C64, rhs: C64| Some(lhs - rhs),
    "Subtract two Rcplx values or an option of C64, modifying the left-hand side in place. Overflows to NA."
);
gen_binopassign!(
    Rcplx,
    C64,
    MulAssign,
    |lhs: C64, rhs: C64| Some(lhs * rhs),
    "Multiply two Rcplx values or an option of C64, modifying the left-hand side in place. Overflows to NA."
);
gen_binopassign!(
    Rcplx,
    C64,
    DivAssign,
    |lhs: C64, rhs: C64| Some(lhs / rhs),
    "Divide two Rcplx values or an option of C64, modifying the left-hand side in place. Overflows to NA."
);

// Generate unary ops for -, !
gen_unop!(Rcplx, Neg, |lhs: C64| Some(-lhs), "Negate a Rcplx value.");

impl TryFrom<Robj> for Rcplx {
    type Error = Error;

    fn try_from(robj: Robj) -> Result<Self> {
        // Check if the value is a scalar
        match robj.len() {
            0 => return Err(Error::ExpectedNonZeroLength(robj)),
            1 => {}
            _ => return Err(Error::ExpectedScalar(robj)),
        };

        // Check if the value is not a missing value.
        if robj.is_na() {
            return Ok(Rcplx::na());
        }

        // This should always work, NA is handled above.
        if let Some(v) = robj.as_real() {
            return Ok(Rcplx::from(v));
        }

        // Any integer (32 bit) can be represented as f64,
        // this always works.
        if let Some(v) = robj.as_integer() {
            return Ok(Rcplx::from(v as f64));
        }

        Err(Error::ExpectedNumeric(robj))
    }
}

impl From<C64> for Robj {
    fn from(_val: C64) -> Self {
        Robj::from(())       
    }
}