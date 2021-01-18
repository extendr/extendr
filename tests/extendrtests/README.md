# Integration Tests for Calling Extendr from an R Package

[![R build status](https://github.com/extendr/extendr/workflows/Tests/badge.svg)](https://github.com/extendr/extendr/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

This package serves as a test to see whether an R package using extendr can successfully build, run, and pass `R CMD check` on all major platforms.

The Rust code that is part of the package is located here: https://github.com/extendr/extendr/tree/master/tests/extendrtests/src/rust

The wrapper scripts calling the Rust functions are located here:
https://github.com/extendr/extendr/blob/master/tests/extendrtests/R/wrappers.R

The test functions that verify that the wrapper and Rust functions work correctly are located here: https://github.com/extendr/extendr/blob/master/tests/extendrtests/tests/testthat/test-wrappers.R

## Running tests locally

The Cargo.toml file hardcodes the relative path of the `extendr` libraries, thus
in order to check this project locally you need to run:

```r
devtools::check(check_dir = "check")
```

This is necessary so the relative path in Cargo.toml points to the correct
location. **Note**: Clicking 'Check' in RStudio will not work because it runs
`devtools::check()` with no arguments and the check are then run in a temporary
directory that doesn't have the same relative path.
