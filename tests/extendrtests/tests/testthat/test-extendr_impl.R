test_that("return self reference", {
  x <- Wrapper$new()
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

test_that("method should return the right reference", {
  x <- Wrapper$new()
  y <- Wrapper$new()
  x$set_a(10)
  y$set_a(42)
  expect_false(isTRUE(identical(x, y)))

  max_class <- x$max_ref(y)
  expect_identical(max_class, y)
  expect_false(isTRUE(identical(x, max_class)))
})

test_that("return the right reference with other parameters present", {
  x <- Wrapper$new()
  y <- Wrapper$new()
  x$set_a(10)
  y$set_a(42)

  # check if you can have regular arguments as well
  x <- Wrapper$new()
  x$set_a(22)
  max_class <- x$max_ref_offset(y, 30)
  expect_identical(max_class, y)
  expect_false(isTRUE(identical(x, max_class)))

  # check use of &Self as other
  max_class <- x$max_ref2(y)
  expect_identical(max_class, y)
  expect_false(isTRUE(identical(x, max_class)))
})

test_that("submodule impl for Wrapper exists", {
  x <- Wrapper$new()
  x$set_a(10)
  expect_identical(x$a_10(), 20L)
})

test_that("enum wrapper for Animal works", {
  cat <- Animal$new_cat()
  expect_identical(cat$speak(), "meow")
  dog <- Animal$new_dog()
  expect_identical(dog$speak(), "woof")
  expect_false(identical(cat, dog))
})
