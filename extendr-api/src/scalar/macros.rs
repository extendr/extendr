/// Generates an implementation of a unary operator Trait for a scalar type
///
/// Generates the implementation of the specified unary operator for both `Type` and
/// `&Type`, using the logic of the provided closure to provide functionality.
///
/// This macro requires the following arguments:
///
/// * `$type`      - The Type the unary operator Trait is implemented for
/// * `$opname`    - The Trait for which the implementation is generated
/// * `$expr`      - A closure providing the logic for the implementation
/// * `$docstring` - String to include as the Doc comment for the Trait implementation
///
/// Example Usage:
///
/// ```rust,ignore
/// gen_unop!(RInt, Neg, |lhs: i32| Some(-lhs), "Doc Comment");
/// ```
///
/// The 'example usage' implements the following trait definitions:
///
/// - `impl Neg for RInt`
/// - `impl Neg for &RInt`
macro_rules! gen_unop {
    ($type : ty, $opname : ident, $expr: expr, $docstring: expr) => {
        // The 'example usage' expands to...
        //
        // impl Neg for RInt {
        //      type Output = RInt;
        //
        //      /// Doc Comment
        //      fn neg(self) -> Self::Output {
        //          if let Some(lhs) = self.into() {
        //              let f = |lhs: i32| Some(-lhs);
        //              if let Some(res) = f(lhs) {
        //                  return RInt::from(res);
        //              }
        //          }
        //          RInt::na()
        //      }
        impl $opname for $type {
            type Output = $type;

            paste::paste! {
                #[doc = $docstring]
                // Note: $opname:lower lowercases the Trait name, i.e. Neg -> neg
                fn [<$opname:lower>](self) -> Self::Output {
                    if let Some(lhs) = self.into() {
                        let f = $expr;
                        if let Some(res) = f(lhs) {
                            // Note that if res is NA, this will also be NA.
                            return $type::from(res);
                        }
                    }
                    $type::na()
                }
            }
        }

        // The 'example usage' expands to...
        //
        // impl Neg for &RInt {
        //      type Output = RInt;
        //
        //      /// Doc Comment
        //      fn neg(self) -> Self::Output {
        //          if let Some(lhs) = (*self).into() {
        //              let f = |lhs: i32| Some(-lhs);
        //              if let Some(res) = f(lhs) {
        //                  return RInt::from(res);
        //              }
        //          }
        //          RInt::na()
        //      }
        impl $opname for &$type {
            type Output = $type;

            paste::paste! {
                #[doc = $docstring]
                // Note: $opname:lower lowercases the Trait name, i.e. Neg -> neg
                fn [< $opname:lower >](self) -> Self::Output {
                    if let Some(lhs) = (*self).into() {
                        let f = $expr;
                        if let Some(res) = f(lhs) {
                            // Note that if res is NA, this will also be NA.
                            return $type::from(res);
                        }
                    }
                    $type::na()
                }
            }
        }
    };
}

