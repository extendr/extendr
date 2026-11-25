# Integration Tests for Calling Extendr from an R Package

[![R build
status](https://github.com/extendr/extendr/actions/workflows/post-merge.yml/badge.svg)](https://github.com/extendr/extendr/actions/workflows/post-merge.yml)
[![License:
MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/license/mit)

This R package serves as a comprehensive integration test for extendr. It 
exercises the full path from Rust through the R FFI, verifying that a package
built with extendr:

- compiles its Rust code ([src/rust](src/rust)),
- generates its R wrappers ([R/extendr-wrappers.R](R/extendr-wrappers.R)),
- passes its `{testthat}` suite ([tests/testthat](tests/testthat)), and
- passes `R CMD check` on all major platforms.

## Running tests locally

Run these [just](https://just.systems) recipes from the root of the extendr
repository:

```sh
just document       # regenerate R/extendr-wrappers.R and man/ via devtools::document()
just devtools-test  # run the {testthat} suite; FILTER=<name> runs a single file
just r-cmd-check    # run R CMD check --as-cran
```

`R CMD check` builds the package in a temporary directory, which breaks the
relative path to `extendr-api` in `src/rust/Cargo.toml`. `just r-cmd-check`
temporarily rewrites that path to an absolute one, so prefer it over
`devtools::check()` or RStudio's "Check" button.
