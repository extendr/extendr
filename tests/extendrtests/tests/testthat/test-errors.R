test_that("Error functions throw clean errors by default", {

  # remove any extendr-traceback env vars
  Sys.unsetenv("EXTENDR_TRACEBACK")

  expect_error(
    error_simple(),
    "This is a simple error message",
    fixed = TRUE
  )

  expect_error(
    error_parse_int("not_a_number"),
    "invalid digit found in string"
  )

  expect_error(
    error_division(10, 0),
    "Division by zero is not allowed",
    fixed = TRUE
  )

  expect_error(
    error_chain("abc"),
    "Parse error:"
  )

  expect_error(
    error_chain("-5"),
    "Negative numbers not allowed",
    fixed = TRUE
  )

  expect_error(
    error_long_message(),
    "This is a longer error message"
  )
})

test_that("Successful Result returns value correctly", {
  expect_equal(error_success(), 42L)
  expect_equal(error_division(10, 2), 5.0)
  expect_equal(error_chain("42"), 42.0)
})

test_that("Error messages do not contain Rust traceback by default", {

  Sys.unsetenv("EXTENDR_TRACEBACK")

  err <- tryCatch(
    error_simple(),
    error = function(e) conditionMessage(e)
  )

  expect_false(grepl("thread 'main' panicked", err))
  expect_false(grepl("stack backtrace:", err))
  expect_false(grepl("at src/", err))

  expect_true(grepl("This is a simple error message", err, fixed = TRUE))
})

test_that("EXTENDR_TRACEBACK=true shows full traceback", {
  orig_val <- Sys.getenv("EXTENDR_TRACEBACK", unset = NA)

  Sys.setenv(EXTENDR_TRACEBACK = "true")

  # Capture the error
  err <- tryCatch(
    error_simple(),
    error = function(e) conditionMessage(e)
  )

  if (is.na(orig_val)) {
    Sys.unsetenv("EXTENDR_TRACEBACK")
  } else {
    Sys.setenv(EXTENDR_TRACEBACK = orig_val)
  }

  expect_true(grepl("unwrap.*Err", err, ignore.case = TRUE))
  expect_true(grepl("This is a simple error message", err))
})

test_that("EXTENDR_TRACEBACK=1 also shows full traceback", {
  orig_val <- Sys.getenv("EXTENDR_TRACEBACK", unset = NA)

  # extewndr traceback should honor a value of 1 too
  Sys.setenv(EXTENDR_TRACEBACK = "1")

  err <- tryCatch(
    error_simple(),
    error = function(e) conditionMessage(e)
  )

  if (is.na(orig_val)) {
    Sys.unsetenv("EXTENDR_TRACEBACK")
  } else {
    Sys.setenv(EXTENDR_TRACEBACK = orig_val)
  }

  expect_true(grepl("unwrap.*Err", err, ignore.case = TRUE))
  expect_true(grepl("This is a simple error message", err))
})

test_that("EXTENDR_TRACEBACK=false shows clean error", {

  orig_val <- Sys.getenv("EXTENDR_TRACEBACK", unset = NA)

  Sys.setenv(EXTENDR_TRACEBACK = "false")

  err <- tryCatch(
    error_simple(),
    error = function(e) conditionMessage(e)
  )

  if (is.na(orig_val)) {
    Sys.unsetenv("EXTENDR_TRACEBACK")
  } else {
    Sys.setenv(EXTENDR_TRACEBACK = orig_val)
  }

  expect_true(grepl("This is a simple error message", err, fixed = TRUE))
  expect_false(grepl("thread 'main' panicked", err))
})