/// Generates an implementation of a binary operator Trait for a scalar type
///
/// This macro requires the following arguments:
///
/// * `$type`      - The Type the binary operator Trait is implemented for
/// * `$type_prim` - The primitive Rust scalar type that corresponds to `$type`
/// * `$opname`    - The Trait for which the implementation is generated
/// * `$expr`      - A closure providing the logic for the implementation
/// * `$docstring` - String to include as the Doc comment for the Trait implementation
///
/// Example Usage:
///
/// ```rust,ignore
/// gen_binop!(RInt, i32, Add, |lhs: i32, rhs| lhs.checked_add(rhs), "Doc Comment");
/// ```
///
/// The 'example usage' implements the following trait definitions:
///
/// - `impl Add<RInt> for RInt`
/// - `impl Add<RInt> for &RInt`
/// - `impl Add<i32> for RInt`
/// - `impl Add<RInt> for i32`
// TODO: binary operators for pairs `(RType, Type)` and `(Type, RType)` using references?
macro_rules! gen_binop {
    ($type : tt, $type_prim : tt, $opname : ident, $expr: expr, $docstring: expr) => {
        // The 'example usage' expands to...
        //
        // impl Add<RInt> for RInt {
        // type Output = RInt;
        //     /// Doc Comment
        //     fn add(self, rhs: RInt) -> Self::Output {
        //         if let Some(lhs) = self.clone().into() {
        //             if let Some(rhs) = rhs.into() {
        //                 let f = |lhs: i32, rhs| lhs.checked_add(rhs);
        //                 if let Some(res) = f(lhs, rhs) {
        //                     return RInt::from(res);
        //                 }
        //             }
        //         }
        //         RInt::na()
        //     }
        // }
        impl $opname<$type> for $type {
            type Output = $type;

            paste::paste! {
                #[doc = $docstring]
                // Note: $opname:lower lowercases the Trait name, i.e. Add -> add
                fn [< $opname:lower >](self, rhs: $type) -> Self::Output {
                    if let Some(lhs) = self.clone().into() {
                        if let Some(rhs) = rhs.into() {
                            let f = $expr;
                            if let Some(res) = f(lhs, rhs) {
                                // Note that if res is NA, this will also be NA.
                                return $type::from(res);
                            }
                        }
                    }
                    $type::na()
                }
            }
        }

        // The 'example usage' expands to...
        //
        // impl Add<RInt> for &RInt {
        //      type Output = RInt;
        //      /// Doc Comment
        //      fn add(self, rhs: RInt) -> Self::Output {
        //          if let Some(lhs) = self.clone().into() {
        //              if let Some(rhs) = rhs.into() {
        //                  let f = |lhs:i32, rhs| lhs.checked_add(rhs);
        //                  if let Some(res) = f(lhs, rhs) {
        //                      return RInt::from(res);
        //                  }
        //              }
        //          }
        //          RInt::na()
        //      }
        // }
        impl $opname<$type> for &$type {
            type Output = $type;

            paste::paste! {
                #[doc = $docstring]
                // Note: $opname:lower lowercases the Trait name, i.e. Add -> add
                fn [< $opname:lower >](self, rhs: $type) -> Self::Output {
                    if let Some(lhs) = self.clone().into() {
                        if let Some(rhs) = rhs.into() {
                            let f = $expr;
                            if let Some(res) = f(lhs, rhs) {
                                // Note that if res is NA, this will also be NA.
                                return $type::from(res);
                            }
                        }
                    }
                    $type::na()
                }
            }
        }

        // The 'example usage' expands to...
        //
        // impl Add<i32> for RInt {
        //      type Output = RInt;
        //      /// Doc Comment
        //      fn add(self, rhs: i32) -> Self::Output {
        //          if let Some(lhs) = self.clone().into() {
        //              let f = |lhs:i32, rhs| lhs.checked_add(rhs);
        //              if let Some(res) = f(lhs, rhs) {
        //                  return RInt::from(res);
        //              }
        //          }
        //          RInt::na()
        //      }
        // }
        impl $opname<$type_prim> for $type {
            type Output = $type;

            paste::paste! {
                #[doc = $docstring]
                // Note: $opname:lower lowercases the Trait name, i.e. Add -> add
                fn [< $opname:lower >](self, rhs: $type_prim) -> Self::Output {
                    if let Some(lhs) = self.clone().into() {
                        let f = $expr;
                        if let Some(res) = f(lhs, rhs) {
                            // Note that if res is NA, this will also be NA.
                            return $type::from(res);
                        }
                    }
                    $type::na()
                }
            }
        }

        // The 'example usage' expands to...
        //
        // impl Add<RInt> for i32 {
        //      type Output = RInt;
        //      /// Doc Comment
        //      fn add(self, rhs: RInt) -> Self::Output {
        //          if let Some(rhs) = self.clone().into() {
        //              let f = |lhs:i32, rhs| lhs.checked_add(rhs);
        //              if let Some(res) = f(lhs, rhs) {
        //                  return RInt::from(res);
        //              }
        //          }
        //          RInt::na()
        //      }
        // }
        impl $opname<$type> for $type_prim {
            type Output = $type;

            paste::paste! {
                #[doc = $docstring]
                // Note: $opname:lower lowercases the Trait name, i.e. Add -> add
                fn [< $opname:lower >](self, rhs: $type) -> Self::Output {
                    if let Some(rhs) = rhs.clone().into() {
                        let f = $expr;
                        if let Some(res) = f(self, rhs) {
                            // Note that if res is NA, this will also be NA.
                            return $type::from(res);
                        }
                    }
                    $type::na()
                }
            }
        }
    };
}

