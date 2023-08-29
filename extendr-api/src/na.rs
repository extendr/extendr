use lazy_static::lazy_static;
use libR_sys::{R_IsNA, R_NaReal};
use std::alloc::{self, Layout};

// To make sure this "NA" is allocated at a different place than any other "NA"
// strings (so that it can be used as a sentinel value), we allocate it by
// ourselves.
lazy_static! {
    static ref EXTENDR_NA_STRING: &'static str = unsafe {
        // Layout::array() can fail when the size exceeds `isize::MAX`, but we
        // only need 2 here, so it's safe to unwrap().
        let layout = Layout::array::<u8>(2).unwrap();

        // We allocate and never free it because we need this pointer to be
        // alive until the program ends.
        let ptr = alloc::alloc(layout);

        let v: &mut [u8] = std::slice::from_raw_parts_mut(ptr, 2);
        v[0] = b'N';
        v[1] = b'A';

        std::str::from_utf8_unchecked(v)
    };
}

/// Trait signifying if a given type may be represented as an `NA` vlaue in an
/// R vector. The `is_na_scalar` method is intended to be used on individual
/// elements of a vector. It is not vectorized and will return `false` if the
/// vector has a length greater than 1.
pub trait CanBeNA {
    /// Returns `true` if it is `NA`.
    fn is_na(&self) -> bool;
    /// Returns the sentinel `NA` value for this type.
    fn na() -> Self;
}

/// ```
/// use extendr_api::prelude::*;
/// test! {
///     assert!(f64::na().is_na());
/// }
/// ```
impl CanBeNA for f64 {
    fn is_na(&self) -> bool {
        unsafe { R_IsNA(*self) != 0 }
    }

    fn na() -> f64 {
        unsafe { R_NaReal }
    }
}

/// ```
/// use extendr_api::prelude::*;
/// test! {
///     assert!(i32::na().is_na());
/// }
/// ```
impl CanBeNA for i32 {
    fn is_na(&self) -> bool {
        *self == i32::na()
    }

    fn na() -> i32 {
        i32::MIN
    }
}

/// Special "NA" string that represents null strings.
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     assert_ne!(<&str>::na().as_ptr(), "NA".as_ptr());
///     assert_eq!(<&str>::na(), "NA");
///     assert_eq!("NA".is_na(), false);
///     assert_eq!(<&str>::na().is_na(), true);
/// }
/// ```
impl CanBeNA for &str {
    /// Check for NA in a string by address.
    fn is_na(&self) -> bool {
        self.as_ptr() == <&str>::na().as_ptr()
    }

    fn na() -> Self {
        &EXTENDR_NA_STRING
    }
}
