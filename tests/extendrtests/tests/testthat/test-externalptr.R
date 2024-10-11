test_that("passing the wrong externalptr to rust", {
  externalptr_numeric <- create_numeric_externalptr(as.numeric(1:10))
  expect_error(
    sum_integer_externalptr(externalptr_numeric)
  )
})