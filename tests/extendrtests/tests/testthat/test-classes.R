test_that("Exported class works", {
  x <- MyClass$new()
  expect_equal(x$a(), 0L)
  x$set_a(10L)
  expect_equal(x$a(), 10L)
  expect_equal(x$me(), x)
})

test_that("Unexported class works", {
  # unexported code works in testthat tests
  x <- MyClassUnexported$new()
  expect_equal(x$a(), 22L)
})
