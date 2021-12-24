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
#[derive(PartialEq, Clone)]
pub struct Raw {
    pub(crate) robj: Robj,
}

impl Raw {
    /// Create a new Raw object of length `len`.
    pub fn new(len: usize) -> Raw {
        let mut robj = Robj::alloc_vector(RAWSXP, len);
        let slice = robj.as_raw_slice_mut().unwrap();
        slice.iter_mut().for_each(|v| *v = 0);
        Raw { robj }
    }

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
            let robj = Robj::from_sexp(sexp);
            let ptr = RAW(sexp);
            for (i, &v) in bytes.iter().enumerate() {
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

impl std::fmt::Debug for Raw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Raw")?;
        f.debug_list().entries(self.as_slice()).finish()
    }
}
