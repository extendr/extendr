/*
// Take the address of the module init function to avoid
// the linker removing the static library.
void R_init_extendrtests(void *);

void *__dummy__ = (void*)&R_init_extendrtests;
*/

#define R_NO_REMAP
#define STRICT_R_HEADERS
#include <Rinternals.h>

SEXP wrap__hello_world();
SEXP wrap__double_scalar(SEXP);
SEXP wrap__int_scalar(SEXP);
SEXP wrap__bool_scalar(SEXP);
SEXP wrap__char_scalar(SEXP);

// Standard R package stuff
static const R_CallMethodDef CallEntries[] = {
  {"wrap__hello_world", (DL_FUNC) &wrap__hello_world, 0},
  {"wrap__double_scalar", (DL_FUNC) &wrap__double_scalar, 1},
  {"wrap__int_scalar", (DL_FUNC) &wrap__int_scalar, 1},
  {"wrap__bool_scalar", (DL_FUNC) &wrap__bool_scalar, 1},
  {"wrap__char_scalar", (DL_FUNC) &wrap__char_scalar, 1},
  {NULL, NULL, 0}
};

void R_init_extendrtests(DllInfo *dll) {
  R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
  R_useDynamicSymbols(dll, FALSE);
  R_forceSymbols(dll, TRUE);
}