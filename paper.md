---
title: "extendr: Frictionless bindings for R and Rust"
tags:
  - R
  - Rust
  - FFI
  - bindings
subtitle: "R extensions in Rusts"
authors:
  - name: "Mossa Merhi Reimert"
    orcid: 0009-0007-9297-1523
    affiliation: 1
  - name: Josiah D. Parry
    orcid: 0000-0001-9910-865X
    affiliation: 3
  - name: Claus O. Wilke
    orcid: 0000-0002-7470-9261
    affiliation: 2
  - name: Ilia Kosenkov
    orcid: 0000-0001-5563-7840
  - name: Andy Thomason
affiliations:
  - name: "Section of Animal Welfare and Disease Control, Department of Veterinary and Animal Sciences, University of Copenhagen, Denmark"
    index: 1
  - name: "The University of Texas at Austin, Texas"
    index: 2
  - name: "Environmental Systems Research Institute (Esri), Redlands, CA, USA"
    index: 3
date: \today
bibliography: paper.bib
---

# Summary

For many Scientists, Statisticians, and Data Scientists, the programming language R is
irreplaceble. R is an interpreted programming language that aims towards statistical software,
and visualisations. It offers support on many platforms, the official interpretor is
written in C, and it is a WebAssembly version called [webR](https://docs.r-wasm.org/webr/latest/).

R's ecosystem contains many packages, primarily written by research scientists,
specialists and professionals, where packages fall between long-standing and
robust solutions, and cutting-edge research software. While a dynamic typed,
interpreted language lends itself towards the non-programmer crowd, there is
a need for performant, maintainable, and versatile qualities. This is achievable
for the R community, by having extensibility as a core feature of the language.

Natively, R provide binding capabilities to Fortran, C and C++, mainly through
its C-API, and also providing integrated tooling for working with other programming
languages in R. On top of official support, there are many community-driven
efforts to offer binding to R, from many languages like Julia, Python, Java,
JavaScript, etc. The official R-package repository CRAN[^CRAN] hosts several packages.

[^CRAN]: Comprehensive R Archive Network

extendr offers a binding project aiming to bring Rust to the R ecosystem. This
is accomplished by providing an opinionated, ergonomics-focused, and rich
suite of software packages in order to facilitate R-users' developer experience with Rust.
There is a community-expectation that R is the front-end to working with other
languages, as well as a tradition for providing automated tooling for scaffolding / boilerplate
for integrating to external languages.

Thus extendr provides an emulation of the R data model within Rust, integration
of Rust tooling in the R-package build systems, a rust developer experience in
R, and functions for preparing publishing of Rust-powered R-packages to CRAN.

For scientific computing, Rust has been used to write software to aid in
scientific tooling, that is on-par or even surpassing state-of-the-art.
For instance, [`gifski`](https://crates.io/crates/gifski) is a high-performance
GIF encoder, which is made acessible in R through a binding R-package [@gifski_cran].
These bindings are written by hand. These tools enables better support for
making animations in R, and they are minimally scoped, thus it is feasible to write explicit
bindings for R. But for scientists, that now uses Rust as a computing platform,
there is a growing need for automated tooling.

<!-- addition maybe?? [@rust_bio] -->
An example of this is agent-based modelling in epidemiology. There are many
off the shelf frameworks to write a particular model or scenario. However,
each setting differ, and custom agent-based models are increasingly desired.
Rust is a great candidate to write such models [@eval_rust_for_custom_abm],
[@epirust_paper]. For modelling African Swine Fever within wild boar using Rust,
there is [SwiFCo-rs](https://ecoepi.eu/ASFWB/), see [@forth_african_2022-1].
These models are large and continuously updated and amended.
And R is more ubiqutious in epidemiological modelling than Rust, and thus
having an automated binding tooling for accessing such code-bases, is where
extendr fulfills an unmet need.
<!-- Another example of rust-based disease spread model [@rust_disease_spread_model_indsci_sim] -->

A webpage with an overview of the extendr-packages, and access to comprehensive
API documentation is provided at [extendr.github.io](https://extendr.github.io/).

# Statement of Need

R is a programming language geared towards statistical software and visualisations.
From its inception, R was meant to be extended, providing tools for building
dynamic libraries in Fortran/C/C++ natively. On Windows, the R-project provides
a toolchain Rtools, which bundles developer environment for Fortran, C and C++.
In Extending R [@chambers2017extending], a book written by an R-core member,
 interfacing with Python, Julia and C++ is described. R provides a C-API by default,
together with command line utilities to compile dynamic libraries for use in R.
The R-project provides documentation for developing extensions in [R-internals](https://cran.r-project.org/doc/manuals/R-ints.html)
and [R-extenstions](https://cran.r-project.org/doc/manuals/R-exts.html).

There are several R packages that facilitates binding to various programming languages,
Rcpp [@rcpp_cran] and cpp11 [@cpp11] for C++, Java via rJava [@rJava], Python
with `reticulate` [@reticulate_cran], and JavaScript on the V8 runtime and the
V8 R-package [@v8_cran]. These packages aim to provide R users a developer environment
for their respective languages, auto-generate boilerplate, etc.

In order to have reasonable performance in R, package developers need to
embed extension code in their packages. This is partially why Rcpp is used
by the majority of packages on CRAN (through other dependencies), the official R-package repository for R [@vriesAndriePagerank2021].
However, performance is not the only virtue that
Scientists require in a compiled language, and Rust has other features that
scientists with ever growing computational needs, have gravitated towards [@perkelWhyScientistsAre2020]. For instance, Rust uses declarative memory management,
i.e. there are compile-time gurantees for memory safety. Note that memory leaks
is not considered memory unsafe, as yielding data from Rust to R, is considered
leaking memory.

Extendr is a suite of software packages, comparable to Rcpp and cpp11.
Includes `libR-sys` which is a crate
providing Rust bindings to R's C-API. Then the main three crates are `extendr-api`, `extendr-engine`,
and `extendr-macros`. These crates provide an R data model in Rust, embedding of R in Rust code.
Lastly, an R-package `rextendr`, which is R users developer environment for Rust.

# Getting Started

First, ensure that Rust is installed, by following [Install Rust](https://www.rust-lang.org/tools/install). Then in an R terminal,

```r
install.packages("rextendr") 
# remotes::install_github("extendr/rextendr") # installs latest dev-version
usethis::create_package("exampleRustRpkg")
rextendr::use_extendr()
```

First, an R-package may be constructed using
the `usethis` R-package [@usethis_cran]. `rextendr` follows the design principles
of the `usethis` package.

The function `use_extendr` setups up the necessary boilerplate for building
Rust library together with an R package, integrating with the package build system,
to ensure support across platforms.

To refresh the wrappers generated, use `rextendr::document()`, as this augments
a call to `devtools::document()`.

<!-- Rust project is in `exampleRustRpkg/src/rust/`. -->

# Features

Features that extendr aims towards

- Integrate R's data model within Rust through `extendr-api`
- Auto-generate R wrappers for embedded Rust code, via `extendr-macros`
- Embed R inside of Rust for use in unit-testing, integration testing, etc. through `extendr-engine`
- Integrate Rust's packaging in R and its package build system, see`rextendr`
- Tools to help adhere to CRAN's extensive rules for publishing Rust-powered R-packages

API documentation for all the extendr packages are available at [extendr.github.io](https://extendr.github.io/),
and the repositories for extendr-packages are under GitHub organisation [github.com/extendr](https://github.com/extendr/).

<!-- `rextendr` also have `rust_source` and `rust_function` equivalent to `Rcpp`'s functions, where arbitrary rust code can be evaluated, and the result is relayed back to R. -->

## Mirroring R's Data model in Rust

Most R data is vector-based, even scalar values are 1-unit vectors. These vectors
are represented in Rust as slices `&[T]` / `&mut [T]`. R data may be allocated
in Rust, but these are invisible to R's garbage collector, and thus have to
be protected. `extendr-api` registers R allocated data in an internal hash-map / dictionary,
that stores a reference count for Rust allocated R data. In contrast, `cpp11` uses
a linked-list approach.

## Automagically generated wrappers for R

A C-function is callable in R, if it returns an `SEXP`, and all of its arguments
are `SEXP`s. These are opaque pointers to an internal R representation of data.
These are callable in R via `.Call`. A rust function is exported to R, must
have all of its arguments and return values convertible to `SEXP`. Annotating
it with `#[extendr]` will add a callable C-function in R, that converts the
custom data types into `SEXP`s.

The surrounding R-package need to know about the exported functions, their
expected signature, and provide the R equivalent code, that calls them via `.Call`
interface. This wrapper code is generated via `rextendr::document()`.
<!-- the type information is stored _in_ the generated rust library... -->

## Inline R execution in Rust

It is possible to instantiate R through its C-API. This allows for custom
R REPL implementations. This can be then be used to instantiate R within unit
tests, and R code can then be tested outside of a user-facing R instance.
Using embedding of R in a process is controversial, in that published packages
on CRAN are not allowed to have these C-API functions in the built package binaries.
This is also why Rcpp separated this capability into another R-package `RInside` [@rinside_cran], and extendr similarly relegates this functionality to `extendr-engine`.

## A rust developer interface in R

With `rextendr` there are `rust_source` and `rust_function`, where the former
evaluates arbitrary rust code, and returns the last value in the block, and
the latter compiles, wraps and returns arbitrary Rust functions. Rcpp provides
similar functions. These functions are very versatile, as they can also be used,
to include 3rd party crates.

For compiling Fortran/C/C++, R provides a CLI option `R CMD SHLIB`. This ensures
that the resulting binary is processed by R's internal build system. Similarly,
`rextendr::use_extendr()` provides Makevars-files, that adapts the compilation
process of an R-package with the embedded Rust binary.
<!-- Actually, we _could_ do a little better job with that, but this
  part is very sparsely documented by R-core...
 -->

## Publishing rust-powered R-packages

For R package authors, being able to publish their code on CRAN is essential.
However, CRAN have many rules for publishing packages in general, e.g. number
of threads that a package uses at build & testing must not exceed 2.

Uniquely, Rust has a package manager, which means that R packages have 3rd party
dependencies external to R and CRAN. These must be vendored ensure package stability (see ["Using Rust in CRAN packages
"](https://cran.r-project.org/web/packages/using_rust.html)). rextendr provides utility functions ensure extendr-based packages are CRAN compliant. `rextendr::use_cran_defaults()` and `rextendr::vendor_pkgs()` will ensure dependencies are built entirely offline and from vendored sources.

# Related work & prior art

There are other software packages providing bindings to R and Rust. There's
`roxido` / [`cargo`](https://github.com/dbdahl/cargo-framework) Rust crate and R-package resp. And an offshoot of extendr,
[savvy](https://github.com/yutannihilation/savvy). They differ to extendr, in
that extendr aims to provide an opinionated API, with a focus on an
ergonomic API design. While extendr provides many of the features that Rcpp offers,
its architecture is inspired by cpp11. Other scientific computing communities
are also introducing Rust as a plug-in, with Python it is [PyO3](https://github.com/PyO3/pyo3),
and Julia has [jlrs](https://github.com/Taaitaaiger/jlrs).

# extendr packages in the R ecosystem

The rust-based DataFrame library [Polars](https://pola.rs/) has bindings to
python (via [`py-polars`](https://github.com/pola-rs/polars/tree/main/py-polars)) and to R via [`rpolars`](https://github.com/pola-rs/r-polars), where the latter is built with extendr.

An example of scientific computing enabled by extendr is package [`changeforest`](https://github.com/mlondschien/changeforest/tree/main), and accompanying publication [@JMLR:v24:22-0512].

The CRAN published R-package [`rsgeo`](https://cran.r-project.org/web/packages/rsgeo/index.html) provides bindings to [`geo-rust`](https://crates.io/crates/geo) geometric primitives and algorithms which are very performant. `rsgeo` is most similar to `geos` [@geos_cran] which provides bindings to the GEOS C library.

  <!-- - prqlr which are bindings to the prql crust compiler library that generates sql queries. -->

  <!-- - rsgeo are bindings to geo-rust geometry primitives and algorithms which are very performant -->

## Acknowledgements

<!-- Acknowledgement of any financial support.  -->

Project lead Andy Thomason received a grant from the R-consortium
[@consortiumConsortiumFundedProject2023].

We would like to acknowledge Jeroen Ooms for his [hellorust](https://github.com/r-rust/hellorust) [@hellorust_cran], and continuous maintenance of a hand-written embedding of Rust in R example project.
Their [github.com/r-rust](https://github.com/r-rust) contains several examples
of hand-crafted bindings to Rust packages for R, such as `gifski` [@gifski_cran].

# References
