test_that("Macro expansion of lib.rs", {
  skip_if_no_cargo_expand()

  # Define a custom criterion for identifying the presence of a folder named '00_pkg_src'
  contains_extendtests <- rprojroot::has_dir("00_pkg_src")

  # Combine with is_r_package (which looks for the DESCRIPTION file)
  combined_criteria <- rprojroot::is_r_package | contains_extendtests

  # Find the root directory based on the combined criteria
  root <- rprojroot::find_root(combined_criteria)

  # If we found a folder containing '00_pkg_src', then we need to go one level deeper
  if (dir.exists(file.path(root, "00_pkg_src"))) {
    root <- file.path(root, "00_pkg_src", "extendrtests")
  }

  cargo_toml_path <- file.path(root, "src", "rust", "Cargo.toml")

  result <- processx::run(
    "cargo",
    args = c("expand", "--manifest-path", cargo_toml_path)
  )
  expect_equal(result$status, 0, info = "cargo expand failed")
  expect_snapshot(cat(result$stdout), cran = TRUE)
})
