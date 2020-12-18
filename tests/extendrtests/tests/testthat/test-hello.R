test_that("Call to Rust function `hello()` works", {
  expect_equal(hello(), "hello")
})
