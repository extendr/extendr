# Integration Tests for Calling Extendr from an R Package

[![R build status](https://github.com/extendr/extendr/workflows/Tests/badge.svg)](https://github.com/extendr/extendr/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

This package serves as a test to see whether an R package using extendr can successfully build, run, and pass `R CMD check` on all major platforms.

The Rust code that is part of the package is located here: https://github.com/extendr/extendr/tree/master/tests/extendrtests/src/rust

The wrapper scripts calling the Rust functions are located here:
https://github.com/extendr/extendr/blob/master/tests/extendrtests/R/wrappers.R

The test functions that verify that the wrapper and Rust functions work correctly are located here: https://github.com/extendr/extendr/blob/master/tests/extendrtests/tests/testthat/test-wrappers.R