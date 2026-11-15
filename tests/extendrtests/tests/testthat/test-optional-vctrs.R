test_that("`make_points()` returns a `Points` vctr", {
  x <- make_points(c(1, 2, NA, 4), c(5, 6, 7, 8))

  expect_s3_class(x, c("Points", "extendr_vctr"), exact = TRUE)
  expect_identical(as.integer(unclass(x)), 1:4)
  expect_identical(length(x), 4L)
  expect_identical(format(x), c("(1.0, 5.0)", "(2.0, 6.0)", "NA", "(4.0, 8.0)"))
})

test_that("`[` subsets a `Points` vctr", {
  x <- make_points(c(1, 2, NA, 4), c(5, 6, 7, 8))

  expect_s3_class(x[2], "Points")
  expect_identical(format(x[c(1L, 4L)]), c("(1.0, 5.0)", "(4.0, 8.0)"))
  expect_identical(format(x[c(1, NA, 10)]), c("(1.0, 5.0)", "NA", "NA"))
})

test_that("`c()` and `vctrs::vec_c()` combine `Points` vctrs", {
  x <- make_points(c(1, 2, NA, 4), c(5, 6, 7, 8))

  expect_identical(format(c(x[1], x[4])), c("(1.0, 5.0)", "(4.0, 8.0)"))
  expect_identical(format(vctrs::vec_c(x[1], x[2])), c("(1.0, 5.0)", "(2.0, 6.0)"))
})
