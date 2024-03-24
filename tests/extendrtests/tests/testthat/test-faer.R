test_that("[faer] Matrix of Reals with NA", {
    m <- matrix(1:9, ncol = 3)
    m[1, 1] <- NA
    m_faer <- faer_mat(m)
    expected <- c(NA, 2:9)

    expect_equal(as.numeric(m_faer), expected)
})

test_that("[faer] Dimensions are provided", {
    m <- matrix(1:9, ncol = 3)
    m_faer <- matrix(faer_mat(m), ncol = 3)

    expect_equal(nrow(m), nrow(m_faer))
    expect_equal(ncol(m), ncol(m_faer))
})

test_that("[faer] Returns NULL for doubles", {
    m <- matrix(1:9, ncol = 3)
    m_faer <- faer_mat(as.double(m))
    expected <- c(1:9)

    expect_equal(m_faer, NULL)
})
