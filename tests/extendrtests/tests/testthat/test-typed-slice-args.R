test_that("mutable integer slice", {
  x <- 1:9
  middle_zero(x)
  expect_equal(x[5], 42)
})

test_that("numeric slice argument", {
  x <- as.numeric(1:9)
  expect_equal(floats_mean(x), mean(x))
})

test_that("Rbool slice argumet", {
  x <- c(TRUE, FALSE, TRUE, FALSE, TRUE)
  expect_equal(logicals_sum(x), 3L)
})
