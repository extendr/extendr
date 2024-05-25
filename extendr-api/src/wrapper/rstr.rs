use super::*;

/// Wrapper for creating CHARSXP objects.
/// These are used only as the contents of a character
/// vector.
///
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let chr = r!(Rstr::from_string("xyz"));
///     assert_eq!(chr.as_char().unwrap().as_str(), "xyz");
/// }
/// ```
///
#[derive(Clone)]
pub struct Rstr {
    pub(crate) robj: Robj,
}

/// Returns a rust string-slice based on the provided `SEXP`, which is of type
/// [`SEXPTYPE::CHARSXP`]. Note that the length of a `CHARSXP` is exactly
/// the number of non-null bytes in said R character.
pub(crate) unsafe fn charsxp_to_str(charsxp: SEXP) -> Option<&'static str> {
    assert_eq!(TYPEOF(charsxp), SEXPTYPE::CHARSXP);
    if charsxp == R_NilValue {
        None
    } else if charsxp == R_NaString {
        Some(<&str>::na())
    } else if charsxp == R_BlankString {
        Some("")
    } else {
        let length = Rf_xlength(charsxp);
        let all_bytes =
            std::slice::from_raw_parts(R_CHAR(charsxp).cast(), length.try_into().unwrap());
        Some(std::str::from_utf8_unchecked(all_bytes))
    }
}

impl Rstr {
    /// Make a character object from a string.
    pub fn from_string(val: &str) -> Self {
        Rstr {
            robj: Robj::from_sexp(str_to_character(val)),
        }
    }

    /// Get the string from a character object.
    /// If the string is NA, then the special na_str() is returned.
    pub fn as_str(&self) -> &str {
        self.into()
    }
}

impl AsRef<str> for Rstr {
    /// Treat a Rstr as a string slice.
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<String> for Rstr {
    /// Convert a String to a Rstr.
    fn from(s: String) -> Self {
        Rstr::from_string(&s)
    }
}

impl From<&str> for Rstr {
    /// Convert a string slice to a Rstr.
    fn from(s: &str) -> Self {
        Rstr::from_string(s)
    }
}

impl From<&Rstr> for &str {
    fn from(value: &Rstr) -> Self {
        unsafe {
            let charsxp = value.robj.get();
            rstr::charsxp_to_str(charsxp).unwrap()
        }
    }
}

impl From<Option<String>> for Rstr {
    fn from(value: Option<String>) -> Self {
        if let Some(string) = value {
            Self::from(string)
        } else {
            Self { robj: na_string() }
        }
    }
}

impl Deref for Rstr {
    type Target = str;

    /// Treat `Rstr` like `&str`.
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl<T> PartialEq<T> for Rstr
where
    T: AsRef<str>,
{
    /// Compare a `Rstr` with a `Rstr`.
    fn eq(&self, other: &T) -> bool {
        self.as_str() == other.as_ref()
    }
}

impl PartialEq<str> for Rstr {
    /// Compare a `Rstr` with a string slice.
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl std::fmt::Debug for Rstr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_na() {
            write!(f, "NA_CHARACTER")
        } else {
            let s = self.as_str();
            write!(f, "{:?}", s)
        }
    }
}

impl std::fmt::Display for Rstr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.as_str();
        write!(f, "{}", s)
    }
}

impl CanBeNA for Rstr {
    fn is_na(&self) -> bool {
        unsafe { self.robj.get() == R_NaString }
    }

    fn na() -> Self {
        unsafe {
            Self {
                robj: Robj::from_sexp(R_NaString),
            }
        }
    }
}
