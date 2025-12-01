//! Embeds a a single R process
//!
//! Using R's C-API requires the embedding of the R runtime.
//! Thus, when using bindings provided by `extendr-ffi`, it is necessary that
//! either an R process is the caller, or that the process instantiates
//! an accompanying R process. Otherwise, a run-time error occurs e.g.
//! `(signal: 11, SIGSEGV: invalid memory reference)` or
//!
//! ```text
//! Caused by:
//! process didn't exit successfully: `/extendr/tests/extendrtest/target/debug/deps/extendrtest-59155c3c146ae614` (signal: 11, SIGSEGV: invalid memory reference)
//! ```
//!
//! ## Testing
//!
//! Within tests, one must use [`test!`] or [`with_r`] as a wrapper around
//! code that uses the R runtime, e.g.
//!
//! ```no_run
//! #[test]
//! fn testing_r_code() {
//!     with_r(|| {
//!
//!     });
//! }
//! ```
//!
//! Similarly with `test!` that is available in `extendr_api`, one may
//!
//! ```no_run
//! #[test]
//! fn testing_r_code() {
//!     test! {
//!
//!     };
//! }
//! ```
//!
//! The advantage of `test!` is that it allows the use of `?` in test code, while
//! `with_r` is not macro-based, thus code formatter `rustfmt` and rust LSPs (Rust Analyzer, Rust Rover, etc.)
//! works within `with_r` without any problems.
//!
//!
//! ## Binaries
//!
//! In a binary program, one may use [`start_r`] directly in the `main`-function.
//!
//! There is no `end_r`, as we terminate the R process setup, when the parent
//! process terminates.
//!
//! [`test!`]: https://docs.rs/extendr-api/latest/extendr_api/macro.test.html
//!
// # Internal documentation
//
// ## Background
//
//
// See [Rembedded.c](https://github.com/wch/r-source/blob/trunk/src/unix/Rembedded.c).
//
// [Rinside](https://github.com/eddelbuettel/rinside)
//
//

use extendr_ffi::{
    setup_Rmainloop, R_CStackLimit, R_CleanTempDir, R_RunExitFinalizers, Rf_initialize_R,
};
use std::os::raw;
use std::sync::Once;

// Generate mutable static strings.
// Much more efficient than `CString`.
// Generates asciiz.
macro_rules! cstr_mut {
    ($s: expr) => {
        concat!($s, "\0").as_ptr().cast::<raw::c_char>().cast_mut()
    };
}

#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut R_ARGV: [*mut raw::c_char; 6] = [
    cstr_mut!("R"),
    cstr_mut!("--arch=i386"),
    cstr_mut!("--slave"),
    cstr_mut!("--no-save"),
    cstr_mut!("--vanilla"),
    std::ptr::null_mut(),
];

#[cfg(not(all(target_os = "windows", target_arch = "x86")))]
static mut R_ARGV: [*mut raw::c_char; 5] = [
    cstr_mut!("R"),
    cstr_mut!("--slave"),
    cstr_mut!("--no-save"),
    cstr_mut!("--vanilla"),
    std::ptr::null_mut(),
];

static START_R: Once = Once::new();

#[allow(static_mut_refs)]
pub fn start_r() {
    START_R.call_once(|| {
        unsafe {
            if std::env::var("R_HOME").is_err() {
                // env! gets the build-time R_HOME stored by extendr-ffi
                std::env::set_var("R_HOME", env!("R_HOME"));
            }

            // Due to Rf_initEmbeddedR using __libc_stack_end
            // We can't call Rf_initEmbeddedR.
            // Instead we must follow rustr's example and call the parts.

            //let res = unsafe { Rf_initEmbeddedR(1, args.as_mut_ptr()) };
            // NOTE: R will crash if this is called twice in the same process.
            #[cfg(all(target_os = "windows", target_arch = "x86"))]
            {
                Rf_initialize_R(5, R_ARGV.as_mut_ptr());
            }

            #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
            {
                Rf_initialize_R(4, R_ARGV.as_mut_ptr());
            }

            // In case you are curious.
            // Maybe 8MB is a bit small.
            // eprintln!("R_CStackLimit={:016x}", R_CStackLimit);
            R_CStackLimit = usize::MAX;

            setup_Rmainloop();
        }
    });
}

/// Close down the R interpreter. Note you won't be able to
/// Restart it, so use with care or not at all.
fn end_r() {
    unsafe {
        //Rf_endEmbeddedR(0);
        R_RunExitFinalizers();
        //CleanEd();
        R_CleanTempDir();
    }
}

/// Ensures that an embedded R instance is present when evaluating
/// `f`.
pub fn with_r<T, E>(f: impl FnOnce() -> std::result::Result<T, E>) -> std::result::Result<T, E> {
    start_r();
    f()
    // For compatibility with `test!` in `extendr-api/src/rmacros.rs`, there
    // is no `end_r()` call here.
}

#[ctor::dtor]
fn shutdown_r() {
    if START_R.is_completed() {
        end_r();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine() {
        // If this is the first call, it should wake up the interpreter.
        start_r();

        // This should do nothing.
        start_r();

        // Ending the interpreter is bad if we are running multiple threads.
        // So avoid doing this in tests.
        //end_r();
    }
}
