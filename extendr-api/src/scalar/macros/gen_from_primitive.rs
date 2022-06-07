/// Generates an implementation of type conversion Traits from a primitive type
///
/// This macro requires the following arguments:
///
/// * `$type`      - The Type the unary operator Trait is implemented for
/// * `$type_prim` - The primitive Rust scalar type that corresponds to `$type`
///
/// Example Usage:
///
/// ```ignore
/// gen_from_primitive!(Rint, i32);
/// ```
///
/// The 'example usage' implements the following trait definitions:
///
/// - `From<i32> for Rint`
/// - `From<Option<i32>> for Rint`
macro_rules! gen_from_primitive {
    ($type : tt, $type_prim : tt) => {
        // The 'example usage' expands to...
        //
        // impl From<i32> for Rint {
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
        // impl From<Option<i32>> for Rint {
        //     fn from(v: Option<i32>) -> Self {
        //         if let Some(v) = v {
        //             v.into()
        //         } else {
        //             Rint::na()
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

pub(in crate::scalar) use gen_from_primitive;
