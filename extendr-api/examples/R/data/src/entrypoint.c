// We need to forward routine registration from C to Rust 
// to avoid the linker removing the static library.

void R_init_data_extendr(void *dll);

void R_init_data(void *dll) {
  R_init_data_extendr(dll);
}

