use crate::*;
use crate::scalar::macros::*;
use std::convert::TryFrom;
use std::ops::{Add, Div, Mul, Neg, Sub};


/// Rfloat is a wrapper for f64 in the context of an R's integer vector.
///
/// Rfloat has a special NA value, obtained from R headers via R_NaReal.
///
/// Rfloat has the same footprint as an f64 value allowing us to use it in zero copy slices.
// TODO: Should this support `Eq`?
#[derive(PartialEq)]
pub struct Rfloat(pub f64);


gen_scalar_impl!(Rfloat, f64, unsafe {libR_sys::R_NaReal});

gen_from!(Rfloat, f64);

gen_sum_iter!(Rfloat, 0f64);

// Generate binary ops for +, -, * and /
gen_binop!(
    Rfloat,
    f64,
    Add,
    add,
    |lhs: f64, rhs : f64| Some(lhs + rhs),
    "Add two Rfloat values or an option of f64."
);
gen_binop!(
    Rfloat,
    f64,
    Sub,
    sub,
    |lhs: f64, rhs : f64| Some(lhs - rhs),
    "Subtract two Rfloat values or an option of f64."
);
gen_binop!(
    Rfloat,
    f64,
    Mul,
    mul,
    |lhs: f64, rhs : f64| Some(lhs * rhs),
    "Multiply two Rfloat values or an option of f64."
);
gen_binop!(
    Rfloat,
    f64,
    Div,
    div,
    |lhs: f64, rhs : f64| Some(lhs / rhs),
    "Divide two Rfloat values or an option of f64."
);

// Generate unary ops for -, !
gen_unop!(
    Rfloat,
    Neg,
    neg,
    |lhs: f64| Some(-lhs),
    "Negate a Rfloat value."
);


impl TryFrom<Robj> for Rfloat {
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
            return Ok(Rfloat::na());
        }

        // This should always work, NA is handled above.
        if let Some(v) = robj.as_real() {
            return Ok(Rfloat::from(v));
        }

        // Any integer (32 bit) can be represented as f64,
        // this always works.
        if let Some(v) = robj.as_integer() {
            return Ok(Rfloat::from(v as f64));
       }

        Err(Error::ExpectedNumeric(robj))
    }
}

