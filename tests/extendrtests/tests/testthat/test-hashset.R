test_that("character vector converts to HashSet", {
  expect_equal(receive_hashset(character()), character())
  expect_equal(receive_hashset(c("beta", "alpha", "beta")), c("alpha", "beta"))
})
