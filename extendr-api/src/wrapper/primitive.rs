use super::*;

/// Wrapper for creating primitive objects.
///
/// Make a primitive object, or `NULL` if not available
///
#[derive(PartialEq, Clone)]
pub struct Primitive {
    pub(crate) robj: Robj,
}

impl Primitive {
    #[cfg(feature = "non-api")]
    /// Make a Primitive object from a string.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let builtin = r!(Primitive::from_string("+")?);
    ///     let special = r!(Primitive::from_string("if")?);
    ///     assert_eq!(builtin.rtype(), Rtype::Builtin);
    ///     assert_eq!(special.rtype(), Rtype::Special);
    /// }
    /// ```
    pub fn from_string(val: &str) -> Result<Self> {
        single_threaded(|| unsafe {
            // Primitives have a special "SYMVALUE" entry in their symbol.
            let sym = Symbol::from_string(val);
            let symvalue = Robj::from_sexp(SYMVALUE(sym.get()));
            if symvalue.is_primitive() {
                Ok(Primitive { robj: symvalue })
            } else {
                Err(Error::ExpectedPrimitive(sym.into()))
            }
        })
    }

    // There is currently no way to convert a primitive to a string.
}

impl std::fmt::Debug for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self.robj.deparse().unwrap();
        write!(f, "{:?}", s)
    }
}
