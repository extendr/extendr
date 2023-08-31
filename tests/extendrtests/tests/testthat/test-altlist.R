test_that("ALTLIST creation works", {
  x <- new_usize(c(1L, NA, 99L))
  expect_true(is.list(x))
  expect_length(x, 3)
})
