use super::*;
use extendr_ffi::{R_BlankString, R_NaString, R_NilValue, Rf_xlength, R_CHAR, SEXPTYPE, TYPEOF};
/// Wrapper for creating CHARSXP objects.
/// These are used only as the contents of a character
/// vector.
///
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let chr = r!(RStr::from("xyz"));
///     assert_eq!(chr.as_char().unwrap().as_ref(), "xyz");
/// }
/// ```
///
#[derive(Clone)]
pub struct RStr {
    pub(crate) robj: RObj,
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

impl RStr {
    /// Make a character object from a string.
    ///
    /// # Deprecated
    /// Use `RStr::from()` or `.into()` instead, which implement the standard `From<&str>` trait.
    ///
    /// # Examples
    /// ```
    /// use extendr_api::prelude::*;
    /// # fn example() {
    /// let rstr = RStr::from("hello");
    /// // or
    /// let rstr: RStr = "hello".into();
    /// # }
    /// ```
    #[deprecated(since = "0.8.1", note = "Use `RStr::from()` or `.into()` instead")]
    pub fn from_string(val: &str) -> Self {
        RStr {
            robj: unsafe { RObj::from_sexp(str_to_character(val)) },
        }
    }

    /// Get the string from a character object.
    /// If the string is NA, then the special na_str() is returned.
    ///
    /// # Deprecated
    /// Use `.as_ref()` (from `AsRef<str>` trait) or rely on `Deref` coercion instead.
    ///
    /// # Examples
    /// ```
    /// use extendr_api::prelude::*;
    /// # fn example() {
    /// # let rstr = RStr::from("hello");
    /// let s: &str = rstr.as_ref();
    /// // or use Deref coercion
    /// let len = rstr.len(); // calls str::len() via Deref
    /// # }
    /// ```
    #[deprecated(
        since = "0.8.1",
        note = "Use `.as_ref()` or rely on `Deref` coercion instead"
    )]
    pub fn as_str(&self) -> &str {
        self.into()
    }
}

impl AsRef<str> for RStr {
    /// Treat a RStr as a string slice.
    fn as_ref(&self) -> &str {
        self.into()
    }
}

impl From<String> for RStr {
    /// Convert a String to a RStr.
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

impl From<&str> for RStr {
    /// Convert a string slice to a RStr.
    fn from(s: &str) -> Self {
        RStr {
            robj: unsafe { RObj::from_sexp(str_to_character(s)) },
        }
    }
}

impl From<&RStr> for &str {
    fn from(value: &RStr) -> Self {
        unsafe {
            let charsxp = value.robj.get();
            rstr::charsxp_to_str(charsxp).unwrap()
        }
    }
}

impl From<Option<String>> for RStr {
    fn from(value: Option<String>) -> Self {
        if let Some(string) = value {
            Self::from(string)
        } else {
            Self { robj: na_string() }
        }
    }
}

impl From<Option<&str>> for RStr {
    fn from(value: Option<&str>) -> Self {
        if let Some(string_ref) = value {
            Self::from(string_ref)
        } else {
            Self { robj: na_string() }
        }
    }
}

impl Deref for RStr {
    type Target = str;

    /// Treat `RStr` like `&str`.
    fn deref(&self) -> &Self::Target {
        self.into()
    }
}

/// Defer comparison to R's string interner
impl PartialEq<RStr> for RStr {
    fn eq(&self, other: &RStr) -> bool {
        unsafe { self.robj.get() == other.robj.get() }
    }
}

/// Let performant than comparing [RStr] directly as
/// we need to convert [RStr] to a string slice first
impl PartialEq<str> for RStr {
    /// Compare a `RStr` with a string slice.
    fn eq(&self, other: &str) -> bool {
        self.as_ref() == other
    }
}

impl PartialEq<RStr> for &str {
    /// Compare a `RStr` with a string slice.
    fn eq(&self, other: &RStr) -> bool {
        *self == other.as_ref()
    }
}

impl PartialEq<&str> for RStr {
    /// Compare a `RStr` with a string slice.
    fn eq(&self, other: &&str) -> bool {
        self.as_ref() == *other
    }
}

impl PartialEq<RStr> for &&str {
    /// Compare a `RStr` with a string slice.
    fn eq(&self, other: &RStr) -> bool {
        **self == other.as_ref()
    }
}

impl std::fmt::Debug for RStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_na() {
            write!(f, "NA_CHARACTER")
        } else {
            let s: &str = self.as_ref();
            write!(f, "{:?}", s)
        }
    }
}

impl std::fmt::Display for RStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: &str = self.as_ref();
        write!(f, "{}", s)
    }
}

impl CanBeNA for RStr {
    fn is_na(&self) -> bool {
        unsafe { self.robj.get() == R_NaString }
    }

    fn na() -> Self {
        unsafe {
            Self {
                robj: RObj::from_sexp(R_NaString),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate as extendr_api;

    #[test]
    fn test_rstr_as_char() {
        test! {
            let chr = r!(RStr::from("xyz"));
            let x = chr.as_char().unwrap();
            assert_eq!(x.as_ref(), "xyz");
        }
    }

    #[test]
    fn test_rstr_from_str_ref() {
        test! {
            assert_eq!(RStr::from(Some("value")), RStr::from("value"));
        }
    }
}

#[deprecated(note = "Use RStr instead", since = "0.9.0")]
pub type Rstr = RStr;
