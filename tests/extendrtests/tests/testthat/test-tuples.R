test_that("tuple conversions work", {
  # normal types
  expect_identical(sum_triplet_ints(as.list(1:12)), 3L)

  # using try from implementation
  expect_identical(
    sum_points(list(c(0.25, 1.3), c(10.25, 11.2))),
    c(10.5, 12.5)
  )

  # error works as anticipated
  expect_error(sum_triplet_ints(as.list(1:3)), "Expected length")
})

test_that("tuple array f64", {
  x <- rnorm(4)
  expect_identical(round_trip_array_f64(x), x)
  expect_error(round_trip_array_f64(x[1:3]), "Expected length")
  expect_error(round_trip_array_f64(rnorm(10)))
})

test_that("tuple array Rfloat", {
  x <- rnorm(4)
  x[2] <- NA
  expect_identical(round_trip_array_rfloat(x), x)
  expect_error(round_trip_array_rfloat(x[1:3]), "Expected length")
  expect_error(round_trip_array_rfloat(rep(x, 3)))
})


test_that("tuple array Rint", {
  x <- c(0L, NA, 2L, 1L)
  expect_identical(round_trip_array_rint(x), x)
  expect_error(round_trip_array_rint(x[1:3]), "Expected length")
  expect_error(round_trip_array_rint(rep(x, 3)))
})

test_that("tuple array i32", {
  x <- c(0L, NA, 2L, 1L)
  expect_identical(round_trip_array_i32(x), x)
  expect_error(round_trip_array_i32(x[1:3]), "Expected length")
  expect_error(round_trip_array_i32(rep(x, 3)))
})

test_that("tuple array Rbool", {
  x <- c(TRUE, FALSE, NA, TRUE)
  expect_identical(round_trip_array_rbool(x), x)
  expect_error(round_trip_array_rbool(x[1:3]), "Expected length")
  expect_error(round_trip_array_rbool(rep(x, 3)))
})

# Create a vector of 4 complex numbers


test_that("tuple array complex", {
  x <- c(1 + 2i, NA, 5 - 6i, 7 - 8i)
  expect_identical(round_trip_array_rcplx(x), x)
  expect_error(round_trip_array_rcplx(x[1:3]), "Expected length")
  expect_error(round_trip_array_rcplx(rep(x, 3)))
})

test_that("tuple array u8", {
  x <- charToRaw("abc!")
  expect_identical(round_trip_array_u8(x), x)
  expect_error(round_trip_array_u8(x[1:3]), "Expected length")
  expect_error(round_trip_array_u8(rep(x, 3)))
})
