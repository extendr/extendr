test_that("warn! signals a plain warning message", {
  expect_warning(
    cnd_warn("something went wrong"),
    "something went wrong",
    fixed = TRUE
  )
})

test_that("warn! with body signals warning with bullet points", {
  expect_warning(
    cnd_warn_with_body("something went wrong", c("detail 1", "detail 2")),
    "detail 1.*detail 2"
  )
})

test_that("abort! signals an error with ! bullet prefix", {
  expect_error(
    cnd_abort("something went wrong"),
    "something went wrong",
    fixed = TRUE
  )
})

test_that("abort! with body signals error with bullet points", {
  expect_error(
    cnd_abort_with_body("something went wrong", c("detail 1", "detail 2")),
    "detail 1.*detail 2"
  )
})

test_that("abort! handles % characters without segfault", {
  expect_error(
    cnd_abort("50% done"),
    "50% done",
    fixed = TRUE
  )
})

test_that("warn! handles % characters without segfault", {
  expect_warning(
    cnd_warn("50% done"),
    "50% done",
    fixed = TRUE
  )
})

test_that("throw_r_error does not segfault with % characters", {
  expect_error(
    throw_error_with_percent("50% done"),
    "50% done",
    fixed = TRUE
  )
})

test_that("abort! with call shows caller in error", {
  my_checker <- function(x) {
    cnd_abort_with_call("something went wrong", call = rlang::current_env())
  }
  err <- tryCatch(my_checker(1), error = function(e) e)

  expect_equal(
    as.character(err$call),
    c("my_checker", "1")
  )
})
