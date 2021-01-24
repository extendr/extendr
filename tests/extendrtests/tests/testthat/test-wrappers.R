test_that("Wrapper code is up-to-date", {
  # if this test fails check whether the wrapper code needs 
  # updating via make_wrappers()
  x <- .Call(
    "wrap__make_extendrtests_wrappers",
    use_symbols = TRUE,
    package_name = "extendrtests"
  )
  x <- strsplit(x, "\n")[[1]]
  
  # testthat run locally?
  tmp <- file.path("..", "..", "R", "extendr-wrappers.R")
  if (file.exists(tmp)) {
    source <- tmp
  } else {
    # testthat run as part of R CMD check
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

test_that("Call to Rust via wrapper functions works", {
  expect_equal(hello_world(), "Hello world!")
})
