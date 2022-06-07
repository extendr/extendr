/// Generates an implementation of `std::iter::Sum` for a scalar type
///
/// This macro requires the following arguments:
///
/// * `$type`   - The Type to implement `std::iter::Sum` for
/// * `$zero`   - The zero value for the primitive counterpart to Type
///
/// Example Usage:
///
/// ```ignore
/// gen_sum_iter!(Rint, 0i32);
/// ```
macro_rules! gen_sum_iter {
    ($type : tt, $zero : expr) => {
        // The 'example usage' expands to...
        //
        // impl std::iter::Sum for $type {
        //     /// Documentation comments/test built by the #[doc] attributes
        //     fn sum<I: Iterator<Item = Rint>>(iter: I) -> Rint {
        //         iter.fold(Rint::from(0i32), |a, b| a + b)
        //     }
        // }
        impl std::iter::Sum for $type {
            paste::paste! {
                #[doc = "Yields NA on overflow if NAs present."]
                #[doc = "```"]
                #[doc = "use extendr_api::prelude::*;"]
                #[doc = "use std::iter::Sum;"]
                #[doc = "test! {"]
                #[doc = "    let x = (0..100).map(|x| " $type "::default());"]
                #[doc = "    assert_eq!(<" $type " as Sum>::sum(x), <" $type ">::default());"]
                #[doc = "}"]
                #[doc = "```"]
                fn sum<I: Iterator<Item = $type>>(iter: I) -> $type {
                    iter.fold($type::from($zero), |a, b| a + b)
                }
            }
        }
    };
}

pub(in crate::scalar) use gen_sum_iter;
