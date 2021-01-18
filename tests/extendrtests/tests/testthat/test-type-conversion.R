test_that("Conversion of R types to Rust types and vice versa works", {
  expect_equal(.Call(wrap__double_scalar, .45), .45)
  expect_equal(.Call(wrap__double_scalar, 15L), 15)
  expect_error(.Call(wrap__double_scalar, TRUE), "unable to convert")
  expect_error(.Call(wrap__double_scalar, "abcxyz"), "unable to convert")
  expect_error(.Call(wrap__double_scalar, NA_real_), "Input must not be NA")
  expect_error(.Call(wrap__double_scalar, c(.45, .46)), "Input must be of length 1")
  
  expect_equal(.Call(wrap__int_scalar, 15L), 15L)
  expect_equal(.Call(wrap__int_scalar, 4.4), 4L) # is this deliberate? seems dangerous
  expect_error(.Call(wrap__int_scalar, TRUE), "unable to convert")
  expect_error(.Call(wrap__int_scalar, "abcxyz"), "unable to convert")
  expect_error(.Call(wrap__int_scalar, NA_integer_), "Input must not be NA")
  expect_error(.Call(wrap__int_scalar, 1L:5L), "Input must be of length 1")
  
  expect_equal(.Call(wrap__bool_scalar, TRUE), TRUE)
  expect_equal(.Call(wrap__bool_scalar, FALSE), FALSE)
  expect_error(.Call(wrap__bool_scalar, .45), "Not a logical object")
  expect_error(.Call(wrap__bool_scalar, 15L), "Not a logical object")
  expect_error(.Call(wrap__bool_scalar, "abcxyz"), "Not a logical object")
  expect_error(.Call(wrap__bool_scalar, NA), "Input must not be NA")
  expect_error(.Call(wrap__bool_scalar, c(TRUE, FALSE, TRUE)), "Input must be of length 1")
  
  expect_equal(.Call(wrap__char_scalar, "abcxyz"), "abcxyz")
  expect_error(.Call(wrap__char_scalar, .45), "not a string object")
  expect_error(.Call(wrap__char_scalar, 15L), "not a string object")
  expect_error(.Call(wrap__char_scalar, TRUE), "not a string object")
  expect_error(.Call(wrap__char_scalar, NA_character_), "Input must not be NA")
  expect_error(.Call(wrap__char_scalar, c("hello", "world")), "not a string object") # why this error message and not "Input must be of length 1"?

  expect_equal(.Call(wrap__char_vec, c("hello", "world")), c("hello", "world"))
  expect_error(.Call(wrap__char_vec, .45), "not a string object")
  expect_error(.Call(wrap__char_vec, 15L), "not a string object")
  expect_error(.Call(wrap__char_vec, TRUE), "not a string object")
  expect_error(.Call(wrap__char_vec, NA_character_), "Input must not be NA")
})

