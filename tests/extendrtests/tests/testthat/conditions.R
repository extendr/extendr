test_that("warn! signals a warning with ! bullet prefix", {
  expect_warning(
    cnd_warn("something went wrong"),
    "! something went wrong",
    fixed = TRUE
  )
})

test_that("warn! with body signals warning with bullet points", {
  w <- tryCatch(
    cnd_warn_with_body("something went wrong", c("detail 1", "detail 2")),
    warning = function(w) w
  )
  msg <- conditionMessage(w)
  expect_true(grepl("! something went wrong", msg, fixed = TRUE))
  expect_true(grepl("\u2022 detail 1", msg, fixed = TRUE))
  expect_true(grepl("\u2022 detail 2", msg, fixed = TRUE))
})

test_that("abort! signals an error with ! bullet prefix", {
  expect_error(
    cnd_abort("something went wrong"),
    "! something went wrong",
    fixed = TRUE
  )
})

test_that("abort! with body signals error with bullet points", {
  err <- tryCatch(
    cnd_abort_with_body("something went wrong", c("detail 1", "detail 2")),
    error = function(e) e
  )
  msg <- conditionMessage(err)
  expect_true(grepl("! something went wrong", msg, fixed = TRUE))
  expect_true(grepl("\u2022 detail 1", msg, fixed = TRUE))
  expect_true(grepl("\u2022 detail 2", msg, fixed = TRUE))
})

test_that("abort! handles % characters without segfault", {
  expect_error(
    cnd_abort("50% done"),
    "! 50% done",
    fixed = TRUE
  )
})

test_that("warn! handles % characters without segfault", {
  expect_warning(
    cnd_warn("50% done"),
    "! 50% done",
    fixed = TRUE
  )
})
