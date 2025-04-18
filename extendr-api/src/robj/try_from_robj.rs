//! There are various ways an [`Robj`] may be converted into different types `T`.
//!
//! This module defines these conversions on `&Robj`. Due to internal reference
//! counting measure of [`ownership`]-module, it is cheaper to copy `&Robj`,
//! than copying `Robj`, as the latter will incur an increase in reference counting.
//!
//!
//! [`ownership`]: crate::ownership
use super::*;
use crate as extendr_api;
use crate::conversions::try_into_int::FloatToInt;

macro_rules! impl_try_from_scalar_integer {
    ($t:ty) => {
        impl TryFrom<&Robj> for $t {
            type Error = Error;

            /// Convert a numeric object to an integer value.
            fn try_from(robj: &Robj) -> Result<Self> {
                // Check if the value is a scalar
                match robj.len() {
                    0 => return Err(Error::ExpectedNonZeroLength(robj.clone())),
                    1 => {}
                    _ => return Err(Error::ExpectedScalar(robj.clone())),
                };

                // Check if the value is not a missing value
                if robj.is_na() {
                    return Err(Error::MustNotBeNA(robj.clone()));
                }

                // If the conversion is int-to-int, check the limits. This
                // needs to be done by `TryFrom` because the conversion by `as`
                // is problematic when converting a negative value to unsigned
                // integer types (e.g. `-1i32 as u8` becomes 255).
                if let Some(v) = robj.as_integer() {
                    return Self::try_from(v).map_err(|_| Error::OutOfLimits(robj.clone()));
                }

                // If the conversion is float-to-int, check if the value is
                // integer-like (i.e., an integer, or a float representing a
                // whole number).
                if let Some(v) = robj.as_real() {
                    return v
                        .try_into_int()
                        .map_err(|conv_err| Error::ExpectedWholeNumber(robj.clone(), conv_err));
                }

                Err(Error::ExpectedNumeric(robj.clone()))
            }
        }
    };
}

macro_rules! impl_try_from_scalar_real {
    ($t:ty) => {
        impl TryFrom<&Robj> for $t {
            type Error = Error;

            /// Convert a numeric object to a real value.
            fn try_from(robj: &Robj) -> Result<Self> {
                // Check if the value is a scalar
                match robj.len() {
                    0 => return Err(Error::ExpectedNonZeroLength(robj.clone())),
                    1 => {}
                    _ => return Err(Error::ExpectedScalar(robj.clone())),
                };

                // Check if the value is not a missing value
                if robj.is_na() {
                    return Err(Error::MustNotBeNA(robj.clone()));
                }

                // `<Robj>::as_xxx()` methods can work only when the underlying
                // `SEXP` is the corresponding type, so we cannot use `as_real()`
                // directly on `INTSXP`.
                if let Some(v) = robj.as_real() {
                    // f64 to f32 and f64 to f64 is always safe.
                    return Ok(v as Self);
                }
                if let Some(v) = robj.as_integer() {
                    // An i32 R integer can be represented exactly by f64, but might be truncated in f32.
                    return Ok(v as Self);
                }

                Err(Error::ExpectedNumeric(robj.clone()))
            }
        }
    };
}

impl_try_from_scalar_integer!(u8);
impl_try_from_scalar_integer!(u16);
impl_try_from_scalar_integer!(u32);
impl_try_from_scalar_integer!(u64);
impl_try_from_scalar_integer!(usize);
impl_try_from_scalar_integer!(i8);
impl_try_from_scalar_integer!(i16);
impl_try_from_scalar_integer!(i32);
impl_try_from_scalar_integer!(i64);
impl_try_from_scalar_integer!(isize);
impl_try_from_scalar_real!(f32);
impl_try_from_scalar_real!(f64);

impl TryFrom<&Robj> for bool {
    type Error = Error;

