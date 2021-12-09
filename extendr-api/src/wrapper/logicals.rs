use super::scalar::Rbool;
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
#[derive(Debug, PartialEq, Clone)]
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

#[cfg(test)]
mod tests {
    use crate::prelude::*;

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
    fn from_values_long() {
        test! {
            // Long (>=64k) vectors a lazy ALTREP objects.
            let vec = Logicals::from_values((0..1000000000).map(|_| Rbool::from(true)));
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
}

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

    /// Treat Logicals as if it is a slice, like Vec<Rint>
    fn deref(&self) -> &Self::Target {
        unsafe {
            let ptr = DATAPTR_RO(self.get()) as *const Rbool;
            std::slice::from_raw_parts(ptr, self.len())
        }
    }
}

impl DerefMut for Logicals {
    /// Treat Logicals as if it is a mutable slice, like Vec<Rint>
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            let ptr = DATAPTR(self.get()) as *mut Rbool;
            std::slice::from_raw_parts_mut(ptr, self.len())
        }
    }
}
