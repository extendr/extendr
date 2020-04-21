//!
//! rapi - A safe and user friendly R extension interface.
//! 
//! This library aims to provide an interface that will be familiar to
//! first-time users of Rust or indeed any compiled language.
//! 
//! Anyone who knows the R library should be able to write R extensions.
//! 
//! See the [Robj](../robj/struct.Robj.html) struct for much of the content of this crate.
//! [Robj](../robj/struct.Robj.html) provides a safe wrapper for the R object type.
//! 
//! This library is just being born, but goals are:
//! 
//! Implement common R functions such as c() and print()
//! 
//! Example:
//! 
//! ```no-run
//! let v = c!(1, 2, 3);
//! let l = list!(a=1, b=2);
//! print!(v, l);
//! ```
//! 
//! Provide a wrapper for r objects.
//! 
//! Example:
//! 
//! ```no-run
//! let s = Robj::from("hello");
//! let i = Robj::from(1);
//! let r = Robj::from(1.0);
//! ```
//! 
//! Provide iterator support for creation and consumption of r vectors.
//! 
//! Example:
//! 
//! ```no-run
//! let res = (1..=100).iter().collect::<Robj>();
//! for x in res {
//!     print!(x);
//! }
//! ```
//! 
//! Provide a procedural macro to adapt Rust functions to R
//! 
//! Example:
//! 
//! ```no-run
//! #[derive(RCallable)]
//! fn fred(a: i32) -> i32 {
//!     a + 1
//! }
//! ```
//! 
//! In R:
//! 
//! ```no-run
//! 
//! result <- .Call("fred", 1)
//! 
//! ```
//! 

mod robj;
mod args;
mod engine;
mod rmacros;

pub use robj::*;
pub use args::*;
pub use engine::*;
pub use rmacros::*;

use libR_sys::{R_BaseEnv, R_GlobalEnv};

// Generic dynamic error type.
pub type AnyError = Box<dyn std::error::Error + Send + Sync>;

pub fn baseenv() -> Robj {
    unsafe { Robj::Sys(R_BaseEnv) }
}

pub fn globalenv() -> Robj {
    unsafe { Robj::Sys(R_GlobalEnv) }
}

