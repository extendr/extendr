test_that("From conversion of R types to Rust types and vice versa works", {
  # Test atomic types and vectors
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

  expect_equal(double_vec(c(0, 1)), c(0, 1))
  expect_equal(double_vec(numeric()), numeric())
  expect_equal(double_vec(c(0, NA_real_)), c(0, NA)) # R type coercion 
  expect_false(identical(double_vec(NA_real_), NA))
  expect_error(double_vec(c("more", "hooey")), "not a floating point vector")
  expect_error(double_vec(15L), "not a floating point vector")
  expect_error(double_vec(TRUE), "not a floating point vector")
  expect_error(double_vec(NA), "not a floating point vector")
  expect_error(double_vec(NULL), "not a floating point vector")

  expect_equal(int_vec(c(0L, 1L)), c(0L, 1L))
  expect_equal(int_vec(integer()), integer())
  expect_equal(int_vec(c(0L, TRUE)), c(0L, 1L)) # R type conversion
  expect_equal(int_vec(c(0L, NA_integer_)), c(0L, NA_integer_)) 
  expect_error(int_vec(c(0L, 0)), "not an integer or logical vector")
  expect_error(int_vec(TRUE), "not an integer or logical vector") # awkward err msg here because of R type conversion behavior. change?
  expect_error(int_vec(.45), "not an integer or logical vector")
  expect_error(int_vec(c("more", "hooey")), "not an integer or logical vector")
  expect_error(int_vec(NA_character_), "not an integer or logical vector")

  expect_equal(char_string_vec(c("hello", "world")), c("hello", "world"))
  expect_equal(char_string_vec(character()), character())
  expect_error(char_string_vec(.45), "Input must be a character vector")
  expect_error(char_string_vec(15L), "Input must be a character vector")
  expect_error(char_string_vec(TRUE), "Input must be a character vector")
  # Is it indended that character vectors reject NA when others do not? see above
  expect_error(char_string_vec(NA_character_), "Input must be a character vector. Got 'NA'.")
  expect_error(char_string_vec(c("hello", NA)), "Input vector cannot contain NA's")

  # Test matrices and arrays
  # TODO:

  # Test non-atomic types

  x <- list(a = 1, b = NA_integer_, u = data.frame(a = 1:4),
            v = c(1, 2), z = "who", f = function(x) x^2)

  x_noname <- x
  names(x_noname) <- NULL

  e <- new.env()
  assign("first", 1)


  # conversion does not preserve list order
  expect_mapequal(list_str_hash(x), x)
  # TODO: unnamed list returns empty list
  # FromRobj for HashMap should fail for unnamed lists?
  expect_setequal(list_str_hash(x_noname), x_noname)
  expect_setequal(list_str_hash(list()), list())
  expect_error(list_str_hash(20:30), "expected a list")
  expect_error(list_str_hash(NA), "expected a list")
  expect_error(list_str_hash(e), "expected a list")


  # Forced failures for unimplemented conversions
  expect_success(list_string_hash())
  expect_success(char_str_vec())
  expect_success(bool_vec())

})

