test_that("rlang error_cnd round-trips", {
  skip_if_not_installed("rlang")
  cnd <- rlang::error_cnd("my_error", message = "something failed")
  result <- roundtrip_condition(cnd)
  expect_s3_class(result, "error")
  expect_s3_class(result, "condition")
  expect_equal(result$message, "something failed")
})

test_that("rlang warning_cnd round-trips", {
  skip_if_not_installed("rlang")
  cnd <- rlang::warning_cnd("my_warning", message = "watch out")
  result <- roundtrip_condition(cnd)
  expect_s3_class(result, "warning")
  expect_s3_class(result, "condition")
  expect_equal(result$message, "watch out")
})

test_that("rlang message_cnd round-trips", {
  skip_if_not_installed("rlang")
  cnd <- rlang::message_cnd("my_message", message = "hello")
  result <- roundtrip_condition(cnd)
  expect_s3_class(result, "message")
  expect_s3_class(result, "condition")
  expect_equal(result$message, "hello")
})

test_that("condition_message extracts message field", {
  skip_if_not_installed("rlang")
  cnd <- rlang::error_cnd(message = "oops")
  expect_equal(condition_message(cnd), "oops")
})

test_that("condition_has_call detects call from rlang::current_call()", {
  skip_if_not_installed("rlang")
  my_fn <- function() {
    cnd <- rlang::error_cnd(message = "oops", call = rlang::current_call())
    condition_has_call(cnd)
  }
  expect_true(my_fn())
})

test_that("round-trip preserves message and call; rlang trace/parent are dropped", {
  skip_if_not_installed("rlang")
  my_fn <- function() {
    rlang::error_cnd(message = "oops", call = rlang::current_call())
  }
  cnd <- my_fn()
  c2 <- roundtrip_condition(cnd)
  expect_identical(cnd$message, c2$message)
  expect_identical(cnd$call, c2$call)
  expect_null(c2$trace)
  expect_null(c2$parent)
})

test_that("condition_has_call returns FALSE when call is NULL", {
  skip_if_not_installed("rlang")
  cnd <- rlang::error_cnd(message = "oops", call = NULL)
  expect_false(condition_has_call(cnd))
})

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
