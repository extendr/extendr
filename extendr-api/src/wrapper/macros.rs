/// Generates `impl` block and required traits for a vector type.
macro_rules! gen_vector_wrapper_impl {
    (
        vector_type: $type : ident,
        scalar_type: $scalar_type : ty,
        primitive_type: $primitive_type : ty,
        r_prefix: $r_prefix : ident,
        SEXP: $sexp : ident,
        doc_name: $doc_name : ident,
        altrep_constructor: $altrep_constructor : ident,
    ) => {

        // Under this size, vectors are manifest.
        // Above this size, vectors are lazy ALTREP objects.
        const SHORT_VECTOR_LENGTH: usize = 64 * 1024;

        impl Default for $type {
            fn default() -> Self {
                $type::new(0)
            }
        }

        impl $type {
            paste::paste!{
                #[doc = "Create a new vector of " $type:lower "."]
                #[doc = "```"]
                #[doc = "use extendr_api::prelude::*;"]
                #[doc = "test! {"]
                #[doc = "   let vec = " $type "::new(10);"]
                #[doc = "   assert_eq!(vec.is_" $r_prefix:lower "(), true);"]
                #[doc = "   assert_eq!(vec.len(), 10);"]
                #[doc = "}"]
                #[doc = "```"]
                pub fn new(len: usize) -> $type {
                    // TODO: Check if impacts performance.
                    let iter = (0..len).map(|_| <$primitive_type>::default());
                    <$type>::from_values(iter)
                }
            }
            paste::paste!{
                #[doc = "Wrapper for creating ALTREP " $doc_name " (" $sexp ") vectors from iterators."]
                #[doc = "The iterator must be exact, cloneable and implement Debug."]
                #[doc = "If you want a more generalised constructor, use `iter.collect::<" $type ">()`."]
                pub fn from_values<V>(values: V) -> Self
                where
                    V: IntoIterator,
                    V::IntoIter: ExactSizeIterator + std::fmt::Debug + Clone + 'static + std::any::Any,
                    V::Item: Into<$scalar_type>,
                {
                    single_threaded(|| {
                        let values: V::IntoIter = values.into_iter();

                        let robj = if values.len() >= SHORT_VECTOR_LENGTH {
                            Altrep::$altrep_constructor(values)
                                .try_into()
                                .unwrap()
                        } else {
                            let mut robj = Robj::alloc_vector($sexp, values.len());
                            let dest: &mut [$scalar_type] = robj.as_typed_slice_mut().unwrap();

                            for (d, v) in dest.iter_mut().zip(values) {
                                *d = v.into();
                            }
                            robj
                        };
                        Self { robj }
                    })
                }
            }

            paste::paste! {
                #[doc = "Get a single element from the vector."]
                #[doc = "Note that this is very inefficient in a tight loop."]
                #[doc = "```"]
                #[doc = "use extendr_api::prelude::*;"]
                #[doc = "test! {"]
                #[doc = "   let vec = " $type "::new(10);"]
                #[doc = "   assert_eq!(vec.elt(0), <"$scalar_type">::default());"]
                #[doc = "   assert_eq!(vec.elt(9), <"$scalar_type">::default());"]
                #[doc = "   assert!(vec.elt(10).is_na());"]
                #[doc = "}"]
                #[doc = "```"]
                pub fn elt(&self, index: usize) -> $scalar_type {
                    // Defensive check for oob
                    // This check will not be needed in later releases of R
                    if(index >= self.len()) {
                        <$scalar_type>::na()
                    } else {
                        unsafe { [<$r_prefix _ELT>](self.get(), index as R_xlen_t).into() }
                    }
                }
            }

            /// Get a region of elements from the vector.
            pub fn get_region(&self, index: usize, dest: &mut [$scalar_type]) -> usize {
                unsafe {
                    let ptr : *mut $primitive_type = dest.as_mut_ptr() as *mut $primitive_type;
                    paste::paste!{ [<$r_prefix _GET_REGION>](self.get(), index as R_xlen_t, dest.len() as R_xlen_t, ptr) as usize }
                }
            }

            /// Return `TRUE` if the vector is sorted, `FALSE` if not, or `NA_BOOL` if unknown.
            pub fn is_sorted(&self) -> Bool {
                unsafe { paste::paste!{ [<$r_prefix _IS_SORTED>](self.get()).into() } }
            }

            /// Return `TRUE` if the vector has no `NA`s, `FALSE` if any, or `NA_BOOL` if unknown.
            pub fn no_na(&self) -> Bool {
                unsafe { paste::paste!{ [<$r_prefix _NO_NA>](self.get()).into() } }
            }

            paste::paste!{
                #[doc = "Return an iterator for a " $doc_name " object."]
                #[doc = "Forces ALTREP objects to manifest."]
                pub fn iter(&self) -> impl Iterator<Item = $scalar_type> {
                    self.as_robj().as_typed_slice().unwrap().iter().cloned()
                }
            }

            paste::paste!{
                #[doc = "Return a writable iterator for a " $doc_name " object."]
                #[doc = "Forces ALTREP objects to manifest."]
                pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut $scalar_type> {
                    self.as_robj_mut().as_typed_slice_mut().unwrap().iter_mut()
                }
            }
        }

        impl FromIterator<$scalar_type> for $type {
            /// A more generalised iterator collector for small vectors.
            /// Generates a non-ALTREP vector.
            fn from_iter<T: IntoIterator<Item = $scalar_type>>(iter: T) -> Self {
                // Collect into a vector first.
                // TODO: specialise for ExactSizeIterator.
                let values: Vec<$scalar_type> = iter.into_iter().collect();

                let mut robj = Robj::alloc_vector($sexp, values.len());
                let dest: &mut [$scalar_type] = robj.as_typed_slice_mut().unwrap();

                for (d, v) in dest.iter_mut().zip(values) {
                    *d = v;
                }

                $type { robj }
            }
        }
    }
}

pub(in crate::wrapper) use gen_vector_wrapper_impl;