/// Generates an implementation of a binary operate-assign Trait for a scalar type
///
/// This macro requires the following arguments:
///
/// * `$type`      - The Type the binary operate-assign Trait is implemented for
/// * `$type_prim` - The primitive Rust scalar type that corresponds to `$type`
/// * `$opname`    - The Trait for which the implementation is generated
/// * `$expr`      - A closure providing the logic for the implementation
/// * `$docstring` - String to include as the Doc comment for the Trait implementation
///
/// Example Usage:
///
/// ```rust,ignore
/// gen_binopassign!(RInt, i32, AddAssign, |lhs: i32, rhs| lhs.checked_add(rhs), "Doc Comment");
/// ```
///
/// The 'example usage' implements the following trait definitions:
///
/// - `impl AddAssign<RInt> for RInt`
/// - `impl AddAssign<RInt> for &mut RInt`
/// - `impl AddAssign<i32> for RInt`
/// - `impl AddAssign<i32> for &mut RInt`
/// - `impl AddAssign<RInt> for Option<i32>`
macro_rules! gen_binopassign {
    ($type : ty, $type_prim : ty, $opname : ident, $expr: expr, $docstring: expr) => {
        // The 'example usage' expands to...
        //
        // impl AddAssign<RInt> for RInt {
        //      /// Doc Comment
        //      fn add_assign(&mut self, other: RInt) {
        //          match (self.clone().into(), other.into()) {
        //              (Some(lhs), Some(rhs)) => {
        //                  let f = |lhs: i32, rhs| lhs.checked_add(rhs);
        //                  match f(lhs, rhs) {
        //                      Some(res) => *self = RInt::from(res),
        //                      None => *self = RInt:na(),
        //                  }
        //              }
        //              _ => *self = RInt::na(),
        //          }
        //      }
        // }
        impl $opname<$type> for $type {
            paste::paste! {
                #[doc = $docstring]
                // Note: $opname:snake snake cases the Trait name, i.e. AddAssign -> add_assign
                fn [< $opname:snake >](&mut self, other: $type) {
                    // `.clone()` is needed to convert &mut RInt -> RInt -> Option<$type_prim>
                    match (self.clone().into(), other.into()) {
                        (Some(lhs), Some(rhs)) => {
                            let f = $expr;
                            match f(lhs, rhs) {
                                Some(res) => *self = $type::from(res),
                                None => *self = $type::na(),
                            }
                        },
                        _ => *self = $type::na(),
                    }
                }
            }
        }

        // The 'example usage' expands to...
        //
        // impl AddAssign<RInt> for &mut RInt {
        //      /// Doc Comment
        //      fn add_assign(&mut self, other: RInt) {
        //          match (self.clone().into(), other.into()) {
        //              (Some(lhs), Some(rhs)) => {
        //                  let f = |lhs: i32, rhs| lhs.checked_add(rhs);
        //                  match f(lhs, rhs) {
        //                      Some(res) => **self = RInt::from(res),
        //                      None => **self = RInt:na(),
        //                  }
        //              }
        //              _ => **self = RInt::na(),
        //          }
        //      }
        // }
        impl $opname<$type> for &mut $type {
            paste::paste! {
                #[doc = $docstring]
                // Note: $opname:snake snake cases the Trait name, i.e. AddAssign -> add_assign
                fn [< $opname:snake >](&mut self, other: $type) {
                    // `.clone()` is needed to convert &mut &mut RInt -> RInt -> Option<$type_prim>
                    match (self.clone().into(), other.into()) {
                        (Some(lhs), Some(rhs)) => {
                            let f = $expr;
                            match f(lhs, rhs) {
                                Some(res) => **self = $type::from(res),
                                None => **self = $type::na(),
                            }
                        },
                        _ => **self = $type::na(),
                    }
                }
            }
        }

        // The 'example usage' expands to...
        //
        // impl AddAssign<i32> for RInt {
        //      /// Doc Comment
        //      fn add_assign(&mut self, other: i32) {
        //          match self.clone().int() {
        //              Some(lhs) => {
        //                  let f = |lhs: i32, rhs| lhs.checked_add(rhs);
        //                  match f(lhs, rhs) {
        //                      Some(res) => *self = RInt::from(res),
        //                      None => *self = RInt:na(),
        //                  }
        //              }
        //              _ => *self = RInt::na(),
        //              }
        //          }
        //      }
        // }
        impl $opname<$type_prim> for $type {
            paste::paste! {
                #[doc = $docstring]
                // Note: $opname:snake snake cases the Trait name, i.e. AddAssign -> add_assign
                fn [< $opname:snake >](&mut self, other: $type_prim) {
                    // `.clone()` is needed to convert &mut RInt -> RInt -> Option<$type_prim>
                    match self.clone().into() {
                        Some(lhs) => {
                            let f = $expr;
                            match f(lhs, other) {
                                Some(res) => *self = $type::from(res),
                                None => *self = $type::na(),
                            }
                        }
                        None => *self = $type::na(),
                    }
                }
            }
        }

        // The 'example usage' expands to...
        //
        // impl AddAssign<i32> for &mut RInt {
        //      /// Doc Comment
        //      fn add_assign(&mut self, other: i32) {
        //          match self.clone().int() {
        //              Some(lhs) => {
        //                  let f = |lhs: i32, rhs| lhs.checked_add(rhs);
        //                  match f(lhs, rhs) {
        //                      Some(res) => **self = RInt::from(res),
        //                      None => **self = RInt:na(),
        //                  }
        //              }
        //              _ => **self = RInt::na(),
        //              }
        //          }
        //      }
        // }
        impl $opname<$type_prim> for &mut $type {
            paste::paste! {
                #[doc = $docstring]
                // Note: $opname:snake snake cases the Trait name, i.e. AddAssign -> add_assign
                fn [< $opname:snake >](&mut self, other: $type_prim) {
                    // `.clone()` is needed to convert &mut &mut RInt -> RInt -> Option<$type_prim>
                    match self.clone().into() {
                        Some(lhs) => {
                            let f = $expr;
                            match f(lhs, other) {
                                Some(res) => **self = $type::from(res),
                                None => **self = $type::na(),
                            }
                        }
                        None => **self = $type::na(),
                    }
                }
            }
        }

        // The 'example usage' expands to...
        //
        //  impl AddAssign<RInt> for Option<i32> {
        //      /// Doc Comment
        //      fn add_assign(&mut self, other: RInt) {
        //          match (*self, other.into()) {
        //              (Some(lhs), Some(rhs)) => {
        //                  let f = |lhs: i32, rhs| lhs.checked_add(rhs);
        //                  *self = f(lhs, rhs);
        //              },
        //              _ => *self = None,
        //          }
        //      }
        //  }
        impl $opname<$type> for Option<$type_prim> {
            paste::paste! {
                #[doc = $docstring]
                // Note: $opname:snake snake cases the Trait name, i.e. AddAssign -> add_assign
                fn [< $opname:snake >](&mut self, other: $type) {
                    match (*self, other.into()) {
                        (Some(lhs), Some(rhs)) => {
                            let f = $expr;
                            let _ = (); // confuse clippy.
                            *self = f(lhs, rhs);
                        },
                        _ => *self = None,
                    }
                }
            }
        }
    };
}

