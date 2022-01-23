test_that("`my_device()` works as expected", {
    expect_output(my_device("foo"), "message from device: foo", fixed = TRUE)

    # expect no error when various drawing operations are called
    expect_silent(plot(0))

    # expect no error as well
    expect_silent(grid::grid.newpage())

    expect_output(dev.off(), "good bye...", fixed = TRUE)
})
