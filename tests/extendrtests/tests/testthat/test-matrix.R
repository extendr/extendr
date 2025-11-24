dims <- list(c("a", "b", "c"), c("d", "e", "f"))

m <- matrix(
  as.double(1:9),
  nrow = 3,
  dimnames = dims
)

test_that("Dimnames can be fetched", {
  expect_equal(fetch_dimnames(m), dims)
})

test_that("Rownames can be fetched", {
  expect_equal(fetch_rownames(m), dims[[1]])
})

test_that("Rownames can be fetched", {
  expect_equal(fetch_colnames(m), dims[[2]])
})

test_that("Dimnames can be changed", {
  m2 <- m
  dimnames(m2) <- list(c("AA", "BB", "CC"), NULL)
  expect_equal(change_dimnames(m), m2)
})

test_that("Matrix3D with non-square shape is returned as is", {
  m3d <- array(as.double(1:(2*3*4)), dim = c(2, 3, 4))
  expect_equal(matrix_3d_return(m3d), m3d)
})

test_that("Matrix4D with non-square shape is returned as is", {
  m3d <- array(as.double(1:(2*3*4*5)), dim = c(2, 3, 4, 5))
  expect_equal(matrix_4d_return(m3d), m3d)
})

test_that("Matrix4D should error when dimensions are different", {
  m3d <- array(as.double(1:(2*3*4*5*6)), dim = c(2, 3, 4, 5, 6))
  expect_error(matrix_4d_return(m3d))
})

test_that("Matrix5D with non-square shape is returned as is", {
  m3d <- array(as.double(1:(2*3*4*5*6)), dim = c(2, 3, 4, 5, 6))
  expect_equal(matrix_5d_return(m3d), m3d)
})

test_that("Matrix5D should error when dimensions are different", {
  m3d <- array(as.double(1:(2*3*4*5)), dim = c(2, 3, 4, 5))
  expect_error(matrix_5d_return(m3d))
})
