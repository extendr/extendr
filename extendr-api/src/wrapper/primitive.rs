use super::*;

/// Wrapper for creating primitive objects.
///
/// Make a primitive object, or NULL if not available.
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let builtin = r!(Primitive::from_str("+"));
///     let special = r!(Primitive::from_str("if"));
/// }
/// ```
///
#[derive(Debug, PartialEq, Clone)]
pub struct Primitive {
    pub(crate) robj: Robj,
}

impl Primitive {
    /// Make a Primitive object from a string.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let builtin = r!(Primitive::from_string("+")?);
    ///     let special = r!(Primitive::from_string("if")?);
    ///     assert_eq!(builtin.rtype(), RType::Builtin);
    ///     assert_eq!(special.rtype(), RType::Special);
    /// }
    /// ```
    pub fn from_string(val: &str) -> Result<Self> {
        single_threaded(|| unsafe {
            // Primitives have a special "SYMVALUE" entry in their symbol.
            let sym = Symbol::from_string(val);
            let symvalue = new_owned(SYMVALUE(sym.get()));
            if symvalue.is_primitive() {
                Ok(Primitive { robj: symvalue })
            } else {
                Err(Error::ExpectedPrimitive(sym.into()))
            }
        })
    }

    // There is currently no way to convert a primitive to a string.
}
