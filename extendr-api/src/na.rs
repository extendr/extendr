use libR_sys::{R_IsNA, R_NaReal};
/// Return true if this primitive is NA.
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
