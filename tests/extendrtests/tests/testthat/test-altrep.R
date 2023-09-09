test_that("ALTLIST creation works", {
  x <- new_usize(c(1L, NA, 99L))
  expect_true(is.list(x))
  expect_length(x, 3)
})

test_that("ALTINTEGER creation works", {
  x <- tst_altinteger()
  expect_true(is.integer(x))
  expect_length(x, 10) 
})

test_that("ALTSTRING creation works", {
  x <- tst_altstring()
  expect_true(is.character(x))
  expect_length(x, 10)
})
