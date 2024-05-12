use crate::scalar::macros::*;
use crate::scalar::Scalar;
use crate::*;
use std::cmp::Ordering::*;
use std::convert::TryFrom;
use std::ops::{Add, Div, Mul, Neg, Not, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

/// `Rint` is a wrapper for `i32` in the context of an R's integer vector.
///
/// `Rint` can have a value between `i32::MIN+1` and `i32::MAX`
///
/// The value `i32::MIN` is used as `"NA"`.
///
/// `Rint` has the same footprint as an `i32` value allowing us to use it in zero copy slices.
#[repr(transparent)]
pub struct Rint(i32);

impl Scalar<i32> for Rint {
    fn inner(&self) -> i32 {
        self.0
    }

    fn new(val: i32) -> Self {
        Rint(val)
    }
}

impl Rint {
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     assert!(Rint::na().min(Rint::default()).is_na());    
    ///     assert!(Rint::default().min(Rint::na()).is_na());
    ///     assert_eq!(Rint::default().min(Rint::default()), Rint::default());
    ///     assert_eq!(Rint::from(1).min(Rint::from(2)), Rint::from(1));    
    ///     assert_eq!(Rint::from(2).min(Rint::from(1)), Rint::from(1));    
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
    ///     assert!(Rint::na().max(Rint::default()).is_na());    
    ///     assert!(Rint::default().max(Rint::na()).is_na());
    ///     assert_eq!(Rint::default().max(Rint::default()), Rint::default());
    ///     assert_eq!(Rint::from(1).max(Rint::from(2)), Rint::from(2));    
    ///     assert_eq!(Rint::from(2).max(Rint::from(1)), Rint::from(2));    
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

gen_trait_impl!(Rint, i32, |x: &Rint| x.0 == i32::MIN, i32::MIN);
gen_from_primitive!(Rint, i32);

impl From<Rint> for Option<i32> {
    fn from(v: Rint) -> Self {
        if v.is_na() {
            None
        } else {
            Some(v.0)
        }
    }
}

gen_sum_iter!(Rint);
gen_partial_ord!(Rint, i32);

// Generate binary ops for `+`, `-`, `*` and `/`
gen_binop!(
    Rint,
    i32,
    Add,
    |lhs: i32, rhs| lhs.checked_add(rhs),
    "Add two Rint values or an option of i32, overflows to NA."
);
gen_binop!(
    Rint,
    i32,
    Sub,
    |lhs: i32, rhs| lhs.checked_sub(rhs),
    "Subtract two Rint values or an option of i32, overflows to NA."
);
gen_binop!(
    Rint,
    i32,
    Mul,
    |lhs: i32, rhs| lhs.checked_mul(rhs),
    "Multiply two Rint values or an option of i32, overflows to NA."
);
gen_binop!(
    Rint,
    i32,
    Div,
    |lhs: i32, rhs| lhs.checked_div(rhs),
    "Divide two Rint values or an option of i32, overflows to NA."
);
gen_binopassign!(
    Rint,
    i32,
    AddAssign,
    |lhs: i32, rhs| lhs.checked_add(rhs),
    "Add two Rint values or an option of i32, modifying the left-hand side in place. Overflows to NA."
);
gen_binopassign!(
    Rint,
    i32,
    SubAssign,
    |lhs: i32, rhs| lhs.checked_sub(rhs),
    "Subtract two Rint values or an option of i32, modifying the left-hand side in place. Overflows to NA."
);
gen_binopassign!(
    Rint,
    i32,
    MulAssign,
    |lhs: i32, rhs| lhs.checked_mul(rhs),
    "Multiply two Rint values or an option of i32, modifying the left-hand side in place. Overflows to NA."
);
gen_binopassign!(
    Rint,
    i32,
    DivAssign,
    |lhs: i32, rhs| lhs.checked_div(rhs),
    "Divide two Rint values or an option of i32, modifying the left-hand side in place. Overflows to NA."
);

// Generate unary ops for -, !
gen_unop!(
    Rint,
    Neg,
    |lhs: i32| Some(-lhs),
    "Negate a Rint value, overflows to NA."
);
gen_unop!(
    Rint,
    Not,
    |lhs: i32| Some(!lhs),
    "Logical not a Rint value, overflows to NA."
);

impl TryFrom<&Robj> for Rint {
    type Error = Error;

    fn try_from(robj: &Robj) -> Result<Self> {
        let i32_val: Result<i32> = robj.try_into();
        match i32_val {
            Ok(v) => Ok(Rint::from(v)),
            // TODO: Currently this results in an extra protection of robj
            Err(Error::MustNotBeNA(_)) => Ok(Rint::na()),
            Err(e) => Err(e),
        }
    }
}

impl std::fmt::Debug for Rint {
    /// Debug format.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_na() {
            write!(f, "NA_INTEGER")
        } else {
            self.inner().fmt(f)
        }
    }
}
