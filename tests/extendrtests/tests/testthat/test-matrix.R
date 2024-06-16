dims <- list(c("a", "b", "c"), c("d", "e", "f"))

m <- matrix(
  as.double(1:9),
  nrow = 3,
  dimnames = dims
)

# test_that("Dimnames can be fetched", {
#   expect_equal(fetch_dimnames(m), dims)
# })

test_that("Rownames can be fetched", {
  # expect_equal(rownames(m), dims[[1]])
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
