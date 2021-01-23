#' Wrappers for Rust test functions.
#' 
#' @rdname wrappers
#' @export
hello_world <- function() {
  .Call(wrap__hello_world)
}

#' Get module metadata as a list.
#' 
#' @rdname wrappers
#' @export
get_extendrtests_metadata <- function() .Call(wrap__get_extendrtests_metadata)

#' Build wrappers 
#' 
#' @param use_symbols use symbols instead of strings.
#' @rdname wrappers
#' @export
make_extendrtests_wrappers <- function(use_symbols=TRUE) .Call(wrap__make_extendrtests_wrappers, use_symbols)


hello_world <- function() .Call(wrap__hello_world)

#' convert a double scalar to itself
#' @param x a number
double_scalar <- function(x) .Call(wrap__double_scalar, x)

#' convert an int scalar to itself
#' @param x a number
int_scalar <- function(x) .Call(wrap__int_scalar, x)

bool_scalar <- function(x) .Call(wrap__bool_scalar, x)

char_scalar <- function(x) .Call(wrap__char_scalar, x)

#' @export
MyClass <- new.env(parent = emptyenv())

MyClass$new <- function() .Call(wrap__MyClass__new)
MyClass$set_a <- function(x) .Call(wrap__MyClass__set_a, self, x)
MyClass$a <- function() .Call(wrap__MyClass__a, self)
MyClass$me <- function() .Call(wrap__MyClass__me, self)
#' @export
`$.MyClass` <- function (self, name) { func <- MyClass[[name]]; environment(func) <- environment(); func }

