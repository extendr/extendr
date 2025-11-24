test_that("Dataframe<T> can be returned", {
  expect_identical(
    test_derive_into_dataframe(),
    data.frame(x = 0:1, y = c("abc", "xyz"))
  )

  expect_identical(
    test_into_robj_dataframe(),
    data.frame(x = 0:1, y = c("abc", "xyz"))
  )
})