    /// Convert an LGLSXP object into a boolean.
    /// NAs are not allowed.
    fn try_from(robj: &Robj) -> Result<Self> {
        if robj.is_na() {
            Err(Error::MustNotBeNA(robj.clone()))
        } else {
            Ok(<Rbool>::try_from(robj)?.is_true())
        }
    }
}

impl TryFrom<&Robj> for &str {
    type Error = Error;

    /// Convert a scalar STRSXP object into a string slice.
    /// NAs are not allowed.
    fn try_from(robj: &Robj) -> Result<Self> {
        if robj.is_na() {
            return Err(Error::MustNotBeNA(robj.clone()));
        }
        match robj.len() {
            0 => Err(Error::ExpectedNonZeroLength(robj.clone())),
            1 => {
                if let Some(s) = robj.as_str() {
                    Ok(s)
                } else {
                    Err(Error::ExpectedString(robj.clone()))
                }
            }
            _ => Err(Error::ExpectedScalar(robj.clone())),
        }
    }
}

impl TryFrom<&Robj> for String {
    type Error = Error;

    /// Convert an scalar STRSXP object into a String.
    /// Note: Unless you plan to store the result, use a string slice instead.
    /// NAs are not allowed.
    fn try_from(robj: &Robj) -> Result<Self> {
        <&str>::try_from(robj).map(|s| s.to_string())
    }
}

impl TryFrom<&Robj> for Vec<i32> {
    type Error = Error;

    /// Convert an INTSXP object into a vector of i32 (integer).
    /// Note: Unless you plan to store the result, use a slice instead.
    /// Use `value.is_na()` to detect NA values.
    fn try_from(robj: &Robj) -> Result<Self> {
        if let Some(v) = robj.as_typed_slice() {
            // TODO: check NAs
            Ok(Vec::from(v))
        } else {
            Err(Error::ExpectedInteger(robj.clone()))
        }
    }
}

impl TryFrom<&Robj> for Vec<f64> {
    type Error = Error;

    /// Convert a REALSXP object into a vector of f64 (double precision floating point).
    /// Note: Unless you plan to store the result, use a slice instead.
    /// Use `value.is_na()` to detect NA values.
    fn try_from(robj: &Robj) -> Result<Self> {
        if let Some(v) = robj.as_typed_slice() {
            // TODO: check NAs
            Ok(Vec::from(v))
        } else {
            Err(Error::ExpectedReal(robj.clone()))
        }
    }
}

impl TryFrom<&Robj> for Vec<u8> {
    type Error = Error;

    /// Convert a RAWSXP object into a vector of bytes.
    /// Note: Unless you plan to store the result, use a slice instead.
    fn try_from(robj: &Robj) -> Result<Self> {
        if let Some(v) = robj.as_typed_slice() {
            Ok(Vec::from(v))
        } else {
            Err(Error::ExpectedRaw(robj.clone()))
        }
    }
}

impl TryFrom<&Robj> for Vec<Rint> {
    type Error = Error;

    /// Convert an INTSXP object into a vector of i32 (integer).
    /// Note: Unless you plan to store the result, use a slice instead.
    /// Use `value.is_na()` to detect NA values.
    fn try_from(robj: &Robj) -> Result<Self> {
        if let Some(v) = robj.as_typed_slice() {
            Ok(Vec::from(v))
        } else {
            Err(Error::ExpectedInteger(robj.clone()))
        }
    }
}

impl TryFrom<&Robj> for Vec<Rfloat> {
    type Error = Error;

    /// Convert a REALSXP object into a vector of f64 (double precision floating point).
    /// Note: Unless you plan to store the result, use a slice instead.
    /// Use `value.is_na()` to detect NA values.
    fn try_from(robj: &Robj) -> Result<Self> {
        if let Some(v) = robj.as_typed_slice() {
            Ok(Vec::from(v))
        } else {
            Err(Error::ExpectedReal(robj.clone()))
        }
    }
}

impl TryFrom<&Robj> for Vec<Rbool> {
    type Error = Error;