/// Generates an implementation of type conversion Traits from a primitive type
///
/// This macro requires the following arguments:
///
/// * `$type`      - The Type the unary operator Trait is implemented for
/// * `$type_prim` - The primitive Rust scalar type that corresponds to `$type`
///
/// Example Usage:
///
/// ```rust,ignore
/// gen_from_primitive!(RInt, i32);
/// ```
///
/// The 'example usage' implements the following trait definitions:
///
/// - `From<i32> for RInt`
/// - `From<Option<i32>> for RInt`
macro_rules! gen_from_primitive {
    ($type : tt, $type_prim : tt) => {
        // The 'example usage' expands to...
        //
        // impl From<i32> for RInt {
        //     fn from(v: i32) -> Self {
        //         Self(v)
        //     }
        // }
        impl From<$type_prim> for $type {
            fn from(v: $type_prim) -> Self {
                Self(v)
            }
        }

        // Same but for references
        impl From<&$type_prim> for $type {
            fn from(v: &$type_prim) -> Self {
                Self(*v)
            }
        }

        // The 'example usage' expands to...
        //
        // impl From<Option<i32>> for RInt {
        //     fn from(v: Option<i32>) -> Self {
        //         if let Some(v) = v {
        //             v.into()
        //         } else {
        //             RInt::na()
        //         }
        //     }
        // }
        impl From<Option<$type_prim>> for $type {
            fn from(v: Option<$type_prim>) -> Self {
                if let Some(v) = v {
                    v.into()
                } else {
                    $type::na()
                }
            }
        }

        // Same but for references
        impl From<Option<&$type_prim>> for $type {
            fn from(v: Option<&$type_prim>) -> Self {
                if let Some(v) = v {
                    v.into()
                } else {
                    $type::na()
                }
            }
        }
    };
}

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
/// ```rust,ignore
/// gen_trait_impl!(RInt, i32, |x: &RInt| x.0 == i32::MIN, i32::MIN);
/// ```
///
/// The 'example usage' implements the following trait definitions:
///
/// - `impl Clone for RInt`
/// - `impl Copy for RInt`
/// - `impl CanBeNA for RInt`             // Includes doc test
/// - `impl Debug for RInt`
/// - `impl PartialEq<RInt> for RInt`     // Includes doc test
/// - `impl PartialEq<i32> for RInt`      // Includes doc test
/// - `impl Default for RInt`             // Includes doc test
macro_rules! gen_trait_impl {
    ($type : ident, $type_prim : ty, $na_check : expr, $na_val : expr) => {
        // The 'example usage' expands to...
        //
        // impl Clone for RInt {
        //     fn clone(&self) -> Self {
        //         Self(self.0)
        //     }
        // }
        impl Clone for $type {
            fn clone(&self) -> Self {
                *self
            }
        }

        // The 'example usage' expands to...
        //
        // impl Copy for RInt {}
        impl Copy for $type {}

        // The 'example usage' expands to...
        //
        // /// Documentation comments/test built by the #[doc] attributes
        // impl CanBeNA for RInt {
        //     fn is_na(&self) -> bool {
        //         (|x: &RInt| x.0 == i32::MIN)(self)
        //     }
        //
        //     fn na() -> Self {
        //         RInt(i32::MIN)
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
        // impl PartialEq<RInt> for RInt {
        //     fn eq(&self, other: &RInt) -> bool {
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
                    !(self.is_na() || other.is_na()) && self.0.eq(&other.0)
                }
            }
        }

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
        paste::paste! {
            #[doc = "```"]
            #[doc = "use extendr_api::prelude::*;"]
            #[doc = "test! {"]
            #[doc = "    assert!(<" $type_prim ">::default().eq(&<" $type ">::default()));"]
            #[doc = "}"]
            #[doc = "```"]
            impl PartialEq<$type> for $type_prim {
                fn eq(&self, other: &$type) -> bool {
                    <Option<$type_prim>>::try_from(*other) == Ok(Some(*self))
                }
            }
        }

        // The 'example usage' expands to...
        //
        // /// Documentation comments/test built by the #[doc] attributes
        // impl std::default::Default for RInt {
        //     fn default() -> Self {
        //         RInt(<i32>::default())
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
        }
    };
}

