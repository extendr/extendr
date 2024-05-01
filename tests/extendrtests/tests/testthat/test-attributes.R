test_that("Attributes are set correctly", {
  expect_equal(
    dbls_named(as.double(1:3)), 
    c("1" = 1.0, "2" = 2.0, "3" = 3.0)
  )

  expect_equal(
    strings_named(letters),
    setNames(letters, letters)
  )

  expect_equal(
    list_named(as.list(1:26), letters),
    structure(as.list(1:26), names = letters)
  )
})