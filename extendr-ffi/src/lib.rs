//! Low level bindings to R's C API
//!
//! `extendr-ffi` powers the `extendr-api` library.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

pub mod graphics;
pub use graphics::*;
pub mod altrep;
pub use altrep::*;
mod symbols;
pub use symbols::*;
pub mod backports;
pub use backports::*;

#[cfg(feature = "non-api")]
mod non_api;
#[cfg(feature = "non-api")]
pub use non_api::*;

#[non_exhaustive]
#[repr(transparent)]
#[derive(Debug)]
pub struct SEXPREC(std::ffi::c_void);

extern "C" {
    /// Returns the [SEXPTYPE] of `x`
    pub fn TYPEOF(x: SEXP) -> SEXPTYPE;
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
/// A C type with enumeration constants FALSE and TRUE defined.
pub enum Rboolean {
    #[doc = ", MAYBE"]
    FALSE = 0,
    #[doc = ", MAYBE"]
    TRUE = 1,
}

#[doc = "These are very similar to those in Rdynpriv.h,\nbut we maintain them separately to give us more freedom to do\nsome computations on the internal versions that are derived from\nthese definitions."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct R_CMethodDef {
    pub name: *const ::std::os::raw::c_char,
    pub fun: DL_FUNC,
    pub numArgs: ::std::os::raw::c_int,
    pub types: *mut R_NativePrimitiveArgType,
}
pub type R_NativePrimitiveArgType = ::std::os::raw::c_uint;
pub type R_ExternalMethodDef = R_CallMethodDef;
pub type R_FortranMethodDef = R_CMethodDef;

pub type Rbyte = ::std::os::raw::c_uchar;
#[doc = "type for length of (standard, not long) vectors etc"]
pub type R_len_t = ::std::os::raw::c_int;
#[repr(u32)]
/// Enum of possible SEXP types
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SEXPTYPE {
    #[doc = "nil = NULL"]
    NILSXP = 0,
    #[doc = "symbols"]
    SYMSXP = 1,
    #[doc = "lists of dotted pairs"]
    LISTSXP = 2,
    #[doc = "closures"]
    CLOSXP = 3,
    #[doc = "environments"]
    ENVSXP = 4,
    #[doc = "promises: \\[un\\]evaluated closure arguments"]
    PROMSXP = 5,
    #[doc = "language constructs (special lists)"]
    LANGSXP = 6,
    #[doc = "special forms"]
    SPECIALSXP = 7,
    #[doc = "builtin non-special forms"]
    BUILTINSXP = 8,
    #[doc = "\"scalar\" string type (internal only)"]
    CHARSXP = 9,
    #[doc = "logical vectors"]
    LGLSXP = 10,
    #[doc = "integer vectors"]
    INTSXP = 13,
    #[doc = "real variables"]
    REALSXP = 14,
    #[doc = "complex variables"]
    CPLXSXP = 15,
    #[doc = "string vectors"]
    STRSXP = 16,
    #[doc = "dot-dot-dot object"]
    DOTSXP = 17,
    #[doc = "make \"any\" args work"]
    ANYSXP = 18,
    #[doc = "generic vectors"]
    VECSXP = 19,
    #[doc = "expressions vectors"]
    EXPRSXP = 20,
    #[doc = "byte code"]
    BCODESXP = 21,
    #[doc = "external pointer"]
    EXTPTRSXP = 22,
    #[doc = "weak reference"]
    WEAKREFSXP = 23,
    #[doc = "raw bytes"]
    RAWSXP = 24,
    #[cfg(not(r_4_4))]
    S4SXP = 25,
    #[cfg(r_4_4)]
    #[doc = "S4 non-vector"]
    OBJSXP = 25,
    #[doc = "fresh node created in new page"]
    NEWSXP = 30,
    #[doc = "node released by GC"]
    FREESXP = 31,
    #[doc = "Closure or Builtin"]
    FUNSXP = 99,
}
pub type SEXP = *mut SEXPREC;

