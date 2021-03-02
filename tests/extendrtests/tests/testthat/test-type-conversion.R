test_that("Conversion of R types to Rust types and vice versa works", {
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
})

test_that("Generic vector convertion", {
  
  expect_equal(
    vec_generic_class(as.list(1:10)),
    sum(1:10)
  )
  
})

