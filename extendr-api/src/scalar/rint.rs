use crate::scalar::macros::*;
use crate::*;
use std::cmp::Ordering::*;
use std::convert::TryFrom;
use std::ops::{Add, Div, Mul, Neg, Not, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

/// `RInt` is a wrapper for `i32` in the context of an R's integer vector.
///
/// `RInt` can have a value between `i32::MIN+1` and `i32::MAX`
///
/// The value `i32::MIN` is used as `"NA"`.
///
/// `RInt` has the same footprint as an `i32` value allowing us to use it in zero copy slices.
#[repr(transparent)]
#[readonly::make]
pub struct RInt(pub i32);

impl RInt {
    pub fn new(val: i32) -> Self {
        RInt(val)
    }

    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     assert!(RInt::na().min(RInt::default()).is_na());    
    ///     assert!(RInt::default().min(RInt::na()).is_na());
    ///     assert_eq!(RInt::default().min(RInt::default()), RInt::default());
    ///     assert_eq!(RInt::from(1).min(RInt::from(2)), RInt::from(1));    
    ///     assert_eq!(RInt::from(2).min(RInt::from(1)), RInt::from(1));    
    /// }
    /// ```
    pub fn min(&self, other: Self) -> Self {
        match self.partial_cmp(&other) {
            Some(Less | Equal) => *self,
            Some(Greater) => other,
            _ => Self::na(),
        }
    }

    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     assert!(RInt::na().max(RInt::default()).is_na());    
    ///     assert!(RInt::default().max(RInt::na()).is_na());
    ///     assert_eq!(RInt::default().max(RInt::default()), RInt::default());
    ///     assert_eq!(RInt::from(1).max(RInt::from(2)), RInt::from(2));    
    ///     assert_eq!(RInt::from(2).max(RInt::from(1)), RInt::from(2));    
    /// }
    /// ```
    pub fn max(&self, other: Self) -> Self {
        match self.partial_cmp(&other) {
            Some(Less) => other,
            Some(Greater | Equal) => *self,
            _ => Self::na(),
        }
    }
}

gen_trait_impl!(RInt, i32, |x: &RInt| x.0 == i32::MIN, i32::MIN);
gen_from_primitive!(RInt, i32);

impl From<RInt> for Option<i32> {
    fn from(v: RInt) -> Self {
        if v.is_na() {
            None
        } else {
            Some(v.0)
        }
    }
}

impl From<RInt> for i32 {
    fn from(v: RInt) -> Self {
        v.0
    }
}

gen_sum_iter!(RInt);
gen_partial_ord!(RInt, i32);

// Generate binary ops for `+`, `-`, `*` and `/`
gen_binop!(
    RInt,
    i32,
    Add,
    |lhs: i32, rhs| lhs.checked_add(rhs),
    "Add two RInt values or an option of i32, overflows to NA."
);
gen_binop!(
    RInt,
    i32,
    Sub,
    |lhs: i32, rhs| lhs.checked_sub(rhs),
    "Subtract two RInt values or an option of i32, overflows to NA."
);
gen_binop!(
    RInt,
    i32,
    Mul,
    |lhs: i32, rhs| lhs.checked_mul(rhs),
    "Multiply two RInt values or an option of i32, overflows to NA."
);
gen_binop!(
    RInt,
    i32,
    Div,
    |lhs: i32, rhs| lhs.checked_div(rhs),
    "Divide two RInt values or an option of i32, overflows to NA."
);
gen_binopassign!(
    RInt,
    i32,
    AddAssign,
    |lhs: i32, rhs| lhs.checked_add(rhs),
    "Add two RInt values or an option of i32, modifying the left-hand side in place. Overflows to NA."
);
gen_binopassign!(
    RInt,
    i32,
    SubAssign,
    |lhs: i32, rhs| lhs.checked_sub(rhs),
    "Subtract two RInt values or an option of i32, modifying the left-hand side in place. Overflows to NA."
);
gen_binopassign!(
    RInt,
    i32,
    MulAssign,
    |lhs: i32, rhs| lhs.checked_mul(rhs),
    "Multiply two RInt values or an option of i32, modifying the left-hand side in place. Overflows to NA."
);
gen_binopassign!(
    RInt,
    i32,
    DivAssign,
    |lhs: i32, rhs| lhs.checked_div(rhs),
    "Divide two RInt values or an option of i32, modifying the left-hand side in place. Overflows to NA."
);

// Generate unary ops for -, !
gen_unop!(
    RInt,
    Neg,
    |lhs: i32| Some(-lhs),
    "Negate a RInt value, overflows to NA."
);
gen_unop!(
    RInt,
    Not,
    |lhs: i32| Some(!lhs),
    "Logical not a RInt value, overflows to NA."
);

impl TryFrom<&RObj> for RInt {
    type Error = Error;

    fn try_from(robj: &RObj) -> Result<Self> {
        let i32_val: Result<i32> = robj.try_into();
        match i32_val {
            Ok(v) => Ok(RInt::from(v)),
            // TODO: Currently this results in an extra protection of robj
            Err(Error::MustNotBeNA(_)) => Ok(RInt::na()),
            Err(e) => Err(e),
        }
    }
}

impl std::fmt::Debug for RInt {
    /// Debug format.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_na() {
            write!(f, "NA_INTEGER")
        } else {
            self.0.fmt(f)
        }
    }
}

#[deprecated(note = "Use RInt instead", since = "0.9.0")]
pub type Rint = RInt;
