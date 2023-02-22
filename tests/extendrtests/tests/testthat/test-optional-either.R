test_that("`type_aware_sum` works over ALTINT", {
    input <- 1:10

    result <- type_aware_sum(input)

    vctrs::vec_assert(result, ptype = integer(), size = 1L)
    expect_equal(sum(input), result)
})

test_that("`type_aware_sum` works over INTEGER", {
    input <- c(1L, 2L, 3L, 4L, 5L, 6L, 7L, 8L, 9L, 10L)

    result <- type_aware_sum(input)

    vctrs::vec_assert(result, ptype = integer(), size = 1L)
    expect_equal(sum(input), result)
})

test_that("`type_aware_sum` works over DOUBLE", {
    input <- c(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)

    result <- type_aware_sum(input)

    vctrs::vec_assert(result, ptype = double(), size = 1L)
    expect_equal(sum(input), result)
})