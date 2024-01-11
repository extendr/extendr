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

fn init_raw<F: FnOnce(&mut [u8])>(len: usize, filler: F) -> Raw {
    let mut robj = Robj::alloc_vector(RAWSXP, len);
    let slice = robj.as_raw_slice_mut().unwrap();
    filler(slice);
    Raw { robj }
}

impl Raw {
    /// Create a new Raw object of length `len`.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let bytes = Raw::new(42);
    ///     assert_eq!(bytes.len(), 42);
    ///     assert_eq!(bytes.as_slice(),  &(0..42).map(|_| 0u8).collect::<Vec<_>>());
    /// }
    /// ```
    pub fn new(len: usize) -> Raw {
        init_raw(len, |slice| slice.fill(0))
    }

    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let bytes = Raw::from_bytes(&[1, 2, 3]);
    ///     assert_eq!(bytes.len(), 3);
    ///     assert_eq!(bytes.as_slice(), &[1, 2, 3]);
    /// }
    /// ```
    pub fn from_bytes(bytes: &[u8]) -> Self {
        init_raw(bytes.len(), |slice| slice.copy_from_slice(bytes))
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