    /// Convert a LGLSXP object into a vector of Rbool (tri-state booleans).
    /// Note: Unless you plan to store the result, use a slice instead.
    /// Use `value.is_na()` to detect NA values.
    fn try_from(robj: &Robj) -> Result<Self> {
        if let Some(v) = robj.as_typed_slice() {
            Ok(Vec::from(v))
        } else {
            Err(Error::ExpectedInteger(robj.clone()))
        }
    }
}

impl TryFrom<&Robj> for Vec<Rcplx> {
    type Error = Error;

    /// Convert a complex object into a vector of Rcplx.
    fn try_from(robj: &Robj) -> Result<Self> {
        if let Some(v) = robj.as_typed_slice() {
            Ok(Vec::from(v))
        } else {
            Err(Error::ExpectedComplex(robj.clone()))
        }
    }
}

impl TryFrom<&Robj> for Vec<String> {
    type Error = Error;

    /// Convert a STRSXP object into a vector of `String`s.
    /// Note: Unless you plan to store the result, use a slice instead.
    fn try_from(robj: &Robj) -> Result<Self> {
        if let Some(iter) = robj.as_str_iter() {
            // check for NA's in the string vector
            if iter.clone().any(|s| s.is_na()) {
                Err(Error::MustNotBeNA(robj.clone()))
            } else {
                Ok(iter.map(|s| s.to_string()).collect::<Vec<String>>())
            }
        } else {
            Err(Error::ExpectedString(robj.clone()))
        }
    }
}

impl TryFrom<&Robj> for &[i32] {
    type Error = Error;

    /// Convert an INTSXP object into a slice of i32 (integer).
    /// Use `value.is_na()` to detect NA values.
    fn try_from(robj: &Robj) -> Result<Self> {
        robj.as_typed_slice()
            .ok_or_else(|| Error::ExpectedInteger(robj.clone()))
    }
}

impl TryFrom<&Robj> for &[Rint] {
    type Error = Error;

    /// Convert an integer object into a slice of Rint (tri-state booleans).
    /// Use `value.is_na()` to detect NA values.
    fn try_from(robj: &Robj) -> Result<Self> {
        robj.as_typed_slice()
            .ok_or_else(|| Error::ExpectedInteger(robj.clone()))
    }
}

impl TryFrom<&Robj> for &[Rfloat] {
    type Error = Error;

    /// Convert a doubles object into a slice of Rfloat (tri-state booleans).
    /// Use `value.is_na()` to detect NA values.
    fn try_from(robj: &Robj) -> Result<Self> {
        robj.as_typed_slice()
            .ok_or_else(|| Error::ExpectedReal(robj.clone()))
    }
}

impl TryFrom<&Robj> for &[Rbool] {
    type Error = Error;

    /// Convert a logical object into a slice of Rbool (tri-state booleans).
    /// Use `value.is_na()` to detect NA values.
    fn try_from(robj: &Robj) -> Result<Self> {
        robj.as_typed_slice()
            .ok_or_else(|| Error::ExpectedLogical(robj.clone()))
    }
}

impl TryFrom<&Robj> for &[Rcplx] {
    type Error = Error;

    /// Convert a complex object into a slice of Rcplx
    /// Use `value.is_na()` to detect NA values.
    fn try_from(robj: &Robj) -> Result<Self> {
        robj.as_typed_slice()
            .ok_or_else(|| Error::ExpectedComplex(robj.clone()))
    }
}

impl TryFrom<&Robj> for &[u8] {
    type Error = Error;

    /// Convert a RAWSXP object into a slice of bytes.
    fn try_from(robj: &Robj) -> Result<Self> {
        robj.as_typed_slice()
            .ok_or_else(|| Error::ExpectedRaw(robj.clone()))
    }
}

impl TryFrom<&Robj> for &[f64] {
    type Error = Error;

    /// Convert a REALSXP object into a slice of f64 (double precision floating point).
    /// Use `value.is_na()` to detect NA values.
    fn try_from(robj: &Robj) -> Result<Self> {
        robj.as_typed_slice()
            .ok_or_else(|| Error::ExpectedReal(robj.clone()))
    }
}

