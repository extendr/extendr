//!
//! extendr - A safe and user friendly R extension interface.
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
//! ```ignore
//! let v = c!(1, 2, 3);
//! let l = list!(a=1, b=2);
//! print!(v, l);
//! ```
//! 
//! Provide a wrapper for r objects.
//! 
//! Example:
//! 
//! ```ignore
//! let s = Robj::from("hello");
//! let i = Robj::from(1);
//! let r = Robj::from(1.0);
//! ```
//! 
//! Provide iterator support for creation and consumption of r vectors.
//! 
//! Example:
//! 
//! ```ignore
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
//! ```ignore
//! #[derive(RCallable)]
//! fn fred(a: i32) -> i32 {
//!     a + 1
//! }
//! ```
//! 
//! In R:
//! 
//! ```ignore
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

/// Generic dynamic error type.
pub type AnyError = Box<dyn std::error::Error + Send + Sync>;

#[cfg(test)]
mod tests {
    use extendr_macros::export_function;

    #[export_function]
    pub fn im_a_function() {

    }

    #[test]
    fn export_test() {
        //im_a_function();
        assert!(false);
    }
}

