

// Take the address of the wrap__hello stub function to avoid
// the linker removing the static library.
//
// This will be removed in future versions with the module macro.
void wrap__hello();

void *x = (void*)&wrap__hello;

