use super::*;

//TODO: It could be an idea to make this a attribute macro for users,
// if they wish to specialise their own types as representable in R.

/// Generates a [`ToVectorValue`] for a type, by inheriting the properties
/// of another type's [`ToVectorValue`]'s implementation.
///
/// This is meant to be used for wrappers of types that may be represented
/// in R. The marshalling rules for this is represented in `ToVectorValue`,
/// and this macro merely co-opts
///
/// Arguments:
///
/// * `$type`         - Target type
/// * `$synonym_type` - A type that has a `ToVectorValue`-impl, and an `inner`-method.
///
/// Requirements: `$type` must have an `inner`-method to extract the
/// wrapped value. It suffices that `$type` implements `Scalar<T>`.
///
/// Example usage:
///
/// ```ignore
/// impl_synonym_type(Rint, i32);
/// ```
///
/// The example here implements
///
/// `impl ToVectorValue for Rint`,
///
/// this entails that `Rint` would be stored in `R` exactly
/// as `i32`.
///
macro_rules! impl_synonym_type {
    ($type: ty, $synonym_type: ty) => {
        impl ToVectorValue for $type {
            fn sexptype() -> SEXPTYPE {
                <$synonym_type as ToVectorValue>::sexptype()
            }

            fn to_real(&self) -> f64
            where
                Self: Sized,
            {
                <$synonym_type as ToVectorValue>::to_real(&self.inner())
            }

            fn to_complex(&self) -> Rcomplex
            where
                Self: Sized,
            {
                <$synonym_type as ToVectorValue>::to_complex(&self.inner())
            }

            fn to_integer(&self) -> i32
            where
                Self: Sized,
            {
                <$synonym_type as ToVectorValue>::to_integer(&self.inner())
            }

            fn to_logical(&self) -> i32
            where
                Self: Sized,
            {
                <$synonym_type as ToVectorValue>::to_logical(&self.inner())
            }

            fn to_raw(&self) -> u8
            where
                Self: Sized,
            {
                <$synonym_type as ToVectorValue>::to_raw(&self.inner())
            }

            fn to_sexp(&self) -> SEXP
            where
                Self: Sized,
            {
                <$synonym_type as ToVectorValue>::to_sexp(&self.inner())
            }
        }
    };
}
impl_synonym_type!(Rfloat, f64);
impl_synonym_type!(&Rfloat, f64);
impl_synonym_type!(Rint, i32);
impl_synonym_type!(&Rint, i32);
