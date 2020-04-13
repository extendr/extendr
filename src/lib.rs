//!
//! rapi - A safe and user friendly R extension interface.
//! 
//! This library aims to provide an interface that will be familiar to
//! first-time users of Rust or indeed any compiled language.
//! 
//! Anyone who knows the R library should be able to write R extensions.
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
//! 

mod robj;
mod args;
mod engine;

pub use robj::*;
pub use args::*;
pub use engine::*;

// Generic dynamic error type.
pub type AnyError = Box<dyn std::error::Error + Send + Sync>;

#[macro_export]
macro_rules! c {
    () => {
        Robj::Null
    };
    ($($($tok: tt)*),*) => {
        let args = args!($($rest)*);
        for (n, v) in args {
            match v.sexptype() {
                NILSXP => true,
                SYMSXP => PRINTNAME(lsexp) == PRINTNAME(rsexp),
                LISTSXP => false,
                CLOSXP => false,
                ENVSXP => false,
                PROMSXP => false,
                LANGSXP => false,
                SPECIALSXP => false,
                BUILTINSXP => false,
                CHARSXP => self.as_str() == rhs.as_str(),
                LGLSXP | INTSXP => self.as_i32_slice() == rhs.as_i32_slice(),
                REALSXP => self.as_f64_slice() == rhs.as_f64_slice(),
                CPLXSXP => false,
                STRSXP => false,
                DOTSXP => false,
                ANYSXP => false,
                VECSXP => false,
                EXPRSXP => false,
                BCODESXP => false,
                EXTPTRSXP => false,
                WEAKREFSXP => false,
                RAWSXP => self.as_u8_slice() == rhs.as_u8_slice(),
                S4SXP => false,
                NEWSXP => false,
                FREESXP => false,
                _ => false
            }
        }
    }
}

