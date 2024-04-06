test_that("Exported class works", {
  x <- MyClass$new()
  expect_equal(x$a(), 0L)
  expect_equal(x[["a"]](), 0L)
  x$set_a(10L)
  expect_equal(x$a(), 10L)
  expect_equal(x[["a"]](), 10L)

  expect_visible(x$a())
  expect_invisible(x$set_a(5L))
})

test_that("Exported class self ptr works", {
  x <- MyClass$new()
  expect_equal(x$me(), x)
  expect_equal(x[["me"]](), x)
})

test_that("Unexported class works", {
  # unexported code works in testthat tests
  x <- MyClassUnexported$new()
  expect_equal(x$a(), 22L)
  expect_equal(x[["a"]](), 22L)
})

test_that("Issue 431: Restore struct as ExternalPtr", {
  x <- MyClass$new()
  x$set_a(42L)

  y <- MyClass$restore_from_robj(x)
  expect_equal(x$a(), y$a())
})
