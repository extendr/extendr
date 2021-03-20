use super::*;

/// Wrapper for creating raw (byte) objects.
///
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let bytes = r!(Raw::from_bytes(&[1, 2, 3]));
///     assert_eq!(bytes.len(), 3);
///     assert_eq!(bytes.as_raw(), Some(Raw::from_bytes(&[1, 2, 3])));
/// }
/// ```
///
#[derive(Debug, PartialEq, Clone)]
pub struct Raw {
    pub(crate) robj: Robj,
}

impl Raw {
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let bytes = r!(Raw::from_bytes(&[1, 2, 3]));
    ///     assert_eq!(bytes.len(), 3);
    ///     assert_eq!(bytes.as_raw(), Some(Raw::from_bytes(&[1, 2, 3])));
    /// }
    /// ```
    pub fn from_bytes(bytes: &[u8]) -> Self {
        single_threaded(|| unsafe {
            let sexp = Rf_allocVector(RAWSXP, bytes.len() as R_xlen_t);
            let robj = new_owned(sexp);
            let ptr = RAW(sexp);
            for (i, &v) in bytes.into_iter().enumerate() {
                *ptr.add(i) = v;
            }
            Raw { robj }
        })
    }

    /// Get a slice of bytes from the Raw object.
    pub fn as_slice(&self) -> &[u8] {
        self.robj.as_typed_slice().unwrap()
    }
}
