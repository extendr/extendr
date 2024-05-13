test_that("From conversion of R types to Rust types and vice versa works", {
  expect_equal(double_scalar(.45), .45)
  expect_equal(double_scalar(15L), 15)
  expect_error(double_scalar(TRUE), "Expected Numeric, got Logicals")
  expect_error(double_scalar("abcxyz"), "Expected Numeric, got Strings")
  expect_error(double_scalar(NA_real_), "Must not be NA")
  expect_error(double_scalar(c(.45, .46)), "Expected Scalar, got Doubles")
  
  expect_equal(int_scalar(15L), 15L)
  expect_error(int_scalar(4.4))
  expect_error(int_scalar(TRUE))
  expect_error(int_scalar("abcxyz"))
  expect_error(int_scalar(NA_integer_))
  expect_error(int_scalar(1L:5L))
  
  expect_equal(bool_scalar(TRUE), TRUE)
  expect_equal(bool_scalar(FALSE), FALSE)
  expect_error(bool_scalar(.45))
  expect_error(bool_scalar(15L))
  expect_error(bool_scalar("abcxyz"))
  expect_error(bool_scalar(NA))
  expect_error(bool_scalar(c(TRUE, FALSE, TRUE)))
  
  expect_equal(char_scalar("abcxyz"), "abcxyz")
  expect_error(char_scalar(.45))
  expect_error(char_scalar(15L))
  expect_error(char_scalar(TRUE))
  expect_error(char_scalar(NA_character_))
  expect_error(char_scalar(c("hello", "world")))

  expect_equal(char_vec(c("hello", "world")), c("hello", "world"))
  expect_error(char_vec(.45))
  expect_error(char_vec(15L))
  expect_error(char_vec(TRUE))
  expect_error(char_vec(NA_character_))
  expect_error(char_vec(c("hello", NA)))

  expect_equal(double_vec(c(0, 1)), c(0, 1))
  expect_equal(double_vec(numeric()), numeric())
  expect_equal(double_vec(c(0, NA_real_)), c(0, NA)) # R type coercion
  expect_false(identical(double_vec(NA_real_), NA))
  expect_error(double_vec(c("more", "hooey")))
  expect_error(double_vec(15L))
  expect_error(double_vec(TRUE))
  expect_error(double_vec(NA))
  expect_error(double_vec(NULL))


  # Non-atomic types
  # TODO
})

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