macro_rules! gen_partial_ord {
    ($type : ident, $type_prim : ty) => {
        paste::paste! {
            #[doc = "```"]
            #[doc = "use extendr_api::prelude::*;"]
            #[doc = "test! {"]
            #[doc = "    assert_eq!(<" $type ">::default() <  <" $type ">::na(), false);"]
            #[doc = "    assert_eq!(<" $type ">::default() <= <" $type ">::na(), false);"]
            #[doc = "    assert_eq!(<" $type ">::default() >  <" $type ">::na(), false);"]
            #[doc = "    assert_eq!(<" $type ">::default() >= <" $type ">::na(), false);"]
            #[doc = "    assert_eq!(<" $type ">::default() <  <" $type ">::default(), false);"]
            #[doc = "    assert_eq!(<" $type ">::default() <= <" $type ">::default(), true);"]
            #[doc = "    assert_eq!(<" $type ">::default() >  <" $type ">::default(), false);"]
            #[doc = "    assert_eq!(<" $type ">::default() >= <" $type ">::default(), true);"]
            #[doc = "}"]
            #[doc = "```"]
            impl std::cmp::PartialOrd<$type> for $type {
                fn partial_cmp(&self, other: &$type) -> Option<std::cmp::Ordering> {
                    if self.is_na() || other.is_na() {
                        None
                    } else {
                        self.0.partial_cmp(&other.0)
                    }
                }
            }
        }

        paste::paste! {
            #[doc = "```"]
            #[doc = "use extendr_api::prelude::*;"]
            #[doc = "test! {"]
            #[doc = "    assert_eq!(<" $type_prim ">::default() <  <" $type ">::na(), false);"]
            #[doc = "    assert_eq!(<" $type_prim ">::default() <= <" $type ">::na(), false);"]
            #[doc = "    assert_eq!(<" $type_prim ">::default() >  <" $type ">::na(), false);"]
            #[doc = "    assert_eq!(<" $type_prim ">::default() >= <" $type ">::na(), false);"]
            #[doc = "    assert_eq!(<" $type_prim ">::default() <  <" $type ">::default(), false);"]
            #[doc = "    assert_eq!(<" $type_prim ">::default() <= <" $type ">::default(), true);"]
            #[doc = "    assert_eq!(<" $type_prim ">::default() >  <" $type ">::default(), false);"]
            #[doc = "    assert_eq!(<" $type_prim ">::default() >= <" $type ">::default(), true);"]
            #[doc = "}"]
            #[doc = "```"]
            impl std::cmp::PartialOrd<$type_prim> for $type {
                fn partial_cmp(&self, other: &$type_prim) -> Option<std::cmp::Ordering> {
                    let other: $type = (*other).try_into().unwrap_or($type::na());
                    self.partial_cmp(&other)
                }
            }
        }

        paste::paste! {
            #[doc = "```"]
            #[doc = "use extendr_api::prelude::*;"]
            #[doc = "test! {"]
            #[doc = "    assert_eq!(<" $type ">::default() <  <" $type ">::default(), false);"]
            #[doc = "    assert_eq!(<" $type ">::default() <= <" $type ">::default(), true);"]
            #[doc = "    assert_eq!(<" $type ">::default() >  <" $type ">::default(), false);"]
            #[doc = "    assert_eq!(<" $type ">::default() >= <" $type ">::default(), true);"]
            #[doc = "}"]
            #[doc = "```"]
            impl std::cmp::PartialOrd<$type> for $type_prim {
                fn partial_cmp(&self, other: &$type) -> Option<std::cmp::Ordering> {
                    let slf: $type = (*self).try_into().unwrap_or($type::na());
                    slf.partial_cmp(other)
                }
            }
        }
    };
}

