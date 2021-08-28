use super::*;

/// Wrapper for creating CHARSXP objects.
/// These are used only as the contents of a character
/// vector.
///
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let chr = r!(Char::from_string("xyz"));
///     assert_eq!(chr.as_char().unwrap().as_str(), "xyz");
/// }
/// ```
///
#[derive(Debug, PartialEq, Clone)]
pub struct Char {
    pub(crate) robj: Robj,
}

impl Char {
    /// Make a character object from a string.
    pub fn from_string(val: &str) -> Self {
        unsafe {
            Char {
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
                na_str()
            } else {
                to_str(R_CHAR(sexp) as *const u8)
            }
        }
    }
}
