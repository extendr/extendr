/*
// Take the address of the wrap__hello stub function to avoid
// the linker removing the static library.
//
// This will be removed in future versions with the module macro.
void wrap__hello();

void *x = (void*)&wrap__hello;
*/


#define R_NO_REMAP
#define STRICT_R_HEADERS
#include <Rinternals.h>

SEXP wrap__hello();

// Standard R package stuff
static const R_CallMethodDef CallEntries[] = {
  {"wrap__hello", (DL_FUNC) &wrap__hello, 0},
  {NULL, NULL, 0}
};

void R_init_textendrtests(DllInfo *dll) {
  R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
  R_useDynamicSymbols(dll, FALSE);
}
