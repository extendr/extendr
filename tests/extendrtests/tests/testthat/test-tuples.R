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
