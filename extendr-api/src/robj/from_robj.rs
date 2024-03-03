use super::*;
use crate::wrapper::matrix::MatrixConversions;

/// Trait used for incomming parameter conversion.
pub trait FromRobj<'a>: Sized {
    // Convert an incomming Robj from R into a value or an error.
    fn from_robj(_robj: &'a Robj) -> std::result::Result<Self, &'static str> {
        Err("unable to convert value from R object")
    }
}

macro_rules! impl_prim_from_robj {
    ($t: ty) => {
        impl<'a> FromRobj<'a> for $t {
            fn from_robj(robj: &'a Robj) -> std::result::Result<Self, &'static str> {
                if let Some(v) = robj.as_integer_slice() {
                    match v.len() {
                        0 => Err("Input must be of length 1. Vector of length zero given."),
                        1 => {
                            if !v[0].is_na() {
                                Ok(v[0] as Self)
                            } else {
                                Err("Input must not be NA.")
                            }
                        }
                        _ => Err("Input must be of length 1. Vector of length >1 given."),
                    }
                } else if let Some(v) = robj.as_real_slice() {
                    match v.len() {
                        0 => Err("Input must be of length 1. Vector of length zero given."),
                        1 => {
                            if !v[0].is_na() {
                                Ok(v[0] as Self)
                            } else {
                                Err("Input must not be NA.")
                            }
                        }
                        _ => Err("Input must be of length 1. Vector of length >1 given."),
                    }
                } else {
                    Err("unable to convert R object to primitive")
                }
            }
        }
    };
}

impl_prim_from_robj!(u8);
impl_prim_from_robj!(u16);
impl_prim_from_robj!(u32);
impl_prim_from_robj!(u64);
impl_prim_from_robj!(i8);
impl_prim_from_robj!(i16);
impl_prim_from_robj!(i32);
impl_prim_from_robj!(i64);
impl_prim_from_robj!(f32);
impl_prim_from_robj!(f64);

impl<'a> FromRobj<'a> for bool {
    fn from_robj(robj: &'a Robj) -> std::result::Result<Self, &'static str> {
        if let Some(v) = robj.as_logical_slice() {
            match v.len() {
                0 => Err("Input must be of length 1. Vector of length zero given."),
                1 => {
                    if !v[0].is_na() {
                        Ok(v[0].to_bool())
                    } else {
                        Err("Input must not be NA.")
                    }
                }
                _ => Err("Input must be of length 1. Vector of length >1 given."),
            }
        } else {
            Err("Not a logical object.")
        }
    }
}

impl<'a> FromRobj<'a> for &'a str {
    fn from_robj(robj: &'a Robj) -> std::result::Result<Self, &'static str> {
        if robj.is_na() {
            Err("Input must not be NA.")
        } else if let Some(s) = robj.as_str() {
            Ok(s)
        } else {
            Err("Not a string object.")
        }
    }
}

impl<'a> FromRobj<'a> for String {
    fn from_robj(robj: &'a Robj) -> std::result::Result<Self, &'static str> {
        if robj.is_na() {
            Err("Input must not be NA.")
        } else if let Some(s) = robj.as_str() {
            Ok(s.to_string())
        } else {
            Err("not a string object")
        }
    }
}

impl<'a> FromRobj<'a> for Vec<i32> {
    fn from_robj(robj: &'a Robj) -> std::result::Result<Self, &'static str> {
        if let Some(v) = robj.as_integer_slice() {
            Ok(Vec::from(v))
        } else {
            Err("not an integer or logical vector")
        }
    }
}

impl<'a> FromRobj<'a> for Vec<f64> {
    fn from_robj(robj: &'a Robj) -> std::result::Result<Self, &'static str> {
        if let Some(v) = robj.as_real_slice() {
            Ok(Vec::from(v))
        } else {
            Err("not a floating point vector")
        }
    }
}

impl<'a> FromRobj<'a> for Vec<String> {
    fn from_robj(robj: &'a Robj) -> std::result::Result<Self, &'static str> {
        if robj.is_na() {
            Err("Input must be a character vector. Got 'NA'.")
        } else if let Some(v) = robj.as_string_vector() {
            let str_vec = v.to_vec();
            // check for NA's in the string vector
            // The check is by-value, so `<&str>::is_na()` cannot be used
            if let Some(_str) = str_vec.iter().find(|&s| *s == <&str>::na()) {
                Err("Input vector cannot contain NA's.")
            } else {
                Ok(str_vec)
            }
        } else {
            Err("Input must be a character vector.")
        }
    }
}

macro_rules! impl_iter_from_robj {
    ($t: ty, $iter_fn: ident, $msg: expr) => {
        impl<'a> FromRobj<'a> for $t {
            fn from_robj(robj: &'a Robj) -> std::result::Result<Self, &'static str> {
                if let Some(v) = robj.$iter_fn() {
                    Ok(v)
                } else {
                    Err($msg)
                }
            }
        }
    };
}

impl_iter_from_robj!(StrIter, as_str_iter, "Not a character vector.");

/// Pass-through Robj conversion, essentially a clone.
impl<'a> FromRobj<'a> for Robj {
    fn from_robj(robj: &'a Robj) -> std::result::Result<Self, &'static str> {
        Ok(unsafe { Robj::from_sexp(robj.get()) })
    }
}

impl<'a> FromRobj<'a> for HashMap<String, Robj> {
    fn from_robj(robj: &'a Robj) -> std::result::Result<Self, &'static str> {
        if let Some(iter) = robj.as_list().map(|l| l.iter()) {
            Ok(iter
                .map(|(k, v)| (k.to_string(), v))
                .collect::<HashMap<String, Robj>>())
        } else {
            Err("expected a list")
        }
    }
}

impl<'a> FromRobj<'a> for HashMap<&str, Robj> {
    fn from_robj(robj: &'a Robj) -> std::result::Result<Self, &'static str> {
        if let Some(iter) = robj.as_list().map(|l| l.iter()) {
            Ok(iter.map(|(k, v)| (k, v)).collect::<HashMap<&str, Robj>>())
        } else {
            Err("expected a list")
        }
    }
}

// NA-sensitive integer input handling
impl<'a> FromRobj<'a> for Option<i32> {
    fn from_robj(robj: &'a Robj) -> std::result::Result<Self, &'static str> {
        if robj.is_na() {
            Ok(None)
        } else if let Some(val) = robj.as_integer() {
            Ok(Some(val))
        } else {
            Err("expected an integer scalar")
        }
    }
}

// NA-sensitive logical input handling
impl<'a> FromRobj<'a> for Option<bool> {
    fn from_robj(robj: &'a Robj) -> std::result::Result<Self, &'static str> {
        if let Some(val) = robj.as_logical() {
            if val.is_na() {
                Ok(None)
            } else {
                Ok(Some(val.is_true()))
            }
        } else {
            Err("expected a logical scalar")
        }
    }
}

// NA-sensitive real input handling
impl<'a> FromRobj<'a> for Option<f64> {
    fn from_robj(robj: &'a Robj) -> std::result::Result<Self, &'static str> {
        if robj.is_na() {
            Ok(None)
        } else if let Some(val) = robj.as_real() {
            Ok(Some(val))
        } else {
            Err("expected a real scalar")
        }
    }
}

// NA-sensitive string input handling
impl<'a> FromRobj<'a> for Option<&'a str> {
    fn from_robj(robj: &'a Robj) -> std::result::Result<Self, &'static str> {
        if robj.is_na() {
            Ok(None)
        } else if let Some(val) = robj.as_str() {
            Ok(Some(val))
        } else {
            Err("expected a character scalar")
        }
    }
}

// NA-sensitive string input handling
impl<'a> FromRobj<'a> for Option<String> {
    fn from_robj(robj: &'a Robj) -> std::result::Result<Self, &'static str> {
        if robj.is_na() {
            Ok(None)
        } else if let Some(val) = robj.as_str() {
            Ok(Some(val.to_string()))
        } else {
            Err("expected a character scalar")
        }
    }
}

impl<'a, T> FromRobj<'a> for &'a [T]
where
    Robj: AsTypedSlice<'a, T>,
{
    fn from_robj(robj: &'a Robj) -> std::result::Result<Self, &'static str> {
        if let Some(slice) = robj.as_typed_slice() {
            Ok(slice)
        } else {
            Err("Expected a vector type.")
        }
    }
}

// Matrix input parameters.
impl<'a, T: 'a> FromRobj<'a> for RArray<T, [usize; 2]>
where
    Robj: AsTypedSlice<'a, T>,
{
    fn from_robj(robj: &'a Robj) -> std::result::Result<Self, &'static str> {
        match robj.as_matrix() {
            Some(x) => Ok(x),
            _ => Err("Expected a matrix."),
        }
    }
}

// Matrix input parameters.
impl<'a, T: 'a> FromRobj<'a> for RMatrix3D<T>
where
    Robj: AsTypedSlice<'a, T>,
{
    fn from_robj(robj: &'a Robj) -> std::result::Result<Self, &'static str> {
        match robj.as_matrix3d() {
            Some(x) => Ok(x),
            _ => Err("Expected a matrix."),
        }
    }
}
