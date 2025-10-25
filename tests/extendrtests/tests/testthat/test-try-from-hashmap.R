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

test_that("HashMap with custom TryFrom<Robj> impl", {
  expect_equal(
    test_hm_custom_try_from(list(x = c(0, 1)))[c("x", "inserted_value")],
    list(x = c(0, 1), inserted_value = c(3, 0.1415))
  )
})

test_that("From<HashMap> for Robj", {
  solar_distance_rust <- test_robj_from_hashmap()

  solar_distance_r <- list(
    Mercury = 0.4,
    Venus = 0.7,
    Earth = 1.0,
    Mars = 1.5
  )

  expect_setequal(solar_distance_rust, solar_distance_r)
  expect_mapequal(solar_distance_rust, solar_distance_r)
})

test_that("From<BTreeMap> for Robj", {
  solar_distance_rust <- test_robj_from_btreemap()

  solar_distance_r <- list(
    Mercury = 0.4,
    Venus = 0.7,
    Earth = 1.0,
    Mars = 1.5
  )

  expect_setequal(solar_distance_rust, solar_distance_r)
  expect_mapequal(solar_distance_rust, solar_distance_r)
})
