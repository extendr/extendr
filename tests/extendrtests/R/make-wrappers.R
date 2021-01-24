#' Generate wrapper code
#' 
#' Run `make_wrappers()` to update the wrapper scripts for this package.
#' This autogenerates the file `R/extendr-wrappers.R`.
#' @export
make_wrappers <- function() {
  x <- .Call("wrap__make_extendrtests_wrappers", use_symbols = TRUE)
  x <- strsplit(x, "\n")[[1]]
  
  outfile <- here::here("R", "extendr-wrappers.R")
  message("Writting wrappers to: ", outfile)
  brio::write_lines(x, outfile)
}