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
