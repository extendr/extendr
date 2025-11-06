test_that("Function with custom error type", {
    # remove any extendr-backtrace env vars
    Sys.unsetenv("EXTENDR_BACKTRACE")

    custom_error_return()
    custom_error_conversion(NULL)
})
