test_that("Call to Rust functions work", {
  expect_equal(hello(), "hello")
  expect_equal(add_ints(3L, 5L), 8L)
})
