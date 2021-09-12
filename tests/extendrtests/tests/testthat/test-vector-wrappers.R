test_that("Can access vector elements", {
    x <- c(42.0, NA_real_)
    expect_equal(get_doubles_element(x, 0), 42.0)
    expect_equal(get_doubles_element(x, 1), NA_real_)
})