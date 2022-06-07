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
/// ```ignore
/// gen_unop!(Rint, Neg, |lhs: i32| Some(-lhs), "Doc Comment");
/// ```
///
/// The 'example usage' implements the following trait definitions:
///
/// - `impl Neg for Rint`
/// - `impl Neg for &Rint`
macro_rules! gen_unop {
    ($type : tt, $opname : ident, $expr: expr, $docstring: expr) => {
        // The 'example usage' expands to...
        //
        // impl Neg for Rint {
        //      type Output = Rint;
        //
        //      /// Doc Comment
        //      fn neg(self) -> Self::Output {
        //          if let Some(lhs) = self.into() {
        //              let f = |lhs: i32| Some(-lhs);
        //              if let Some(res) = f(lhs) {
        //                  return Rint::from(res);
        //              }
        //          }
        //          Rint::na()
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
        // impl Neg for &Rint {
        //      type Output = Rint;
        //
        //      /// Doc Comment
        //      fn neg(self) -> Self::Output {
        //          if let Some(lhs) = (*self).into() {
        //              let f = |lhs: i32| Some(-lhs);
        //              if let Some(res) = f(lhs) {
        //                  return Rint::from(res);
        //              }
        //          }
        //          Rint::na()
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

pub(in crate::scalar) use gen_unop;