test_that("Wrapper code is up-to-date", {
  # What we're doing here is generating the latest wrappers for the
  # Rust library and comparing to the wrappers file stored in the
  # package R code. There are two reasons why this test may fail:
  # 1. The wrapper code needs updating via `make_wrappers()`.
  # 2. The Rust code that generates the wrappers has a problem.
  #
  # Make sure you know which it is before running `make_wrappers()`.
  
  x <- .Call(
    "wrap__make_extendrtests_wrappers",
    use_symbols = TRUE,
    package_name = "extendrtests"
  )
  x <- strsplit(x, "\n")[[1]]
  
  # locating the file containing the R wrappers is a bit complicated,
  # because it depends on whether testthat is run locally or as part
  # of R CMD check.
  
  tmp <- file.path("..", "..", "R", "extendr-wrappers.R")
  if (file.exists(tmp)) { # testthat run locally?
    source <- tmp
  } else {
    # testthat run as part of R CMD check?
    tmp <- file.path("..", "..", "00_pkg_src", "extendrtests", "R", "extendr-wrappers.R")
    if (file.exists(tmp)) {
      source <- tmp
    } else {
      skip("Cannot locate wrapper code.")
    }
  }
  
  y <- brio::read_lines(source)
  expect_equal(x, y)
})

test_that("Rust `_arg` is correctly wrapped in R", {
  expect_invisible(special_param_names(`_y` = 42L, `_x` = 100L))
})

test_that("Call to Rust via wrapper functions works", {
  expect_equal(hello_world(), "Hello world!")
})
