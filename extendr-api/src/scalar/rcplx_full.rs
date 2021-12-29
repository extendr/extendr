use crate::scalar::macros::*;
use crate::scalar::Rfloat;
use crate::*;
use std::convert::TryFrom;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

#[allow(non_camel_case_types)]
pub type c64 = num_complex::Complex<f64>;

impl CanBeNA for c64 {
    fn is_na(&self) -> bool {
        unsafe { R_IsNA(self.re) != 0 }
    }

    fn na() -> c64 {
        unsafe { c64::new(R_NaReal, R_NaReal) }
    }
}

/// Rcplx is a wrapper for f64 in the context of an R's complex vector.
///
/// Rcplx has a special NA value, obtained from R headers via R_NaReal.
///
/// Rcplx has the same footprint as R's complex value allowing us to use it in zero copy slices.
pub struct Rcplx(c64);

impl Rcplx {
    gen_impl!(Rcplx, c64);

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
        Rcplx(c64::from(val))
    }
}

impl From<(f64, f64)> for Rcplx {
    fn from(val: (f64, f64)) -> Self {
        Rcplx(c64::new(val.0, val.1))
    }
}

impl From<Rfloat> for Rcplx {
    fn from(val: Rfloat) -> Self {
        Rcplx(c64::from(val.inner()))
    }
}

impl From<Rcomplex> for Rcplx {
    fn from(val: Rcomplex) -> Self {
        Rcplx(c64::new(val.r, val.i))
    }
}

impl From<Rcplx> for Option<c64> {
    fn from(val: Rcplx) -> Self {
        Some(c64::new(val.re().inner(), val.im().inner()))
    }
}

// `NA_real_` is a `NaN` with specific bit representation.
// Check that underlying `f64` is `NA_real_`.
gen_trait_impl!(Rcplx, c64, |x: &Rcplx| x.inner().re.is_na(), c64::na());
gen_from_primitive!(Rcplx, c64);
// gen_from_scalar!(Rcplx, c64);
gen_sum_iter!(Rcplx, 0f64);

// Generate binary ops for +, -, * and /
gen_binop!(
    Rcplx,
    c64,
    Add,
    |lhs: c64, rhs: c64| Some(lhs + rhs),
    "Add two Rcplx values or an option of c64."
);
gen_binop!(
    Rcplx,
    c64,
    Sub,
    |lhs: c64, rhs: c64| Some(lhs - rhs),
    "Subtract two Rcplx values or an option of c64."
);
gen_binop!(
    Rcplx,
    c64,
    Mul,
    |lhs: c64, rhs: c64| Some(lhs * rhs),
    "Multiply two Rcplx values or an option of c64."
);
gen_binop!(
    Rcplx,
    c64,
    Div,
    |lhs: c64, rhs: c64| Some(lhs / rhs),
    "Divide two Rcplx values or an option of c64."
);
gen_binopassign!(
    Rcplx,
    c64,
    AddAssign,
    |lhs: c64, rhs: c64| Some(lhs + rhs),
    "Add two Rcplx values or an option of c64, modifying the left-hand side in place. Overflows to NA."
);
gen_binopassign!(
    Rcplx,
    c64,
    SubAssign,
    |lhs: c64, rhs: c64| Some(lhs - rhs),
    "Subtract two Rcplx values or an option of c64, modifying the left-hand side in place. Overflows to NA."
);
gen_binopassign!(
    Rcplx,
    c64,
    MulAssign,
    |lhs: c64, rhs: c64| Some(lhs * rhs),
    "Multiply two Rcplx values or an option of c64, modifying the left-hand side in place. Overflows to NA."
);
gen_binopassign!(
    Rcplx,
    c64,
    DivAssign,
    |lhs: c64, rhs: c64| Some(lhs / rhs),
    "Divide two Rcplx values or an option of c64, modifying the left-hand side in place. Overflows to NA."
);

// Generate unary ops for -, !
gen_unop!(Rcplx, Neg, |lhs: c64| Some(-lhs), "Negate a Rcplx value.");

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

        // Complex slices return their first element.
        if let Some(s) = robj.as_typed_slice() {
            return Ok(s[0]);
        }

        Err(Error::ExpectedComplex(robj))
    }
}

impl PartialEq<f64> for Rcplx {
    fn eq(&self, other: &f64) -> bool {
        self.re().inner() == *other && self.im() == 0.0
    }
}
