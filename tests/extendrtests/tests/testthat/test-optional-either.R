patrick::with_parameters_test_that(
    "`type_aware_sum()` is type-stable and computes correct sums given",
    {
        result <- type_aware_sum(input)

        vctrs::vec_assert(result, ptype = expected_ptype, size = 1L)
        expect_equal(sum(input), result)
    },

    patrick::cases(
        ALTINT = list(input = 1:10, expected_ptype = integer()),
        INTEGER = list(input = c(1L, 2L, 3L, 4L, 5L, 6L, 7L, 8L, 9L, 10L), expected_ptype = integer()),
        DOUBLE = list(input = c(1, 2, 3, 4, 5, 6, 7, 8, 9, 10), expected_ptype = double()),
        `Integers with NA` = list(input = c(1:5, NA_integer_), expected_ptype = integer())
    )
)

patrick::with_parameters_test_that(
    "`type_aware_sum()` does not accept types other than integers or doubles as input, given",
    {
        expect_error(type_aware_sum(input))
    },
    patrick::cases(
        LOGICAL = list(input = c(TRUE, FALSE, NA)),
        CHARACTER = list(input = letters),
        COMPLEX = list(input = 1:10 + 1i * 11:20),
        `NULL` = list(input = NULL),
        `DATA.FRAME` = list(input = mtcars),
        LIST = list(input = lapply(1:10, identity))
    )
)