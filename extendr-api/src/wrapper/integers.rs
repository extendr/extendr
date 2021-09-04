use super::scalar::Rint;
use super::*;
use std::iter::FromIterator;

#[derive(Debug, PartialEq, Clone)]
pub struct Integers {
    pub(crate) robj: Robj,
}

impl Default for Integers {
    fn default() -> Self {
        Integers::new(0)
    }
}

// Under this size, vectors are manifest.
// Above this size, vectors are lazy ALTREP objects.
const SHORT_VECTOR_LENGTH: usize = 64 * 1024;

impl Integers {
    /// Create a new vector of integers.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let vec = Integers::new(10);
    ///     assert_eq!(vec.is_integer(), true);
    ///     assert_eq!(vec.len(), 10);
    /// }
    /// ```
    pub fn new(len: usize) -> Integers {
        let iter = (0..len).map(|_| 0);
        Integers::from_values(iter)
    }

    /// Wrapper for creating ALTREP integer (INTSXP) vectors from iterators.
    /// The iterator must be exact, clonable and implement Debug.
    ///
    /// If you want a more generalised constructor, use `iter.collect::<Integers>()`.
    ///
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     // Short (<64k) vectors are allocated.
    ///     let vec = Integers::from_values((0..3).map(|i| 2-i));
    ///     assert_eq!(vec.is_altrep(), false);
    ///     assert_eq!(r!(vec.clone()), r!([2, 1, 0]));
    ///     assert_eq!(vec.elt(1), 1);
    ///     let mut dest = [0; 2];
    ///     vec.get_region(1, &mut dest);
    ///     assert_eq!(dest, [1, 0]);
    ///
    ///     // Long (>=64k) vectors are lazy ALTREP objects.
    ///     let vec = Integers::from_values(0..1000000000);
    ///     assert_eq!(vec.is_altrep(), true);
    ///     assert_eq!(vec.elt(12345678), 12345678);
    ///     let mut dest = [0; 2];
    ///     vec.get_region(12345678, &mut dest);
    ///     assert_eq!(dest, [12345678, 12345679]);
    /// }
    /// ```
    pub fn from_values<V>(values: V) -> Self
    where
        V: IntoIterator,
        V::IntoIter: ExactSizeIterator + std::fmt::Debug + Clone + 'static + std::any::Any,
        V::Item: Into<i32>,
    {
        single_threaded(|| {
            let values: V::IntoIter = values.into_iter();

            let robj = if values.len() >= SHORT_VECTOR_LENGTH {
                Altrep::make_altinteger_from_iterator(values)
                    .try_into()
                    .unwrap()
            } else {
                let mut robj = Robj::alloc_vector(INTSXP, values.len());
                let dest: &mut [i32] = robj.as_typed_slice_mut().unwrap();

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
    pub fn elt(&self, index: usize) -> Rint {
        unsafe { INTEGER_ELT(self.get(), index as R_xlen_t).into() }
    }

    /// Get a region of elements from the vector.
    pub fn get_region(&self, index: usize, dest: &mut [i32]) {
        unsafe {
            let ptr = dest.as_mut_ptr();
            INTEGER_GET_REGION(self.get(), index as R_xlen_t, dest.len() as R_xlen_t, ptr);
        }
    }

    /// Return TRUE if the vector is sorted, FALSE if not, or NA_BOOL if unknown.
    pub fn is_sorted(&self) -> Bool {
        unsafe { INTEGER_IS_SORTED(self.get()).into() }
    }

    /// Return TRUE if the vector has NAs, FALSE if not, or NA_BOOL if unknown.
    pub fn no_na(&self) -> Bool {
        unsafe { INTEGER_NO_NA(self.get()).into() }
    }

    /// Return an iterator for for an integer object.
    /// Forces ALTREP objects to manifest.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let vec = Integers::from_values(0..3);
    ///     assert_eq!(vec.iter().sum::<Rint>(), 3);
    /// }
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = Rint> {
        self.as_typed_slice().unwrap().iter().cloned()
    }

    /// Return a writable iterator for an integer object.
    /// Forces ALTREP objects to manifest.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let mut vec = Integers::from_values(0..3);
    ///     vec.iter_mut().for_each(|v| *v = *v + 1);
    ///     assert_eq!(vec, Integers::from_values(1..4));
    /// }
    /// ```
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Rint> {
        self.as_typed_slice_mut().unwrap().iter_mut()
    }
}

impl FromIterator<i32> for Integers {
    /// A more generalised iterator collector for small vectors.
    /// Generates a non-ALTREP vector.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let vec : Integers = (0..3).collect();
    ///     assert_eq!(vec, Integers::from_values([0, 1, 2]));
    /// }
    /// ```
    fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Self {
        // Collect into a vector first.
        // TODO: specialise for ExactSizeIterator.
        let values: Vec<i32> = iter.into_iter().collect();

        let mut robj = Robj::alloc_vector(INTSXP, values.len());
        let dest: &mut [i32] = robj.as_typed_slice_mut().unwrap();

        for (d, v) in dest.iter_mut().zip(values) {
            *d = v;
        }

        Integers { robj }
    }
}
