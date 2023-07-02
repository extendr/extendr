mod macros;
mod rbool;
mod rfloat;
mod rint;
pub use rbool::Rbool;
pub use rfloat::Rfloat;
pub use rint::Rint;

#[cfg(feature = "num-complex")]
mod rcplx_full;

#[cfg(feature = "num-complex")]
pub use rcplx_full::{c64, Rcplx};

#[cfg(not(feature = "num-complex"))]
mod rcplx_default;

#[cfg(not(feature = "num-complex"))]
pub use rcplx_default::{c64, Rcplx};

pub trait Scalar<T>: crate::CanBeNA
where
    T: PartialEq + Copy,
{
    fn inner(&self) -> T;
    fn new(val: T) -> Self;
}