test_that("TryFrom conversions work", {
  # Test atomic types and vectors
  expect_equal(try_double_scalar(.45), .45)
  expect_equal(try_double_scalar(15L), 15)
  expect_error(try_double_scalar(TRUE), "Expected Numeric, got Logical")
  expect_error(try_double_scalar("abcxyz"), "Expected Numeric, got String")
  expect_error(try_double_scalar(NA), "Expected Numeric, got Logical")
  expect_error(try_double_scalar(NA_real_), "Must not be NA")
  expect_error(try_double_scalar(c(.45, .46)), "Expected Scalar, got Real")
  
  expect_equal(try_int_scalar(15L), 15L)
  expect_equal(try_int_scalar(4.4), 4L)
  expect_error(try_int_scalar(TRUE), "Expected Numeric, got Logical")
  expect_error(try_int_scalar("abcxyz"), "Expected Numeric, got String")
  expect_error(try_int_scalar(NA), "Expected Numeric, got Logical")
  expect_error(try_int_scalar(NA_integer_), "Must not be NA")
  expect_error(try_int_scalar(1L:5L), "Expected Scalar, got Integer")
  
  expect_equal(try_bool_scalar(TRUE), TRUE)
  expect_equal(try_bool_scalar(FALSE), FALSE)
  # minor inconsistencies in commas across these errors
  expect_error(try_bool_scalar(.45), "Expected Logical got Real")
  expect_error(try_bool_scalar(15L), "Expected Logical got Integer")
  expect_error(try_bool_scalar("abcxyz"), "Expected Logical got String")
  expect_error(try_bool_scalar(NA), "Must not be NA")
  expect_error(try_bool_scalar(c(TRUE, FALSE, TRUE)), "Expected Scalar, got Logical")
  
  expect_equal(try_char_scalar("abcxyz"), "abcxyz")
  # minor inconsistencies in commas across these errors
  expect_error(try_char_scalar(.45), "Expected String got Real")
  expect_error(try_char_scalar(15L), "Expected String got Integer")
  expect_error(try_char_scalar(TRUE), "Expected String got Logical")
  expect_error(try_char_scalar(NA_character_), "Must not be NA")
  expect_error(try_char_scalar(NULL), "Expected String got Null")
  # awkward error message
  # aside: should rtype not be Character?
  expect_error(try_char_scalar(c("hello", "world")), "Expected String got String") 

  expect_equal(try_double_vec(c(0, 1)), c(0, 1))
  expect_equal(try_double_vec(c(0, NA_real_)), c(0, NA)) # R type conversion
  expect_equal(try_double_vec(numeric()), numeric()) 
  expect_false(identical(try_double_vec(NA_real_), NA))
  # TODO: these fail. typo in extendr-api/src/robj/try_from_robj.rs l:123 
  expect_error(try_double_vec(c("more", "hooey")), "Expected Real got String")
  expect_error(try_double_vec(15L), "Expected Real got Integer")
  expect_error(try_double_vec(TRUE), "Expected Real got Logical")
  expect_error(try_double_vec(NA), "Expected Real got Logical")
  expect_error(try_double_vec(NULL), "Expected Real got Null")

  expect_equal(try_int_vec(c(0L, 1L)), c(0L, 1L))
  expect_equal(try_int_vec(c(0L, TRUE)), c(0L, 1L)) # R type conversion
  expect_equal(try_int_vec(c(0L, NA_integer_)), c(0L, NA_integer_)) 
  expect_equal(try_int_vec(integer()), integer()) 
  expect_error(try_int_vec(c(0L, 0)), "Expected Integer got Real")
  expect_error(try_int_vec(TRUE), "Expected Integer got Logical") 
  expect_error(try_int_vec(.45), "Expected Integer got Real")
  expect_error(try_int_vec(NA_character_), "Expected Integer got String")
  expect_error(try_int_vec(NULL), "Expected Integer got Null")

  # See note above about String here instead of Character
  expect_equal(try_char_vec(c("hello", "world")), c("hello", "world"))
  expect_equal(try_char_vec(character()), character())
  expect_error(try_char_vec(.45), "Expected String got Real")
  expect_error(try_char_vec(15L), "Expected String got Integer")
  expect_error(try_char_vec(TRUE), "Expected String got Logical")
  # Is it indended that character vectors reject NA when others do not? see above
  expect_error(try_char_vec(NA_character_), "Must not be NA")
  expect_error(try_char_vec(c("hello", NA)), "Must not be NA")

  # Test matrices and arrays
  # TODO:

  # Test non-atomic types
  x <- list(a = 1, b = NA_integer_, u = data.frame(a = 1:4),
            v = c(1, 2), z = "who", f = function(x) x^2)

  x_noname <- x
  names(x_noname) <- NULL

  e <- new.env()
  assign("first", 1)


  # conversion does not preserve list order
  expect_mapequal(try_list_str_hash(x), x)
  # TODO: unnamed list returns empty list
  # TryFrom for HashMap should fail for unnamed lists?
  expect_setequal(try_list_str_hash(x_noname), x_noname)
  expect_setequal(try_list_str_hash(list()), list())
  expect_error(try_list_str_hash(20:30), "Expected List got Integer")
  expect_error(try_list_str_hash(NA), "Expected List got Logical")
  # TODO: typo in 'Enviroment'
  expect_error(try_list_str_hash(e), "Expected List got Enviroment")


  # Forced failures for unimplemented conversions
  expect_success(try_list_string_hash())
  expect_success(try_bool_vec())
})
