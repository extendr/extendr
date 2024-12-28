test_that("unsafe externalptr", {
  msg <- "Hello World!"
  unsafe_externalptr <- charToRaw(msg)
  unsafe_externalptr <- .Internal(address(unsafe_externalptr))

  expect_equal(
    unsafe_externalptr_to_strings(unsafe_externalptr),
    msg
  )
})
