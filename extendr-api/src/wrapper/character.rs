use super::*;

/// Wrapper for creating character objects.
/// These are used only as the contents of a character
/// vector.
///
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let chr = r!(Character("xyz"));
///     assert_eq!(chr.as_character(), Some(Character("xyz")));
/// }
/// ```
///
#[derive(Debug, PartialEq, Clone)]
pub struct Character<'a>(pub &'a str);

/// Convert a wrapped string ref to an R character object (element of a character vector).
impl<'a> From<Character<'a>> for Robj {
    fn from(val: Character) -> Self {
        single_threaded(|| unsafe { new_owned(str_to_character(val.0)) })
    }
}

impl<'a> FromRobj<'a> for Character<'a> {
    /// Convert an input value to a Character wrapper around a string.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let robj = r!(Character("xyz"));
    ///     assert_eq!(<Character>::from_robj(&robj).unwrap(), Character("xyz"));
    /// }
    /// ```
    fn from_robj(robj: &'a Robj) -> std::result::Result<Self, &'static str> {
        if let Some(f) = robj.as_character() {
            Ok(f)
        } else {
            Err("Not a character object.")
        }
    }
}
