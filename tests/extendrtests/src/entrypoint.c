// Take the address of the module init function to avoid
// the linker removing the static library.
void R_init_extendrtests(void *);

void *__dummy__ = (void*)&R_init_extendrtests;
