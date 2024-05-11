//!
//!
//!
use super::scalar::{Rint, Scalar};
use super::*;

#[derive(PartialEq, Clone)]
pub struct Factors {
    pub(crate) robj: Robj,
}

use libR_sys::SEXPTYPE::INTSXP;
crate::wrapper::macros::gen_vector_wrapper_impl!(
    vector_type: Factors, // Implements for
    scalar_type: Rint,     // Element type
    primitive_type: i32,   // Raw element type
    r_prefix: INTEGER,     // `R` functions prefix
    SEXP: INTSXP,          // `SEXP`
    doc_name: integer,     // Singular type name used in docs
    altrep_constructor: make_altinteger_from_iterator,
);

impl Factors {
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
impl Factors {
    pub fn set_elt(&mut self, index: usize, val: Rint) {
        single_threaded(|| unsafe {
            SET_INTEGER_ELT(self.get(), index as R_xlen_t, val.inner());
        })
    }
}

impl Deref for Factors {
    type Target = [Rint];

    /// Treat Integers as if it is a slice, like `Vec<Rint>`
    fn deref(&self) -> &Self::Target {
        unsafe {
            let ptr = DATAPTR_RO(self.get()).cast::<Rint>();
            std::slice::from_raw_parts(ptr, self.len())
        }
    }
}

impl DerefMut for Factors {
    /// Treat Integers as if it is a mutable slice, like `Vec<Rint>`
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            let ptr = DATAPTR(self.get_mut()).cast::<Rint>();
            std::slice::from_raw_parts_mut(ptr, self.len())
        }
    }
}

impl std::fmt::Debug for Factors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.len() == 1 {
            write!(f, "{:?}", self.elt(0))
        } else {
            //TODO: add the levels to the debug printing
            f.debug_list().entries(self.iter()).finish()
        }
    }
}

impl TryFrom<Vec<i32>> for Factors {
    type Error = Error;

    fn try_from(value: Vec<i32>) -> std::result::Result<Self, Self::Error> {
        //FIXME: what are the levels of the factor here?
        Ok(Self {
            robj: <Robj>::try_from(value)?,
        })
    }
}

impl AsStrIter for Factors {
    fn as_str_iter(&self) -> Option<StrIter> {
        self.robj.as_str_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_to_integers() {
        extendr_engine::with_r(|| {
            let a_factor = R!("iris$Species").unwrap();

            rprintln!("`a_factor` has an Rtype of {:?}", a_factor.rtype());
            rprintln!("{:?}", Integers::try_from(&a_factor));
            rprintln!("{:?}", Factors::try_from(&a_factor));

            assert!(
                Integers::try_from(&a_factor).is_err(),
                "factors aren't strictly integers"
            );
            assert!(Factors::try_from(&a_factor).is_ok());
        });
    }
    #[test]
    fn test_factor_str_iter() {
        extendr_engine::with_r(|| {
            let a_factor = R!("iris$Species").unwrap();

            let a_factor_str = a_factor.as_str_iter();
            assert!(a_factor_str.is_some());

            let a_factor_str = a_factor_str.unwrap();
            let a_factor_str = a_factor_str.take(5).collect::<Vec<_>>();
            rprintln!("{a_factor_str:?}");
        });
    }
}
