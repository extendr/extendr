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

use libR_sys::SEXPTYPE::INTSXP;
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

// TODO: this should be a trait.
impl Integers {
    pub fn set_elt(&mut self, index: usize, val: Rint) {
        single_threaded(|| unsafe {
            SET_INTEGER_ELT(self.get(), index as R_xlen_t, val.inner());
        })
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
            let ptr = DATAPTR(self.get_mut()) as *mut Rint;
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

impl TryFrom<Vec<i32>> for Integers {
    type Error = Error;

    fn try_from(value: Vec<i32>) -> std::result::Result<Self, Self::Error> {
        Ok(Self { robj: value.into() })
    }
}

#[cfg(test)]
mod tests {
    use crate as extendr_api;
    use crate::prelude::*;

    #[test]
    fn from_iterator() {
        test! {
            let vec : Integers = (0..3).map(|i| i.into()).collect();
            assert_eq!(vec, Integers::from_values([0, 1, 2]));
        }
    }

    #[test]
    fn iter_mut() {
        test! {
            let mut vec = Integers::from_values(0..3);
            vec.iter_mut().for_each(|v| *v += 1);
            assert_eq!(vec, Integers::from_values(1..4));
        }
    }

    #[test]
    fn iter() {
        test! {
            let vec = Integers::from_values(0..3);
            assert_eq!(vec.iter().sum::<Rint>(), 3);
        }
    }

    #[test]
    fn from_values_short() {
        test! {
            // Short (<64k) vectors are allocated.
            let vec = Integers::from_values((0..3).map(|i| 2-i));
            assert_eq!(vec.is_altrep(), false);
            assert_eq!(r!(vec.clone()), r!([2, 1, 0]));
            assert_eq!(vec.elt(1), 1);
            let mut dest = [0.into(); 2];
            vec.get_region(1, &mut dest);
            assert_eq!(dest, [1, 0]);
        }
    }

    #[test]
    fn from_values_altrep() {
        test! {
            let vec = Integers::from_values_altrep(0..1000000000);
            assert_eq!(vec.is_altrep(), true);
            assert_eq!(vec.elt(12345678), 12345678);
            let mut dest = [0.into(); 2];
            vec.get_region(12345678, &mut dest);
            assert_eq!(dest, [12345678, 12345679]);
        }
    }

    #[test]
    fn new() {
        test! {
            let vec = Integers::new(10);
            assert_eq!(vec.is_integer(), true);
            assert_eq!(vec.len(), 10);
        }
    }

    #[test]
    fn test_vec_i32_integers_conversion() {
        test! {
            let int_vec = vec![3,4,0,-2];
            let int_vec_robj: Robj = int_vec.clone().try_into().unwrap();
            // unsafe { libR_sys::Rf_PrintValue(rint_vec_robj.get())}
            assert_eq!(int_vec_robj.as_integer_slice().unwrap(), &int_vec);
        }
    }
}
