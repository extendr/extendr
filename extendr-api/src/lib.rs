//!
//! A safe and user friendly R extension interface.
//!
//! * Build rust extensions to R.
//! * Convert R packages to Rust crates.
//!
//! This library aims to provide an interface that will be familiar to
//! first-time users of Rust or indeed any compiled language.
//!
//! See [`Robj`] for much of the content of this crate.
//! [`Robj`] provides a safe wrapper for the R object type.
//!
//! ## Examples
//!
//! Use attributes and macros to export to R.
//!
//! ```ignore
//! use extendr_api::prelude::*;
//! // Export a function or impl to R.
//! #[extendr]
//! fn fred(a: i32) -> i32 {
//!     a + 1
//! }
//!
//! // define exports using extendr_module
//! extendr_module! {
//!    mod mymodule;
//!    fn fred;    
//! }
//!
//! ```
//!
//! In R:
//!
//! ```ignore
//! result <- fred(1)
//! ```
//!
//! [Robj] is a wrapper for R objects.
//! The r!() and R!() macros let you build R objects
//! using Rust and R syntax respectively.
//! ```
//! use extendr_api::prelude::*;
//! test! {
//!     // An R object with a single string "hello"
//!     let character = r!("hello");
//!     let character = r!(["hello", "goodbye"]);
//!    
//!     // An R integer object with a single number 1L.
//!     // Note that in Rust, 1 is an integer and 1.0 is a real.
//!     let integer = r!(1);
//!    
//!     // An R real object with a single number 1.
//!     // Note that in R, 1 is a real and 1L is an integer.
//!     let real = r!(1.0);
//!    
//!     // An R real vector.
//!     let real_vector = r!([1.0, 2.0]);
//!     let real_vector = &[1.0, 2.0].iter().collect_robj();
//!     let real_vector = r!(vec![1.0, 2.0]);
//!    
//!     // An R function object.
//!     let function = R!("function(x, y) { x + y }")?;
//!    
//!     // A named list using the list! macro.
//!     let list = list!(a = 1, b = 2);
//!    
//!     // An unnamed list (of R objects) using the List wrapper.
//!     let list = r!(List::from_values(vec![1, 2, 3]));
//!     let list = r!(List::from_values(vec!["a", "b", "c"]));
//!     let list = r!(List::from_values(&[r!("a"), r!(1), r!(2.0)]));
//!
//!     // A symbol
//!     let sym = sym!(wombat);
//!
//!     // A R vector using collect_robj()
//!     let vector = (0..3).map(|x| x * 10).collect_robj();
//! }
//! ```
//!
//! In Rust, we prefer to use iterators rather than loops.
//!
//! ```
//! use extendr_api::prelude::*;
//! test! {
//!     // 1 ..= 100 is the same as 1:100
//!     let res = r!(1 ..= 100);
//!     assert_eq!(res, R!("1:100")?);
//!    
//!     // Rust arrays are zero-indexed so it is more common to use 0 .. 100.
//!     let res = r!(0 .. 100);
//!     assert_eq!(res.len(), 100);
//!    
//!     // Using map is a super fast way to generate vectors.
//!     let iter = (0..3).map(|i| format!("fred{}", i));
//!     let character = iter.collect_robj();
//!     assert_eq!(character, r!(["fred0", "fred1", "fred2"]));
//! }
//! ```
//!
//! To index a vector, first convert it to a slice and then
//! remember to use 0-based indexing. In Rust, going out of bounds
//! will cause and error (a panic) unlike C++ which may crash.
//! ```
//! use extendr_api::prelude::*;
//! test! {
//!     let vals = r!([1.0, 2.0]);
//!     let slice = vals.as_real_slice().ok_or("expected slice")?;
//!     let one = slice[0];
//!     let two = slice[1];
//!     // let error = slice[2];
//!     assert_eq!(one, 1.0);
//!     assert_eq!(two, 2.0);
//! }
//! ```
//!
//! Much slower, but more general are these methods:
//! ```
//! use extendr_api::prelude::*;
//! test! {
//!     let vals = r!([1.0, 2.0, 3.0]);
//!    
//!     // one-based indexing [[i]], returns an object.
//!     assert_eq!(vals.index(1)?, r!(1.0));
//!    
//!     // one-based slicing [x], returns an object.
//!     assert_eq!(vals.slice(1..=2)?, r!([1.0, 2.0]));
//!    
//!     // $ operator, returns an object
//!     let list = list!(a = 1.0, b = "xyz");
//!     assert_eq!(list.dollar("a")?, r!(1.0));
//! }
//! ```
//!
//! The [R!] macro lets you embed R code in Rust
//! and takes Rust expressions in {{ }} pairs.
//!
//! The [Rraw!] macro will not expand the {{ }} pairs.
//! ```
//! use extendr_api::prelude::*;
//! test! {
//!     // The text "1 + 1" is parsed as R source code.
//!     // The result is 1.0 + 1.0 in Rust.
//!     assert_eq!(R!("1 + 1")?, r!(2.0));
//!
//!     let a = 1.0;
//!     assert_eq!(R!("1 + {{a}}")?, r!(2.0));
//!
//!     assert_eq!(R!(r"
//!         x <- {{ a }}
//!         x + 1
//!     ")?, r!(2.0));
//!
//!     assert_eq!(R!(r#"
//!         x <- "hello"
//!         x
//!     "#)?, r!("hello"));
//!
//!     // Use the R meaning of {{ }} and do not expand.
//!     assert_eq!(Rraw!(r"
//!         x <- {{ 1 }}
//!         x + 1
//!     ")?, r!(2.0));
//! }
//! ```
//!
//! The [r!] macro converts a rust object to an R object
//! and takes parameters.
//! ```
//! use extendr_api::prelude::*;
//! test! {
//!     // The text "1.0+1.0" is parsed as Rust source code.
//!     let one = 1.0;
//!     assert_eq!(r!(one+1.0), r!(2.0));
//! }
//! ```
//!
//! You can call R functions and primitives using the [call!] macro.
//! ```
//! use extendr_api::prelude::*;
//! test! {
//!
//!     // As one R! macro call
//!     let confint1 = R!("confint(lm(weight ~ group - 1, PlantGrowth))")?;
//!    
//!     // As many parameterized calls.
//!     let formula = lang!("~", sym!(weight), lang!("-", sym!(group), 1.0)).set_class(["formula"])?;
//!     let plant_growth = global!(PlantGrowth)?;
//!     let model = call!("lm", formula, plant_growth)?;
//!     let confint2 = call!("confint", model)?;
//!    
//!     assert_eq!(confint1.as_real_vector(), confint2.as_real_vector());
//! }
//! ```
//!
//! Rust has a concept of "Owned" and "Borrowed" objects.
//!
//! Owned objects, such as [Vec] and [String] allocate memory
//! which is released when the object lifetime ends.
//!
//! Borrowed objects such as &[i32] and &str are just pointers
//! to annother object's memory and can't live longer than the
//! object they reference.
//!
//! Borrowed objects are much faster than owned objects and use less
//! memory but are used only for temporary access.
//!
//! When we take a slice of an R vector, for example, we need the
//! original R object to be alive or the data will be corrupted.
//!
//! ```
//! use extendr_api::prelude::*;
//! test! {
//!     // robj is an "Owned" object that controls the memory allocated.
//!     let robj = r!([1, 2, 3]);
//!    
//!     // Here slice is a "borrowed" reference to the bytes in robj.
//!     // and cannot live longer than robj.
//!     let slice = robj.as_integer_slice().ok_or("expected slice")?;
//!     assert_eq!(slice.len(), 3);
//! }
//! ```
//!
//! ## Writing tests
//!
//! To test the functions exposed to R, wrap your code in the [`test!`] macro.
//! This macro starts up the necessary R machinery for tests to work.
//!
//! ```no_run
//! use extendr_api::prelude::*;
//!
//! #[extendr]
//! fn things() ->  Strings {
//!     Strings::from_values(vec!["Test", "this"])
//! }
//!
//! // define exports using extendr_module
//! extendr_module! {
//!    mod mymodule;
//!    fn things;    
//! }
//!
//!
//! #[cfg(test)]
//! mod test {
//!     use super::*;
//!     use extendr_api::prelude::*;
//!
//!     #[test]
//!     fn test_simple_function() {
//!         assert_eq!(things().elt(0), "Test")
//!     }
//! }
//! ```
//!
//! ## Returning Result<T,E> to R
//!
//! Currently, `throw_r_error()` leaks memory because it jumps to R without properly dropping
//! some rust objects.
//!
//! The memory-safe way to do error handling with extendr is to return a Result<T, E>
//! to R. By default, any Err will trigger a panic! on the rust side which unwinds the stack.
//! The rust error trace will be printed to stderr, not R terminal. Any Ok value is returned
//! as is.
//!
//! Alternatively, two experimental non-leaking features, `result_list` and `result_condition`,
//! can be toggled to avoid panics on `Err`. Instead, an `Err` `x` is returned as either
//!  - list: `list(ok=NULL, err=x)` when `result_list` is enabled,
//!  - error condition: `<error: extendr_error>`, with `x` placed in `condition$value`, when `resultd_condition` is enabled.
//!
//! It is currently solely up to the user to handle any result on R side.
//!
//! The minimal overhead of calling an extendr function is in the ballpark of 2-4us.
//! Returning a condition or list increases the overhead to 4-8us. Checking & handling the result
//! on R side will likely increase overall overhead to 8-16us, depending on how efficiently the
//! result is handled.
//!
//! ```ignore
//! use extendr_api::prelude::*;
//! // simple function always returning an Err string
//! #[extendr]
//! fn oups(a: i32) -> std::result::Result<i32, String> {
//!     Err("I did it again".to_string())
//! }
//!
//! // define exports using extendr_module
//! extendr_module! {
//!    mod mymodule;
//!    fn oups;    
//! }
//!
//! ```
//!
//! In R:
//!
//! ```ignore
//! #default result_panic feature
//! oups(1)
//! > ... long panic traceback from rust printed to stderr
//!
//! #result_list feature
//! lst <-oups(1)
//! print(lst)
//! > list(ok=NULL, err="I did it again")
//!
//! #result_condition feature
//! cnd = oups(1)
//! print(cnd)
//! > <error: extendr_error>
//! print(cnd$value)
//! > "I did it again"
//!
//! #handling example for result_condition
//! oups_handled = function(a) {
//!   val_or_err = oups(1)  
//!   if(inherits(val_or_err,"extendr_error")) stop(val_or_err)
//!   val_or_err
//! }
//!
//! ```
//!
//! ## Feature gates
//!
//! extendr-api has some optional features behind these feature gates:
//!
//! - `ndarray`: provides the conversion between R's matrices and [ndarray](https://docs.rs/ndarray/latest/ndarray/).
//! - `num-complex`: provides the conversion between R's complex numbers and [num-complex](https://docs.rs/num-complex/latest/num_complex/).
//! - `serde`: provides the [Serde](https://serde.rs/) support.
//! - `graphics`: provides the functionality to control or implement graphics devices.
//!
//! extendr-api supports three ways of returning a Result<T,E> to R. Only one behavior feature can be enabled at a time.
//! - `result_panic`: Default behavior, return `Ok` as is, panic! on any `Err`
//!
//! Default behavior can be overridden by specifying `extend_api` features, i.e. `extendr-api = {..., default-features = false, features= ["result_condition"]}`
//! These features are experimental and are subject to change.
//! - `result_list`: return `Ok` as `list(ok=?, err=NULL)` or `Err` `list(ok=NULL, err=?)`
//! - `result_condition`: return `Ok` as is or `Err` as $value in an R error condition.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/extendr/extendr/master/extendr-logo-256.png"
)]

