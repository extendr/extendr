use crate::scalar::macros::*;
use crate::*;
use std::convert::TryFrom;

/// Rbool is a wrapper for i32 in the context of an R's logical vector.
///
/// Rbool can have a value of 0, 1 or i32::MIN.
///
/// The value i32::MIN is used as "NA".
///
/// Rbool has the same footprint as an i32 value allowing us to use it in zero copy slices.
pub struct Rbool(i32);

impl Rbool {
    gen_impl!(Rbool, i32);

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

    /// Return `true` if this triboolean is `true` but not NA.
    pub fn is_true(&self) -> bool {
        self.inner() != 0 && !self.is_na()
    }

    /// Return `true` if this triboolean is `false` but not NA.
    pub fn is_false(&self) -> bool {
        self.inner() == 0 && !self.is_na()
    }
}

gen_trait_impl!(Rbool, bool, |x: &Rbool| x.inner() == i32::MIN, i32::MIN);
gen_from_primitive!(Rbool, i32);

impl TryFrom<Robj> for Rbool {
    type Error = Error;

    fn try_from(robj: Robj) -> Result<Self> {
        // Check if the value is a scalar
        match robj.len() {
            0 => return Err(Error::ExpectedNonZeroLength(robj)),
            1 => {}
            _ => return Err(Error::ExpectedScalar(robj)),
        };

        if let Some(s) = robj.as_typed_slice() {
            Ok(s[0])
        } else {
            Err(Error::ExpectedLogical(robj))
        }
    }
}

impl From<bool> for Rbool {
    fn from(v: bool) -> Self {
        Rbool(if v { 1 } else { 0 })
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
