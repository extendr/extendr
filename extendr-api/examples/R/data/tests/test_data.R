
library(data)

input = list(
    # scalars
    an_integer = 123L,
    a_number = 2.5,
    a_string = "hello",
    a_bool = TRUE,
    a_list = list(a=1L, 2L, c=3L),

    # vectors
    an_integer_array = c(1L, 2L, 3L),
    a_number_array = c(1., 2., 3.),
    a_string_array = c("1", "2", "3"),
    a_logical_array = c(TRUE, FALSE, TRUE)
)

stopifnot(
    data(input)
)
