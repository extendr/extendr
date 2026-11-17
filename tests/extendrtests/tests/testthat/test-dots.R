test_that("bullet_list() collects `...`", {
  skip_if(getRversion() < "4.6.0")
  x <- bullet_list("a", "b", NA_character_)
  expect_s3_class(x, "bullet_list")
  expect_equal(unclass(x), c("a", "b", NA))
  expect_length(bullet_list(), 0)
})

test_that("print.bullet_list() takes `...` and prints bullets", {
  skip_if(getRversion() < "4.6.0")
  x <- bullet_list("a", "b")
  expect_output(print(x), "• a\n• b")
  expect_output(print(x, symbol = "-"), "- a\n- b")
  expect_output(print(bullet_list("abcdef"), width = 3), "• abc")
  expect_error(print(x, width = "wide"))
  expect_invisible(print(x, extra = 1))
})
