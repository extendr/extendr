use libR_sys::{R_IsNA, R_NaReal};
use once_cell::sync::Lazy;
use std::alloc::{self, Layout};

// To make sure this "NA" is allocated at a different place than any other "NA"
// strings (so that it can be used as a sentinel value), we allocate it by
// ourselves.
static EXTENDR_NA_STRING: Lazy<&'static str> = Lazy::new(|| unsafe {
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
});

/// Return true if this primitive is `NA`.
pub trait CanBeNA {
    fn is_na(&self) -> bool;
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
