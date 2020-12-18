# Minimal Example of Calling Rust from R

[![R build status](https://github.com/extendr/helloextendr/workflows/R-CMD-check/badge.svg)](https://github.com/extendr/helloextendr/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

This is a template package to demonstrate how to call Rust from R using the [extendr-api](https://crates.io/crates/extendr-api) crate.


# Installation

Before you can install this package, you need to install a working Rust toolchain. 

To run rust-bindgen, you'll need to install libclang/llvm. See for instructions here: https://github.com/rust-lang/rust-bindgen/blob/master/book/src/requirements.md

To build this package from within RStudio, you'll also have to make sure llvm is available in your path under R. To do so, add something like the following to your `.Renviron` file:
```
PATH=/usr/local/opt/llvm/bin:${PATH}
```
