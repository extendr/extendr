use crate::scalar::macros::*;
use crate::scalar::{Rfloat, Scalar};
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
#[repr(transparent)]
pub struct Rcplx(c64);

impl Scalar<c64> for Rcplx {
    fn inner(&self) -> c64 {
        self.0
    }

    fn new(val: c64) -> Self {
        Rcplx(val)
    }
}

impl Rcplx {
    pub fn new(re: f64, im: f64) -> Self {
        Self(c64::new(re, im))
    }

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

impl From<(Rfloat, Rfloat)> for Rcplx {
    fn from(val: (Rfloat, Rfloat)) -> Self {
        Rcplx(c64::new(val.0.inner(), val.1.inner()))
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
        if val.is_na() {
            None
        } else {
            Some(c64::new(val.re().inner(), val.im().inner()))
        }
    }
}

// `NA_real_` is a `NaN` with specific bit representation.
// Check that underlying `f64` is `NA_real_`.
gen_trait_impl!(Rcplx, c64, |x: &Rcplx| x.inner().re.is_na(), c64::na());
gen_from_primitive!(Rcplx, c64);
// gen_from_scalar!(Rcplx, c64);
gen_sum_iter!(Rcplx);

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

impl PartialEq<f64> for Rcplx {
    fn eq(&self, other: &f64) -> bool {
        self.re().inner() == *other && self.im() == 0.0
    }
}

impl std::fmt::Debug for Rcplx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_na() {
            write!(f, "NA_COMPLEX")
        } else {
            write!(
                f,
                "{:?} {} {:?}i",
                self.re(),
                if self.im().is_sign_negative() {
                    '-'
                } else {
                    '+'
                },
                self.im().abs()
            )
        }
    }
}
