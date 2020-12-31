use super::*;

/// Trait used for incomming parameter conversion.
pub trait FromRobj<'a>: Sized {
    // Convert an incomming Robj from R into a value or an error.
    fn from_robj(_robj: &'a Robj) -> Result<Self, &'static str> {
        Err("unable to convert value from R object")
    }
}

macro_rules! impl_prim_from_robj {
    ($t: ty) => {
        impl<'a> FromRobj<'a> for $t {
            fn from_robj(robj: &'a Robj) -> Result<Self, &'static str> {
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
    fn from_robj(robj: &'a Robj) -> Result<Self, &'static str> {
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
    fn from_robj(robj: &'a Robj) -> Result<Self, &'static str> {
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
    fn from_robj(robj: &'a Robj) -> Result<Self, &'static str> {
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
    fn from_robj(robj: &'a Robj) -> Result<Self, &'static str> {
        if let Some(v) = robj.as_integer_slice() {
            Ok(Vec::from(v))
        } else {
            Err("not an integer or logical vector")
        }
    }
}

impl<'a> FromRobj<'a> for Vec<f64> {
    fn from_robj(robj: &'a Robj) -> Result<Self, &'static str> {
        if let Some(v) = robj.as_real_slice() {
            Ok(Vec::from(v))
        } else {
            Err("not a floating point vector")
        }
    }
}

macro_rules! impl_iter_from_robj {
    ($t: ty, $iter_fn: ident, $msg: expr) => {
        impl<'a> FromRobj<'a> for $t {
            fn from_robj(robj: &'a Robj) -> Result<Self, &'static str> {
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
impl_iter_from_robj!(ListIter, as_list_iter, "Not a list.");
impl_iter_from_robj!(IntegerIter<'a>, as_integer_iter, "Not an integer vector.");
impl_iter_from_robj!(RealIter<'a>, as_real_iter, "Not a real vector.");
impl_iter_from_robj!(LogicalIter<'a>, as_logical_iter, "Not a logical vector.");

/// Pass-through Robj conversion.
impl<'a> FromRobj<'a> for Robj {
    fn from_robj(robj: &'a Robj) -> Result<Self, &'static str> {
        Ok(unsafe { new_borrowed(robj.get()) })
    }
}

impl<'a> FromRobj<'a> for HashMap<String, Robj> {
    fn from_robj(robj: &'a Robj) -> Result<Self, &'static str> {
        if let Some(iter) = robj.as_named_list_iter() {
            Ok(iter
                .map(|(k, v)| (k.to_string(), v.to_owned()))
                .collect::<HashMap<String, Robj>>())
        } else {
            Err("expected a list")
        }
    }
}

impl<'a> FromRobj<'a> for HashMap<&str, Robj> {
    fn from_robj(robj: &'a Robj) -> Result<Self, &'static str> {
        if let Some(iter) = robj.as_named_list_iter() {
            Ok(iter.map(|(k, v)| (k, v)).collect::<HashMap<&str, Robj>>())
        } else {
            Err("expected a list")
        }
    }
}

// NA-sensitive integer input handling
impl<'a> FromRobj<'a> for Option<i32> {
    fn from_robj(robj: &'a Robj) -> Result<Self, &'static str> {
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
    fn from_robj(robj: &'a Robj) -> Result<Self, &'static str> {
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
    fn from_robj(robj: &'a Robj) -> Result<Self, &'static str> {
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
    fn from_robj(robj: &'a Robj) -> Result<Self, &'static str> {
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
    fn from_robj(robj: &'a Robj) -> Result<Self, &'static str> {
        if robj.is_na() {
            Ok(None)
        } else if let Some(val) = robj.as_str() {
            Ok(Some(val.to_string()))
        } else {
            Err("expected a character scalar")
        }
    }
}
