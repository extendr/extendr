test_that("Macro expansion of lib.rs", {
  # skip_if_no_cargo_expand()
  expansion <- processx::run(
    "cargo",
    args = c("expand", "--manifest-path", "../../src/rust/Cargo.toml"),
    error_on_status = FALSE
  )
  stop(c(expansion$stdout, expansion$stderr))
  expect_snapshot_output(cat(expansion$stdout), cran = TRUE)
})
