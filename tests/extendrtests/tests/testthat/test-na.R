test_that("`extendr` returns correct `NA`s", {
    expect_true(is.na(try_rfloat_na()))
    expect_true(is.na(try_rint_na()))
})

test_that("`extendr` can check for `NA`", {
    expect_true(check_rfloat_na(NA_real_))
    expect_true(check_rint_na(NA_integer_))

    expect_false(check_rfloat_na(NaN))
    expect_false(check_rfloat_na(42))
    expect_false(check_rfloat_na(Inf))
    expect_false(check_rfloat_na(-Inf))

    expect_false(check_rint_na(42L))
})