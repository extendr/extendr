test_that("vctrs vectors print and report length and missingness", {
  p <- points(c(1, NA, 3), c(4, 5, 6))
  expect_s3_class(p, c("Points", "extendr_vctr", "vctrs_vctr"), exact = TRUE)
  expect_length(p, 3)
  expect_equal(is.na(p), c(FALSE, TRUE, FALSE))
  expect_equal(format(p), c("(1, 4)", NA, "(3, 6)"))
  expect_equal(as.character(p), format(p))
  expect_output(print(p), "<Points[3]>", fixed = TRUE)
})

test_that("subsetting goes through vec_restore", {
  p <- points(c(1, NA, 3), c(4, 5, 6))
  expect_equal(format(p[c(3, 1)]), c("(3, 6)", "(1, 4)"))
  expect_equal(format(p[[1]]), "(1, 4)")
  expect_equal(format(rev(p)), c("(3, 6)", NA, "(1, 4)"))
  expect_length(rep(p, 2), 6)
  expect_equal(is.na(p[c(1, NA)]), c(FALSE, TRUE))
  expect_equal(format(vctrs::vec_slice(p, 2:3)), c(NA, "(3, 6)"))
  expect_length(vctrs::vec_ptype(p), 0)
  expect_equal(points_sum_x(p[3]), 3)
})

test_that("c() combines values in Rust", {
  p <- points(c(1, 2), c(3, 4))
  q <- c(p, NULL, points(5, 6))
  expect_s3_class(q, "Points")
  expect_equal(points_sum_x(q), 8)
  expect_error(c(p, labels("a")), "can't combine argument 2")
})

test_that("vctrs vectors work as data frame columns", {
  df <- data.frame(id = 1:3)
  df$p <- points(c(1, 2, 3), c(4, 5, 6))
  expect_equal(points_sum_x(df[c(1, 3), "p"]), 4)
})

test_that("r_name sets the class and the wrong type errors", {
  l <- labels(c("a", "b"))
  expect_s3_class(l, "extendr_labels")
  expect_equal(format(c(l, l)[3:4]), c("\"a\"", "\"b\""))
  expect_error(points_sum_x(l))
  expect_error(points_sum_x(1:3))
})
