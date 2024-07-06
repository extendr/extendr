# `extendr-engine`

This crate facilitates embedding an R process together with a standalone binaries.
In rust-powered R-packages, the R code calls Rust, and thus there is already an
accompanying R process. Instead, this is meant to be used in unit tests, integration
test and binaries that are standalone from R code.

**This crate does not adhere to the non-API requirements of CRAN.**

## Using it in R-packages

Within `Cargo.toml` add `extendr-engine` under `dev-dependencies`.

```toml
[dev-dependencies]
extendr-engine = "*"
```

Then, you may use `extendr_engine` within unit tests, integration tests,
and binaries. If `extendr-engine` is added under `[dependencies]`, then the
surrounding R-package will flag a CRAN note about non-API usage.

## About

See documentation on [doc.rs](https://docs.rs/extendr-engine/latest/extendr_engine/), or the latest development version on [extendr website](https://extendr.github.io/extendr/extendr_engine/index.html).

This crate is similar in spirit as [`{Rinside}`, on CRAN](https://cran.r-project.org/web/packages/RInside/index.html).
