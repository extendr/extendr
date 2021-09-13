test_that("Access elements of double vector", {
    x <- c(42.0, NA_real_, 100.0)
    expect_equal(get_doubles_element(x, 0), 42.0)
    expect_equal(get_doubles_element(x, 1), NA_real_)
    expect_equal(get_doubles_element(x, 3), NA_real_)
})

test_that("Access elements of integer vector", {
    x <- c(42L, NA_integer_, 100L)
    expect_equal(get_integers_element(x, 0), 42L)
    expect_equal(get_integers_element(x, 1), NA_integer_)
    expect_equal(get_integers_element(x, 3), NA_integer_)
})

test_that("Construct double vector from squares of given values", {
    x <- c(1.0 * (1:100), rep(NA_real_, 5), rep(NaN, 5))
    x <- sample(x, length(x))

    expect_equal(doubles_square(x), x * x)
})

test_that("Construct integer vector from squares of given values", {
    x <- c(1:100, rep(NA_integer_, 5))
    x <- sample(x, length(x))

    expect_equal(integers_square(x), x * x)
})

test_that("Double argument type safety", {
    expect_error(doubles_square(1L))
})

test_that("Integer argument type safety", {
    expect_error(integers_square(1.5))
})