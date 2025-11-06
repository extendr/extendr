test_that("interrupts are respected", {
  expect_equal(
    tryCatch(
      eval(test_signal(), envir = new.env()),
      interrupt = function(e) {
        "interrupted"
      }
    ),
    "interrupted"
  )
})
