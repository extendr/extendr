//! Conversions to Robj

use super::*;

macro_rules! impl_try_from_scalar {
    ($t: ty) => {
        impl TryFrom<Robj> for $t {
            type Error = Error;

            fn try_from(robj: Robj) -> Result<Self> {
                if let Some(v) = robj.as_integer_slice() {
                    match v.len() {
                        0 => Err(Error::ExpectedNonZeroLength(robj)),
                        1 => {
                            if !v[0].is_na() {
                                Ok(v[0] as Self)
                            } else {
                                Err(Error::MustNotBeNA(robj))
                            }
                        }
                        _ => Err(Error::ExpectedScalar(robj)),
                    }
                } else if let Some(v) = robj.as_real_slice() {
                    match v.len() {
                        0 => Err(Error::ExpectedNonZeroLength(robj)),
                        1 => {
                            if !v[0].is_na() {
                                Ok(v[0] as Self)
                            } else {
                                Err(Error::MustNotBeNA(robj))
                            }
                        }
                        _ => Err(Error::ExpectedScalar(robj)),
                    }
                } else {
                    Err(Error::ExpectedNumeric(robj))
                }
            }
        }
    };
}

impl_try_from_scalar!(u8);
impl_try_from_scalar!(u16);
impl_try_from_scalar!(u32);
impl_try_from_scalar!(u64);
impl_try_from_scalar!(i8);
impl_try_from_scalar!(i16);
impl_try_from_scalar!(i32);
impl_try_from_scalar!(i64);
impl_try_from_scalar!(f32);
impl_try_from_scalar!(f64);

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
            Err(Error::MustNotBeNA(robj))
        } else if let Some(s) = robj.as_str() {
            Ok(s)
        } else {
            Err(Error::ExpectedString(robj))
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
            Err(Error::ExpectedInteger(robj))
        }
    }
}

impl TryFrom<Robj> for Vec<String> {
    type Error = Error;

    fn try_from(robj: Robj) -> Result<Self> {
        if let Some(iter) = robj.as_str_iter() {
            // check for NA's in the string vector
            if iter.clone().find(|&s| s.is_na()).is_some() {
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

impl_option!(i32);
impl_option!(f64);
impl_option!(Bool);
impl_option!(bool);
impl_option!(&str);
impl_option!(String);
impl_option!(Vec<i32>);
impl_option!(Vec<f64>);
impl_option!(Vec<String>);

impl TryFrom<Robj> for HashMap<String, Robj> {
    type Error = Error;

    fn try_from(robj: Robj) -> Result<Self> {
        if let Some(iter) = robj.as_list().map(|l| l.iter()) {
            Ok(iter
                .map(|(k, v)| (k.to_string(), v.to_owned()))
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
            Ok(iter
                .map(|(k, v)| (k, v.to_owned()))
                .collect::<HashMap<&str, Robj>>())
        } else {
            Err(Error::ExpectedList(robj))
        }
    }
}
