# Integration Tests for Calling Extendr from an R Package

[![R build status](https://github.com/extendr/extendr/workflows/Tests/badge.svg)](https://github.com/extendr/extendr/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/license/mit/)

This package serves as a test to see whether an R package using extendr can successfully build, run, and pass `R CMD check` on all major platforms.

The Rust code that is part of the package is located here: <https://github.com/extendr/extendr/tree/master/tests/extendrtests/src/rust>

The wrapper scripts calling the Rust functions are located here:
<https://github.com/extendr/extendr/blob/master/tests/extendrtests/R/make-wrappers.R>

The test functions that verify that the wrapper and Rust functions work correctly are located here: <https://github.com/extendr/extendr/blob/master/tests/extendrtests/tests/testthat/test-wrappers.R>

## Running tests locally

The `Cargo.toml` file hard-codes the relative path of the `extendr` libraries. You can build and install `extendrtests` from RStudio as normal using the menu items in the "Build" menu. However, "Check" does not work. To check this project locally you need to run:

```r
rcmdcheck::rcmdcheck(check_dir = "../../../")
```

This is necessary so the relative paths in `Cargo.toml` points to the correct location.

Clicking "Check" in RStudio will not work because it runs `devtools::check()` (that resolves to `rcmdcheck::rcmdcheck()`) with no arguments and the checks are then run in a temporary directory that doesn't have the same relative path.

## Running tests via `cargo`

It is also possible to run `R CMD check` in the root directory of `extendr`,
by invoking `cargo extendr`:

```shell
cargo extendr r-cmd-check
```
