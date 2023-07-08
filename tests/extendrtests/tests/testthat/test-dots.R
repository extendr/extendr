test_that("Dots collect reamaining unnamed values", {
  result <- collect_dots(x = 42, y = "y", 1, 2, 3, 4, 5)
  expected <- list(x = 42, dots = list(1, 2, 3, 4, 5), y = "y")
  expect_equal(result, expected)
})

test_that("Dots collect reamaining partially-named values", {
  result <- collect_dots(x = 42, y = "y", 1, 2, q = 3, 4, p = 5)
  expected <- list(x = 42, dots = list(1, 2, q = 3, 4, p = 5), y = "y")
  expect_equal(result, expected)
})

test_that("Dots allow trailing comma", {
  result <- collect_dots(x = 42, y = "y", )
  expected <- list(x = 42, dots = list(), y = "y")
  expect_equal(result, expected)
})

test_that("Dots allow values and trailing comma", {
  result <- collect_dots(x = 42, y = "y", 1, 2, 3, )
  expected <- list(x = 42, dots = list(1, 2, 3), y = "y")
  expect_equal(result, expected)
})

test_that("Dots throw if missing value in the middle", {
  expect_error(collect_dots(x = 42, y = "y", 1, 2, , 3))
})