pub mod error;
pub mod functions;
pub mod io;
pub mod iter;
pub mod lang_macros;
pub mod metadata;
pub mod ownership;
pub mod prelude;
pub mod rmacros;

#[cfg(feature = "serde")]
pub mod serializer;

#[cfg(feature = "serde")]
pub mod deserializer;

#[cfg(feature = "graphics")]
pub mod graphics;

pub mod robj;
pub mod scalar;
pub mod thread_safety;
pub mod wrapper;

pub mod na;

pub mod optional;

pub use std::convert::{TryFrom, TryInto};
pub use std::ops::Deref;
pub use std::ops::DerefMut;

pub use robj::Robj;

//////////////////////////////////////////////////
// Note these pub use statements are deprecated
//
// `use extendr_api::prelude::*;`
//
// instead.

pub use error::*;
pub use functions::*;
pub use lang_macros::*;
pub use na::*;
pub use rmacros::*;
pub use robj::*;
pub use thread_safety::{
    catch_r_error, handle_panic, single_threaded, this_thread_id, throw_r_error,
};
pub use wrapper::*;

pub use extendr_macros::*;

use scalar::Rbool;

//////////////////////////////////////////////////

/// TRUE value eg. `r!(TRUE)`
pub const TRUE: Rbool = Rbool::true_value();

