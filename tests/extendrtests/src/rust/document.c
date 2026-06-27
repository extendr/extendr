// Build-time helper: loaded transiently by R during package build to call
// the wrapper-generation routine that `extendr_module!` already emits as
// `wrap__make_<modname>_wrappers`. NOT compiled into the final package .so.
// See Makevars.in for how this is invoked.

#include <R.h>
#include <Rinternals.h>
#include <R_ext/Rdynload.h>

extern SEXP wrap__make_extendrtests_wrappers(SEXP use_symbols, SEXP package_name);

static const R_CallMethodDef CallEntries[] = {
    {"write_wrappers", (DL_FUNC) &wrap__make_extendrtests_wrappers, 2},
    {NULL, NULL, 0}
};

void R_init_document(DllInfo *dll) {
    R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
    R_useDynamicSymbols(dll, FALSE);
}
