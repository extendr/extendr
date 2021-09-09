use crate::*;
use crate::scalar::macros::*;
use std::convert::TryFrom;
use std::ops::{Add, Div, Mul, Neg, Not, Sub};


/// Rint is a wrapper for i32 in the context of an R's integer vector.
///
/// Rint can have a value between i32::MIN+1 and i32::MAX
///
/// The value i32::MIN is used as "NA".
///
/// Rint has the same footprint as an i32 value allowing us to use it in zero copy slices.
#[derive(PartialEq, Eq)]
pub struct Rint(pub i32);

impl Rint {
    /// Construct a NA Rint.
    pub fn na() -> Self {
        Rint(i32::MIN)
    }

    /// Get an integer or i32::MIN for NA.
    pub fn inner(&self) -> i32 {
        self.0
    }
}

impl Clone for Rint {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl Copy for Rint {}

impl IsNA for Rint {
    /// Return true is the is a NA value.
    fn is_na(&self) -> bool {
        self.0 == std::i32::MIN
    }
}

gen_from!(Rint, i32);

impl std::fmt::Debug for Rint {
    /// Debug format an Rint.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let z: Option<i32> = (*self).into();
        if let Some(val) = z {
            write!(f, "{}", val)
        } else {
            write!(f, "na")
        }
    }
}




// Generate binary ops for +, -, * and /
gen_binop!(
    Rint,
    i32,
    Add,
    add,
    |lhs: i32, rhs| lhs.checked_add(rhs),
    "Add two Rint values or an option of i32, overflows to NA."
);
gen_binop!(
    Rint,
    i32,
    Sub,
    sub,
    |lhs: i32, rhs| lhs.checked_sub(rhs),
    "Subtract two Rint values or an option of i32, overflows to NA."
);
gen_binop!(
    Rint,
    i32,
    Mul,
    mul,
    |lhs: i32, rhs| lhs.checked_mul(rhs),
    "Multiply two Rint values or an option of i32, overflows to NA."
);
gen_binop!(
    Rint,
    i32,
    Div,
    div,
    |lhs: i32, rhs| lhs.checked_div(rhs),
    "Divide two Rint values or an option of i32, overflows to NA."
);

// Generate unary ops for -, !
gen_unnop!(
    Rint,
    Neg,
    neg,
    |lhs: i32| Some(-lhs),
    "Negate a Rint value, overflows to NA."
);
gen_unnop!(
    Rint,
    Not,
    not,
    |lhs: i32| Some(!lhs),
    "Logical not a Rint value, overflows to NA."
);

impl std::iter::Sum for Rint {
    /// Sum an integer iterator over Rint.
    /// Yields NA on overflow of NAs present.
    fn sum<I: Iterator<Item = Rint>>(iter: I) -> Rint {
        iter.fold(Rint::from(0), |a, b| a + b)
    }
}

impl PartialEq<i32> for Rint {
    /// Compare a Rint with an integer. NA always fails.
    fn eq(&self, other: &i32) -> bool {
        !self.is_na() && self.0 == *other
    }
}

impl TryFrom<Robj> for Rint {
    type Error = Error;

    fn try_from(robj: Robj) -> Result<Self> {
        // Check if the value is a scalar
        match robj.len() {
            0 => return Err(Error::ExpectedNonZeroLength(robj)),
            1 => {}
            _ => return Err(Error::ExpectedScalar(robj)),
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
                return Err(Error::OutOfLimits(robj));
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
                return Err(Error::ExpectedWholeNumber(robj));
            }
        }

        Err(Error::ExpectedNumeric(robj))
    }
}

