/*!
A set of optional features and third-party crate integrations, usually hidden behind feature gates.
*/
#[cfg(feature = "either")]
pub mod either;
#[cfg(feature = "faer")]
mod faer;
#[cfg(feature = "nalgebra")]
pub mod nalgebra;
#[cfg(feature = "ndarray")]
pub mod ndarray;
