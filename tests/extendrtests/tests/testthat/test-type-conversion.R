test_that("TryFrom conversions work", {
  # Atomic types
  expect_equal(try_double_vec(c(0, 1)), c(0, 1))
  expect_equal(try_double_vec(c(0, NA_real_)), c(0, NA)) # R type conversion
  expect_equal(try_double_vec(numeric()), numeric())
  expect_false(identical(try_double_vec(NA_real_), NA))
  expect_error(try_double_vec(c("more", "hooey")), "Expected Doubles got String")
  expect_error(try_double_vec(15L), "Expected Doubles got Integer")
  expect_error(try_double_vec(TRUE), "Expected Doubles got Logical")
  expect_error(try_double_vec(NA), "Expected Doubles got Logical")
  expect_error(try_double_vec(NULL), "Expected Doubles got Null")

  # Non-atomic types
  # TODO
})
