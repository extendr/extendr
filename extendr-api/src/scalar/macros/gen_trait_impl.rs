/// Generates an implementation of a number of Traits for the specified Type
///
/// This macro requires the following arguments:
///
/// * `$type`      - The Type the Traits are implemented for
/// * `$type_prim` - The primitive Rust scalar type that corresponds to `$type`
/// * `$na_check`  - Closure that provides `NA`-checking logic
/// * `$na_val`    - The Rust-native value that translates to `NA`
///
/// Example Usage:
///
/// ```ignore
/// gen_trait_impl!(Rint, i32, |x: &Rint| x.0 == i32::MIN, i32::MIN);
/// ```
///
/// The 'example usage' implements the following trait definitions:
///
/// - `impl Clone for Rint`
/// - `impl Copy for Rint`
/// - `impl CanBeNA for Rint`             // Includes doc test
/// - `impl Debug for Rint`
/// - `impl PartialEq<Rint> for Rint`     // Includes doc test
/// - `impl PartialEq<i32> for Rint`      // Includes doc test
/// - `impl Default for Rint`             // Includes doc test
macro_rules! gen_trait_impl {
    ($type : ident, $type_prim : ty, $na_check : expr, $na_val : expr) => {
        // The 'example usage' expands to...
        //
        // impl Clone for Rint {
        //     fn clone(&self) -> Self {
        //         Self(self.0)
        //     }
        // }
        impl Clone for $type {
            fn clone(&self) -> Self {
                Self(self.0)
            }
        }

        // The 'example usage' expands to...
        //
        // impl Copy for Rint {}
        impl Copy for $type {}

        // The 'example usage' expands to...
        //
        // /// Documentation comments/test built by the #[doc] attributes
        // impl CanBeNA for Rint {
        //     fn is_na(&self) -> bool {
        //         (|x: &Rint| x.0 == i32::MIN)(self)
        //     }
        //
        //     fn na() -> Self {
        //         Rint(i32::MIN)
        //     }
        // }
        paste::paste! {
            #[doc = "```"]
            #[doc = "use extendr_api::prelude::*;"]
            #[doc = "test! {"]
            #[doc = "    assert!((<" $type ">::na()).is_na());"]
            #[doc = "}"]
            #[doc = "```"]
            impl CanBeNA for $type {
                /// Return true is the is a NA value.
                fn is_na(&self) -> bool {
                    $na_check(self)
                }
                /// Construct a NA.
                fn na() -> Self {
                    $type($na_val)
                }
            }
        }

        // The 'example usage' expands to...
        //
        //
        // /// Documentation comments/test built by the #[doc] attributes
        // impl PartialEq<Rint> for Rint {
        //     fn eq(&self, other: &Rint) -> bool {
        //         !(self.is_na() || other.is_na()) && self.0 == other.0
        //     }
        // }
        paste::paste! {
            #[doc = "```"]
            #[doc = "use extendr_api::prelude::*;"]
            #[doc = "test! {"]
            #[doc = "    assert!(<" $type ">::default().eq(&<" $type ">::default()));"]
            #[doc = "    assert!(!<" $type ">::na().eq(&<" $type ">::na()));"]
            #[doc = "}"]
            #[doc = "```"]
            impl PartialEq<$type> for $type {
                fn eq(&self, other: &$type) -> bool {
                    !(self.is_na() || other.is_na()) && self.0 == other.0
                }
            }
        }

        // The 'example usage' expands to...
        //
        // /// Documentation comments/test built by the #[doc] attributes
        // impl PartialEq<i32> for Rint {
        //     fn eq(&self, other: &i32) -> bool {
        //         !self.is_na() && self.0 == *other
        //     }
        // }
        paste::paste! {
            #[doc = "```"]
            #[doc = "use extendr_api::prelude::*;"]
            #[doc = "test! {"]
            #[doc = "    assert!(<" $type ">::default().eq(&<" $type_prim ">::default()));"]
            #[doc = "}"]
            #[doc = "```"]
            impl PartialEq<$type_prim> for &$type {
                /// NA always fails.
                fn eq(&self, other: &$type_prim) -> bool {
                    <Option<$type_prim>>::try_from(**self) == Ok(Some(*other))
                }
            }
        }

        // The 'example usage' expands to...
        //
        // /// Documentation comments/test built by the #[doc] attributes
        // impl PartialEq<i32> for Rint {
        //     fn eq(&self, other: &i32) -> bool {
        //         !self.is_na() && self.0 == *other
        //     }
        // }
        paste::paste! {
            #[doc = "```"]
            #[doc = "use extendr_api::prelude::*;"]
            #[doc = "test! {"]
            #[doc = "    assert!(<" $type ">::default().eq(&<" $type_prim ">::default()));"]
            #[doc = "}"]
            #[doc = "```"]
            impl PartialEq<$type_prim> for $type {
                /// NA always fails.
                fn eq(&self, other: &$type_prim) -> bool {
                    <Option<$type_prim>>::try_from(self.clone()) == Ok(Some(*other))
                }
            }
        }

        // The 'example usage' expands to...
        //
        // /// Documentation comments/test built by the #[doc] attributes
        // impl std::default::Default for Rint {
        //     fn default() -> Self {
        //         Rint(<i32>::default())
        //     }
        // }
        paste::paste! {
            #[doc = "```"]
            #[doc = "use extendr_api::prelude::*;"]
            #[doc = "test! {"]
            #[doc = "    assert_eq!(<" $type ">::default(), <" $type_prim ">::default());"]
            #[doc = "}"]
            #[doc = "```"]
            impl std::default::Default for $type {
                fn default() -> Self {
                    $type::from(<$type_prim>::default())
                }
            }

            impl crate::contract::scalar::GenericScalar for $type {}
        }
    };
}

pub(in crate::scalar) use gen_trait_impl;
