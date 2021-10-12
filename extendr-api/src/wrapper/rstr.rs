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
#[derive(Debug, PartialEq, Clone)]
pub struct Rstr {
    pub(crate) robj: Robj,
}

pub(crate) unsafe fn sexp_to_str(sexp: SEXP) -> &'static str {
    if sexp == R_NaString {
        <&str>::na()
    } else {
        std::mem::transmute(to_str(R_CHAR(sexp) as *const u8))
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
        unsafe { sexp_to_str(self.robj.get()) }
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
