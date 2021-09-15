test_that("Access elements of double vector", {
    x <- c(42.0, NA_real_, 100.0)
    expect_equal(get_doubles_element(x, 0), 42.0)
    expect_equal(get_doubles_element(x, 2), 100.0)
    # Retrieving NA
    expect_true(is.na(get_doubles_element(x, 1)))
    # OOB returns NA
    expect_true(is.na(get_doubles_element(x, 3)))
})

test_that("Access elements of integer vector", {
    x <- c(42L, NA_integer_, 100L)
    expect_equal(get_integers_element(x, 0), 42L)
    expect_equal(get_integers_element(x, 2), 100L)
    # Retrieving NA
    expect_true(is.na(get_integers_element(x, 1)))
    # OOB returns NA
    expect_true(is.na(get_integers_element(x, 3)))
})

test_that("Construct double vector from squares of given values", {
    x <- c(1.0 * (1:100))
    expect_equal(doubles_square(x), x * x)
})

test_that("Construct integer vector from squares of given values", {
    x <- c(1:100)
    expect_equal(integers_square(x), x * x)
})

test_that("Double argument type safety", {
    expect_error(doubles_square(1L))
})

test_that("Integer argument type safety", {
    expect_error(integers_square(1.5))
})