test_that("Submodule functions can be called", {
  expect_equal(hello_submodule(), "Hello World!")
})

test_that("Classes defined in submodules also works", {
  x <- MySubmoduleClass$new()
  x$set_a(10)
  expect_equal(x$a(), 10)
})
