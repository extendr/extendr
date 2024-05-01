#test_that("[faer] Matrix of Reals with NA", {
#    m <- matrix(1:9, ncol = 3)
#    m[1, 1] <- NA
#    m_faer <- faer_mat(m)
#    expected <- c(NA, 2:9)
#
#    expect_equal(as.numeric(m_faer), expected)
#})
#
#test_that("[faer] Dimensions are provided", {
#    m <- matrix(1:9, ncol = 3)
#    m_faer <- matrix(faer_mat(m), ncol = 3)
#
#    expect_equal(nrow(m), nrow(m_faer))
#    expect_equal(ncol(m), ncol(m_faer))
#})
#
#test_that("[faer] Returns NULL for doubles", {
#    m <- matrix(1:9, ncol = 3)
#    m_faer <- faer_mat(as.double(m))
#    expected <- c(1:9)
#
#    expect_equal(m_faer, NULL)
#})

library(testthat)

m <- matrix(rnorm(100), ncol = 10)
expect_identical(m, mat_to_rmat(m))
expect_identical(m, mat_to_robj(m))
expect_identical(m, mat_to_rmatfloat(m))
expect_identical(m, mat_to_mat(m))
expect_identical(m, rmat_to_mat(m))
expect_identical(m, robj_to_mat(m))
