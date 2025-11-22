test_that("String as HashMap key & Robj T", {
  expect_equal(
    test_hm_string(list(x = 10))[c("inserted_value", "x")],
    list(inserted_value = list(), x = 10)
  )
})

test_that("String as Key and i32 as T", {
  expect_identical(
    test_hm_i32(list()),
    list(inserted_value = 314L)
  )
})

test_that("HashMap TryFrom works both directions", {
  hm <- list(x = 1, y = letters)
  res <- test_try_from_hm(hm)

  expect_equal(sort(names(res)), names(hm))

  for (nm in names(hm)) {
    expect_identical(res[[nm]], hm[[nm]])
  }
})

test_that("HashMap with custom TryFrom<Robj> impl", {
  expect_equal(
    test_hm_custom_try_from(list(x = c(0, 1)))[c("x", "inserted_value")],
    list(x = c(0, 1), inserted_value = c(3, 0.1415))
  )
})
