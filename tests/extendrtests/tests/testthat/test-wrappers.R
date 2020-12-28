test_that("Call to Rust via wrapper functions works", {
  expect_equal(hello_world(), "Hello world!")
})
