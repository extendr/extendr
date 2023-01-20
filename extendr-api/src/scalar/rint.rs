use crate::scalar::macros::*;
use crate::*;
use std::convert::TryFrom;
use std::ops::{Add, Div, Mul, Neg, Not, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

/// Rint is a wrapper for i32 in the context of an R's integer vector.
///
/// Rint can have a value between i32::MIN+1 and i32::MAX
///
/// The value i32::MIN is used as "NA".
///
/// Rint has the same footprint as an i32 value allowing us to use it in zero copy slices.
pub struct Rint(pub i32);

impl Rint {
    gen_impl!(Rint, i32);
}

gen_trait_impl!(Rint, i32, |x: &Rint| x.0 == i32::MIN, i32::MIN);
gen_from_primitive!(Rint, i32);
gen_from_scalar!(Rint, i32);
gen_sum_iter!(Rint);

// Generate binary ops for +, -, * and /
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
