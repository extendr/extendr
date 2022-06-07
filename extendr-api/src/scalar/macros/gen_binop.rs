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
/// ```ignore
/// gen_binop!(Rint, i32, Add, |lhs: i32, rhs| lhs.checked_add(rhs), "Doc Comment");
/// ```
///
/// The 'example usage' implements the following trait definitions:
///
/// - `impl Add<Rint> for Rint`
/// - `impl Add<Rint> for &Rint`
/// - `impl Add<i32> for Rint`
/// - `impl Add<Rint> for i32`
// TODO: binary operators for pairs `(Rtype, Type)` and `(Type, Rtype)` using references?
macro_rules! gen_binop {
    ($type : tt, $type_prim : tt, $opname : ident, $expr: expr, $docstring: expr) => {
        // The 'example usage' expands to...
        //
        // impl Add<Rint> for Rint {
        // type Output = Rint;
        //     /// Doc Comment
        //     fn add(self, rhs: Rint) -> Self::Output {
        //         if let Some(lhs) = self.clone().into() {
        //             if let Some(rhs) = rhs.into() {
        //                 let f = |lhs: i32, rhs| lhs.checked_add(rhs);
        //                 if let Some(res) = f(lhs, rhs) {
        //                     return Rint::from(res);
        //                 }
        //             }
        //         }
        //         Rint::na()
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
        // impl Add<Rint> for &Rint {
        //      type Output = Rint;
        //      /// Doc Comment
        //      fn add(self, rhs: Rint) -> Self::Output {
        //          if let Some(lhs) = self.clone().into() {
        //              if let Some(rhs) = rhs.into() {
        //                  let f = |lhs:i32, rhs| lhs.checked_add(rhs);
        //                  if let Some(res) = f(lhs, rhs) {
        //                      return Rint::from(res);
        //                  }
        //              }
        //          }
        //          Rint::na()
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
        // impl Add<i32> for Rint {
        //      type Output = Rint;
        //      /// Doc Comment
        //      fn add(self, rhs: i32) -> Self::Output {
        //          if let Some(lhs) = self.clone().into() {
        //              let f = |lhs:i32, rhs| lhs.checked_add(rhs);
        //              if let Some(res) = f(lhs, rhs) {
        //                  return Rint::from(res);
        //              }
        //          }
        //          Rint::na()
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
        // impl Add<Rint> for i32 {
        //      type Output = Rint;
        //      /// Doc Comment
        //      fn add(self, rhs: Rint) -> Self::Output {
        //          if let Some(rhs) = self.clone().into() {
        //              let f = |lhs:i32, rhs| lhs.checked_add(rhs);
        //              if let Some(res) = f(lhs, rhs) {
        //                  return Rint::from(res);
        //              }
        //          }
        //          Rint::na()
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

pub(in crate::scalar) use gen_binop;