/// FALSE value eg. `r!(FALSE)`
pub const FALSE: Rbool = Rbool::false_value();

/// NULL value eg. `r!(NULL)`
pub const NULL: () = ();

/// NA value for integers eg. `r!(NA_INTEGER)`
pub const NA_INTEGER: Option<i32> = None;

/// NA value for real values eg. `r!(NA_REAL)`
pub const NA_REAL: Option<f64> = None;

/// NA value for strings. `r!(NA_STRING)`
pub const NA_STRING: Option<&str> = None;

/// NA value for logical. `r!(NA_LOGICAL)`
pub const NA_LOGICAL: Rbool = Rbool::na_value();

#[doc(hidden)]
pub use std::collections::HashMap;

#[doc(hidden)]
pub use libR_sys::DllInfo;

#[doc(hidden)]
pub use libR_sys::SEXP;

#[doc(hidden)]
use libR_sys::*;

#[doc(hidden)]
use std::ffi::CString;

pub use metadata::Metadata;

#[doc(hidden)]
pub struct CallMethod {
    pub call_symbol: std::ffi::CString,
    pub func_ptr: *const u8,
    pub num_args: i32,
}

unsafe fn make_method_def(
    cstrings: &mut Vec<std::ffi::CString>,
    rmethods: &mut Vec<libR_sys::R_CallMethodDef>,
    func: &metadata::Func,
    wrapped_name: &str,
) {
    cstrings.push(std::ffi::CString::new(wrapped_name).unwrap());
    rmethods.push(libR_sys::R_CallMethodDef {
        name: cstrings.last().unwrap().as_ptr(),
        fun: Some(std::mem::transmute(func.func_ptr)),
        numArgs: func.args.len() as i32,
    });
}

