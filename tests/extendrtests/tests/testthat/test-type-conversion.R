test_that("From conversion of R types to Rust types and vice versa works", {
  expect_equal(double_scalar(.45), .45)
  expect_equal(double_scalar(15L), 15)
  expect_error(double_scalar(TRUE), "unable to convert")
  expect_error(double_scalar("abcxyz"), "unable to convert")
  expect_error(double_scalar(NA_real_), "Input must not be NA")
  expect_error(double_scalar(c(.45, .46)), "Input must be of length 1")
  
  expect_equal(int_scalar(15L), 15L)
  expect_equal(int_scalar(4.4), 4L) # is this deliberate? seems dangerous
  expect_error(int_scalar(TRUE), "unable to convert")
  expect_error(int_scalar("abcxyz"), "unable to convert")
  expect_error(int_scalar(NA_integer_), "Input must not be NA")
  expect_error(int_scalar(1L:5L), "Input must be of length 1")
  
  expect_equal(bool_scalar(TRUE), TRUE)
  expect_equal(bool_scalar(FALSE), FALSE)
  expect_error(bool_scalar(.45), "Not a logical object")
  expect_error(bool_scalar(15L), "Not a logical object")
  expect_error(bool_scalar("abcxyz"), "Not a logical object")
  expect_error(bool_scalar(NA), "Input must not be NA")
  expect_error(bool_scalar(c(TRUE, FALSE, TRUE)), "Input must be of length 1")
  
  expect_equal(char_scalar("abcxyz"), "abcxyz")
  expect_error(char_scalar(.45), "not a string object")
  expect_error(char_scalar(15L), "not a string object")
  expect_error(char_scalar(TRUE), "not a string object")
  expect_error(char_scalar(NA_character_), "Input must not be NA")
  expect_error(char_scalar(c("hello", "world")), "not a string object") # why this error message and not "Input must be of length 1"?

  expect_equal(char_vec(c("hello", "world")), c("hello", "world"))
  expect_error(char_vec(.45), "Input must be a character vector")
  expect_error(char_vec(15L), "Input must be a character vector")
  expect_error(char_vec(TRUE), "Input must be a character vector")
  expect_error(char_vec(NA_character_), "Input must be a character vector. Got 'NA'.")
  expect_error(char_vec(c("hello", NA)), "Input vector cannot contain NA's")

  expect_equal(double_vec(c(0, 1)), c(0, 1))
  expect_equal(double_vec(numeric()), numeric())
  expect_equal(double_vec(c(0, NA_real_)), c(0, NA)) # R type coercion
  expect_false(identical(double_vec(NA_real_), NA))
  expect_error(double_vec(c("more", "hooey")), "not a floating point vector")
  expect_error(double_vec(15L), "not a floating point vector")
  expect_error(double_vec(TRUE), "not a floating point vector")
  expect_error(double_vec(NA), "not a floating point vector")
  expect_error(double_vec(NULL), "not a floating point vector")


  # Non-atomic types
  # TODO
})

test_that("TryFrom conversions work", {
  # Atomic types
  expect_equal(try_double_vec(c(0, 1)), c(0, 1))
  expect_equal(try_double_vec(c(0, NA_real_)), c(0, NA)) # R type conversion
  expect_equal(try_double_vec(numeric()), numeric())
  expect_false(identical(try_double_vec(NA_real_), NA))
  expect_error(try_double_vec(c("more", "hooey")), "Expected Real got String")
  expect_error(try_double_vec(15L), "Expected Real got Integer")
  expect_error(try_double_vec(TRUE), "Expected Real got Logical")
  expect_error(try_double_vec(NA), "Expected Real got Logical")
  expect_error(try_double_vec(NULL), "Expected Real got Null")

  # Non-atomic types
  # TODO
})

