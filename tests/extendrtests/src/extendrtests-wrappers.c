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
//   dll <- dyn.load("src/extendrtests-wrappers.so")
//   .Call(getNativeSymbolInfo("wrap__make_extendrtests_wrappers", PACKAGE = dll)$address,
//         TRUE, "extendrtests")
//   dyn.unload(dll[["path"]])

#include <Rinternals.h>
#include <R_ext/Rdynload.h>

// Rust function from the static library
SEXP wrap__make_extendrtests_wrappers(SEXP use_symbols_sexp, SEXP package_name_sexp);

// Registration table
static const R_CallMethodDef CallEntries[] = {
    {"wrap__make_extendrtests_wrappers", (DL_FUNC) &wrap__make_extendrtests_wrappers, 2},
    {NULL, NULL, 0}
};

void R_init_extendrtests_wrappers(DllInfo *dll) {
    R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
    // Allow dynamic symbol lookup for getNativeSymbolInfo() to work
    R_useDynamicSymbols(dll, TRUE);
}
