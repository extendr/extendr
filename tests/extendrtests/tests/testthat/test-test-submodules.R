test_that("Submodule functions can be called", {
  expect_equal(hello_submodule(), "Hello World!")
})

test_that("Classes defined in submodules also works", {
  x <- MySubmoduleClass$new()
  x
  # TODO: uncomment these pointer check tests when externalptr_address
  # is implemented
  # ptr <- \(xptr) Rcpp:::externalptr_address(xptr)
  # xptr <- ptr(x)
  x$set_a(10)
  expect_equal(x$a(), 10)
  
  # expect_equal(ptr(x$me_ref()), xptr)
  # expect_equal(ptr(x$me_mut()), xptr)
  # expect_equal(ptr(x$me_explicit_ref()), xptr)
  # expect_equal(ptr(x$me_explicit_mut()), xptr)
  # 
  # # this "copies"
  # expect_true(ptr(x$me_owned()) != xptr)
})
