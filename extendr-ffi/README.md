## extendr-ffi

`extendr-ffi` is a hand curated subset of the bindings provided by libR-sys. Supporting R version 4.2 and onwards. extendr-ffi provides backports to ensure that `extendr-api` can be used across multiple versions of R.

## Motivation

extendr has historically relied on [libR-sys](https://github.com/extendr/libR-sys) to interface with R's C API. As R has moved towards standardizing and stabilizing the C API, relying on libR-sys' generated bindings has presented challenges. Among them are that the bindings are cumbersome to generate, platform specific, and does not provide backports to address the maturing R C API.

## Creating backports

As R's C API moves into stabilization, many macros and functions that we previously used are being moved to **non-API** status. This means the use of them will create a NOTE or WARNING during `R CMD check`. To address this, we create **backports**.

To create a backport: 

- Ensure there is a version `cfg` flag in `extendr-ffi/build.rs`
- Move now non-API C binding to `src/backports.rs`'s `extern "C"` block
- Add `#[cfg(not(r_maj_min))]` to old function
- Add new replaced function with `#[cfg(r_maj_min)]`
- Create a new inline function to be used in `extendr-api`
- Replace old function calls in `extendr-api` with new backport

For example, in R 4.5 `Rf_isFrame` is replaced by `Rf_isDataFrame`.
In `backports.rs`

```
extern "C" {
    // all the other backports...
    
    #[cfg(not(r_4_5))]
    fn Rf_isFrame(x: SEXP) -> Rboolean;

    #[cfg(r_4_5)]
    fn Rf_isDataFrame(x: SEXP) -> Rboolean;
}

/// Check is data.frame
///
/// # Safety
///
/// This function dereferences a raw SEXP pointer.
/// The caller must ensure that `x` is a valid SEXP pointer.
#[inline]
pub unsafe fn is_data_frame(x: SEXP) -> Rboolean {
    #[cfg(not(r_4_5))]
    {
        Rf_isFrame(x)
    }
    #[cfg(r_4_5)]
    {
        Rf_isDataFrame(x)
    }
}
```
