use super::*;

/// Wrapper for creating and reading Primitive functions.
///
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let robj = r!(Primitive("+"));
///     assert!(robj.is_primitive());
///     assert!(!r!(Primitive("not_a_primitive")).is_primitive());
/// }
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct Primitive<'a>(pub &'a str);

impl<'a> From<Primitive<'a>> for Robj {
    /// Make a primitive object, or NULL if not available.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let builtin = r!(Primitive("+"));
    ///     let special = r!(Primitive("if"));
    /// }
    /// ```
    fn from(name: Primitive) -> Self {
        single_threaded(|| unsafe {
            let sym = make_symbol(name.0);
            let symvalue = new_sys(SYMVALUE(sym));
            if symvalue.is_primitive() {
                symvalue
            } else {
                r!(NULL)
            }
        })
    }
}
