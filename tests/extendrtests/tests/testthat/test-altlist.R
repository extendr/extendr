# Results in a panic!
# thread '<unnamed>' panicked at .../extendr/extendr-api/src/wrapper/altrep.rs:544:13:
#   misaligned pointer dereference: address must be a multiple of 0x8 but is 0x1f100000d
# note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
# thread caused non-unwinding panic. aborting.

# test_that("ALTLIST creation works", {
#   x <- new_usize(c(1L, NA, 99L))
#   expect_true(is.list(x))
#   expect_length(x, 3)
# })
# 

# Results in the same panic 
# test_that("ALTINTEGER creation works", {
#   x <- tst_altinteger()
#   expect_true(is.integer(x))
#   expect_length(x, 10) 
# })
# 

# Results in a memory leak test failure
# test_that("ALTSTRING creation works", {
#   x <- tst_altstring()
#   expect_true(is.character(x))
#   expect_length(x, 10)
# })
