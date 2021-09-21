use libR_sys::{R_IsNA, R_NaReal};
use crate::{Bool, na_str};
/// Return true if this primitive is NA.
pub trait IsNA {
    fn is_na(&self) -> bool;
    fn na() -> Self;
}

impl IsNA for f64 {
    fn is_na(&self) -> bool {
        unsafe { R_IsNA(*self) != 0 }
    }

    fn na() -> f64 {
        unsafe{R_NaReal}
    }
}

impl IsNA for i32 {
    fn is_na(&self) -> bool {
        *self == i32::MIN
    }

    fn na() -> i32 {
        i32::MIN
    }
}

impl IsNA for Bool {
    fn is_na(&self) -> bool {
        self.0 == i32::MIN
    }

    fn na() -> Bool {
        Bool::from(i32::MIN)
    }
}

impl IsNA for &str {
    /// Check for NA in a string by address.
    fn is_na(&self) -> bool {
        self.as_ptr() == na_str().as_ptr()
    }

    fn na() -> Self {
        na_str()
    }
}