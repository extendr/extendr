use crate::scalar::RFloat;
use crate::*;
use extendr_ffi::{R_IsNA, R_NaReal, Rcomplex};

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct c64 {
    re: f64,
    im: f64,
}

impl c64 {
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }
}

impl From<f64> for c64 {
    fn from(val: f64) -> Self {
        c64::new(val, 0.0)
    }
}

impl std::fmt::Display for c64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.re, self.im)
    }
}

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
#[derive(Clone, Copy, Default, PartialEq)]
#[repr(transparent)]
#[readonly::make]
pub struct RCplx(pub c64);

impl RCplx {
    pub fn new(re: f64, im: f64) -> Self {
        Self(c64::new(re, im))
    }

    pub fn re(&self) -> RFloat {
        RFloat::from(self.0.re)
    }

    pub fn im(&self) -> RFloat {
        RFloat::from(self.0.im)
    }
}

impl CanBeNA for RCplx {
    fn is_na(&self) -> bool {
        unsafe { R_IsNA(self.0.re) != 0 }
    }

    fn na() -> RCplx {
        unsafe { RCplx::from(c64::new(R_NaReal, R_NaReal)) }
    }
}

impl From<c64> for RCplx {
    fn from(val: c64) -> Self {
        RCplx(val)
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
