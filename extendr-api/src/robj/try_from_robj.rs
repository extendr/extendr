//! Conversions to Robj

use super::*;

macro_rules! impl_try_from_scalar_integer {
    ($t:ty) => {
        impl TryFrom<Robj> for $t {
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
                    return Err(Error::MustNotBeNA(robj));
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
                    let result = v as Self;
                    if ((result as f64 - v).abs() < f64::EPSILON) {
                        return Ok(result);
                    } else {
                        return Err(Error::ExpectedWholeNumber(robj));
                    }
                }

                Err(Error::ExpectedNumeric(robj))
            }
        }
    };
}

macro_rules! impl_try_from_scalar_real {
    ($t:ty) => {
        impl TryFrom<Robj> for $t {
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
                    return Err(Error::MustNotBeNA(robj));
                }

                // <Robj>::as_xxx() methods can work only when the underlying
                // SEXP is the corresponding type, so we cannot use as_real()
                // directly on INTSXP.
                if let Some(v) = robj.as_real() {
                    return Ok(v as Self);
                }
                if let Some(v) = robj.as_integer() {
                    return Ok(v as Self);
                }

                Err(Error::ExpectedNumeric(robj))
            }
        }
    };
}

impl_try_from_scalar_integer!(u8);
impl_try_from_scalar_integer!(u16);
impl_try_from_scalar_integer!(u32);
impl_try_from_scalar_integer!(u64);
impl_try_from_scalar_integer!(i8);
impl_try_from_scalar_integer!(i16);
impl_try_from_scalar_integer!(i32);
impl_try_from_scalar_integer!(i64);
impl_try_from_scalar_real!(f32);
impl_try_from_scalar_real!(f64);

impl TryFrom<Robj> for Bool {
    type Error = Error;

    fn try_from(robj: Robj) -> Result<Self> {
        if let Some(v) = robj.as_logical_slice() {
            match v.len() {
                0 => Err(Error::ExpectedNonZeroLength(robj)),
                1 => Ok(v[0]),
                _ => Err(Error::ExpectedScalar(robj)),
            }
        } else {
            Err(Error::ExpectedLogical(robj))
        }
    }
}

impl TryFrom<Robj> for bool {
    type Error = Error;

    fn try_from(robj: Robj) -> Result<Self> {
        if robj.is_na() {
            Err(Error::MustNotBeNA(robj))
        } else {
            Ok(<Bool>::try_from(robj)?.is_true())
        }
    }
}

impl TryFrom<Robj> for &str {
    type Error = Error;

    fn try_from(robj: Robj) -> Result<Self> {
        if robj.is_na() {
            return Err(Error::MustNotBeNA(robj));
        }
        match robj.len() {
            0 => Err(Error::ExpectedNonZeroLength(robj)),
            1 => {
                if let Some(s) = robj.as_str() {
                    Ok(s)
                } else {
                    Err(Error::ExpectedString(robj))
                }
            }
            _ => Err(Error::ExpectedScalar(robj)),
        }
    }
}

impl TryFrom<Robj> for String {
    type Error = Error;

    fn try_from(robj: Robj) -> Result<Self> {
        <&str>::try_from(robj).map(|s| s.to_string())
    }
}

impl TryFrom<Robj> for Vec<i32> {
    type Error = Error;

    fn try_from(robj: Robj) -> Result<Self> {
        if let Some(v) = robj.as_integer_slice() {
            Ok(Vec::from(v))
        } else {
            Err(Error::ExpectedInteger(robj))
        }
    }
}

impl TryFrom<Robj> for Vec<f64> {
    type Error = Error;

    fn try_from(robj: Robj) -> Result<Self> {
        if let Some(v) = robj.as_real_slice() {
            Ok(Vec::from(v))
        } else {
            Err(Error::ExpectedReal(robj))
        }
    }
}

impl TryFrom<Robj> for Vec<Bool> {
    type Error = Error;

    fn try_from(robj: Robj) -> Result<Self> {
        if let Some(v) = robj.as_logical_slice() {
            Ok(Vec::from(v))
        } else {
            Err(Error::ExpectedInteger(robj))
        }
    }
}

impl TryFrom<Robj> for Vec<u8> {
    type Error = Error;

    fn try_from(robj: Robj) -> Result<Self> {
        if let Some(v) = robj.as_raw_slice() {
            Ok(Vec::from(v))
        } else {
            Err(Error::ExpectedInteger(robj))
        }
    }
}

impl TryFrom<Robj> for Vec<String> {
    type Error = Error;

    fn try_from(robj: Robj) -> Result<Self> {
        if let Some(iter) = robj.as_str_iter() {
            // check for NA's in the string vector
            if iter.clone().any(|s| s.is_na()) {
                Err(Error::MustNotBeNA(robj))
            } else {
                Ok(iter.map(|s| s.to_string()).collect::<Vec<String>>())
            }
        } else {
            Err(Error::ExpectedString(robj))
        }
    }
}

macro_rules! impl_option {
    ($type : ty) => {
        impl TryFrom<Robj> for Option<$type> {
            type Error = Error;

            fn try_from(robj: Robj) -> Result<Self> {
                if robj.is_na() {
                    Ok(None)
                } else {
                    Ok(Some(<$type>::try_from(robj)?))
                }
            }
        }
    };
}

impl_option!(u8);
impl_option!(u16);
impl_option!(u32);
impl_option!(u64);
impl_option!(i8);
impl_option!(i16);
impl_option!(i32);
impl_option!(i64);
impl_option!(f32);
impl_option!(f64);
impl_option!(Bool);
impl_option!(bool);
impl_option!(&str);
impl_option!(String);
impl_option!(Vec<i32>);
impl_option!(Vec<f64>);
impl_option!(Vec<String>);

impl TryFrom<Robj> for &[i32] {
    type Error = Error;

    fn try_from(robj: Robj) -> Result<Self> {
        robj.as_typed_slice().ok_or_else(|| Error::ExpectedInteger(robj))
    }
}

impl TryFrom<Robj> for &[Bool] {
    type Error = Error;

    fn try_from(robj: Robj) -> Result<Self> {
        robj.as_typed_slice().ok_or_else(|| Error::ExpectedLogical(robj))
    }
}

impl TryFrom<Robj> for &[u8] {
    type Error = Error;

    fn try_from(robj: Robj) -> Result<Self> {
        robj.as_typed_slice().ok_or_else(|| Error::ExpectedRaw(robj))
    }
}

impl TryFrom<Robj> for &[f64] {
    type Error = Error;

    fn try_from(robj: Robj) -> Result<Self> {
        robj.as_typed_slice().ok_or_else(|| Error::ExpectedReal(robj))
    }
}

impl TryFrom<Robj> for HashMap<String, Robj> {
    type Error = Error;

    fn try_from(robj: Robj) -> Result<Self> {
        if let Some(iter) = robj.as_list().map(|l| l.iter()) {
            Ok(iter
                .map(|(k, v)| (k.to_string(), v))
                .collect::<HashMap<String, Robj>>())
        } else {
            Err(Error::ExpectedList(robj))
        }
    }
}

impl TryFrom<Robj> for HashMap<&str, Robj> {
    type Error = Error;

    fn try_from(robj: Robj) -> Result<Self> {
        if let Some(iter) = robj.as_list().map(|l| l.iter()) {
            Ok(iter.map(|(k, v)| (k, v)).collect::<HashMap<&str, Robj>>())
        } else {
            Err(Error::ExpectedList(robj))
        }
    }
}
