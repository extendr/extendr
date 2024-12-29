test_that("unsafe externalptr", {
  msg <- "Hello World!"
  unsafe_externalptr <- charToRaw(msg)
  unsafe_externalptr <- .Internal(address(unsafe_externalptr))

  expect_equal(
    unsafe_externalptr_to_strings(unsafe_externalptr),
    msg
  )
})


test_that("raw as externalptr", {
  msg <- "Hello World!"
  raw_msg <- charToRaw(msg)
  msg_as_externalptr <- .Internal(address(raw_msg))

  expect_equal(
    externalptr_as_raw(msg_as_externalptr),
    msg
  )
})
