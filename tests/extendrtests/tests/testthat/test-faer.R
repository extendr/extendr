test_that("[faer] Dimensions are provided", {
    m <- matrix(rnorm(9), ncol = 3)
    m_faer <- rmat_to_mat(m)

    expect_equal(nrow(m), nrow(m_faer))
    expect_equal(ncol(m), ncol(m_faer))
})

library(testthat)

m <- matrix(rnorm(100), ncol = 10)
expect_identical(m, mat_to_mat(m))
expect_identical(m, mat_to_rmat(m))
expect_identical(m, mat_to_robj(m))
expect_identical(m, mat_to_rmatfloat(m))
expect_identical(m, rmat_to_mat(m))
expect_identical(m, robj_to_mat(m))
expect_identical(m, matref_to_mat(m))
