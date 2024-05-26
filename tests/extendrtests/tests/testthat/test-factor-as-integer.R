test_that("test factor as integer", {

  # expect_error(from_factor_as_integers(iris$Species))
  from_factor_as_integers(iris$Species)
  from_factor(iris$Species)

})