/// Generates an implementation of `std::iter::Sum` for a scalar type
///
/// This macro requires the following argument:
///
/// * `$type`   - The Type to implement `std::iter::Sum` for
///
/// Example Usage:
///
/// ```rust,ignore
/// gen_sum_iter!(RInt);
/// ```
macro_rules! gen_sum_iter {
    ($type : ty) => {
        // The 'example usage' expands to...
        //
        // impl std::iter::Sum for $type {
        //     /// Documentation comments/test built by the #[doc] attributes
        //     fn sum<I: Iterator<Item = RInt>>(iter: I) -> RInt {
        //         iter.fold(RInt::default(), |a, b| a + b)
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
                    iter.fold($type::default(), |a, b| a + b)
                }
            }
        }

        // The 'example usage' expands to...
        //
        // impl std::iter::Sum for &$type {
        //     /// Documentation comments/test built by the #[doc] attributes
        //     fn sum<I: Iterator<Item = &RInt>>(iter: I) -> RInt {
        //         iter.fold(RInt::default(), |a, b| a + b)
        //     }
        // }
        impl<'a> std::iter::Sum<&'a $type> for $type {
            paste::paste! {
                #[doc = "Yields NA on overflow if NAs present."]
                #[doc = "```"]
                #[doc = "use extendr_api::prelude::*;"]
                #[doc = "use std::iter::Sum;"]
                #[doc = "test! {"]
                #[doc = "    let z =" $type "::default();"]
                #[doc = "    let x = (0..100).map(|_| &z);"]
                #[doc = "    assert_eq!(<" $type " as Sum<& " $type ">>::sum(x), <" $type ">::default());"]
                #[doc = "}"]
                #[doc = "```"]
                fn sum<I: Iterator<Item = &'a $type>>(iter: I) -> $type {
                    iter.fold($type::default(), |a, b| a + *b)
                }
            }
        }
    };
}

pub(in crate::scalar) use gen_binop;
pub(in crate::scalar) use gen_binopassign;
pub(in crate::scalar) use gen_from_primitive;
pub(in crate::scalar) use gen_partial_ord;
pub(in crate::scalar) use gen_sum_iter;
pub(in crate::scalar) use gen_trait_impl;
pub(in crate::scalar) use gen_unop;
