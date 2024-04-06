test_that("Submodule functions can be called", {
  expect_equal(hello_submodule(), "Hello World!")
})

test_that("Classes defined in submodules also works", {
  x <- MySubmoduleClass$new()
  x$set_a(10)
  expect_equal(x$a(), 10)

  expect_equal(x$me_ref(), x)
  expect_equal(x$me_mut(), x)
  expect_equal(x$me_explicit_ref(), x)
  expect_equal(x$me_explicit_mut(), x)
  # this "copies"
  expect_false(
    isTRUE(identical(x$me_owned(), x))
  )
})