#[repr(u32)]
#[doc = "cetype_t is an identifier reseved by POSIX, but it is\nwell established as public.  Could remap by a #define though"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cetype_t {
    CE_NATIVE = 0,
    CE_UTF8 = 1,
    CE_LATIN1 = 2,
    CE_BYTES = 3,
    CE_SYMBOL = 5,
    CE_ANY = 99,
}
#[doc = "Finalization interface"]
pub type R_CFinalizer_t = ::std::option::Option<unsafe extern "C" fn(arg1: SEXP)>;
pub type Int32 = ::std::os::raw::c_uint;
#[doc = "R 4.3 redefined `Rcomplex` to a union for compatibility with Fortran.\n But the old definition is compatible both the union version\n and the struct version.\n See: <https://github.com/extendr/extendr/issues/524>\n <div rustbindgen replaces=\"Rcomplex\"></div>"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Rcomplex {
    pub r: f64,
    pub i: f64,
}

#[doc = "R_xlen_t is defined as int on 32-bit platforms, and\n that confuses Rust. Keeping it always as ptrdiff_t works\n fine even on 32-bit.\n <div rustbindgen replaces=\"R_xlen_t\"></div>"]
pub type R_xlen_t = isize;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _DllInfo {
    _unused: [u8; 0],
}
pub type DllInfo = _DllInfo;

#[repr(u32)]
#[doc = "PARSE_NULL will not be returned by R_ParseVector"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ParseStatus {
    PARSE_NULL = 0,
    PARSE_OK = 1,
    PARSE_INCOMPLETE = 2,
    PARSE_ERROR = 3,
    PARSE_EOF = 4,
}

#[doc = "Called with a variable argument set after casting to a compatible\nfunction pointer."]
pub type DL_FUNC = ::std::option::Option<unsafe extern "C" fn() -> *mut ::std::os::raw::c_void>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct R_CallMethodDef {
    pub name: *const ::std::os::raw::c_char,
    pub fun: DL_FUNC,
    pub numArgs: ::std::os::raw::c_int,
}

pub type R_outpstream_t = *mut R_outpstream_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct R_outpstream_st {
    pub data: R_pstream_data_t,
    pub type_: R_pstream_format_t,
    pub version: ::std::os::raw::c_int,
    pub OutChar: ::std::option::Option<
        unsafe extern "C" fn(arg1: R_outpstream_t, arg2: ::std::os::raw::c_int),
    >,
    pub OutBytes: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: R_outpstream_t,
            arg2: *mut ::std::os::raw::c_void,
            arg3: ::std::os::raw::c_int,
        ),
    >,
    pub OutPersistHookFunc:
        ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: SEXP) -> SEXP>,
    pub OutPersistHookData: SEXP,
}

pub type R_pstream_data_t = *mut ::std::os::raw::c_void;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum R_pstream_format_t {
    R_pstream_any_format = 0,
    R_pstream_ascii_format = 1,
    R_pstream_binary_format = 2,
    R_pstream_xdr_format = 3,
    R_pstream_asciihex_format = 4,
}
pub type R_inpstream_t = *mut R_inpstream_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct R_inpstream_st {
    pub data: R_pstream_data_t,
    pub type_: R_pstream_format_t,
    pub InChar:
        ::std::option::Option<unsafe extern "C" fn(arg1: R_inpstream_t) -> ::std::os::raw::c_int>,
    pub InBytes: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: R_inpstream_t,
            arg2: *mut ::std::os::raw::c_void,
            arg3: ::std::os::raw::c_int,
        ),
    >,
    pub InPersistHookFunc:
        ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: SEXP) -> SEXP>,
    pub InPersistHookData: SEXP,
    pub native_encoding: [::std::os::raw::c_char; 64usize],
    pub nat2nat_obj: *mut ::std::os::raw::c_void,
    pub nat2utf8_obj: *mut ::std::os::raw::c_void,
}