// Internal function used to implement the .Call interface.
// This is called from the code generated by the #[extendr] attribute.
#[doc(hidden)]
pub unsafe fn register_call_methods(info: *mut libR_sys::DllInfo, metadata: Metadata) {
    let mut rmethods = Vec::new();
    let mut cstrings = Vec::new();
    for func in metadata.functions {
        let wrapped_name = format!("wrap__{}", func.mod_name);
        make_method_def(&mut cstrings, &mut rmethods, &func, wrapped_name.as_str());
    }

    for imp in metadata.impls {
        for func in imp.methods {
            let wrapped_name = format!("wrap__{}__{}", imp.name, func.mod_name);
            make_method_def(&mut cstrings, &mut rmethods, &func, wrapped_name.as_str());
        }
    }

    rmethods.push(libR_sys::R_CallMethodDef {
        name: std::ptr::null(),
        fun: None,
        numArgs: 0,
    });

    libR_sys::R_registerRoutines(
        info,
        std::ptr::null(),
        rmethods.as_ptr(),
        std::ptr::null(),
        std::ptr::null(),
    );

    // This seems to allow both symbols and strings,
    libR_sys::R_useDynamicSymbols(info, 0);
    libR_sys::R_forceSymbols(info, 0);
}

/// Type of R objects used by [Robj::rtype].
#[derive(Debug, PartialEq)]
pub enum Rtype {
    Null,        // NILSXP
    Symbol,      // SYMSXP
    Pairlist,    // LISTSXP
    Function,    // CLOSXP
    Environment, // ENVSXP
    Promise,     // PROMSXP
    Language,    // LANGSXP
    Special,     // SPECIALSXP
    Builtin,     // BUILTINSXP
    Rstr,        // CHARSXP
    Logicals,    // LGLSXP
    Integers,    // INTSXP
    Doubles,     // REALSXP
    Complexes,   // CPLXSXP
    Strings,     // STRSXP
    Dot,         // DOTSXP
    Any,         // ANYSXP
    List,        // VECSXP
    Expressions, // EXPRSXP
    Bytecode,    // BCODESXP
    ExternalPtr, // EXTPTRSXP
    WeakRef,     // WEAKREFSXP
    Raw,         // RAWSXP
    S4,          // S4SXP
    Unknown,
}

