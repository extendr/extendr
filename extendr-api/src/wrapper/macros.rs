/// Generates `impl` block and required traits for a vector type.
macro_rules! gen_vector_wrapper_impl {
    (
        /// Vector type for which traits are implemented.
        $type : ident,
        /// `NA`-aware element type.
        $type_elem : ty,
        /// Underlying primitive type.
        $type_prim : ty,
        /// `R` type name.
        $r_type : ident,
        /// `R` `SEXP`.
        $sexp : ident,
        /// Singular name of the type, used in doc strings.
        $doc_name : ident
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
                #[doc = "   assert_eq!(vec.is_" $r_type:lower "(), true);"]
                #[doc = "   assert_eq!(vec.len(), 10);"]
                #[doc = "}"]
                #[doc = "```"]
                pub fn new(len: usize) -> $type {
                    // TODO: Check if impacts performance.
                    let iter = (0..len).map(|_| <$type_prim>::default());
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
                    V::Item: Into<$type_prim>,
                {
                    single_threaded(|| {
                        let values: V::IntoIter = values.into_iter();

                        let robj = if values.len() >= SHORT_VECTOR_LENGTH {
                            Altrep::[<make_alt $r_type:lower _from_iterator>](values)
                                .try_into()
                                .unwrap()
                        } else {
                            let mut robj = Robj::alloc_vector($sexp, values.len());
                            let dest: &mut [$type_prim] = robj.as_typed_slice_mut().unwrap();

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
                #[doc = "   assert_eq!(vec.elt(0), <"$type_elem">::default());"]
                #[doc = "   assert_eq!(vec.elt(9), <"$type_elem">::default());"]
                #[doc = "   assert!(vec.elt(10).is_na());"]
                #[doc = "}"]
                #[doc = "```"]
                pub fn elt(&self, index: usize) -> $type_elem {
                    // Defensive check for oob
                    if(index >= self.len()) {
                        <$type_elem>::na()
                    } else {
                        unsafe { [<$r_type _ELT>](self.get(), index as R_xlen_t).into() }
                    }
                }
            }

            /// Get a region of elements from the vector.
            pub fn get_region(&self, index: usize, dest: &mut [$type_prim]) {
                unsafe {
                    let ptr = dest.as_mut_ptr();
                    paste::paste!{ [<$r_type _GET_REGION>](self.get(), index as R_xlen_t, dest.len() as R_xlen_t, ptr); }
                }
            }

            /// Return `TRUE` if the vector is sorted, `FALSE` if not, or `NA_BOOL` if unknown.
            pub fn is_sorted(&self) -> Bool {
                unsafe { paste::paste!{ [<$r_type _IS_SORTED>](self.get()).into() } }
            }

            /// Return `TRUE` if the vector has `NA`s, `FALSE` if not, or `NA_BOOL` if unknown.
            pub fn no_na(&self) -> Bool {
                unsafe { paste::paste!{ [<$r_type _NO_NA>](self.get()).into() } }
            }

            paste::paste!{
                #[doc = "Return an iterator for a " $doc_name " object."]
                #[doc = "Forces ALTREP objects to manifest."]
                pub fn iter(&self) -> impl Iterator<Item = $type_elem> {
                    self.as_typed_slice().unwrap().iter().cloned()
                }
            }

            paste::paste!{
                #[doc = "Return a writable iterator for a " $doc_name " object."]
                #[doc = "Forces ALTREP objects to manifest."]
                pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut $type_elem> {
                    self.as_typed_slice_mut().unwrap().iter_mut()
                }
            }
        }

        impl FromIterator<$type_elem> for $type {
            /// A more generalised iterator collector for small vectors.
            /// Generates a non-ALTREP vector.
            fn from_iter<T: IntoIterator<Item = $type_elem>>(iter: T) -> Self {
                // Collect into a vector first.
                // TODO: specialise for ExactSizeIterator.
                let values: Vec<$type_elem> = iter.into_iter().collect();

                let mut robj = Robj::alloc_vector($sexp, values.len());
                let dest: &mut [$type_elem] = robj.as_typed_slice_mut().unwrap();

                for (d, v) in dest.iter_mut().zip(values) {
                    *d = v;
                }

                $type { robj }
            }
        }
    }
}

pub(in crate::wrapper) use gen_vector_wrapper_impl;
