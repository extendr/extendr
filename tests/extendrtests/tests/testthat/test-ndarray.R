test_that("[ndarray] Euclidean distance", {
    m <- matrix(1.0 * 1:100, nrow = 10, ncol = 10)
    dist <- euclidean_dist(m)
    expected_dist <- dist(m, method = "euclidean") |> as.numeric()

    expect_equal(dist, expected_dist)
})

test_that("[ndarray] Euclidean distance returns `NULL` when `NULL` is passed", {
    dist <- euclidean_dist(NULL)

    expect_equal(dist, NULL)
})
