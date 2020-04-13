//! A sigleton instance of the R interpreter.
//! 
//! Only call this from main() if you want to run stand-alone.
//! 
//! Its principal use is for testing. 
//! 
//! See https://github.com/wch/r-source/blob/trunk/src/unix/Rembedded.c


use libR_sys::*;
use std::os::raw;

// Generate mutable static strings.
// Much more efficient than CString.
// Generates asciiz.
macro_rules! cstr_mut {
    ($s: expr) => {
        concat!($s, "\0").as_ptr() as *mut raw::c_char
    };
}

static mut STARTED : bool = false;

pub fn start_r() {
    unsafe {
        if STARTED { return; }
        STARTED = true;

        // TODO: get the default home dir from libR-sys.
        if cfg!(unix) {
            if std::env::var("R_HOME").is_err() {
                // env! gets the build-time R_HOME made in build.rs
                std::env::set_var("R_HOME", "/usr/lib/R");
            }
        }

        // Due to Rf_initEmbeddedR using __libc_stack_end
        // We can't call Rf_initEmbeddedR.
        // Instead we must follow rustr's example and call the parts.

        //let res = unsafe { Rf_initEmbeddedR(1, args.as_mut_ptr()) };
        // NOTE: R will crash if this is called twice in the same process.
        Rf_initialize_R(1,
            [
                cstr_mut!("R"),
                cstr_mut!("--slave"),
                cstr_mut!("--no-restore"),
            ].as_mut_ptr());

        // In case you are curious.
        // Maybe 8MB is a bit small.
        // eprintln!("R_CStackLimit={:016x}", R_CStackLimit);

        R_CStackLimit = usize::max_value();

        setup_Rmainloop();
    }
}

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
        start_r();
        //end_r();
    }
}
