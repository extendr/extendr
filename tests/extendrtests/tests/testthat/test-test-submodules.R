test_that("submodule functions can be called", {
  expect_equal( .Call("wrap__hello_submodule"), "Hello World!")
})
