## extendr-ffi

`extendr-ffi` is a hand curated subset of the bindings provided by libR-sys. Supporting R version 4.2 and onwards. extendr-ffi provides backports to ensure that `extendr-api` can be used across multiple versions of R.

## Motivation

extendr has historically relied on [libR-sys](https://github.com/extendr/libR-sys) to interface with R's C API. As R has moved towards standardizing and stabilizing the C API, relying on libR-sys' generated bindings has presented challenges. Among them are that the bindings are cumbersome to generate, platform specific, and does not provide backports to address the maturing R C API.
