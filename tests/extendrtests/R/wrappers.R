#' Wrappers for Rust test functions.
#' 
#' Wrappers for Rust test functions.
#' @rdname wrappers
#' @export
hello_world <- function() {
  .Call("wrap__hello_world")
}