impl TryFrom<&mut Robj> for &mut [i32] {
    type Error = Error;

    /// Convert an INTSXP object into a mutable slice of i32 (integer).
    /// Use `value.is_na()` to detect NA values.
    fn try_from(robj: &mut Robj) -> Result<Self> {
        robj.as_typed_slice_mut()
            .ok_or_else(|| Error::ExpectedInteger(robj.clone()))
    }
}

impl TryFrom<&mut Robj> for &mut [Rint] {
    type Error = Error;

    /// Convert an integer object into a mutable slice of Rint (tri-state booleans).
    /// Use `value.is_na()` to detect NA values.
    fn try_from(robj: &mut Robj) -> Result<Self> {
        robj.as_typed_slice_mut()
            .ok_or_else(|| Error::ExpectedInteger(robj.clone()))
    }
}

impl TryFrom<&mut Robj> for &mut [Rfloat] {
    type Error = Error;

    /// Convert a doubles object into a mutable slice of Rfloat (tri-state booleans).
    /// Use `value.is_na()` to detect NA values.
    fn try_from(robj: &mut Robj) -> Result<Self> {
        robj.as_typed_slice_mut()
            .ok_or_else(|| Error::ExpectedReal(robj.clone()))
    }
}

impl TryFrom<&mut Robj> for &mut [Rbool] {
    type Error = Error;

    /// Convert a logical object into a mutable slice of Rbool (tri-state booleans).
    /// Use `value.is_na()` to detect NA values.
    fn try_from(robj: &mut Robj) -> Result<Self> {
        robj.as_typed_slice_mut()
            .ok_or_else(|| Error::ExpectedLogical(robj.clone()))
    }
}

impl TryFrom<&mut Robj> for &mut [Rcplx] {
    type Error = Error;

    /// Convert a complex object into a mutable slice of Rcplx
    /// Use `value.is_na()` to detect NA values.
    fn try_from(robj: &mut Robj) -> Result<Self> {
        robj.as_typed_slice_mut()
            .ok_or_else(|| Error::ExpectedComplex(robj.clone()))
    }
}

impl TryFrom<&mut Robj> for &mut [u8] {
    type Error = Error;

    /// Convert a RAWSXP object into a mutable slice of bytes.
    fn try_from(robj: &mut Robj) -> Result<Self> {
        robj.as_typed_slice_mut()
            .ok_or_else(|| Error::ExpectedRaw(robj.clone()))
    }
}

impl TryFrom<&mut Robj> for &mut [f64] {
    type Error = Error;

    /// Convert a REALSXP object into a mutable slice of f64 (double precision floating point).
    /// Use `value.is_na()` to detect NA values.
    fn try_from(robj: &mut Robj) -> Result<Self> {
        robj.as_typed_slice_mut()
            .ok_or_else(|| Error::ExpectedReal(robj.clone()))
    }
}

impl TryFrom<&Robj> for Rcplx {
    type Error = Error;

    fn try_from(robj: &Robj) -> Result<Self> {
        // Check if the value is a scalar
        match robj.len() {
            0 => return Err(Error::ExpectedNonZeroLength(robj.clone())),
            1 => {}
            _ => return Err(Error::ExpectedScalar(robj.clone())),
        };

        // Check if the value is not a missing value.
        if robj.is_na() {
            return Ok(Rcplx::na());
        }

        // This should always work, NA is handled above.
        if let Some(v) = robj.as_real() {
            return Ok(Rcplx::from(v));
        }

        // Any integer (32 bit) can be represented as f64,
        // this always works.
        if let Some(v) = robj.as_integer() {
            return Ok(Rcplx::from(v as f64));
        }

        // Complex slices return their first element.
        if let Some(s) = robj.as_typed_slice() {
            return Ok(s[0]);
        }

        Err(Error::ExpectedComplex(robj.clone()))
    }
}

