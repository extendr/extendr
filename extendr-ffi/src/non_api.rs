// Functions used by the non-api feature of extendr
use crate::{Rboolean, SEXP, SEXPTYPE};
extern "C" {
    #[doc = "Unbound marker"]
    pub static R_UnboundValue: SEXP;
    pub fn Rf_isValidString(arg1: SEXP) -> Rboolean;
    pub fn Rf_isValidStringF(arg1: SEXP) -> Rboolean;
    pub fn SYMVALUE(x: SEXP) -> SEXP;
    pub fn PRSEEN(x: SEXP) -> ::std::os::raw::c_int;
    pub fn SET_ENCLOS(x: SEXP, v: SEXP);
    pub fn ENVFLAGS(x: SEXP) -> ::std::os::raw::c_int;
    pub fn SET_ENVFLAGS(x: SEXP, v: ::std::os::raw::c_int);
    pub fn ENCLOS(x: SEXP) -> SEXP;
    pub fn HASHTAB(x: SEXP) -> SEXP;
    pub fn FRAME(x: SEXP) -> SEXP;
    pub fn Rf_allocSExp(arg1: SEXPTYPE) -> SEXP;
    pub fn SET_FORMALS(x: SEXP, v: SEXP);
    pub fn SET_BODY(x: SEXP, v: SEXP);
    pub fn SET_CLOENV(x: SEXP, v: SEXP);
    pub fn SET_PRCODE(x: SEXP, v: SEXP);
    pub fn SET_PRENV(x: SEXP, v: SEXP);
    pub fn SET_PRVALUE(x: SEXP, v: SEXP);
    #[doc = "Promise Access Functions"]
    pub fn PRCODE(x: SEXP) -> SEXP;
    pub fn PRENV(x: SEXP) -> SEXP;
    pub fn PRVALUE(x: SEXP) -> SEXP;
}
