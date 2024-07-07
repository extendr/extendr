use super::scalar::{Rfloat, Scalar};
use super::*;
use std::iter::FromIterator;

/// An obscure `NA`-aware wrapper for R's double vectors.
/// Can be used to iterate over vectors obtained from R
/// or to create new vectors that can be returned back to R.
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let mut vec = (0..5).map(|i| (i as f64).into()).collect::<Doubles>();
///     vec.iter_mut().for_each(|v| *v = *v + 10.0);
///     assert_eq!(vec.elt(0), 10.0);
///     let sum = vec.iter().sum::<Rfloat>();
///     assert_eq!(sum, 60.0);
/// }
/// ```  
#[derive(PartialEq, Clone)]
pub struct Doubles {
    pub(crate) robj: Robj,
}

use libR_sys::SEXPTYPE::REALSXP;
crate::wrapper::macros::gen_vector_wrapper_impl!(
    vector_type: Doubles,
    scalar_type: Rfloat,
    primitive_type: f64,
    r_prefix: REAL,
    SEXP: REALSXP,
    doc_name: double,
    altrep_constructor: make_altreal_from_iterator,
);

impl Doubles {
    /// Get a region of elements from the vector.
    pub fn get_region(&self, index: usize, dest: &mut [Rfloat]) -> usize {
        unsafe {
            let ptr: *mut f64 = dest.as_mut_ptr() as *mut f64;
            REAL_GET_REGION(self.get(), index as R_xlen_t, dest.len() as R_xlen_t, ptr) as usize
        }
    }

    /// Return `TRUE` if the vector is sorted, `FALSE` if not, or `NA_BOOL` if unknown.
    pub fn is_sorted(&self) -> Rbool {
        unsafe { REAL_IS_SORTED(self.get()).into() }
    }

    /// Return `TRUE` if the vector has no `NA`s, `FALSE` if any, or `NA_BOOL` if unknown.
    pub fn no_na(&self) -> Rbool {
        unsafe { REAL_NO_NA(self.get()).into() }
    }
}

// TODO: this should be a trait.
impl Doubles {
    pub fn set_elt(&mut self, index: usize, val: Rfloat) {
        single_threaded(|| unsafe {
            SET_REAL_ELT(self.get_mut(), index as R_xlen_t, val.inner());
        })
    }
}

impl Deref for Doubles {
    type Target = [Rfloat];

    /// Treat Doubles as if it is a slice, like `Vec<Rfloat>`
    fn deref(&self) -> &Self::Target {
        unsafe {
            let ptr = DATAPTR_RO(self.get()) as *const Rfloat;
            std::slice::from_raw_parts(ptr, self.len())
        }
    }
}

impl DerefMut for Doubles {
    /// Treat Doubles as if it is a mutable slice, like `Vec<Rfloat>`
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            let ptr = DATAPTR(self.get_mut()) as *mut Rfloat;
            std::slice::from_raw_parts_mut(ptr, self.len())
        }
    }
}

impl std::fmt::Debug for Doubles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.len() == 1 {
            write!(f, "{:?}", self.elt(0))
        } else {
            f.debug_list().entries(self.iter()).finish()
        }
    }
}

impl TryFrom<Vec<f64>> for Doubles {
    type Error = Error;

    fn try_from(value: Vec<f64>) -> std::result::Result<Self, Self::Error> {
        Ok(Self { robj: value.into() })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate as extendr_api;

    #[test]
    fn test_vec_f64_doubles_conversion() {
        test! {
            let test_vec = vec![0., 1., std::f64::consts::PI, -1.];
            let test_doubles: Doubles = test_vec.clone().try_into().unwrap();
            let test_doubles_slice = test_doubles.robj.as_real_slice().unwrap();
            assert_eq!(test_doubles_slice, test_vec);
        }
    }
}
