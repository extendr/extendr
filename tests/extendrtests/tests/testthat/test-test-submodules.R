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

  expect_identical(x$me_ref(), x)
  expect_identical(x$me_mut(), x)
  expect_identical(x$me_explicit_ref(), x)
  expect_identical(x$me_explicit_mut(), x)
  # this "copies"
  expect_false(
    isTRUE(identical(x$me_owned(), x))
  )
})

test_that("Return the right externalptr", {
  x <- MySubmoduleClass$new()
  y <- MySubmoduleClass$new()
  x$set_a(10)
  y$set_a(42)
  expect_false(isTRUE(identical(x, y)))

  max_class <- x$max_ref(y)
  expect_identical(max_class, y)
  expect_false(isTRUE(identical(x, max_class)))

  # check if you can have regular arguments as well
  x <- MySubmoduleClass$new()
  x$set_a(22)
  x$max_ref_offset(y, 30)
})
