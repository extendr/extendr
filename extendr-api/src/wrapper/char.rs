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

impl Rstr {
    /// Make a character object from a string.
    pub fn from_string(val: &str) -> Self {
        unsafe {
            Rstr {
                robj: new_owned(str_to_character(val)),
            }
        }
    }

    /// Get the string from a character object.
    /// If the string is NA, then the special na_str() is returned.
    pub fn as_str(&self) -> &str {
        unsafe {
            let sexp = self.robj.get();
            if sexp == R_NaString {
                <&str>::na()
            } else {
                to_str(R_CHAR(sexp) as *const u8)
            }
        }
    }
}
