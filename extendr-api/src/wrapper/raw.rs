use super::*;

/// Wrapper for creating raw (byte) objects.
///
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let bytes = r!(Raw(&[1, 2, 3]));
///     assert_eq!(bytes.len(), 3);
///     assert_eq!(bytes.as_raw(), Some(Raw(&[1, 2, 3])));
/// }
/// ```
///
#[derive(Debug, PartialEq, Clone)]
pub struct Raw<'a>(pub &'a [u8]);

impl<'a> From<Raw<'a>> for Robj {
    /// Make a raw object from bytes.
    fn from(val: Raw<'a>) -> Self {
        single_threaded(|| unsafe {
            let val = val.0;
            let sexp = Rf_allocVector(RAWSXP, val.len() as R_xlen_t);
            ownership::protect(sexp);
            let ptr = RAW(sexp);
            for (i, &v) in val.iter().enumerate() {
                *ptr.offset(i as isize) = v;
            }
            Robj::Owned(sexp)
        })
    }
}

impl<'a> FromRobj<'a> for Raw<'a> {
    /// Convert an input value to a Raw wrapper around bytes.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let robj = r!(Raw(&[1, 2]));
    ///     assert_eq!(<Raw>::from_robj(&robj).unwrap(), Raw(&[1, 2]));
    /// }
    /// ```
    fn from_robj(robj: &'a Robj) -> std::result::Result<Self, &'static str> {
        if let Some(f) = robj.as_raw() {
            Ok(f)
        } else {
            Err("Not a raw object.")
        }
    }
}
