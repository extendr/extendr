test_that("result = 'list' encodes ok/err slots", {
  res_ok <- extendrtests:::result_list_ok()
  expect_equal(res_ok$ok, 123L)
  expect_true(is.null(res_ok$err))

  res_err <- extendrtests:::result_list_err()
  expect_true(is.null(res_err$ok))
  expect_equal(res_err$err, "list mode oops")
})

test_that("result = 'condition' encodes extendr_error", {
  res_ok <- extendrtests:::result_condition_ok()
  expect_equal(res_ok, 321L)

  err <- tryCatch(
    extendrtests:::result_condition_err(),
    error = function(e) e
  )

  expect_true(inherits(err, "extendr_error"))
  expect_equal(err$value, "condition mode oops")
})
