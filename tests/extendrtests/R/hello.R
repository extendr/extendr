#' Call Rust function `hello()`
#' 
#' Call Rust function `hello()`.
#' @export
hello <- function() {
  .Call("wrap__hello")
}