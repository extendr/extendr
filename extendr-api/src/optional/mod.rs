/*!
A set of optional features and third-party crate integrations, usually hidden behind feature gates.
*/
#[cfg(feature = "either")]
pub mod either;
#[cfg(feature = "faer")]
mod faer;

#[cfg(feature = "ndarray_0_15")]
pub mod ndarray_0_15;

#[cfg(feature = "ndarray_0_16")]
pub mod ndarray_0_16;
