#' Wrappers for Rust functions.
#' 
#' Wrappers for Rust functions.
#' @rdname wrappers
#' @export
hello <- function() {
  .Call("wrap__hello")
}

#' @rdname wrappers
#' @param x,y parameters
#' @export
add_ints <- function(x, y) {
  .Call("wrap__add_ints", x, y)
}