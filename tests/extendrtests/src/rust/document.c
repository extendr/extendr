// Build-time helper: linked against the package's Rust staticlib and run by
// Makevars to write R/extendr-wrappers.R. NOT part of the final package .so.
// See Makevars.in for how this is invoked.

extern int write__make_extendrtests_wrappers(const char *package_name, const char *out_path);

int main(int argc, char **argv) {
    if (argc != 3) return 2;
    return write__make_extendrtests_wrappers(argv[1], argv[2]);
}
