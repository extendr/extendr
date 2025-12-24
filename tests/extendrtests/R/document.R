#' Regenerate extendr wrappers
#'
#' Regenerates R wrappers from the Rust library by calling the
#' `exported_make_{package}_wrappers` function exported by the wrappers shlib.
#'
#' @param pkg Path to the package root. Defaults to current directory.
#'
#' @details
#' This function searches for the wrappers shlib in multiple locations:
#' - Source builds: `src/rust/target/{debug,release}/wrappers/`
#' - Cross-compilation: `src/rust/target/{target-triple}/{debug,release}/wrappers/`
#' - Installed package: `{libpath}/wrappers/`
#'
#' When multiple candidates exist, the most recently modified one is used.
#'
#' The wrappers shlib is a C wrapper around the Rust staticlib that exports
#' `exported_make_{package}_wrappers`. This allows regenerating R wrappers without
#' requiring a cdylib crate type.
#'
#' @keywords internal
document <- function(pkg = ".") {
  # Get package name from DESCRIPTION (don't rely on loaded package)
  desc_file <- file.path(pkg, "DESCRIPTION")
  if (!file.exists(desc_file)) {
    stop("DESCRIPTION file not found at: ", desc_file, call. = FALSE)
  }
  desc <- read.dcf(desc_file)
  package_name <- desc[1, "Package"]

  stopifnot(
    "Could not determine package name" = !is.null(package_name)
  )

  # Platform-specific shlib name (uses R's convention: .so on Unix, .dll on Windows)
  shlib_name <- switch(
    Sys.info()[["sysname"]],
    "Windows" = paste0(package_name, ".dll"),
    paste0("lib", package_name, .Platform$dynlib.ext)
  )

  # Search for wrappers shlib in multiple locations (debug, release, cross-compilation)
  search_patterns <- c(
    file.path(pkg, "src", "rust", "target", "debug", "wrappers", shlib_name),
    file.path(pkg, "src", "rust", "target", "release", "wrappers", shlib_name),
    file.path(pkg, "src", "rust", "target", "*", "debug", "wrappers", shlib_name),
    file.path(pkg, "src", "rust", "target", "*", "release", "wrappers", shlib_name)
  )
  candidates <- unlist(lapply(search_patterns, Sys.glob))

  # Also check installed package location
  installed_path <- system.file("libs", .Platform$r_arch, "wrappers", shlib_name,
                                 package = package_name, mustWork = FALSE)
  if (nzchar(installed_path) && file.exists(installed_path)) {
    candidates <- c(candidates, installed_path)
  }

  # Pick the most recently modified
  if (length(candidates) > 0) {
    mtimes <- file.mtime(candidates)
    library_path <- candidates[which.max(mtimes)]
  } else {
    library_path <- NULL
  }

  if (is.null(library_path)) {
    stop(
      "Development wrappers shlib not found.\n",
      "Searched patterns:\n",
      paste("  -", search_patterns, collapse = "\n"), "\n",
      "  - installed package\n\n",
      "Run `just document` to build and install the package.",
      call. = FALSE
    )
  }

  dll <- dyn.load(library_path)

  # Use tryCatch to ensure unload happens even on error
  wrapper_text <- tryCatch({
    # Get the symbol address from the dynamically loaded library.
    # Using do.call prevents R CMD check from trying to statically
    # analyze the .Call() arguments.
    symbol_name <- sprintf("exported_make_%s_wrappers", package_name)
    symbol_info <- getNativeSymbolInfo(symbol_name, PACKAGE = dll)
    do.call(.Call, list(symbol_info$address, TRUE, package_name))
  }, finally = {
    dyn.unload(dll[["path"]])
  })

  cat(
    "# nolint start\n\n",
    wrapper_text,
    "# nolint end\n",
    sep = "",
    file = file.path(pkg, "R", "extendr-wrappers.R"),
    append = FALSE
  )
}
