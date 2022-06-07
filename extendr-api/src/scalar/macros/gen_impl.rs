/// Generates an implementation of the instance `inner()` method for a type
///
/// This macro requires the following arguments:
///
/// * `$type`      - The Type the `inner()` method is implemented for
/// * `$type_prim` - The primitive Rust scalar type that corresponds to `$type`
///
/// Example Usage:
///
/// ```ignore
/// gen_impl!(Rint, i32);
/// ```
macro_rules! gen_impl {
    ($type : ident, $type_prim : ty) => {
        /// Get underlying value.
        pub fn inner(&self) -> $type_prim {
            self.0
        }
    };
}
pub(in crate::scalar) use gen_impl;
