use crate::scalar::macros::*;
use crate::*;
use std::convert::TryFrom;

/// `RBool` is a wrapper for `i32` in the context of an R's logical vector.
///
/// `RBool` can have a value of `0`, `1` or `i32::MIN`.
///
/// The value `i32::MIN` is used as `NA`.
///
/// `RBool` has the same footprint as an `i32` value allowing us to use it in zero copy slices.
#[repr(transparent)]
#[readonly::make]
pub struct RBool(pub i32);

impl RBool {
    pub fn new(val: i32) -> Self {
        RBool(val)
    }

    /// Return a `true` `RBool`.
    pub const fn true_value() -> RBool {
        RBool(1)
    }

    /// Return a `false` `RBool`.
    pub const fn false_value() -> RBool {
        RBool(0)
    }

    /// Return a `NA` `RBool`.
    pub const fn na_value() -> RBool {
        RBool(i32::MIN)
    }

    /// Return `true` if this triboolean is `true` but not `NA`.
    pub fn is_true(&self) -> bool {
        self.0 != 0 && !self.is_na()
    }

    /// Return `true` if this triboolean is `false` but not `NA`.
    pub fn is_false(&self) -> bool {
        self.0 == 0 && !self.is_na()
    }

    /// Convert this `RBool` to a bool. Note `NA` will be true.
    pub fn to_bool(&self) -> bool {
        self.0 != 0
    }

    /// Convert this construct a `RBool` from a rust boolean.
    pub fn from_bool(val: bool) -> Self {
        RBool(val as i32)
    }
}

gen_trait_impl!(RBool, bool, |x: &RBool| x.0 == i32::MIN, i32::MIN);
gen_from_primitive!(RBool, i32);
gen_partial_ord!(RBool, bool);

impl From<bool> for RBool {
    fn from(v: bool) -> Self {
        RBool(i32::from(v))
    }
}

impl From<Option<bool>> for RBool {
    fn from(v: Option<bool>) -> Self {
        if let Some(v) = v {
            RBool::from(v)
        } else {
            RBool::na()
        }
    }
}

impl From<RBool> for Option<bool> {
    fn from(v: RBool) -> Self {
        if v.0.is_na() {
            None
        } else {
            Some(v.0 != 0)
        }
    }
}

impl From<RBool> for bool {
    fn from(v: RBool) -> Self {
        v.0 != 0
    }
}

impl std::ops::Not for RBool {
    type Output = Self;

    fn not(self) -> Self::Output {
        if self.is_na() {
            RBool::na()
        } else if self.is_true() {
            RBool::false_value()
        } else {
            RBool::true_value()
        }
    }
}

impl std::fmt::Debug for RBool {
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

impl TryFrom<&RObj> for RBool {
    type Error = Error;

    /// Convert an `LGLSXP` object into a `RBool` (tri-state boolean).
    /// Use `value.is_na()` to detect NA values.
    fn try_from(robj: &RObj) -> Result<Self> {
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

#[deprecated(note = "Use RBool instead", since = "0.9.0")]
pub type Rbool = RBool;