/// Enum use to unpack R objects into their specialist wrappers.
// Todo: convert all Robj types to wrappers.
// Note: this only works if the wrappers are all just SEXPs.
#[derive(Debug, PartialEq)]
pub enum Rany<'a> {
    Null(&'a Robj),               // NILSXP
    Symbol(&'a Symbol),           // SYMSXP
    Pairlist(&'a Pairlist),       // LISTSXP
    Function(&'a Function),       // CLOSXP
    Environment(&'a Environment), // ENVSXP
    Promise(&'a Promise),         // PROMSXP
    Language(&'a Language),       // LANGSXP
    Special(&'a Primitive),       // SPECIALSXP
    Builtin(&'a Primitive),       // BUILTINSXP
    Rstr(&'a Rstr),               // CHARSXP
    Logicals(&'a Logicals),       // LGLSXP
    Integers(&'a Integers),       // INTSXP
    Doubles(&'a Doubles),         // REALSXP
    Complexes(&'a Complexes),     // CPLXSXP
    Strings(&'a Strings),         // STRSXP
    Dot(&'a Robj),                // DOTSXP
    Any(&'a Robj),                // ANYSXP
    List(&'a List),               // VECSXP
    Expressions(&'a Expressions), // EXPRSXP
    Bytecode(&'a Robj),           // BCODESXP
    ExternalPtr(&'a Robj),        // EXTPTRSXP
    WeakRef(&'a Robj),            // WEAKREFSXP
    Raw(&'a Raw),                 // RAWSXP
    S4(&'a S4),                   // S4SXP
    Unknown(&'a Robj),
}

/// Convert extendr's Rtype to R's SEXPTYPE.
/// Panics if the type is Unknown.
pub fn rtype_to_sxp(rtype: Rtype) -> i32 {
    use Rtype::*;
    (match rtype {
        Null => NILSXP,
        Symbol => SYMSXP,
        Pairlist => LISTSXP,
        Function => CLOSXP,
        Environment => ENVSXP,
        Promise => PROMSXP,
        Language => LANGSXP,
        Special => SPECIALSXP,
        Builtin => BUILTINSXP,
        Rstr => CHARSXP,
        Logicals => LGLSXP,
        Integers => INTSXP,
        Doubles => REALSXP,
        Complexes => CPLXSXP,
        Strings => STRSXP,
        Dot => DOTSXP,
        Any => ANYSXP,
        List => VECSXP,
        Expressions => EXPRSXP,
        Bytecode => BCODESXP,
        ExternalPtr => EXTPTRSXP,
        WeakRef => WEAKREFSXP,
        Raw => RAWSXP,
        S4 => S4SXP,
        Unknown => panic!("attempt to use Unknown Rtype"),
    }) as i32
}

/// Convert R's SEXPTYPE to extendr's Rtype.
pub fn sxp_to_rtype(sxptype: i32) -> Rtype {
    use Rtype::*;
    match sxptype as u32 {
        NILSXP => Null,
        SYMSXP => Symbol,
        LISTSXP => Pairlist,
        CLOSXP => Function,
        ENVSXP => Environment,
        PROMSXP => Promise,
        LANGSXP => Language,
        SPECIALSXP => Special,
        BUILTINSXP => Builtin,
        CHARSXP => Rstr,
        LGLSXP => Logicals,
        INTSXP => Integers,
        REALSXP => Doubles,
        CPLXSXP => Complexes,
        STRSXP => Strings,
        DOTSXP => Dot,
        ANYSXP => Any,
        VECSXP => List,
        EXPRSXP => Expressions,
        BCODESXP => Bytecode,
        EXTPTRSXP => ExternalPtr,
        WEAKREFSXP => WeakRef,
        RAWSXP => Raw,
        S4SXP => S4,
        _ => Unknown,
    }
}

const PRINTF_NO_FMT_CSTRING: &[i8] = &[37, 115, 0]; // same as "%s\0"
#[doc(hidden)]
pub fn print_r_output<T: Into<Vec<u8>>>(s: T) {
    let cs = CString::new(s).expect("NulError");
    unsafe {
        Rprintf(PRINTF_NO_FMT_CSTRING.as_ptr(), cs.as_ptr());
    }
}

#[doc(hidden)]
pub fn print_r_error<T: Into<Vec<u8>>>(s: T) {
    let cs = CString::new(s).expect("NulError");
    unsafe {
        REprintf(PRINTF_NO_FMT_CSTRING.as_ptr(), cs.as_ptr());
    }
}

#[cfg(test)]
mod tests;
