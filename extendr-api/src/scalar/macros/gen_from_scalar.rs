/// Generates an implementation of type conversion Traits from a scalar type
///
/// This macro requires the following arguments:
///
/// * `$type`      - The Type the unary operator Trait is implemented for
/// * `$type_prim` - The primitive Rust scalar type that corresponds to `$type`
///
/// Example Usage:
///
/// ```ignore
/// gen_from_scalar!(Rint, i32);
/// ```
///
/// The 'example usage' implements the following trait definitions:
///
/// - `From<Rint> for Option<i32>`
/// - `From<Rint> for Robj`
macro_rules! gen_from_scalar {
    ($type : tt, $type_prim : tt) => {
        // The 'example usage' expands to...
        //
        // impl From<Rint> for Option<i32> {
        //     fn from(v: Rint) -> Self {
        //         if v.is_na() {
        //             None
        //         } else {
        //             Some(v.0)
        //         }
        //     }
        // }
        impl From<$type> for Option<$type_prim> {
            fn from(v: $type) -> Self {
                if v.is_na() {
                    None
                } else {
                    Some(v.0)
                }
            }
        }

        // The 'example usage' expands to...
        //
        // impl From<Rint> for Robj {
        //     fn from(value: Rint) -> Self {
        //         Robj::from(value.0)
        //     }
        // }
        impl From<$type> for Robj {
            fn from(value: $type) -> Self {
                Robj::from(value.0)
            }
        }
    };
}

pub(in crate::scalar) use gen_from_scalar;
