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

use SEXPTYPE::LGLSXP;
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

// TODO: this should be a trait.
impl Logicals {
    pub fn set_elt(&mut self, index: usize, val: Rbool) {
        single_threaded(|| unsafe {
            SET_INTEGER_ELT(self.get_mut(), index as R_xlen_t, val.inner());
        })
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
            let ptr = DATAPTR(self.get_mut()) as *mut Rbool;
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

impl TryFrom<Vec<bool>> for Logicals {
    type Error = Error;

    fn try_from(value: Vec<bool>) -> std::result::Result<Self, Self::Error> {
        Ok(Self { robj: value.into() })
    }
}

#[cfg(test)]
mod tests {
    use crate as extendr_api;
    use crate::r;
    use crate::scalar::Rbool;
    use crate::Rinternals;
    use extendr_api::test;
    use extendr_api::Logicals;

    #[test]
    fn from_iterator() {
        test! {
            let vec : Logicals = (0..3).map(|i| (i % 2 == 0).into()).collect();
            assert_eq!(vec, Logicals::from_values([true, false, true]));
        }
    }

    #[test]
    fn iter_mut() {
        test! {
            let mut vec = Logicals::from_values([true, false, true]);
            vec.iter_mut().for_each(|v| *v = true.into());
            assert_eq!(vec, Logicals::from_values([true, true, true]));
        }
    }

    // #[test]
    // fn iter() {
    //     test! {
    //         let mut vec = Logicals::from_values([true, false, true]);
    //         assert_eq!(vec.iter().sum::<Rint>(), 3);
    //     }
    // }

    #[test]
    fn from_values_short() {
        test! {
            // Short (<64k) vectors are allocated.
            let vec = Logicals::from_values([true, false, true]);
            assert_eq!(vec.is_altrep(), false);
            assert_eq!(r!(vec.clone()), r!([true, false, true]));
            assert_eq!(vec.elt(1), false);
            let mut dest = [false.into(); 2];
            vec.get_region(1, &mut dest);
            assert_eq!(dest, [false, true]);
        }
    }

    #[test]
    fn from_values_altrep() {
        test! {
            let vec = Logicals::from_values_altrep((0..1000000000).map(|_| Rbool::from(true)));
            assert_eq!(vec.is_altrep(), true);
            assert_eq!(vec.elt(12345678), true);
            let mut dest = [false.into(); 2];
            vec.get_region(12345678, &mut dest);
            assert_eq!(dest, [true, true]);
        }
    }

    #[test]
    fn new() {
        test! {
            let vec = Logicals::new(10);
            assert_eq!(vec.is_logical(), true);
            assert_eq!(vec.len(), 10);
        }
    }

    #[test]
    fn test_vec_bool_logicals_conversion() {
        test! {
            let test = vec![false, true, true, false];
            let test_rbool: Vec<Rbool> = test.clone().into_iter().map(|x|x.into()).collect();
            let test_logicals: Logicals = test.try_into().unwrap();
            assert_eq!(test_logicals.robj.as_logical_slice().unwrap(), &test_rbool);
        }
    }
}
