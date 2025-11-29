// Development-time helper for generating R wrappers.
//
// This file is NOT part of the normal package build. It provides access to
// wrap__make_extendrtests_wrappers from the Rust static library, which is
// needed by rextendr::document() to regenerate R/extendr-wrappers.R.
//
// Build with: R CMD SHLIB -o extendrtests-wrappers.so extendrtests-wrappers.c \
//             -L./rust/target/debug -lextendrtests
//
// Usage from R:
//   dll <- dyn.load("src/extendrtests-wrappers.so")  # or .dll on Windows
//   .Call(getNativeSymbolInfo("exported_make_extendrtests_wrappers", PACKAGE = dll)$address,
//         TRUE, "extendrtests")
//   dyn.unload(dll[["path"]])

#include <Rinternals.h>
#include <R_ext/Rdynload.h>

#ifdef _WIN32
#define FORCE_EXPORT __declspec(dllexport)
#else
#define FORCE_EXPORT
#endif

// Rust function from the static library (linked from libextendrtests.a)
extern SEXP wrap__make_extendrtests_wrappers(SEXP use_symbols_sexp, SEXP package_name_sexp);

// Re-export wrapper to force symbol visibility on Windows.
// On Windows, R CMD SHLIB only auto-exports symbols defined in .o files,
// not symbols pulled in from static libraries. This wrapper is defined
// in the .o file and calls through to the Rust function.
FORCE_EXPORT SEXP exported_make_extendrtests_wrappers(SEXP use_symbols_sexp, SEXP package_name_sexp) {
    return wrap__make_extendrtests_wrappers(use_symbols_sexp, package_name_sexp);
}

// Registration table - register the exported wrapper
static const R_CallMethodDef CallEntries[] = {
    {"exported_make_extendrtests_wrappers", (DL_FUNC) &exported_make_extendrtests_wrappers, 2},
    {NULL, NULL, 0}
};

void R_init_extendrtests_wrappers(DllInfo *dll) {
    R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
    // Allow dynamic symbol lookup for getNativeSymbolInfo() to work
    R_useDynamicSymbols(dll, TRUE);
}
