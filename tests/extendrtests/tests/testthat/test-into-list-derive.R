test_that("IntoList derive works with basic types", {
  result <- make_basic_struct()

  expect_true(is.list(result))
  expect_equal(length(result), 4)
  expect_equal(names(result), c("int_field", "double_field", "bool_field", "string_field"))

  expect_equal(result$int_field, 42L)
  expect_equal(result$double_field, 3.1415927)
  expect_equal(result$bool_field, TRUE)
  expect_equal(result$string_field, "hello from rust")
})

test_that("IntoList derive works with R wrapper types (Doubles, Logicals, Strings, Raw)", {
  result <- make_rwrapper_struct()

  expect_true(is.list(result))
  expect_equal(length(result), 4)
  expect_equal(names(result), c("doubles", "logicals", "strings", "raw"))

  expect_equal(result$doubles, c(1.0, 2.0, 3.0))
  expect_equal(result$logicals, c(TRUE, FALSE, TRUE))
  expect_equal(result$strings, c("alpha", "beta", "gamma"))
  expect_equal(result$raw, as.raw(c(0xDE, 0xAD, 0xBE, 0xEF)))
})

test_that("IntoList derive works with List field", {
  result <- make_with_list()

  expect_true(is.list(result))
  expect_equal(length(result), 2)
  expect_equal(names(result), c("name", "data"))

  expect_equal(result$name, "my_list")
  expect_true(is.list(result$data))
  expect_equal(result$data, list(x = 1, y = 2, z = 3))
})

test_that("IntoList derive works with Robj field", {
  result <- make_with_robj()

  expect_true(is.list(result))
  expect_equal(length(result), 2)
  expect_equal(names(result), c("label", "value"))

  expect_equal(result$label, "answer")
  expect_equal(result$value, 42)
})

test_that("IntoList derive works with Function field", {
  result <- make_with_function()

  expect_true(is.list(result))
  expect_equal(length(result), 2)
  expect_equal(names(result), c("func_name", "func"))

  expect_equal(result$func_name, "sum")
  expect_true(is.function(result$func))
  expect_identical(result$func, sum)
})

test_that("IntoList derive works with Pairlist field", {
  result <- make_with_pairlist()

  expect_true(is.list(result))
  expect_equal(length(result), 2)
  expect_equal(names(result), c("description", "pairs"))

  expect_equal(result$description, "pairlist container")
  expect_true(is.pairlist(result$pairs))
  expect_equal(length(result$pairs), 3)
})

test_that("IntoList derive works with Environment field", {
  result <- make_with_environment()

  expect_true(is.list(result))
  expect_equal(length(result), 2)
  expect_equal(names(result), c("env_name", "env"))

  expect_equal(result$env_name, "my_environment")
  expect_true(is.environment(result$env))
  expect_equal(result$env$x, 100)
  expect_equal(result$env$y, "test")
})

test_that("IntoList derive respects #[into_list(ignore)] attribute", {
  result <- make_with_ignored()

  expect_true(is.list(result))
  # Should only have 2 visible fields, not 4
  expect_equal(length(result), 2)
  expect_equal(names(result), c("visible_name", "visible_count"))

  expect_equal(result$visible_name, "public data")
  expect_equal(result$visible_count, 99L)

  # Verify the ignored fields are NOT present
  expect_null(result$internal_ptr)
  expect_null(result$private_buffer)
})

test_that("IntoList derive works with vector fields", {
  result <- make_with_vectors()

  expect_true(is.list(result))
  expect_equal(length(result), 4)
  expect_equal(names(result), c("int_vec", "double_vec", "string_vec", "bool_vec"))

  expect_equal(result$int_vec, 1L:5L)
  expect_equal(result$double_vec, c(1.1, 2.2, 3.3))
  expect_equal(result$string_vec, c("one", "two", "three"))
  expect_equal(result$bool_vec, c(TRUE, FALSE, TRUE, TRUE, FALSE))
})

test_that("IntoList derive works with nested structs", {
  result <- make_nested_struct()

  expect_true(is.list(result))
  expect_equal(length(result), 3)
  expect_equal(names(result), c("name", "count", "nested_data"))

  expect_equal(result$name, "outer")
  expect_equal(result$count, 2L)

  # Check nested structure
  expect_true(is.list(result$nested_data))
  expect_equal(length(result$nested_data), 2)
  expect_equal(names(result$nested_data), c("x", "y"))
  expect_equal(result$nested_data$x, 10.5)
  expect_equal(result$nested_data$y, 20.5)
})

test_that("IntoList derive works with function metadata example", {
  result <- make_function_metadata()

  expect_true(is.list(result))
  # Should have 6 fields (func_ptr is ignored)
  expect_equal(length(result), 6)
  expect_equal(names(result), c("doc", "rust_name", "r_name", "return_type", "num_args", "is_hidden"))

  expect_equal(result$doc, "Example function documentation")
  expect_equal(result$rust_name, "example_fn")
  expect_equal(result$r_name, "exampleFn")
  expect_equal(result$return_type, "Robj")
  expect_equal(result$num_args, 3L)
  expect_equal(result$is_hidden, FALSE)

  # Verify func_ptr is NOT present
  expect_null(result$func_ptr)
})

test_that("IntoList derive works with all R types in one struct", {
  result <- make_all_r_types()

  expect_true(is.list(result))
  expect_equal(length(result), 9)
  expect_equal(names(result), c(
    "doubles_field", "logicals_field", "raw_field", "strings_field",
    "list_field", "robj_field", "function_field", "pairlist_field",
    "environment_field"
  ))

  # Verify Doubles field
  expect_equal(result$doubles_field, c(1.0, 2.0))

  # Verify Logicals field
  expect_equal(result$logicals_field, c(TRUE, FALSE))

  # Verify Raw field
  expect_equal(result$raw_field, as.raw(c(0x01, 0x02)))

  # Verify Strings field
  expect_equal(result$strings_field, c("a", "b"))

  # Verify List field
  expect_true(is.list(result$list_field))
  expect_equal(result$list_field, list(x = 1))

  # Verify Robj field
  expect_equal(result$robj_field, 42)

  # Verify Function field
  expect_true(is.function(result$function_field))
  expect_identical(result$function_field, mean)

  # Verify Pairlist field
  expect_true(is.pairlist(result$pairlist_field))

  # Verify Environment field
  expect_true(is.environment(result$environment_field))
  expect_equal(result$environment_field$test, "value")
})