extern "C" {
    #[doc = "IEEE NaN"]
    pub static R_NaN: f64;
    #[doc = "IEEE Inf"]
    pub static R_PosInf: f64;
    #[doc = "IEEE -Inf"]
    pub static R_NegInf: f64;
    #[doc = "NA_REAL: IEEE"]
    pub static R_NaReal: f64;
    #[doc = "NA_INTEGER:= INT_MIN currently"]
    pub static R_NaInt: ::std::os::raw::c_int;
    #[doc = "NA_STRING is a SEXP, so defined in Rinternals.h"]
    pub fn R_IsNA(arg1: f64) -> ::std::os::raw::c_int;
    pub fn R_IsNaN(arg1: f64) -> ::std::os::raw::c_int;
    pub fn R_finite(arg1: f64) -> ::std::os::raw::c_int;
    pub fn Rprintf(arg1: *const ::std::os::raw::c_char, ...);
    pub fn REprintf(arg1: *const ::std::os::raw::c_char, ...);
    pub fn CAR(e: SEXP) -> SEXP;
    pub fn CDR(e: SEXP) -> SEXP;
    pub fn COMPLEX(x: SEXP) -> *mut Rcomplex;
    pub fn COMPLEX_GET_REGION(sx: SEXP, i: R_xlen_t, n: R_xlen_t, buf: *mut Rcomplex) -> R_xlen_t;
    pub fn DATAPTR(x: SEXP) -> *mut ::std::os::raw::c_void;
    pub fn DATAPTR_RO(x: SEXP) -> *const ::std::os::raw::c_void;
    pub fn DATAPTR_OR_NULL(x: SEXP) -> *const ::std::os::raw::c_void;
    pub fn GetRNGstate();
    pub fn PutRNGstate();
    pub fn INTEGER(x: SEXP) -> *mut ::std::os::raw::c_int;
    pub fn LOGICAL(x: SEXP) -> *mut ::std::os::raw::c_int;
    pub fn LENGTH(x: SEXP) -> ::std::os::raw::c_int;
    pub fn XLENGTH(x: SEXP) -> R_xlen_t;
    pub fn RAW(x: SEXP) -> *mut Rbyte;
    pub fn REAL(x: SEXP) -> *mut f64;
    pub fn INTEGER_GET_REGION(
        sx: SEXP,
        i: R_xlen_t,
        n: R_xlen_t,
        buf: *mut ::std::os::raw::c_int,
    ) -> R_xlen_t;
    pub fn INTEGER_IS_SORTED(x: SEXP) -> ::std::os::raw::c_int;
    pub fn INTEGER_NO_NA(x: SEXP) -> ::std::os::raw::c_int;
    pub fn REAL_IS_SORTED(x: SEXP) -> ::std::os::raw::c_int;
    pub fn REAL_NO_NA(x: SEXP) -> ::std::os::raw::c_int;
    pub fn STRING_IS_SORTED(x: SEXP) -> ::std::os::raw::c_int;
    pub fn STRING_NO_NA(x: SEXP) -> ::std::os::raw::c_int;
    pub fn LOGICAL_GET_REGION(
        sx: SEXP,
        i: R_xlen_t,
        n: R_xlen_t,
        buf: *mut ::std::os::raw::c_int,
    ) -> R_xlen_t;
    pub fn MARK_NOT_MUTABLE(x: SEXP);
    pub fn PRINTNAME(x: SEXP) -> SEXP;
    #[doc = "The nil object"]
    pub static R_NilValue: SEXP;
    #[doc = "The base environment; formerly R_NilValue"]
    pub static R_BaseEnv: SEXP;
    #[doc = "The (fake) namespace for base"]
    pub static R_BaseNamespace: SEXP;
    #[doc = "NA_STRING as a CHARSXP"]
    pub static R_NaString: SEXP;
    #[doc = "srcref related functions"]
    pub fn R_CHAR(x: SEXP) -> *const ::std::os::raw::c_char;
    pub fn R_CleanTempDir();
    #[doc = "External pointer interface"]
    pub fn R_MakeExternalPtr(p: *mut ::std::os::raw::c_void, tag: SEXP, prot: SEXP) -> SEXP;
    pub fn R_ExternalPtrAddr(s: SEXP) -> *mut ::std::os::raw::c_void;
    pub fn R_ExternalPtrTag(s: SEXP) -> SEXP;
    pub fn R_ExternalPtrProtected(s: SEXP) -> SEXP;
    pub fn R_ClearExternalPtr(s: SEXP);
    pub fn R_SetExternalPtrAddr(s: SEXP, p: *mut ::std::os::raw::c_void);
    pub fn R_SetExternalPtrTag(s: SEXP, tag: SEXP);
    pub fn R_SetExternalPtrProtected(s: SEXP, p: SEXP);
    pub fn R_compute_identical(arg1: SEXP, arg2: SEXP, arg3: ::std::os::raw::c_int) -> Rboolean;
    #[doc = "C stack limit"]
    pub static mut R_CStackLimit: usize;
    pub fn R_do_slot(obj: SEXP, name: SEXP) -> SEXP;
    pub fn R_do_slot_assign(obj: SEXP, name: SEXP, value: SEXP) -> SEXP;
    #[doc = "An empty environment at the root of the\nenvironment tree"]
    pub static R_EmptyEnv: SEXP;
    pub fn R_forceSymbols(info: *mut DllInfo, value: Rboolean) -> Rboolean;
    pub fn R_GetCurrentEnv() -> SEXP;
    #[doc = "srcref related functions"]
    pub fn R_GetCurrentSrcref(arg1: ::std::os::raw::c_int) -> SEXP;
    pub fn R_GetSrcFilename(arg1: SEXP) -> SEXP;
    #[doc = "The \"global\" environment"]
    pub static R_GlobalEnv: SEXP;
    pub fn R_has_slot(obj: SEXP, name: SEXP) -> ::std::os::raw::c_int;
    pub fn R_IsNamespaceEnv(rho: SEXP) -> Rboolean;
    pub fn R_IsPackageEnv(rho: SEXP) -> Rboolean;
    pub fn R_MakeUnwindCont() -> SEXP;
    #[doc = "Missing argument marker"]
    pub static R_MissingArg: SEXP;
    pub fn R_NamespaceEnvSpec(rho: SEXP) -> SEXP;
    #[doc = "Registry for registered namespaces"]
    pub static R_NamespaceRegistry: SEXP;
    #[doc = "Environment and Binding Features"]
    pub fn R_NewEnv(arg1: SEXP, arg2: ::std::os::raw::c_int, arg3: ::std::os::raw::c_int) -> SEXP;
    pub fn R_PackageEnvName(rho: SEXP) -> SEXP;
    pub fn R_ParseVector(
        arg1: SEXP,
        arg2: ::std::os::raw::c_int,
        arg3: *mut ParseStatus,
        arg4: SEXP,
    ) -> SEXP;
    #[doc = "preserve objects across GCs"]
    pub fn R_PreserveObject(arg1: SEXP);
    pub fn R_RegisterCFinalizerEx(s: SEXP, fun: R_CFinalizer_t, onexit: Rboolean);
    pub fn R_registerRoutines(
        info: *mut DllInfo,
        croutines: *const R_CMethodDef,
        callRoutines: *const R_CallMethodDef,
        fortranRoutines: *const R_FortranMethodDef,
        externalRoutines: *const R_ExternalMethodDef,
    ) -> ::std::os::raw::c_int;
    pub fn R_ReleaseObject(arg1: SEXP);
    pub fn R_RunExitFinalizers();
    pub fn R_Serialize(s: SEXP, ops: R_outpstream_t);
    #[doc = "Current srcref, for debuggers"]
    pub static R_Srcref: SEXP;
    pub fn R_tryEval(arg1: SEXP, arg2: SEXP, arg3: *mut ::std::os::raw::c_int) -> SEXP;
    pub fn R_tryEvalSilent(arg1: SEXP, arg2: SEXP, arg3: *mut ::std::os::raw::c_int) -> SEXP;
    #[doc = "Unbound marker"]
    pub static R_UnboundValue: SEXP;
    pub fn R_unif_index(arg1: f64) -> f64;
    pub fn R_Unserialize(ips: R_inpstream_t) -> SEXP;
    pub fn R_UnwindProtect(
        fun: ::std::option::Option<unsafe extern "C" fn(data: *mut ::std::os::raw::c_void) -> SEXP>,
        data: *mut ::std::os::raw::c_void,
        cleanfun: ::std::option::Option<
            unsafe extern "C" fn(data: *mut ::std::os::raw::c_void, jump: Rboolean),
        >,
        cleandata: *mut ::std::os::raw::c_void,
        cont: SEXP,
    ) -> SEXP;
    pub fn R_useDynamicSymbols(info: *mut DllInfo, value: Rboolean) -> Rboolean;
    pub fn RAW_GET_REGION(sx: SEXP, i: R_xlen_t, n: R_xlen_t, buf: *mut Rbyte) -> R_xlen_t;
    pub fn REAL_GET_REGION(sx: SEXP, i: R_xlen_t, n: R_xlen_t, buf: *mut f64) -> R_xlen_t;
    pub fn Rf_allocMatrix(
        arg1: SEXPTYPE,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    ) -> SEXP;
    pub fn Rf_allocVector(arg1: SEXPTYPE, arg2: R_xlen_t) -> SEXP;
    pub fn Rf_asChar(arg1: SEXP) -> SEXP;
    pub fn Rf_asCharacterFactor(x: SEXP) -> SEXP;
    pub fn Rf_coerceVector(arg1: SEXP, arg2: SEXPTYPE) -> SEXP;
    pub fn Rf_conformable(arg1: SEXP, arg2: SEXP) -> Rboolean;
    pub fn Rf_cons(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_defineVar(arg1: SEXP, arg2: SEXP, arg3: SEXP);
    pub fn Rf_dimgets(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_dimnamesgets(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_duplicate(arg1: SEXP) -> SEXP;
    pub fn Rf_error(arg1: *const ::std::os::raw::c_char, ...) -> !;
    pub fn Rf_findFun(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_GetArrayDimnames(arg1: SEXP) -> SEXP;
    pub fn Rf_getAttrib(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_GetColNames(arg1: SEXP) -> SEXP;
    pub fn Rf_GetRowNames(arg1: SEXP) -> SEXP;
    pub fn Rf_initialize_R(
        ac: ::std::os::raw::c_int,
        av: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn Rf_install(arg1: *const ::std::os::raw::c_char) -> SEXP;
    pub fn Rf_isArray(arg1: SEXP) -> Rboolean;
    pub fn Rf_isComplex(s: SEXP) -> Rboolean;
    pub fn Rf_isEnvironment(s: SEXP) -> Rboolean;
    pub fn Rf_isExpression(s: SEXP) -> Rboolean;
    pub fn Rf_isFactor(arg1: SEXP) -> Rboolean;
    pub fn Rf_isFrame(arg1: SEXP) -> Rboolean;
    pub fn Rf_isFunction(arg1: SEXP) -> Rboolean;
    pub fn Rf_isInteger(arg1: SEXP) -> Rboolean;
    pub fn Rf_isLanguage(arg1: SEXP) -> Rboolean;
    pub fn Rf_isList(arg1: SEXP) -> Rboolean;
    pub fn Rf_isLogical(s: SEXP) -> Rboolean;
    pub fn Rf_isMatrix(arg1: SEXP) -> Rboolean;
    pub fn Rf_isNewList(arg1: SEXP) -> Rboolean;
    pub fn Rf_isNull(s: SEXP) -> Rboolean;
    pub fn Rf_isNumber(arg1: SEXP) -> Rboolean;
    pub fn Rf_isObject(s: SEXP) -> Rboolean;
    pub fn Rf_isPrimitive(arg1: SEXP) -> Rboolean;
    pub fn Rf_isReal(s: SEXP) -> Rboolean;
    pub fn Rf_isString(s: SEXP) -> Rboolean;
    pub fn Rf_isSymbol(s: SEXP) -> Rboolean;
    pub fn Rf_isTs(arg1: SEXP) -> Rboolean;
    pub fn Rf_isUserBinop(arg1: SEXP) -> Rboolean;
    pub fn Rf_isVector(arg1: SEXP) -> Rboolean;
    pub fn Rf_isVectorAtomic(arg1: SEXP) -> Rboolean;
    pub fn Rf_isVectorizable(arg1: SEXP) -> Rboolean;
    pub fn Rf_isVectorList(arg1: SEXP) -> Rboolean;
    pub fn Rf_lang1(arg1: SEXP) -> SEXP;
    pub fn Rf_lcons(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_mkCharLenCE(
        arg1: *const ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
        arg3: cetype_t,
    ) -> SEXP;
    pub fn Rf_namesgets(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_ncols(arg1: SEXP) -> ::std::os::raw::c_int;
    pub fn Rf_NoDevices() -> ::std::os::raw::c_int;
    pub fn Rf_nrows(arg1: SEXP) -> ::std::os::raw::c_int;
    pub fn Rf_NumDevices() -> ::std::os::raw::c_int;
    pub fn Rf_PairToVectorList(x: SEXP) -> SEXP;
    pub fn Rf_PrintValue(arg1: SEXP);
    pub fn Rf_protect(arg1: SEXP) -> SEXP;
    pub fn Rf_runif(arg1: f64, arg2: f64) -> f64;
    pub fn Rf_ScalarInteger(arg1: ::std::os::raw::c_int) -> SEXP;
    pub fn Rf_setAttrib(arg1: SEXP, arg2: SEXP, arg3: SEXP) -> SEXP;
    pub fn Rf_unprotect(arg1: ::std::os::raw::c_int);
    pub fn Rf_VectorToPairList(x: SEXP) -> SEXP;
    pub fn Rf_xlength(arg1: SEXP) -> R_xlen_t;
    pub fn Rf_xlengthgets(arg1: SEXP, arg2: R_xlen_t) -> SEXP;
    pub fn SET_ATTRIB(x: SEXP, v: SEXP);
    pub fn SET_INTEGER_ELT(x: SEXP, i: R_xlen_t, v: ::std::os::raw::c_int);
    pub fn SET_OBJECT(x: SEXP, v: ::std::os::raw::c_int);
    pub fn SET_REAL_ELT(x: SEXP, i: R_xlen_t, v: f64);
    pub fn SET_STRING_ELT(x: SEXP, i: R_xlen_t, v: SEXP);
    pub fn SET_TAG(x: SEXP, y: SEXP);
    pub fn SET_VECTOR_ELT(x: SEXP, i: R_xlen_t, v: SEXP) -> SEXP;
    pub fn SETCDR(x: SEXP, y: SEXP) -> SEXP;
    pub fn setup_Rmainloop();
    pub fn REAL_ELT(x: SEXP, i: R_xlen_t) -> f64;
    pub fn COMPLEX_ELT(x: SEXP, i: R_xlen_t) -> Rcomplex;
    pub fn INTEGER_ELT(x: SEXP, i: R_xlen_t) -> ::std::os::raw::c_int;
    pub fn STRING_ELT(x: SEXP, i: R_xlen_t) -> SEXP;
    pub fn LOGICAL_ELT(x: SEXP, i: R_xlen_t) -> ::std::os::raw::c_int;
    pub fn STRING_PTR_RO(x: SEXP) -> *const SEXP;
    pub fn TAG(e: SEXP) -> SEXP;
    pub fn VECTOR_ELT(x: SEXP, i: R_xlen_t) -> SEXP;
    pub fn SETLEVELS(x: SEXP, v: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    // pub fn Rf_isS4(arg1: SEXP) -> Rboolean;

}

pub unsafe fn Rf_isS4(arg1: SEXP) -> Rboolean {
    unsafe {
        if secret::Rf_isS4_original(arg1) == 0 {
            Rboolean::FALSE
        } else {
            Rboolean::TRUE
        }
    }
}

mod secret {
    use super::*;
    extern "C" {
        #[link_name = "Rf_isS4"]
        pub fn Rf_isS4_original(arg1: SEXP) -> u32;
    }
}

impl From<Rboolean> for bool {
    fn from(value: Rboolean) -> Self {
        match value {
            Rboolean::FALSE => false,
            Rboolean::TRUE => true,
        }
    }
}

impl From<bool> for Rboolean {
    fn from(value: bool) -> Self {
        match value {
            true => Rboolean::TRUE,
            false => Rboolean::FALSE,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::raw;

    // Generate constant static strings.
    // Much more efficient than CString.
    // Generates asciiz.
    macro_rules! cstr {
        ($s: expr) => {
            concat!($s, "\0").as_ptr() as *const raw::c_char
        };
    }

    // Generate mutable static strings.
    // Much more efficient than CString.
    // Generates asciiz.
    macro_rules! cstr_mut {
        ($s: expr) => {
            concat!($s, "\0").as_ptr() as *mut raw::c_char
        };
    }

    // Thanks to @qinwf and @scottmmjackson for showing the way here.
    fn start_R() {
        unsafe {
            if std::env::var("R_HOME").is_err() {
                // env! gets the build-time R_HOME made in build.rs
                std::env::set_var("R_HOME", env!("R_HOME"));
            }

            // Due to Rf_initEmbeddedR using __libc_stack_end
            // We can't call Rf_initEmbeddedR.
            // Instead we must follow rustr's example and call the parts.

            //let res = unsafe { Rf_initEmbeddedR(1, args.as_mut_ptr()) };
            if cfg!(target_os = "windows") && cfg!(target_arch = "x86") {
                Rf_initialize_R(
                    4,
                    [
                        cstr_mut!("R"),
                        cstr_mut!("--arch=i386"),
                        cstr_mut!("--slave"),
                        cstr_mut!("--no-save"),
                    ]
                    .as_mut_ptr(),
                );
            } else {
                Rf_initialize_R(
                    3,
                    [cstr_mut!("R"), cstr_mut!("--slave"), cstr_mut!("--no-save")].as_mut_ptr(),
                );
            }

            // In case you are curious.
            // Maybe 8MB is a bit small.
            // eprintln!("R_CStackLimit={:016x}", R_CStackLimit);

            if cfg!(not(target_os = "windows")) {
                R_CStackLimit = usize::MAX;
            }

            setup_Rmainloop();
        }
    }

    // Run some R code. Check the result.
    #[test]
    fn test_eval() {
        start_R();
        use crate::*;
        extern "C" {
            pub fn R_ParseEvalString(arg1: *const ::std::os::raw::c_char, arg2: SEXP) -> SEXP;
        }
        unsafe {
            println!("Evaluating 1");
            let val = Rf_protect(R_ParseEvalString(cstr!("1"), R_NilValue));
            Rf_PrintValue(val);
            assert_eq!(TYPEOF(val), SEXPTYPE::REALSXP);
            assert_eq!(*REAL(val), 1.);
            Rf_unprotect(1);
        }
        // There is one pathological example of `Rf_is*` where `TRUE` is not 1,
        // but 16. We show here that the casting is done as intended
        unsafe {
            let sexp = R_ParseEvalString(cstr!(r#"new("factor")"#), R_GlobalEnv);
            Rf_protect(sexp);
            Rf_PrintValue(sexp);

            assert_eq!(Rboolean::TRUE, Rboolean::TRUE);
            // assert_eq!(Rboolean::from(is_s4), Rboolean::TRUE);
            // assert_eq!(
            //     std::mem::discriminant(&Rf_isS4(sexp)),
            //     std::mem::discriminant(&Rboolean::TRUE),
            // );
            assert!(<Rboolean as Into<bool>>::into(Rf_isS4(sexp)));
            assert!(
                (Rboolean::FALSE == Rf_isS4(sexp)) || (Rboolean::TRUE == Rf_isS4(sexp)),
                "PartialEq implementation is broken"
            );
            assert!(Rboolean::TRUE == Rf_isS4(sexp));
            assert_eq!(Rf_isS4(sexp), Rboolean::TRUE);
            Rf_unprotect(1);
        }
    }
}
