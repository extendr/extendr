//! A singleton instance of the R interpreter.
//!
//! Only call this from `main()` if you want to run stand-alone.
//!
//! Its principal use is for testing.
//!
//! See [Rembedded.c](https://github.com/wch/r-source/blob/trunk/src/unix/Rembedded.c).
//!

use libR_sys::*;
use std::os::raw;
use std::sync::Once;

// Generate mutable static strings.
// Much more efficient than `CString`.
// Generates asciiz.
macro_rules! cstr_mut {
    ($s: expr) => {
        concat!($s, "\0").as_ptr() as *mut raw::c_char
    };
}

static START_R: Once = Once::new();

pub fn start_r() {
    START_R.call_once(|| {
        unsafe {
            if std::env::var("R_HOME").is_err() {
                // env! gets the build-time R_HOME stored by libR-sys
                std::env::set_var("R_HOME", env!("R_HOME"));
            }

            // Due to Rf_initEmbeddedR using __libc_stack_end
            // We can't call Rf_initEmbeddedR.
            // Instead we must follow rustr's example and call the parts.

            //let res = unsafe { Rf_initEmbeddedR(1, args.as_mut_ptr()) };
            // NOTE: R will crash if this is called twice in the same process.
            Rf_initialize_R(
                3,
                [cstr_mut!("R"), cstr_mut!("--slave"), cstr_mut!("--no-save")].as_mut_ptr(),
            );

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
pub fn end_r() {
    unsafe {
        //Rf_endEmbeddedR(0);
        R_RunExitFinalizers();
        //CleanEd();
        R_CleanTempDir();
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