// Convert TryFrom<&Robj> into TryFrom<Robj>. Sadly, we are unable to make a blanket
// conversion using GetSexp with the current version of Rust.
macro_rules! impl_try_from_robj {
    (&mut [$type:ty]) => {
        impl TryFrom<Robj> for &mut [$type] {
            type Error = Error;

            fn try_from(mut robj: Robj) -> Result<Self> {
                Self::try_from(&mut robj)
            }
        }

        impl TryFrom<&mut Robj> for Option<&mut [$type]> {
            type Error = Error;

            fn try_from(robj: &mut Robj) -> Result<Self> {
                if robj.is_null() || robj.is_na() {
                    Ok(None)
                } else {
                    Ok(Some(<&mut [$type]>::try_from(robj)?))
                }
            }
        }

        impl TryFrom<Robj> for Option<&mut [$type]> {
            type Error = Error;

            fn try_from(mut robj: Robj) -> Result<Self> {
                Self::try_from(&mut robj)
            }
        }
    };
    ($(@generics<$generics:tt>)? $type:ty $(where $($where_clause:tt)*)?) => {
        impl$(<$generics>)? TryFrom<Robj> for $type $(where $($where_clause)*)? {
            type Error = Error;

            fn try_from(robj: Robj) -> Result<Self> {
                Self::try_from(&robj)
            }
        }

        impl$(<$generics>)? TryFrom<&Robj> for Option<$type> $(where $($where_clause)*)? {
            type Error = Error;

            fn try_from(robj: &Robj) -> Result<Self> {
                if robj.is_null() || robj.is_na() {
                    Ok(None)
                } else {
                    Ok(Some(<$type>::try_from(robj)?))
                }
            }
        }

        impl$(<$generics>)? TryFrom<Robj> for Option<$type> $(where $($where_clause)*)? {
            type Error = Error;

            fn try_from(robj: Robj) -> Result<Self> {
                Self::try_from(&robj)
            }
        }
    };
}
#[rustfmt::skip]
impl_try_from_robj!(u8);
impl_try_from_robj!(u16);
impl_try_from_robj!(u32);
impl_try_from_robj!(u64);
impl_try_from_robj!(usize);

impl_try_from_robj!(i8);
impl_try_from_robj!(i16);
impl_try_from_robj!(i32);
impl_try_from_robj!(i64);
impl_try_from_robj!(isize);

impl_try_from_robj!(bool);

impl_try_from_robj!(Rint);
impl_try_from_robj!(Rfloat);
impl_try_from_robj!(Rbool);
impl_try_from_robj!(Rcplx);

impl_try_from_robj!(f32);
impl_try_from_robj!(f64);

impl_try_from_robj!(Vec::<String>);
impl_try_from_robj!(Vec::<Rint>);
impl_try_from_robj!(Vec::<Rfloat>);
impl_try_from_robj!(Vec::<Rbool>);
impl_try_from_robj!(Vec::<Rcplx>);
impl_try_from_robj!(Vec::<u8>);
impl_try_from_robj!(Vec::<i32>);
impl_try_from_robj!(Vec::<f64>);

impl_try_from_robj!(&[Rint]);
impl_try_from_robj!(&[Rfloat]);
impl_try_from_robj!(&[Rbool]);
impl_try_from_robj!(&[Rcplx]);
impl_try_from_robj!(&[u8]);
impl_try_from_robj!(&[i32]);
impl_try_from_robj!(&[f64]);

impl_try_from_robj!(&mut [Rint]);
impl_try_from_robj!(&mut [Rfloat]);
impl_try_from_robj!(&mut [Rbool]);
impl_try_from_robj!(&mut [Rcplx]);
impl_try_from_robj!(&mut [u8]);
impl_try_from_robj!(&mut [i32]);
impl_try_from_robj!(&mut [f64]);

impl_try_from_robj!(&str);
impl_try_from_robj!(String);

impl_try_from_robj!(@generics<T> HashMap::<&str, T> where T: TryFrom<Robj, Error = error::Error>);
impl_try_from_robj!(@generics<T> HashMap::<String,T> where T: TryFrom<Robj, Error = error::Error>);

