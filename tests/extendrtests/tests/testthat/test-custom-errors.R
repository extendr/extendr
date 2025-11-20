test_that("Function with custom error type", {
    # remove any extendr-backtrace env vars
    Sys.unsetenv("EXTENDR_BACKTRACE")

    custom_error_return()
    custom_error_conversion(NULL)

    b_like = list(`.0` = 41.);
    expect_equal(take_and_return_B(b_like), list(`.0`=42.))
})
