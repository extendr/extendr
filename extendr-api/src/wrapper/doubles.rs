use super::scalar::Rfloat;
use super::*;
use std::iter::FromIterator;

#[derive(Debug, PartialEq, Clone)]
pub struct Doubles {
    pub(crate) robj: Robj,
}

impl Default for Doubles {
    fn default() -> Self {
        Doubles::new(0)
    }
}

// Under this size, vectors are manifest.
// Above this size, vectors are lazy ALTREP objects.
const SHORT_VECTOR_LENGTH: usize = 64 * 1024;

impl Doubles {
    /// Create a new vector of doubles.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let vec = Doubles::new(10);
    ///     assert_eq!(vec.is_real(), true);
    ///     assert_eq!(vec.len(), 10);
    /// }
    /// ```
    pub fn new(len: usize) -> Doubles {
        let iter = (0..len).map(|_| 0f64);
        Doubles::from_values(iter)
    }

    /// Wrapper for creating ALTREP double (REALSXP) vectors from iterators.
    /// The iterator must be exact, clonable and implement Debug.
    ///
    /// If you want a more generalised constructor, use `iter.collect::<Doubles>()`.
    ///
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     // Short (<64k) vectors are allocated.
    ///     let vec = Doubles::from_values((0..3).map(|i| 2f64 - i as f64));
    ///     assert_eq!(vec.is_altrep(), false);
    ///     assert_eq!(r!(vec.clone()), r!([2f64, 1f64, 0f64]));
    ///     assert_eq!(vec.elt(1), 1f64);
    ///     let mut dest = [0f64; 2];
    ///     vec.get_region(1, &mut dest);
    ///     assert_eq!(dest, [1f64, 0f64]);
    ///
    ///     // Long (>=64k) vectors are lazy ALTREP objects.
    ///     let vec = Doubles::from_values((0..1000000000).map(|x| x as f64));
    ///     assert_eq!(vec.is_altrep(), true);
    ///     assert_eq!(vec.elt(12345678), 12345678f64);
    ///     let mut dest = [0f64; 2];
    ///     vec.get_region(12345678, &mut dest);
    ///     assert_eq!(dest, [12345678f64, 12345679f64]);
    /// }
    /// ```
    pub fn from_values<V>(values: V) -> Self
    where
        V: IntoIterator,
        V::IntoIter: ExactSizeIterator + std::fmt::Debug + Clone + 'static + std::any::Any,
        V::Item: Into<f64>,
    {
        single_threaded(|| {
            let values: V::IntoIter = values.into_iter();

            let robj = if values.len() >= SHORT_VECTOR_LENGTH {
                Altrep::make_altreal_from_iterator(values)
                    .try_into()
                    .unwrap()
            } else {
                let mut robj = Robj::alloc_vector(REALSXP, values.len());
                let dest: &mut [f64] = robj.as_typed_slice_mut().unwrap();

                for (d, v) in dest.iter_mut().zip(values) {
                    *d = v.into();
                }
                robj
            };
            Self { robj }
        })
    }

    /// Get a single element from the vector.
    /// Note that this is very inefficient in a tight loop.
    pub fn elt(&self, index: usize) -> Rfloat {
        unsafe { REAL_ELT(self.get(), index as R_xlen_t).into() }
    }

    /// Get a region of elements from the vector.
    pub fn get_region(&self, index: usize, dest: &mut [f64]) {
        unsafe {
            let ptr = dest.as_mut_ptr();
            REAL_GET_REGION(self.get(), index as R_xlen_t, dest.len() as R_xlen_t, ptr);
        }
    }

    /// Return TRUE if the vector is sorted, FALSE if not, or NA_BOOL if unknown.
    pub fn is_sorted(&self) -> Bool {
        unsafe { REAL_IS_SORTED(self.get()).into() }
    }

    /// Return TRUE if the vector has NAs, FALSE if not, or NA_BOOL if unknown.
    pub fn no_na(&self) -> Bool {
        unsafe { REAL_NO_NA(self.get()).into() }
    }

    /// Return an iterator for a double object.
    /// Forces ALTREP objects to manifest.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let vec = Doubles::from_values(0..3);
    ///     assert_eq!(vec.iter().sum::<Rfloat>(), 3f64);
    /// }
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = Rfloat> {
        self.as_typed_slice().unwrap().iter().cloned()
    }

    /// Return a writable iterator for a double object.
    /// Forces ALTREP objects to manifest.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let mut vec = Doubles::from_values([0f64, 1f64, 2f64, 3f64]);
    ///     vec.iter_mut().for_each(|v| *v = *v + 1f64);
    ///     assert_eq!(vec, Doubles::from_values([1f64, 2f64, 3f64, 4f64]));
    /// }
    /// ```
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Rfloat> {
        self.as_typed_slice_mut().unwrap().iter_mut()
    }
}

impl FromIterator<Rfloat> for Doubles {
    /// A more generalised iterator collector for small vectors.
    /// Generates a non-ALTREP vector.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let vec : Doubles = (0..3).map(|i| (i as f64).into()).collect();
    ///     assert_eq!(vec, Doubles::from_values([0f64, 1f64, 2f64]));
    /// }
    /// ```
    fn from_iter<T: IntoIterator<Item = Rfloat>>(iter: T) -> Self {
        // Collect into a vector first.
        // TODO: specialise for ExactSizeIterator.
        let values: Vec<Rfloat> = iter.into_iter().collect();

        let mut robj = Robj::alloc_vector(REALSXP, values.len());
        let dest: &mut [Rfloat] = robj.as_typed_slice_mut().unwrap();

        for (d, v) in dest.iter_mut().zip(values) {
            *d = v;
        }

        Doubles { robj }
    }
}
