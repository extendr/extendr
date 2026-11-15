test_that("`make_points()` returns a `Points` vctr", {
  x <- make_points(c(1, 2, NA, 4), c(5, 6, 7, 8))

  expect_s3_class(x, c("Points", "extendr_vctr"), exact = TRUE)
  expect_identical(as.integer(unclass(x)), 1:4)
  expect_identical(length(x), 4L)
  expect_identical(
    format(x),
    c("Point { x: 1.0, y: 5.0 }", "Point { x: 2.0, y: 6.0 }", "NA", "Point { x: 4.0, y: 8.0 }")
  )
})

test_that("`[` subsets a `Points` vctr", {
  x <- make_points(c(1, 2, NA, 4), c(5, 6, 7, 8))

  expect_s3_class(x[2], "Points")
  expect_identical(format(x[c(1L, 4L)]), c("Point { x: 1.0, y: 5.0 }", "Point { x: 4.0, y: 8.0 }"))
  expect_identical(format(x[c(1, NA, 10)]), c("Point { x: 1.0, y: 5.0 }", "NA", "NA"))
})

test_that("`my_centroid()` borrows a `Points` vctr", {
  x <- make_points(c(1, 2, NA, 6), c(5, 6, 7, 10))

  expect_identical(format(my_centroid(x)), "Point { x: 3.0, y: 7.0 }")
  expect_identical(format(my_centroid(x[3])), "NA")
})

test_that("`c()` and `vctrs::vec_c()` combine `Points` vctrs", {
  x <- make_points(c(1, 2, NA, 4), c(5, 6, 7, 8))

  expect_identical(format(c(x[1], x[4])), c("Point { x: 1.0, y: 5.0 }", "Point { x: 4.0, y: 8.0 }"))
  expect_identical(format(vctrs::vec_c(x[1], x[2])), c("Point { x: 1.0, y: 5.0 }", "Point { x: 2.0, y: 6.0 }"))
})

test_that("`Vctr<T>` uses the default `format.extendr_vctr`", {
  start <- make_points(c(0, 1, NA), c(0, 1, 2))
  end <- make_points(c(3, 1, 4), c(4, 2, 5))
  lines <- make_lines(start, end)

  expect_s3_class(lines, "extendr_vctr", exact = TRUE)
  expect_identical(as.integer(unclass(lines)), 1:3)
  expect_identical(
    format(lines),
    c("Line((0, 0) (3, 4))", "Line((1, 1) (1, 2))", "NA")
  )
})

test_that("`Vctr<T>` converts back to its inner value", {
  start <- make_points(c(0, 1, NA), c(0, 1, 2))
  end <- make_points(c(3, 1, 4), c(4, 2, 5))

  expect_identical(line_lengths(make_lines(start, end)), c(5, 1, NA))
  expect_error(line_lengths(start))
})

test_that("`print.extendr_vctr` prints a header and the formatted elements", {
  lines <- make_lines(make_points(c(0, 1, NA), c(0, 1, 2)), make_points(c(3, 1, 4), c(4, 2, 5)))
  points <- make_points(c(1, NA), c(5, 6))

  expect_output(
    expect_invisible(print(lines)),
    "<extendr_vctr[3]>\n[1] Line((0, 0) (3, 4)) Line((1, 1) (1, 2)) NA",
    fixed = TRUE
  )
  expect_output(print(points), "<Points[2]>\n[1] Point { x: 1.0, y: 5.0 } NA", fixed = TRUE)
})
