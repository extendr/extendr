# Custom installation script for extendrtests
#
# This script installs:
# 1. The standard package shared objects (following R's default pattern)
# 2. The development wrappers shlib (for document() to regenerate R wrappers)
#
# Variables available (from R):
#   R_PACKAGE_NAME    - "extendrtests"
#   R_PACKAGE_SOURCE  - path to source directory
#   R_PACKAGE_DIR     - path to installation directory
#   R_ARCH            - arch-dependent path component (often empty)
#   SHLIB_EXT         - shared object extension (.so, .dll)
#   WINDOWS           - TRUE on Windows, FALSE elsewhere

# Standard installation (from "Writing R Extensions" manual)
dest <- file.path(R_PACKAGE_DIR, paste0("libs", R_ARCH))
dir.create(dest, recursive = TRUE, showWarnings = FALSE)
files <- Sys.glob(paste0("*", SHLIB_EXT))
file.copy(files, dest, overwrite = TRUE)
if (file.exists("symbols.rds")) {
  file.copy("symbols.rds", dest, overwrite = TRUE)
}

# Install the development wrappers shlib (built by Makevars)
# Note: wrappers shlib uses "lib" prefix on Unix (like a cdylib would)
wrappers_shlib_name <- if (WINDOWS) {
  paste0(R_PACKAGE_NAME, SHLIB_EXT)
} else {
  paste0("lib", R_PACKAGE_NAME, SHLIB_EXT)
}

# Search for wrappers shlib in multiple locations (debug, release, cross-compilation targets)
# and pick the most recently modified one
wrappers_search_patterns <- c(
  file.path("rust", "target", "debug", "wrappers", wrappers_shlib_name),
  file.path("rust", "target", "release", "wrappers", wrappers_shlib_name),
  file.path("rust", "target", "*", "debug", "wrappers", wrappers_shlib_name),
  file.path("rust", "target", "*", "release", "wrappers", wrappers_shlib_name)
)
wrappers_candidates <- unlist(lapply(wrappers_search_patterns, Sys.glob))

# Pick the most recently modified
wrappers_source <- if (length(wrappers_candidates) > 0) {
  mtimes <- file.mtime(wrappers_candidates)
  wrappers_candidates[which.max(mtimes)]
} else {
  NULL
}

if (!is.null(wrappers_source) && file.exists(wrappers_source)) {
  wrappers_dest <- file.path(dest, "wrappers")
  dir.create(wrappers_dest, recursive = TRUE, showWarnings = FALSE)
  file.copy(wrappers_source, wrappers_dest, overwrite = TRUE)
}
