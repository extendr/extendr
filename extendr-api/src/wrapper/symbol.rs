use super::*;

/// Wrapper for creating symbol objects.
///
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let chr = r!(Symbol::from_str("xyz"));
///     assert_eq!(chr.as_symbol().unwrap().as_str(), "xyz");
/// }
/// ```
///
#[derive(Debug, PartialEq, Clone)]
pub struct Symbol {
    pub(crate) robj: Robj,
}

impl Symbol {
    /// Make a symbol object from a string.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let chr = r!(Symbol::from_str("xyz"));
    ///     assert_eq!(chr, sym!(xyz));
    /// }
    /// ```
    pub fn from_str(val: &str) -> Self {
        Symbol {
            robj: unsafe { new_owned(make_symbol(val)) },
        }
    }

    /// Get the string from a symbol object.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     assert_eq!(sym!(xyz).as_symbol().unwrap().as_str(), "xyz");
    /// }
    /// ```
    pub fn as_str(&self) -> &str {
        unsafe {
            let sexp = self.robj.get();
            let printname = PRINTNAME(sexp);
            assert!(TYPEOF(printname) as u32 == CHARSXP);
            to_str(R_CHAR(printname) as *const u8)
        }
    }
}
