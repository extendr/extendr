use super::scalar::{Rcplx, C64};
use super::*;
use std::iter::FromIterator;

/// An obscure `NA`-aware wrapper for R's complex vectors.
/// Can be used to iterate over vectors obtained from R
/// or to create new vectors that can be returned back to R.
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let mut vec = (0..5).map(|i| (i as f64).into()).collect::<Complexes>();
///     vec.iter_mut().for_each(|v| *v = *v + Rcplx::from(10.0));
///     assert_eq!(vec.elt(0), Rcplx::from(10.0));
///     let sum = vec.iter().sum::<Rcplx>();
///     assert_eq!(sum, Rcplx::from(60.0));
/// }
/// ```  
#[derive(Debug, PartialEq, Clone)]
pub struct Complexes {
    pub(crate) robj: Robj,
}

crate::wrapper::macros::gen_vector_wrapper_impl!(
    vector_type: Complexes,
    scalar_type: Rcplx,
    primitive_type: C64,
    r_prefix: COMPLEX,
    SEXP: CPLXSXP,
    doc_name: complex,
    altrep_constructor: make_altcomplex_from_iterator,
);

impl Complexes {
    /// Get a region of elements from the vector.
    pub fn get_region(&self, index: usize, dest: &mut [Rcplx]) -> usize {
        unsafe {
            let ptr : *mut Rcomplex = dest.as_mut_ptr() as *mut Rcomplex;
            COMPLEX_GET_REGION(self.get(), index as R_xlen_t, dest.len() as R_xlen_t, ptr) as usize
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn from_iterator() {
        test! {
            let vec : Complexes = (0..3).map(|i| (i as f64).into()).collect();
            assert_eq!(vec, Complexes::from_values([0.0, 1.0, 2.0]));
        }
    }
    #[test]
    fn iter_mut() {
        test! {
            let mut vec = Complexes::from_values([0.0, 1.0, 2.0, 3.0]);
            vec.iter_mut().for_each(|v| *v = *v + Rcplx::from(1.0));
            assert_eq!(vec, Complexes::from_values([1.0, 2.0, 3.0, 4.0]));
        }
    }

    #[test]
    fn iter() {
        test! {
            let vec = Complexes::from_values([0.0, 1.0, 2.0, 3.0]);
            assert_eq!(vec.iter().sum::<Rcplx>(), Rcplx::from(6.0));
        }
    }

    #[test]
    fn from_values_short() {
        test! {
            // Short (<64k) vectors are allocated.
            let vec = Complexes::from_values((0..3).map(|i| 2.0 - i as f64));
            assert_eq!(vec.is_altrep(), false);
            assert_eq!(r!(vec.clone()), r!([Rcplx::from(2.0), Rcplx::from(1.0), Rcplx::from(0.0)]));
            assert_eq!(vec.elt(1), Rcplx::from(1.0));
            let mut dest = [0.0.into(); 2];
            vec.get_region(1, &mut dest);
            assert_eq!(dest, [Rcplx::from(1.0), Rcplx::from(0.0)]);
        }
    }
    #[test]
    fn from_values_long() {
        test! {
            // Long (>=64k) vectors are lazy ALTREP objects.
            let vec = Complexes::from_values((0..1000000000).map(|x| x as f64));
            assert_eq!(vec.is_altrep(), true);
            assert_eq!(vec.elt(12345678), Rcplx::from(12345678.0));
            let mut dest = [0.0.into(); 2];
            vec.get_region(12345678, &mut dest);
            assert_eq!(dest, [Rcplx::from(12345678.0), Rcplx::from(12345679.0)]);
        }
    }

    #[test]
    fn new() {
        test! {
            let vec = Complexes::new(10);
            assert_eq!(vec.is_complex(), true);
            assert_eq!(vec.len(), 10);
        }
    }
}

// There is no SET_COMPLEX_ELT
//
// impl Complexes {
//     pub fn set_elt(&mut self, index: usize, val: Rcplx) {
//         unsafe {
//             SET_COMPLEX_ELT(self.get(), index as R_xlen_t, val.inner());
//         }
//     }
// }

impl Deref for Complexes {
    type Target = [Rcplx];

    /// Treat Complexes as if it is a slice, like Vec<Rcplx>
    fn deref(&self) -> &Self::Target {
        unsafe {
            let ptr = DATAPTR_RO(self.get()) as *const Rcplx;
            std::slice::from_raw_parts(ptr, self.len())
        }
    }
}

impl DerefMut for Complexes {
    /// Treat Complexes as if it is a mutable slice, like Vec<Rcplx>
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            let ptr = DATAPTR(self.get()) as *mut Rcplx;
            std::slice::from_raw_parts_mut(ptr, self.len())
        }
    }
}

