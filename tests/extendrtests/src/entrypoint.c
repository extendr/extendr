// We need to forward routine registration from C to Rust 
// to avoid the linker removing the static library.

#include <Rinternals.h>

void R_init_extendrtests_extendr(DllInfo *dll);

void R_init_extendrtests(DllInfo *dll) {
  R_init_extendrtests_extendr(dll);
}
