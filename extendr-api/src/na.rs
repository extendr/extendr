use libR_sys::R_IsNA;
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
}

impl IsNA for i32 {
    fn is_na(&self) -> bool {
        *self == std::i32::MIN
    }
}

impl IsNA for Bool {
    fn is_na(&self) -> bool {
        self.0 == std::i32::MIN
    }
}

impl IsNA for &str {
    /// Check for NA in a string by address.
    fn is_na(&self) -> bool {
        self.as_ptr() == na_str().as_ptr()
    }
}