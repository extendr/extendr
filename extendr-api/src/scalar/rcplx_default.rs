use crate::scalar::Rfloat;
use crate::scalar::Scalar;
use crate::*;

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

/// Rcplx is a wrapper for f64 in the context of an R's complex vector.
///
/// Rcplx has a special NA value, obtained from R headers via R_NaReal.
///
/// Rcplx has the same footprint as R's complex value allowing us to use it in zero copy slices.
#[derive(Clone, Copy, Default, PartialEq)]
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
    // gen_impl!(Rcplx, c64);
    pub fn new(re: f64, im: f64) -> Self {
        Self(c64::new(re, im))
    }

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
