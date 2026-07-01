mod macros;
mod rbool;
mod rfloat;
mod rint;
pub use rbool::RBool;
pub use rfloat::RFloat;
pub use rint::RInt;

#[deprecated(note = "Use RInt instead", since = "0.9.0")]
pub use rint::Rint;

#[deprecated(note = "Use RBool instead", since = "0.9.0")]
pub use rbool::Rbool;

#[deprecated(note = "Use RFloat instead", since = "0.9.0")]
pub use rfloat::Rfloat;

#[cfg(feature = "num-complex")]
mod rcplx_full;

#[cfg(feature = "num-complex")]
pub use rcplx_full::{c64, RCplx};

#[cfg(not(feature = "num-complex"))]
mod rcplx_default;

#[cfg(not(feature = "num-complex"))]
pub use rcplx_default::{c64, RCplx};

#[deprecated(note = "Use RCplx instead", since = "0.9.0")]
pub type Rcplx = RCplx;
