#' Generate wrapper code
#' 
#' Run `make_wrappers()` to update the wrapper scripts for this package.
#' This autogenerates the file `R/extendr-wrappers.R`.
#' @export
make_wrappers <- function() {
  rextendr::document()
}