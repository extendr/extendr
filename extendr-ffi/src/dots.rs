use crate::{Rboolean, SEXP};

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum R_DotType_t {
    R_DotTypeValue = 0,
    R_DotTypeMissing = 1,
    R_DotTypeDelayed = 2,
    R_DotTypeForced = 3,
}

extern "C" {
    // R_DotType_t R_GetDotType(int i, SEXP env);
    pub fn R_GetDotType(i: ::std::os::raw::c_int, env: SEXP) -> R_DotType_t;

    // SEXP R_DotsNames(SEXP env);
    pub fn R_DotsNames(env: SEXP) -> SEXP;

    // int R_DotsLength(SEXP env);
    pub fn R_DotsLength(env: SEXP) -> ::std::os::raw::c_int;

    // Rboolean R_DotsExist(SEXP env);
    pub fn R_DotsExist(env: SEXP) -> Rboolean;

    // SEXP R_findDotsEnv(SEXP env);
    pub fn R_findDotsEnv(env: SEXP) -> SEXP;

    // SEXP R_DotsElt(int i, SEXP env);
    pub fn R_DotsElt(i: ::std::os::raw::c_int, env: SEXP) -> SEXP;

    // SEXP R_DotForcedExpression(int i, SEXP env);
    pub fn R_DotForcedExpression(i: ::std::os::raw::c_int, env: SEXP) -> SEXP;

    // SEXP R_DotDelayedExpression(int i, SEXP env);
    pub fn R_DotDelayedExpression(i: ::std::os::raw::c_int, env: SEXP) -> SEXP;

    // SEXP R_DotDelayedEnvironment(int i, SEXP env);
    pub fn R_DotDelayedEnvironment(i: ::std::os::raw::c_int, env: SEXP) -> SEXP;
}
