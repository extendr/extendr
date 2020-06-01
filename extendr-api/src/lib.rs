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
//! #[export_function]
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
    use crate as extendr_api;
    use super::Robj;

    #[export_function]
    pub fn inttypes(a: i8, b: u8, c:i16, d: u16, e: i32, f: u32, g: i64, h: u64) {
        assert_eq!(a, 1);
        assert_eq!(b, 2);
        assert_eq!(c, 3);
        assert_eq!(d, 4);
        assert_eq!(e, 5);
        assert_eq!(f, 6);
        assert_eq!(g, 7);
        assert_eq!(h, 8);
    }

    #[export_function]
    pub fn floattypes(a: f32, b: f64) {
        assert_eq!(a, 1.);
        assert_eq!(b, 2.);
    }

    #[export_function]
    pub fn strtypes(a: &str, b: String) {
        assert_eq!(a, "abc");
        assert_eq!(b, "def");
    }

    #[export_function]
    pub fn vectortypes(a: Vec::<i32>, b: Vec::<f64>) {
        assert_eq!(a, [1, 2, 3]);
        assert_eq!(b, [4., 5., 6.]);
    }

    #[export_function]
    pub fn robjtype(a: Robj) {
        assert_eq!(a, Robj::from(1))
    }

    #[export_function]
    pub fn return_u8() -> u8 {
        123
    }

    #[export_function]
    pub fn return_u16() -> u16 {
        123
    }

    #[export_function]
    pub fn return_u32() -> u32 {
        123
    }

    #[export_function]
    pub fn return_u64() -> u64 {
        123
    }

    #[export_function]
    pub fn return_i8() -> i8 {
        123
    }

    #[export_function]
    pub fn return_i16() -> i16 {
        123
    }

    #[export_function]
    pub fn return_i32() -> i32 {
        123
    }

    #[export_function]
    pub fn return_i64() -> i64 {
        123
    }

    #[export_function]
    pub fn return_f32() -> f32 {
        123.
    }

    #[export_function]
    pub fn return_f64() -> f64 {
        123.
    }

    #[test]
    fn export_test() {
        use super::*;
        // Call the exported functions through their generated C wrappers.
        unsafe {
            __wrap__inttypes(Robj::from(1).get(), Robj::from(2).get(), Robj::from(3).get(), Robj::from(4).get(), Robj::from(5).get(), Robj::from(6).get(), Robj::from(7).get(), Robj::from(8).get());
            __wrap__inttypes(Robj::from(1.).get(), Robj::from(2.).get(), Robj::from(3.).get(), Robj::from(4.).get(), Robj::from(5.).get(), Robj::from(6.).get(), Robj::from(7.).get(), Robj::from(8.).get());
            __wrap__floattypes(Robj::from(1.).get(), Robj::from(2.).get());
            __wrap__floattypes(Robj::from(1).get(), Robj::from(2).get());
            __wrap__strtypes(Robj::from("abc").get(), Robj::from("def").get());
            __wrap__vectortypes(Robj::from(&[1, 2, 3] as &[i32]).get(), Robj::from(&[4., 5., 6.] as &[f64]).get());
            __wrap__robjtype(Robj::from(1).get());
            assert_eq!(new_borrowed(__wrap__return_u8()), Robj::from(123));
            assert_eq!(new_borrowed(__wrap__return_u16()), Robj::from(123));
            assert_eq!(new_borrowed(__wrap__return_u32()), Robj::from(123));
            assert_eq!(new_borrowed(__wrap__return_u64()), Robj::from(123));
            assert_eq!(new_borrowed(__wrap__return_i8()), Robj::from(123));
            assert_eq!(new_borrowed(__wrap__return_i16()), Robj::from(123));
            assert_eq!(new_borrowed(__wrap__return_i32()), Robj::from(123));
            assert_eq!(new_borrowed(__wrap__return_i64()), Robj::from(123));
            assert_eq!(new_borrowed(__wrap__return_f32()), Robj::from(123.));
            assert_eq!(new_borrowed(__wrap__return_f64()), Robj::from(123.));
        }
    }
}

