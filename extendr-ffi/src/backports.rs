//! Backport functions
//!
//! R's C API is changing and is stabilizing. As such some functions
//! that were available in previous versions of R are not available
//! in later versions of R, or they cause a warning in `R CMD check`.
//!
//! Use the functions in this module to ensure backwards compatibility.

// R 4.5 backports notes saved in wayback machine here:
// https://web.archive.org/web/20250325171443/https://rstudio.github.io/r-manuals/r-exts/The-R-API.html#moving-into-c-api-compliance

use crate::SEXP;
// In your extern "C" block:
extern "C" {
    #[cfg(not(r_4_5))]
    fn ENCLOS(x: SEXP) -> SEXP;

    #[cfg(r_4_5)]
    fn R_ParentEnv(x: SEXP) -> SEXP;

    #[cfg(not(r_4_5))]
    fn Rf_findVar(arg1: SEXP, arg2: SEXP) -> SEXP;

    #[cfg(r_4_5)]
    fn R_getVar(arg1: SEXP, arg2: SEXP) -> SEXP;

    #[cfg(not(r_4_5))]
    fn Rf_findVarInFrame(arg1: SEXP, arg2: SEXP) -> SEXP;

    #[cfg(r_4_5)]
    fn R_getVarEx(arg1: SEXP, arg2: SEXP) -> SEXP;

    #[cfg(not(r_4_5))]
    fn CLOENV(x: SEXP) -> SEXP;

    #[cfg(r_4_5)]
    fn R_ClosureEnv(x: SEXP) -> SEXP;

    #[cfg(not(r_4_5))]
    fn BODY(x: SEXP) -> SEXP;

    #[cfg(r_4_5)]
    fn R_ClosureBody(x: SEXP) -> SEXP;

    #[cfg(not(r_4_5))]
    fn FORMALS(x: SEXP) -> SEXP;

    #[cfg(r_4_5)]
    fn R_ClosureFormals(x: SEXP) -> SEXP;

    #[cfg(not(r_4_5))]
    fn DATAPTR(x: SEXP) -> *mut ::std::os::raw::c_void;

    #[cfg(r_4_5)]
    fn DATAPTR_RO(x: SEXP) -> *const ::std::os::raw::c_void;
}

/// Returns the enclosing environment of env, which will usually be of type ENVSXP, except for the special environment R_EmptyEnv, which terminates the environment chain; its enclosing environment is R_NilValue.
#[inline]
pub unsafe fn get_parent_env(x: SEXP) -> SEXP {
    #[cfg(not(r_4_5))]
    {
        ENCLOS(x)
    }
    #[cfg(r_4_5)]
    {
        R_ParentEnv(x)
    }
}

/// Returns a variable from an environment
#[inline]
pub unsafe fn get_var(symbol: SEXP, env: SEXP) -> SEXP {
    #[cfg(not(r_4_5))]
    {
        Rf_findVar(symbol, env)
    }
    #[cfg(r_4_5)]
    {
        R_getVar(symbol, env)
    }
}

/// Returns the value of the requested variable in an environment
#[inline]
pub unsafe fn get_var_in_frame(symbol: SEXP, env: SEXP) -> SEXP {
    #[cfg(not(r_4_5))]
    {
        Rf_findVarInFrame(symbol, env)
    }
    #[cfg(r_4_5)]
    {
        R_getVarEx(env, symbol)
    }
}

/// Return the environment of a closure
#[inline]
pub unsafe fn get_closure_env(x: SEXP) -> SEXP {
    #[cfg(not(r_4_5))]
    {
        CLOENV(x)
    }
    #[cfg(r_4_5)]
    {
        R_ClosureEnv(x)
    }
}

/// Return the body of a closure
#[inline]
pub unsafe fn get_closure_body(x: SEXP) -> SEXP {
    #[cfg(not(r_4_5))]
    {
        BODY(x)
    }
    #[cfg(r_4_5)]
    {
        R_ClosureBody(x)
    }
}

/// Access a closure's arguments
#[inline]
pub unsafe fn get_closure_formals(x: SEXP) -> SEXP {
    #[cfg(not(r_4_5))]
    {
        FORMALS(x)
    }
    #[cfg(r_4_5)]
    {
        R_ClosureFormals(x)
    }
}

/// Access a DATAPTR
#[inline]
pub unsafe fn dataptr(x: SEXP) -> *const ::std::os::raw::c_void {
    #[cfg(not(r_4_5))]
    {
        DATAPTR(x) as *const _
    }
    #[cfg(r_4_5)]
    {
        DATAPTR_RO(x)
    }
}
