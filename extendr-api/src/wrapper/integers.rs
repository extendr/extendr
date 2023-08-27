use super::scalar::{Rint, Scalar};
use super::*;
use std::iter::FromIterator;

/// An obscure `NA`-aware wrapper for R's integer vectors.
/// Can be used to iterate over vectors obtained from R
/// or to create new vectors that can be returned back to R.
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let mut vec = (0..5).map(|i| i.into()).collect::<Integers>();
///     vec.iter_mut().for_each(|v| *v = *v + 10);
///     assert_eq!(vec.elt(0), 10);
///     let sum = vec.iter().sum::<Rint>();
///     assert_eq!(sum, 60);
/// }
/// ```  
#[derive(PartialEq, Clone)]
pub struct Integers {
    pub(crate) robj: Robj,
}

crate::wrapper::macros::gen_vector_wrapper_impl!(
    vector_type: Integers, // Implements for
    scalar_type: Rint,     // Element type
    primitive_type: i32,   // Raw element type
    r_prefix: INTEGER,     // `R` functions prefix
    SEXP: INTSXP,          // `SEXP`
    doc_name: integer,     // Singular type name used in docs
    altrep_constructor: make_altinteger_from_iterator,
);

impl Integers {
    /// Get a region of elements from the vector.
    pub fn get_region(&self, index: usize, dest: &mut [Rint]) -> usize {
        unsafe {
            let ptr: *mut i32 = dest.as_mut_ptr() as *mut i32;
            INTEGER_GET_REGION(self.get(), index as R_xlen_t, dest.len() as R_xlen_t, ptr) as usize
        }
    }

    /// Return `TRUE` if the vector is sorted, `FALSE` if not, or `NA_BOOL` if unknown.
    pub fn is_sorted(&self) -> Rbool {
        unsafe { INTEGER_IS_SORTED(self.get()).into() }
    }

    /// Return `TRUE` if the vector has no `NA`s, `FALSE` if any, or `NA_BOOL` if unknown.
    pub fn no_na(&self) -> Rbool {
        unsafe { INTEGER_NO_NA(self.get()).into() }
    }
}

#[cfg(test)]
mod tests;

// TODO: this should be a trait.
impl Integers {
    pub fn set_elt(&mut self, index: usize, val: Rint) {
        unsafe {
            SET_INTEGER_ELT(self.get(), index as R_xlen_t, val.inner());
        }
    }
}

impl Deref for Integers {
    type Target = [Rint];

    /// Treat Integers as if it is a slice, like `Vec<Rint>`
    fn deref(&self) -> &Self::Target {
        unsafe {
            let ptr = DATAPTR_RO(self.get()) as *const Rint;
            std::slice::from_raw_parts(ptr, self.len())
        }
    }
}

impl DerefMut for Integers {
    /// Treat Integers as if it is a mutable slice, like `Vec<Rint>`
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            let ptr = DATAPTR(self.get()) as *mut Rint;
            std::slice::from_raw_parts_mut(ptr, self.len())
        }
    }
}

impl std::fmt::Debug for Integers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.len() == 1 {
            write!(f, "{:?}", self.elt(0))
        } else {
            f.debug_list().entries(self.iter()).finish()
        }
    }
}
