use super::scalar::{c64, Rcplx};
use super::*;
use std::iter::FromIterator;

/// An obscure `NA`-aware wrapper for R's complex vectors.
/// Can be used to iterate over vectors obtained from R
/// or to create new vectors that can be returned back to R.
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let mut vec = (0..5).map(|i| (i as f64).into()).collect::<Complexes>();
///     assert_eq!(vec.len(), 5);
/// }
/// ```  
#[derive(PartialEq, Clone)]
pub struct Complexes {
    pub(crate) robj: Robj,
}

use libR_sys::SEXPTYPE::CPLXSXP;
crate::wrapper::macros::gen_vector_wrapper_impl!(
    vector_type: Complexes,
    scalar_type: Rcplx,
    primitive_type: c64,
    r_prefix: COMPLEX,
    SEXP: CPLXSXP,
    doc_name: complex,
    altrep_constructor: make_altcomplex_from_iterator,
);

impl Complexes {
    /// Get a region of elements from the vector.
    pub fn get_region(&self, index: usize, dest: &mut [Rcplx]) -> usize {
        unsafe {
            let ptr: *mut Rcomplex = dest.as_mut_ptr() as *mut Rcomplex;
            COMPLEX_GET_REGION(self.get(), index as R_xlen_t, dest.len() as R_xlen_t, ptr) as usize
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

    /// Treat Complexes as if it is a slice, like `Vec<Rcplx>`
    fn deref(&self) -> &Self::Target {
        unsafe {
            let ptr = DATAPTR_RO(self.get()) as *const Rcplx;
            std::slice::from_raw_parts(ptr, self.len())
        }
    }
}

impl DerefMut for Complexes {
    /// Treat Complexes as if it is a mutable slice, like `Vec<Rcplx>`
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            let ptr = DATAPTR(self.get_mut()) as *mut Rcplx;
            std::slice::from_raw_parts_mut(ptr, self.len())
        }
    }
}

impl std::fmt::Debug for Complexes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.len() == 1 {
            write!(f, "{:?}", self.elt(0))
        } else {
            f.debug_list().entries(self.iter()).finish()
        }
    }
}

impl TryFrom<Vec<c64>> for Complexes {
    type Error = Error;

    fn try_from(value: Vec<c64>) -> std::result::Result<Self, Self::Error> {
        Ok(Self { robj: value.into() })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate as extendr_api;

    #[test]
    fn test_try_from_vec_c64_conversion() {
        test! {
            let vec = vec![c64::new(0., 0.), c64::new(1., 1.), c64::new(0., 1.)];
            let vec_rob: Complexes = vec.clone().try_into().unwrap();
            let vec_rob_slice: &[c64] = vec_rob.robj.as_typed_slice().unwrap();
            assert_eq!(vec_rob_slice, vec.as_slice());
        }
    }
}
