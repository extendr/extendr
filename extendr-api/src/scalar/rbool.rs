use crate::scalar::macros::*;
use crate::scalar::Scalar;
use crate::*;
use std::convert::TryFrom;

/// `Rbool` is a wrapper for `i32` in the context of an R's logical vector.
///
/// `Rbool` can have a value of `0`, `1` or `i32::MIN`.
///
/// The value `i32::MIN` is used as `NA`.
///
/// `Rbool` has the same footprint as an `i32` value allowing us to use it in zero copy slices.
#[repr(transparent)]
pub struct Rbool(i32);

impl Scalar<i32> for Rbool {
    fn inner(&self) -> i32 {
        self.0
    }

    fn new(val: i32) -> Self {
        Rbool(val)
    }
}

impl Rbool {
    /// Return a `true` `Rbool`.
    pub const fn true_value() -> Rbool {
        Rbool(1)
    }

    /// Return a `false` `Rbool`.
    pub const fn false_value() -> Rbool {
        Rbool(0)
    }

    /// Return a `NA` `Rbool`.
    pub const fn na_value() -> Rbool {
        Rbool(i32::MIN)
    }

    /// Return `true` if this triboolean is `true` but not `NA`.
    pub fn is_true(&self) -> bool {
        self.inner() != 0 && !self.is_na()
    }

    /// Return `true` if this triboolean is `false` but not `NA`.
    pub fn is_false(&self) -> bool {
        self.inner() == 0 && !self.is_na()
    }

    /// Convert this `Rbool` to a bool. Note `NA` will be true.
    pub fn to_bool(&self) -> bool {
        self.inner() != 0
    }

    /// Convert this construct a `Rbool` from a rust boolean.
    pub fn from_bool(val: bool) -> Self {
        Rbool(val as i32)
    }
}

gen_trait_impl!(Rbool, bool, |x: &Rbool| x.inner() == i32::MIN, i32::MIN);
gen_from_primitive!(Rbool, i32);
gen_partial_ord!(Rbool, bool);

impl From<bool> for Rbool {
    fn from(v: bool) -> Self {
        Rbool(i32::from(v))
    }
}

impl From<Option<bool>> for Rbool {
    fn from(v: Option<bool>) -> Self {
        if let Some(v) = v {
            Rbool::from(v)
        } else {
            Rbool::na()
        }
    }
}

impl From<Rbool> for Option<bool> {
    fn from(v: Rbool) -> Self {
        if v.inner().is_na() {
            None
        } else {
            Some(v.inner() != 0)
        }
    }
}

impl std::ops::Not for Rbool {
    type Output = Self;

    fn not(self) -> Self::Output {
        if self.is_na() {
            Rbool::na()
        } else if self.is_true() {
            Rbool::false_value()
        } else {
            Rbool::true_value()
        }
    }
}

impl std::fmt::Debug for Rbool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            if self.is_na() {
                "NA_LOGICAL"
            } else if self.is_true() {
                "TRUE"
            } else {
                "FALSE"
            }
        )
    }
}

impl TryFrom<&Robj> for Rbool {
    type Error = Error;

    /// Convert an `LGLSXP` object into a `Rbool` (tri-state boolean).
    /// Use `value.is_na()` to detect NA values.
    fn try_from(robj: &Robj) -> Result<Self> {
        if let Some(v) = robj.as_logical_slice() {
            match v.len() {
                0 => Err(Error::ExpectedNonZeroLength(robj.clone())),
                1 => Ok(v[0]),
                _ => Err(Error::ExpectedScalar(robj.clone())),
            }
        } else {
            Err(Error::ExpectedLogical(robj.clone()))
        }
    }
}
