use crate::scalar::Rfloat;
use crate::*;
use std::convert::TryFrom;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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

/// Rcplx is a wrapper for f64 in the context of an R's complex vector.
///
/// Rcplx has a special NA value, obtained from R headers via R_NaReal.
///
/// Rcplx has the same footprint as R's complex value allowing us to use it in zero copy slices.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rcplx(c64);

impl Rcplx {
    // gen_impl!(Rcplx, c64);

    pub fn re(&self) -> Rfloat {
        Rfloat::from(self.0.re)
    }

    pub fn im(&self) -> Rfloat {
        Rfloat::from(self.0.im)
    }
}

impl CanBeNA for Rcplx {
    fn is_na(&self) -> bool {
        unsafe { R_IsNA(self.0.re) != 0 }
    }

    fn na() -> Rcplx {
        unsafe { Rcplx::from(c64::new(R_NaReal, R_NaReal)) }
    }
}

impl From<c64> for Rcplx {
    fn from(val: c64) -> Self {
        Rcplx(val)
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
