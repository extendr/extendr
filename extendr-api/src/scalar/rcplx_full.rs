use crate::scalar::macros::*;
use crate::scalar::RFloat;
use crate::*;
use extendr_ffi::{R_IsNA, R_NaReal, Rcomplex};
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

/// RCplx is a wrapper for f64 in the context of an R's complex vector.
///
/// RCplx has a special NA value, obtained from R headers via R_NaReal.
///
/// RCplx has the same footprint as R's complex value allowing us to use it in zero copy slices.
#[repr(transparent)]
#[readonly::make]
pub struct RCplx(pub c64);

impl RCplx {
    pub fn new(re: f64, im: f64) -> Self {
        Self(c64::new(re, im))
    }

    pub fn is_nan(&self) -> bool {
        self.0.is_nan()
    }

    pub fn is_infinite(&self) -> bool {
        self.0.is_infinite()
    }

    pub fn re(&self) -> RFloat {
        RFloat::from(self.0.re)
    }

    pub fn im(&self) -> RFloat {
        RFloat::from(self.0.im)
    }
}

impl From<f64> for RCplx {
    fn from(val: f64) -> Self {
        RCplx(c64::from(val))
    }
}

impl From<(f64, f64)> for RCplx {
    fn from(val: (f64, f64)) -> Self {
        RCplx(c64::new(val.0, val.1))
    }
}

impl From<(RFloat, RFloat)> for RCplx {
    fn from(val: (RFloat, RFloat)) -> Self {
        RCplx(c64::new(val.0 .0, val.1 .0))
    }
}

impl From<RFloat> for RCplx {
    fn from(val: RFloat) -> Self {
        RCplx(c64::from(val.0))
    }
}

impl From<Rcomplex> for RCplx {
    fn from(val: Rcomplex) -> Self {
        RCplx(c64::new(val.r, val.i))
    }
}

impl From<RCplx> for Option<c64> {
    fn from(val: RCplx) -> Self {
        if val.is_na() {
            None
        } else {
            Some(c64::new(val.re().0, val.im().0))
        }
    }
}

impl From<RCplx> for c64 {
    fn from(val: RCplx) -> Self {
        c64::new(val.re().0, val.im().0)
    }
}

// `NA_real_` is a `NaN` with specific bit representation.
// Check that underlying `f64` is `NA_real_`.
gen_trait_impl!(RCplx, c64, |x: &RCplx| x.0.re.is_na(), c64::na());
gen_from_primitive!(RCplx, c64);
// gen_from_scalar!(RCplx, c64);
gen_sum_iter!(RCplx);

// Generate binary ops for +, -, * and /
gen_binop!(
    RCplx,
    c64,
    Add,
    |lhs: c64, rhs: c64| Some(lhs + rhs),
    "Add two RCplx values or an option of c64."
);
gen_binop!(
    RCplx,
    c64,
    Sub,
    |lhs: c64, rhs: c64| Some(lhs - rhs),
    "Subtract two RCplx values or an option of c64."
);
gen_binop!(
    RCplx,
    c64,
    Mul,
    |lhs: c64, rhs: c64| Some(lhs * rhs),
    "Multiply two RCplx values or an option of c64."
);
gen_binop!(
    RCplx,
    c64,
    Div,
    |lhs: c64, rhs: c64| Some(lhs / rhs),
    "Divide two RCplx values or an option of c64."
);
gen_binopassign!(
    RCplx,
    c64,
    AddAssign,
    |lhs: c64, rhs: c64| Some(lhs + rhs),
    "Add two RCplx values or an option of c64, modifying the left-hand side in place. Overflows to NA."
);
gen_binopassign!(
    RCplx,
    c64,
    SubAssign,
    |lhs: c64, rhs: c64| Some(lhs - rhs),
    "Subtract two RCplx values or an option of c64, modifying the left-hand side in place. Overflows to NA."
);
gen_binopassign!(
    RCplx,
    c64,
    MulAssign,
    |lhs: c64, rhs: c64| Some(lhs * rhs),
    "Multiply two RCplx values or an option of c64, modifying the left-hand side in place. Overflows to NA."
);
gen_binopassign!(
    RCplx,
    c64,
    DivAssign,
    |lhs: c64, rhs: c64| Some(lhs / rhs),
    "Divide two RCplx values or an option of c64, modifying the left-hand side in place. Overflows to NA."
);

// Generate unary ops for -, !
gen_unop!(RCplx, Neg, |lhs: c64| Some(-lhs), "Negate a RCplx value.");

impl PartialEq<f64> for RCplx {
    fn eq(&self, other: &f64) -> bool {
        self.re().0 == *other && self.im() == 0.0
    }
}

impl std::fmt::Debug for RCplx {
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
