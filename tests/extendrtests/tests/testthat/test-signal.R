test_that("interrupts are respected", {
  expect_equal(
    tryCatch(
      test_signal(),
      interrupt = function(e) {
        "interrupted"
      }
    ),
    "interrupted"
  )
})

test_that("interrupts can be suspended", {
  expect_equal(
    capture.output(
      suspendInterrupts(test_signal())
    ),
    c(
      "iteration 0",
      "iteration 1",
      "iteration 2",
      "iteration 3",
      "signaling interrupt",
      "iteration 4",
      "iteration 5",
      "iteration 6",
      "iteration 7",
      "iteration 8",
      "iteration 9"
    )
  )
})
