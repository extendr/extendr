//! Backport functions
//!
//! R's C API is changing and is stabilizing. As such some functions
//! that were available in previous versions of R are not available
//! in later versions of R, or they cause a warning in `R CMD check`.
//!
//! Use the functions in this module to ensure backwards compatibility.

// R 4.5 backports notes saved in wayback machine here:
// https://web.archive.org/web/20250325171443/https://rstudio.github.io/r-manuals/r-exts/The-R-API.html#moving-into-c-api-compliance

use crate::{Rboolean, SEXP};

#[cfg(r_4_5)]
use crate::{R_NilValue, Rf_setAttrib, CAR, CDR, TAG};

#[cfg(not(r_4_5))]
use crate::R_UnboundValue;

extern "C" {
    #[cfg(not(r_4_5))]
    fn ENCLOS(x: SEXP) -> SEXP;

    #[cfg(r_4_5)]
    fn R_ParentEnv(x: SEXP) -> SEXP;

    #[cfg(not(r_4_5))]
    fn Rf_findVar(arg1: SEXP, arg2: SEXP) -> SEXP;

    #[cfg(r_4_5)]
    fn R_getVar(sym: SEXP, rho: SEXP, inherits: Rboolean) -> SEXP;

    #[cfg(not(r_4_5))]
    fn Rf_findVarInFrame(arg1: SEXP, arg2: SEXP) -> SEXP;

    #[cfg(r_4_5)]
    fn R_getVarEx(sym: SEXP, rho: SEXP, inherits: Rboolean, ifnotfound: SEXP) -> SEXP;

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

    #[cfg(not(r_4_5))]
    fn Rf_isFrame(x: SEXP) -> Rboolean;

    #[cfg(r_4_5)]
    fn Rf_isDataFrame(x: SEXP) -> Rboolean;

    #[cfg(not(r_4_5))]
    fn SET_ATTRIB(x: SEXP, v: SEXP);

    #[cfg(r_4_5)]
    fn CLEAR_ATTRIB(x: SEXP);
}

/// Returns the enclosing environment of env, which will usually be of type ENVSXP, except for the special environment R_EmptyEnv, which terminates the environment chain; its enclosing environment is R_NilValue.
///
/// # Safety
///
/// This function dereferences a raw SEXP pointer.
/// The caller must ensure that `x` is a valid SEXP pointer to an environment.
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
///
/// # Safety
///
/// This function dereferences raw SEXP pointers.
/// The caller must ensure that `symbol` and `env` are valid SEXP pointers.
#[inline]
pub unsafe fn get_var(symbol: SEXP, env: SEXP) -> SEXP {
    #[cfg(not(r_4_5))]
    {
        Rf_findVar(symbol, env)
    }
    #[cfg(r_4_5)]
    {
        R_getVar(symbol, env, Rboolean::TRUE)
    }
}

/// Returns a variable from an environment, or `None` if unbound.
///
/// On R < 4.5, uses `Rf_findVar` and checks against `R_UnboundValue`.
/// On R >= 4.5, uses `R_getVar` which signals an error if not found;
/// callers should wrap this with `catch_r_error` if needed.
///
/// # Safety
///
/// This function dereferences raw SEXP pointers.
/// The caller must ensure that `symbol` and `env` are valid SEXP pointers.
#[inline]
pub unsafe fn get_var_safe(symbol: SEXP, env: SEXP) -> Option<SEXP> {
    #[cfg(not(r_4_5))]
    {
        let var = Rf_findVar(symbol, env);
        if var == R_UnboundValue {
            None
        } else {
            Some(var)
        }
    }
    #[cfg(r_4_5)]
    {
        Some(R_getVar(symbol, env, Rboolean::TRUE))
    }
}

/// Returns the value of the requested variable in an environment
///
/// # Safety
///
/// This function dereferences raw SEXP pointers.
/// The caller must ensure that `env` and `symbol` are valid SEXP pointers.
#[inline]
pub unsafe fn get_var_in_frame(env: SEXP, symbol: SEXP) -> SEXP {
    #[cfg(not(r_4_5))]
    {
        Rf_findVarInFrame(env, symbol)
    }
    #[cfg(r_4_5)]
    {
        R_getVarEx(symbol, env, Rboolean::FALSE, R_NilValue)
    }
}

/// Return the environment of a closure
///
/// # Safety
///
/// This function dereferences a raw SEXP pointer.
/// The caller must ensure that `x` is a valid SEXP pointer to a closure.
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
///
/// # Safety
///
/// This function dereferences a raw SEXP pointer.
/// The caller must ensure that `x` is a valid SEXP pointer to a closure.
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
///
/// # Safety
///
/// This function dereferences a raw SEXP pointer.
/// The caller must ensure that `x` is a valid SEXP pointer to a closure.
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
///
/// # Safety
///
/// This function dereferences a raw SEXP pointer.
/// The caller must ensure that `x` is a valid SEXP pointer.
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

/// Check is data.frame
///
/// # Safety
///
/// This function dereferences a raw SEXP pointer.
/// The caller must ensure that `x` is a valid SEXP pointer.
#[inline]
pub unsafe fn is_data_frame(x: SEXP) -> Rboolean {
    #[cfg(not(r_4_5))]
    {
        Rf_isFrame(x)
    }
    #[cfg(r_4_5)]
    {
        Rf_isDataFrame(x)
    }
}

/// Replace all attributes of `x` with the pairlist `attributes`
///
/// On R >= 4.5, where `SET_ATTRIB` is no longer part of the API, this clears
/// the attributes with `CLEAR_ATTRIB` and sets each one with `Rf_setAttrib`.
///
/// # Safety
///
/// This function dereferences raw SEXP pointers.
/// The caller must ensure that `x` is a valid SEXP pointer and that `attributes`
/// is a valid pairlist or `R_NilValue`, and that both are protected.
#[inline]
pub unsafe fn set_attrib(x: SEXP, attributes: SEXP) {
    #[cfg(not(r_4_5))]
    {
        SET_ATTRIB(x, attributes)
    }
    #[cfg(r_4_5)]
    {
        CLEAR_ATTRIB(x);
        let mut node = attributes;
        while node != R_NilValue {
            Rf_setAttrib(x, TAG(node), CAR(node));
            node = CDR(node);
        }
    }
}
