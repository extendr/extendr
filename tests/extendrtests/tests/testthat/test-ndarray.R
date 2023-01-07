test_that("[ndarray] Euclidean distance", {
    m <- matrix(1.0 * 1:6, nrow = 3, ncol = 2)
    dist <- euclidean_dist(m)

    expect_equal(dist, c(sqrt(2), 2 * sqrt(2), sqrt(2)))
})

test_that("[ndarray] Euclidean distance returns `NULL` when `NULL` is passed", {
    dist <- euclidean_dist(NULL)

    expect_equal(dist, NULL)
})
