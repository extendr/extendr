use super::scalar::{Rbool, Scalar};
use super::*;
use std::iter::FromIterator;

/// An obscure `NA`-aware wrapper for R's logical vectors.
/// Can be used to iterate over vectors obtained from R
/// or to create new vectors that can be returned back to R.
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     // Collect builds a Logicals from an iterator
///     let mut vec = (0..5).map(|i| (i % 2 == 0).into()).collect::<Logicals>();
///     // elt accesses a single element (altrep aware).
///     assert_eq!(vec.elt(0), true);
///     // Logicals behaves like &[Rbool]
///     assert_eq!(vec[1], false);
/// }
/// ```  
#[derive(PartialEq, Clone)]
pub struct Logicals {
    pub(crate) robj: Robj,
}

crate::wrapper::macros::gen_vector_wrapper_impl!(
    vector_type: Logicals, // Implements for
    scalar_type: Rbool,    // Element type
    primitive_type: i32,   // Raw element type
    r_prefix: LOGICAL,     // `R` functions prefix
    SEXP: LGLSXP,          // `SEXP`
    doc_name: logical,     // Singular type name used in docs
    altrep_constructor: make_altlogical_from_iterator,
);

impl Logicals {
    /// Get a region of elements from the vector.
    pub fn get_region(&self, index: usize, dest: &mut [Rbool]) -> usize {
        unsafe {
            let ptr: *mut i32 = dest.as_mut_ptr() as *mut i32;
            LOGICAL_GET_REGION(self.get(), index as R_xlen_t, dest.len() as R_xlen_t, ptr) as usize
        }
    }
}

#[cfg(test)]
mod tests;

// TODO: this should be a trait.
impl Logicals {
    pub fn set_elt(&mut self, index: usize, val: Rbool) {
        unsafe {
            SET_INTEGER_ELT(self.get(), index as R_xlen_t, val.inner());
        }
    }
}

impl Deref for Logicals {
    type Target = [Rbool];

    /// Treat Logicals as if it is a slice, like `Vec<Rint>`
    fn deref(&self) -> &Self::Target {
        unsafe {
            let ptr = DATAPTR_RO(self.get()) as *const Rbool;
            std::slice::from_raw_parts(ptr, self.len())
        }
    }
}

impl DerefMut for Logicals {
    /// Treat Logicals as if it is a mutable slice, like `Vec<Rint>`
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            let ptr = DATAPTR(self.get()) as *mut Rbool;
            std::slice::from_raw_parts_mut(ptr, self.len())
        }
    }
}

impl std::fmt::Debug for Logicals {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.len() == 1 {
            write!(f, "{:?}", self.elt(0))
        } else {
            f.debug_list().entries(self.iter()).finish()
        }
    }
}
