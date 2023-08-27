//! Provides `TryFrom` implementations for Rust's Nonzero primitives, as
//! found in [`std::num`].
//! 
//! Note that the conversions are exact, as R may only represent `i32` and
//! `f64`.
use super::*;

macro_rules! impl_try_from_scalar_real_nonzero {
    ($t:ty) => {
        impl TryFrom<&Robj> for $t {
            type Error = Error;

            /// Convert a possibly non-zero R numeric object to a guaranteed non-zero Rust type.
            fn try_from(robj: &Robj) -> Result<Self> {
                Self::new(robj.try_into()?).ok_or(Error::ExpectedNonZeroValue(robj.clone()))
            }
        }
    };
}

impl_try_from_scalar_real_nonzero!(std::num::NonZeroI8);
impl_try_from_scalar_real_nonzero!(std::num::NonZeroI16);
impl_try_from_scalar_real_nonzero!(std::num::NonZeroI32);
impl_try_from_scalar_real_nonzero!(std::num::NonZeroI64);
impl_try_from_scalar_real_nonzero!(std::num::NonZeroIsize);
impl_try_from_scalar_real_nonzero!(std::num::NonZeroU8);
impl_try_from_scalar_real_nonzero!(std::num::NonZeroU16);
impl_try_from_scalar_real_nonzero!(std::num::NonZeroU32);
impl_try_from_scalar_real_nonzero!(std::num::NonZeroU64);
impl_try_from_scalar_real_nonzero!(std::num::NonZeroUsize);

impl_try_from_robj!(
  std::num::NonZeroI8
  std::num::NonZeroI16
  std::num::NonZeroI32
  std::num::NonZeroI64
  std::num::NonZeroIsize
  std::num::NonZeroU8
  std::num::NonZeroU16
  std::num::NonZeroU32
  std::num::NonZeroU64
  std::num::NonZeroUsize
);