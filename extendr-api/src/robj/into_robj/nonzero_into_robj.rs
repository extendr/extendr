use super::*;

macro_rules! impl_into_robj_nonzero {
    ($type: ty) => {
        impl From<$type> for Robj {
            fn from(value: $type) -> Self {
                // A nonzero Rust type is automatically guaranteed
                // to be a valid R type if the normal type can be converted to R
                value.get().into()
            }
        }
    };
}

impl_into_robj_nonzero!(std::num::NonZeroU8);
impl_into_robj_nonzero!(std::num::NonZeroU16);
impl_into_robj_nonzero!(std::num::NonZeroU32);
impl_into_robj_nonzero!(std::num::NonZeroU64);
impl_into_robj_nonzero!(std::num::NonZeroUsize);
impl_into_robj_nonzero!(std::num::NonZeroI8);
impl_into_robj_nonzero!(std::num::NonZeroI16);
impl_into_robj_nonzero!(std::num::NonZeroI32);
impl_into_robj_nonzero!(std::num::NonZeroI64);
// NOTE: `NonZeroIsize` is missing
