use super::scalar::Rfloat;
use super::*;
use std::iter::FromIterator;

/// An obscure `NA`-aware wrapper for R's double vectors.
/// Can be used to iterate over vectors obtained from R
/// or to create new vectors that can be returned back to R.
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let mut vec = (0..5).map(|i| (i as f64).into()).collect::<Doubles>();
///     vec.iter_mut().for_each(|v| *v = *v + 10f64);
///     assert_eq!(vec.elt(0), 10f64);
///     let sum = vec.iter().sum::<Rfloat>();
///     assert_eq!(sum, 60f64);
/// }
/// ```  
#[derive(Debug, PartialEq, Clone)]
pub struct Doubles {
    pub(crate) robj: Robj,
}

crate::wrapper::macros::gen_vector_wrapper_impl!(
    Doubles, // Implements for
    Rfloat,  // Element type
    f64,     // Raw element type
    REAL,    // `R` functions prefix
    REALSXP, // `SEXP`
    double   // Singular type name used in docs
);

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn from_iterator() {
        test! {
            let vec : Doubles = (0..3).map(|i| (i as f64).into()).collect();
            assert_eq!(vec, Doubles::from_values([0f64, 1f64, 2f64]));
        }
    }
    #[test]
    fn iter_mut() {
        test! {
            let mut vec = Doubles::from_values([0f64, 1f64, 2f64, 3f64]);
            vec.iter_mut().for_each(|v| *v = *v + 1f64);
            assert_eq!(vec, Doubles::from_values([1f64, 2f64, 3f64, 4f64]));
        }
    }

    #[test]
    fn iter() {
        test! {
            let vec = Doubles::from_values(0..3);
            assert_eq!(vec.iter().sum::<Rfloat>(), 3f64);
        }
    }

    #[test]
    fn from_values_short() {
        test! {
            // Short (<64k) vectors are allocated.
            let vec = Doubles::from_values((0..3).map(|i| 2f64 - i as f64));
            assert_eq!(vec.is_altrep(), false);
            assert_eq!(r!(vec.clone()), r!([2f64, 1f64, 0f64]));
            assert_eq!(vec.elt(1), 1f64);
            let mut dest = [0f64; 2];
            vec.get_region(1, &mut dest);
            assert_eq!(dest, [1f64, 0f64]);
        }
    }
    #[test]
    fn from_values_long() {
        test! {
            // Long (>=64k) vectors are lazy ALTREP objects.
            let vec = Doubles::from_values((0..1000000000).map(|x| x as f64));
            assert_eq!(vec.is_altrep(), true);
            assert_eq!(vec.elt(12345678), 12345678f64);
            let mut dest = [0f64; 2];
            vec.get_region(12345678, &mut dest);
            assert_eq!(dest, [12345678f64, 12345679f64]);
        }
    }

    #[test]
    fn new() {
        test! {
            let vec = Doubles::new(10);
            assert_eq!(vec.is_real(), true);
            assert_eq!(vec.len(), 10);
        }
    }
}
