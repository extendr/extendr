# `extendr` - A safe and user friendly R extension interface using Rust

[![Github Actions Build Status](https://github.com/extendr/extendr/workflows/Tests/badge.svg)](https://github.com/extendr/extendr/actions)
[![Crates.io](https://img.shields.io/crates/v/extendr-api.svg)](https://crates.io/crates/extendr-api)
[![Documentation](https://docs.rs/extendr-api/badge.svg)](https://docs.rs/extendr-api)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![DOI](https://joss.theoj.org/papers/10.21105/joss.06394/status.svg)](https://doi.org/10.21105/joss.06394)

[![Logo](https://github.com/extendr/extendr/raw/main/extendr-logo-256.png)](https://github.com/extendr/extendr/raw/main/extendr-logo-256.png)

## Welcome

extendR is a suite of software packages, see the website [extendR](https://extendr.github.io/) for an overview.

This repository is for the rust crates that are part of extendR,
see also [`{rextendr}`](https://extendr.github.io/rextendr/) for the R-package that facilitates using extendR.

A complete user guide detailing how to use extendR is [available here](https://extendr.github.io/user-guide/).

The main crate `extendr-api` is published on [crates.io](https://crates.io/crates/extendr-api).

## Getting started

There are many ways to use extendR from R. In an interactive R session one may
use [`rextendr::rust_function` and friends](https://extendr.github.io/rextendr/reference/rust_source.html)
to quickly prototype Rust code.

In an R package context, one may use [`rextendr::use_extendr()`](https://extendr.github.io/rextendr/reference/use_extendr.html) to setup a Rust powered R-package. See also [vignette on R-packages](https://extendr.github.io/rextendr/articles/package.html).

It is also possible to inline Rust code in `RMarkdown`/`knitr`, see [vignette on extendr `knitr-engine`](https://extendr.github.io/rextendr/articles/rmarkdown.html).

See [rextendr](https://extendr.github.io/rextendr/) package for more information
on the available functionality from an R session.

## Overview

It is intended to be easier to use than the C interface and
Rcpp as Rust gives type safety and freedom from segfaults.

The following code illustrates a simple structure trait
which is written in Rust. The data is defined in the `struct`
declaration and the methods in the `impl`.

```rust
use extendr_api::prelude::*;

#[extendr]
struct Person {
    pub name: String,
}

#[extendr]
impl Person {
    fn new() -> Self {
        Self { name: "".to_string() }
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

#[extendr]
fn aux_func() {
}


// Macro to generate exports
extendr_module! {
    mod classes;
    impl Person;
    fn aux_func;
}
```

The `#[extendr]` attribute causes the compiler to generate
wrapper and registration functions for R which are called
when the package is loaded.

The `extendr_module!` macro lists the module name and exported functions
and interfaces.

This library aims to provide an interface that will be familiar to
first-time users of Rust or indeed any compiled language.

## Goals of the project

Instead of wrapping R objects, we convert to Rust native objects
on entry to a function. This makes the wrapped code clean and dependency
free. The ultimate goal is to allow the wrapping of existing
Rust libraries without markup, but in the meantime, the markup
is as light as possible.

```rust
#[extendr]
pub fn my_sum(v: &[f64]) -> f64 {
    v.iter().sum()
}
```

You can interact in more detail with R objects using the `Robj`
type which wraps the native R object type. This supports a large
subset of the R internals functions, but wrapped to prevent
accidental segfaults and failures.

## Contributing

We are happy about any contributions!

To get started you can take a look at our [Github issues](https://github.com/extendr/extendr/issues).

You can also get in contact via our [Discord server](https://discord.gg/7hmApuc)!

### Development

The documentation for the latest development version of `extendr-api` is available here:
<https://extendr.github.io/extendr/extendr_api/>