impl_try_from_robj!(HashMap::<&str, Robj>);
impl_try_from_robj!(HashMap::<String, Robj>);

// NOTE: this is included for compatibility with previously defined `FromRobj`
// One should prefer `List::from_hashmap` instead,
// and this `impl` should be deprecated next.

impl<T> TryFrom<&Robj> for HashMap<&str, T>
where
    T: TryFrom<Robj, Error = error::Error>,
{
    type Error = Error;

    fn try_from(value: &Robj) -> Result<Self> {
        let value: List = value.try_into()?;

        let value = value
            .iter()
            .map(|(name, value)| -> Result<(&str, T)> { value.try_into().map(|x| (name, x)) })
            .collect::<Result<HashMap<_, _>>>()?;

        Ok(value)
    }
}

impl<T> TryFrom<&Robj> for HashMap<String, T>
where
    T: TryFrom<Robj, Error = error::Error>,
{
    type Error = Error;
    fn try_from(value: &Robj) -> Result<Self> {
        let value: HashMap<&str, _> = value.try_into()?;
        Ok(value.into_iter().map(|(k, v)| (k.to_string(), v)).collect())
    }
}

macro_rules! impl_try_from_robj_for_arrays {
    ($slice_type:ty) => {
        impl<const N: usize> TryFrom<&Robj> for [$slice_type; N] {
            type Error = Error;

            fn try_from(value: &Robj) -> Result<Self> {
                let value: &[$slice_type] = value.try_into()?;
                if value.len() != N {
                    return Err(Error::ExpectedLength(N));
                }
                let value: Self = value
                    .try_into()
                    .map_err(|error| format!("{}", error).to_string())?;
                Ok(value)
            }
        }

        // TODO: the following can be integrated into `impl_try_from_robj` later

        impl<const N: usize> TryFrom<Robj> for [$slice_type; N] {
            type Error = Error;

            fn try_from(robj: Robj) -> Result<Self> {
                Self::try_from(&robj)
            }
        }

        impl<const N: usize> TryFrom<&Robj> for Option<[$slice_type; N]> {
            type Error = Error;

            fn try_from(robj: &Robj) -> Result<Self> {
                if robj.is_null() || robj.is_na() {
                    Ok(None)
                } else {
                    Ok(Some(<[$slice_type; N]>::try_from(robj)?))
                }
            }
        }

        impl<const N: usize> TryFrom<Robj> for Option<[$slice_type; N]> {
            type Error = Error;

            fn try_from(robj: Robj) -> Result<Self> {
                Self::try_from(&robj)
            }
        }
    };
}

impl_try_from_robj_for_arrays!(Rint);
impl_try_from_robj_for_arrays!(Rfloat);
impl_try_from_robj_for_arrays!(Rbool);
impl_try_from_robj_for_arrays!(Rcplx);
impl_try_from_robj_for_arrays!(u8);
impl_try_from_robj_for_arrays!(i32);
impl_try_from_robj_for_arrays!(f64);

// Choosing arity 12.. As the Rust compiler did for these [Tuple to array conversion](https://doc.rust-lang.org/stable/std/primitive.tuple.html#trait-implementations-1)
impl_try_from_robj_tuples!((1, 12));

// The following is necessary because it is impossible to define `TryFrom<Robj> for &Robj` as
// it requires returning a reference to a owned (moved) value

impl TryFrom<&Robj> for HashMap<&str, Robj> {
    type Error = Error;

    fn try_from(value: &Robj) -> Result<Self> {
        let value: List = value.try_into()?;
        Ok(value.into_iter().collect())
    }
}

impl TryFrom<&Robj> for HashMap<String, Robj> {
    type Error = Error;
    fn try_from(value: &Robj) -> Result<Self> {
        let value: HashMap<&str, _> = value.try_into()?;
        Ok(value.into_iter().map(|(k, v)| (k.to_string(), v)).collect())
    }
}
