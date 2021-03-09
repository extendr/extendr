use super::*;

/// Wrapper for creating symbols.
///
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let symbol = r!(Symbol("xyz"));
///     assert_eq!(symbol.as_symbol(), Some(Symbol("xyz")));
///     assert!(symbol.is_symbol());
/// }
/// ```
/// Note that creating a symbol from a string is expensive
/// and so you may want to cache them.
///
#[derive(Debug, PartialEq, Clone)]
pub struct Symbol<'a>(pub &'a str);

impl<'a> From<Symbol<'a>> for Robj {
    /// Make a symbol object.
    fn from(name: Symbol) -> Self {
        single_threaded(|| unsafe { new_owned(make_symbol(name.0)) })
    }
}

/// Allow you to skip the Symbol() in some cases.
impl<'a> From<&'a str> for Symbol<'a> {
    fn from(val: &'a str) -> Self {
        Self(val)
    }
}

impl<'a> FromRobj<'a> for Symbol<'a> {
    /// Convert an Robj to a Symbol wrapper.
    fn from_robj(robj: &'a Robj) -> std::result::Result<Self, &'static str> {
        if let Some(x) = robj.as_symbol() {
            Ok(x)
        } else {
            Err("Expected a symbol.")
        }
    }
}
