test_that("check_user_interrupt() turns Ctrl-C into an R interrupt condition", {
  before <- interrupt_drop_count()
  res <- tryCatch(
    interrupt_loop(100L, 3L),
    interrupt = function(e) "interrupted"
  )
  expect_identical(res, "interrupted")
  # the Rust destructor ran before the interrupt was handed to R
  expect_identical(interrupt_drop_count(), before + 1L)
  # the interrupt was consumed: R is not left with a pending interrupt
  expect_identical(interrupt_loop(10L, -1L), 10L)
})

test_that("check_user_interrupt() is a no-op when nothing is pending", {
  before <- interrupt_drop_count()
  expect_identical(interrupt_loop(10L, -1L), 10L)
  expect_identical(interrupt_drop_count(), before + 1L)
})

test_that("calling handlers see the interrupt and can let it continue to top level", {
  seen <- NULL
  res <- tryCatch(
    withCallingHandlers(
      interrupt_loop(100L, 5L),
      interrupt = function(e) seen <<- class(e)
    ),
    interrupt = function(e) "interrupted"
  )
  expect_identical(res, "interrupted")
  expect_true("interrupt" %in% seen)
})

test_that("interrupt_requested() lets Rust return a partial result", {
  before <- interrupt_drop_count()
  expect_identical(interrupt_loop_partial(100L, 3L), 3L)
  expect_identical(interrupt_drop_count(), before + 1L)
  expect_identical(interrupt_loop_partial(10L, -1L), 10L)
})
