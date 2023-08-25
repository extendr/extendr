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
        // Check if the value is a scalar
        match robj.len() {
            0 => return Err(Error::ExpectedNonZeroLength(robj.clone())),
            1 => {}
            _ => return Err(Error::ExpectedScalar(robj.clone())),
        };

        // Check if the value is not a missing value
        if robj.is_na() {
            return Ok(Rint::na());
        }

        // If the conversion is int-to-int, check the limits. This
        // needs to be done by `TryFrom` because the conversion by `as`
        // is problematic when converting a negative value to unsigned
        // integer types (e.g. `-1i32 as u8` becomes 255).
        if let Some(v) = robj.as_integer() {
            if let Ok(v) = Self::try_from(v) {
                return Ok(v);
            } else {
                return Err(Error::OutOfLimits(robj.clone()));
            }
        }

        // If the conversion is float-to-int, check if the value is
        // integer-like (i.e., an integer, or a float representing a
        // whole number). This needs to be down with `as`, as no
        // `TryFrom` is implemented for float types. `FloatToInt` trait
        // might eventually become available in future, though.
        if let Some(v) = robj.as_real() {
            let result = v as i32;
            if (result as f64 - v).abs() < f64::EPSILON {
                return Ok(Rint::from(result));
            } else {
                return Err(Error::ExpectedWholeNumber(robj.clone()));
            }
        }

        Err(Error::ExpectedNumeric(robj.clone()))
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
