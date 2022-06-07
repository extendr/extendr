
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
/// ```ignore
/// gen_binop_assign!(Rint, i32, AddAssign, |lhs: i32, rhs| lhs.checked_add(rhs), "Doc Comment");
/// ```
///
/// The 'example usage' implements the following trait definitions:
///
/// - `impl AddAssign<Rint> for Rint`
/// - `impl AddAssign<Rint> for &mut Rint`
/// - `impl AddAssign<i32> for Rint`
/// - `impl AddAssign<i32> for &mut Rint`
/// - `impl AddAssign<Rint> for Option<i32>`
macro_rules! gen_binop_assign {
    ($type : tt, $type_prim : tt, $opname : ident, $expr: expr, $docstring: expr) => {
        // The 'example usage' expands to...
        //
        // impl AddAssign<Rint> for Rint {
        //      /// Doc Comment
        //      fn add_assign(&mut self, other: Rint) {
        //          match (self.clone().into(), other.into()) {
        //              (Some(lhs), Some(rhs)) => {
        //                  let f = |lhs: i32, rhs| lhs.checked_add(rhs);
        //                  match f(lhs, rhs) {
        //                      Some(res) => *self = Rint::from(res),
        //                      None => *self = Rint:na(),
        //                  }
        //              }
        //              _ => *self = Rint::na(),
        //          }
        //      }
        // }
        impl $opname<$type> for $type {
            paste::paste! {
                #[doc = $docstring]
                // Note: $opname:snake snake cases the Trait name, i.e. AddAssign -> add_assign
                fn [< $opname:snake >](&mut self, other: $type) {
                    // `.clone()` is needed to convert &mut Rint -> Rint -> Option<$type_prim>
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
        // impl AddAssign<Rint> for &mut Rint {
        //      /// Doc Comment
        //      fn add_assign(&mut self, other: Rint) {
        //          match (self.clone().into(), other.into()) {
        //              (Some(lhs), Some(rhs)) => {
        //                  let f = |lhs: i32, rhs| lhs.checked_add(rhs);
        //                  match f(lhs, rhs) {
        //                      Some(res) => **self = Rint::from(res),
        //                      None => **self = Rint:na(),
        //                  }
        //              }
        //              _ => **self = Rint::na(),
        //          }
        //      }
        // }
        impl $opname<$type> for &mut $type {
            paste::paste! {
                #[doc = $docstring]
                // Note: $opname:snake snake cases the Trait name, i.e. AddAssign -> add_assign
                fn [< $opname:snake >](&mut self, other: $type) {
                    // `.clone()` is needed to convert &mut &mut Rint -> Rint -> Option<$type_prim>
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
        // impl AddAssign<i32> for Rint {
        //      /// Doc Comment
        //      fn add_assign(&mut self, other: i32) {
        //          match self.clone().int() {
        //              Some(lhs) => {
        //                  let f = |lhs: i32, rhs| lhs.checked_add(rhs);
        //                  match f(lhs, rhs) {
        //                      Some(res) => *self = Rint::from(res),
        //                      None => *self = Rint:na(),
        //                  }
        //              }
        //              _ => *self = Rint::na(),
        //              }
        //          }
        //      }
        // }
        impl $opname<$type_prim> for $type {
            paste::paste! {
                #[doc = $docstring]
                // Note: $opname:snake snake cases the Trait name, i.e. AddAssign -> add_assign
                fn [< $opname:snake >](&mut self, other: $type_prim) {
                    // `.clone()` is needed to convert &mut Rint -> Rint -> Option<$type_prim>
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
        // impl AddAssign<i32> for &mut Rint {
        //      /// Doc Comment
        //      fn add_assign(&mut self, other: i32) {
        //          match self.clone().int() {
        //              Some(lhs) => {
        //                  let f = |lhs: i32, rhs| lhs.checked_add(rhs);
        //                  match f(lhs, rhs) {
        //                      Some(res) => **self = Rint::from(res),
        //                      None => **self = Rint:na(),
        //                  }
        //              }
        //              _ => **self = Rint::na(),
        //              }
        //          }
        //      }
        // }
        impl $opname<$type_prim> for &mut $type {
            paste::paste! {
                #[doc = $docstring]
                // Note: $opname:snake snake cases the Trait name, i.e. AddAssign -> add_assign
                fn [< $opname:snake >](&mut self, other: $type_prim) {
                    // `.clone()` is needed to convert &mut &mut Rint -> Rint -> Option<$type_prim>
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
        //  impl AddAssign<Rint> for Option<i32> {
        //      /// Doc Comment
        //      fn add_assign(&mut self, other: Rint) {
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

pub(in crate::scalar) use gen_binop_assign;