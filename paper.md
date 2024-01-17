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
date: \today
bibliography: paper.bib
---

# Summary

# Statement of Need

R is a programming language geared towards statistical software and visualisations.
From its inception, R was meant to be extended, providing tools for building
dynamic libraries in Fortran/C/C++ natively. On Windows, the R-project provides
a toolchain Rtools, which bundles developer environment for Fortran, C and C++.
In Extending R [@chambers2017extending] details interfacing with Python, Julia
and C++, which is written by an R-core developer. R provides a C-API by default,
together with command line utilities to compile dynamic libraries for use in R.

There are several R packages that facilitates binding to various programming languages,
Rcpp [@rcpp_cran] and cpp11 [@cpp11] for C++, Java via rJava [@rJava], Python
with `reticulate` [@reticulate_cran], and JavaScript on the V8 runtime and the
V8 R-package [@v8_cran]. These packages aim to provide R users a developer environment
for their respective languages, auto-generate boilerplate, etc.

In order to achieve reasonable performance in R, package developers need to
embed extension code in their packages. This is partially why Rcpp is used
by the majority of packages on CRAN, the official R-package repository for R[@vriesAndriePagerank2021]. However, performance is not the only virtue that
Scientists require in a compiled language, and Rust has other features that
scientists with ever growing computational needs, have gravitated towards [@perkelWhyScientistsAre2020]. For instance, Rust uses declarative memory management,
i.e. there are compile-time analysis on memory safety.
<!-- 
Extendr provides integration the R data model, embedding calls to R within Rust,
automatically generated wrappers, provide an R-based developer interface for Rust, -->
<!-- and much more. -->
<!-- Like with Fortran/C/C++, -->
<!-- Rust does not have a garbage collector (gc) -->
<!-- This is what extendr provides, an integration of R's data-model, l -->

Extendr is a suite of software packages, comparable to Rcpp and cpp11.
Includes `libR-sys` which is a crate
providing Rust bindings to R's C-API. Then the main three crates are `extendr-api`, `extendr-engine`,
and `extendr-macros`. These crates provide an R data model in Rust, embedding of R in Rust code.
Lastly, an R-package `rextendr`, which is R users developer environment for Rust.

# Getting Started

First, ensure that Rust is installed, by following [Install Rust on rust-lang.org](https://www.rust-lang.org/tools/install). Then in an R terminal,

```r
install.packages("rextendr") 
# remotes::install_github("extendr/rextendr") # latest dev-version
usethis::create_package("exampleRustRpkg")
rextendr::use_extendr()
```

The function `use_extendr` setups up the necessary boilerplate for building
Rust library together with an R package. An R-package may be constructed using
the `usethis` R-package [@usethis_cran].

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
dependencies external to R and CRAN. These must be vendored. Since Fortran[^fortran_pkg_mgr]/C/C++ do not have official package managers. In `rextendr::` ...
<!-- TODO: please contribute to this bit @JosiahParry -->

[^fortran_pkg_mgr]: Fortran Package Manager is a community-driven project in alpha release as of this writing.

<!-- Today, we are seeing the proliferation of the Rust programming language. According to StackOverflow, Rust is the most admired programming language for many years running—and for good reason (<https://survey.stackoverflow.co/2023/>). Rust provides similar performance such as C and C++ while also being far more ergonomic ([@perkelWhyScientistsAre2020]). But most importantly, Rust provides guarantees memory that make exceptionally safe. For all of these reasons and more, providing R package developers a way to integrate Rust and R is necessary for the continued growth of the R ecosystem. The extendr Rust library and its companion R package `{rextendr}` make the process of marrying R and Rust simple. -->

<!-- ## Implementation

extendr utilizes R's C API via the libR-sys library crate. libR-sys utilizes the Rust library bindgen to automatically create foreign function interface (FFI) bindings for all major distributions (<https://github.com/rust-lang/rust-bindgen>). The bindings provided by libR-sys create Rust representations of the structs defined in R's C API. On top of these exceptionally low-level bindings, extendr is built. extendr defines a number of user friendly Rust structs that can be be passed directly to and from R and Rust. -->

# extendr packages in the R ecosystem

The rust-based DataFrame library [Polars](https://pola.rs/) has bindings to
python (via [`py-polars`](https://github.com/pola-rs/polars/tree/main/py-polars)) and to R via [`rpolars`](https://github.com/pola-rs/r-polars), where the latter is built with extendr.

An example of scientific computing enabled by extendr is [`changeforest`](https://github.com/mlondschien/changeforest/tree/main) [@JMLR:v24:22-0512].

[`rsgeo`](https://cran.r-project.org/web/packages/rsgeo/index.html) are bindings to [`geo-rust`](https://crates.io/crates/geo) geometry primitives and algorithms which are very performant. `rsgeo` is similar to the 
<!-- TODO -->


<!-- ## Related work

Integration of Rust and R has been explored in other libraries. The `roxido` library crate and accompanying R package `cargo` by David B. Dahl are an alternative approach to creating Rust bindings ([@cargo_cran]). An offshoot of extendr, [savvy](https://github.com/yutannihilation/savvy), is an "unfriendly" lower-level approach to generating Rust and R bindings in which "ergonomics is not included.". -->

## Notes

<!-- - r is an interface langauge. it comes with a C api to build extensions -->

<!-- - over a decade ago Rcpp was released revoltionizing R package development making it easy to tap into high performance library from C++ -->

  <!-- - cpp11 is a fairly recent take on the same objective by the folks from the r-lib team -->

  <!-- - V8 is another take on interfacing with another language enabling R users to call javaScript via V8 -->

<!-- - today we are seeing very fast growth in the adoption of Rust due to its ease of use, safety, and performance. -->

<!-- - to ensure that the R ecosystem can stay on op of the maturing data science ecosystem, we need to be able to tap into Rust libraries and make bindings to them in R -->

  <!-- - PyO3 serves this role for the python ecosystem and has led to wildly successful libraries such as polars -->

<!-- - extendr is a rust library that provides R package developers with a way to create R packages that utilize the power and safety of Rust -->

<!-- - it creates bindings to R's C API via the low-level Rust crate libR-sys that supports extendr. -->

<!-- - extendr comes with a companion R package called {rextendr} -->

  <!-- - rextendr is a user friendly package that is used for creating the scaffolding of a rust-enabled R package -->

  <!-- - it documents Rust functions and creats wrappers to rust functions that are then exported to R via the `.Call()` function interface -->

<!-- - extendr works by creating a staticlib that is called by R -->

<!-- - extendr has already seen a fair amount of adoption in the R ecosystem. Notably it has been used to develop the R package {rpolars} which are R bindings to polars rust data frame library. -->

  <!-- - prqlr which are bindings to the prql crust compiler library that generates sql queries. -->

  <!-- - rsgeo are bindings to geo-rust geometry primitives and algorithms which are very performant -->

<!-- - extendr is extensible meaning that other rust-crates can be developed to integrate external rust libraries with extendr and thus R -->

<!-- - a recent example is the arrow-extendr library crate which enables conversion from from R's arrow and nanoarrow R packages to Apache Arrow arrow-rs rust implementation. -->

<!-- The R Project for Statistical Computing, referred to simply as R, has a long history of being an interface language. -->

<!-- - "Writing R extensions" discusses in detail how to create a new interface between an external library or language and R's C API. -->

<!-- - R's C API is one of the reasons why it is such language. Rcpp's in 2011 [@rcpp_jss] -->

<!-- - extendr started as an R-consortium funded project by Andy Thomason. -->
<!-- - interfaces with R's C API -->

<!-- <https://www.r-consortium.org/all-projects/awarded-projects/2021-group-1#extendr+-+rust+extensions+for+r>. -->

<!-- related software Rcpp, cpp11, -->

<!-- Acknowledge [hellorust](https://github.com/r-rust/hellorust) [@hellorust_cran] -->

## Requirements from JOSS

Your paper should include:

A list of the authors of the software and their affiliations, using the correct format (see the example below).

A summary describing the high-level functionality and purpose of the software for a diverse, non-specialist audience.

<!-- A Statement of need section that clearly illustrates the research purpose of the software and places it in the context of related work. -->

<!-- A list of key references, including to other software addressing related needs. Note that the references should include full names of venues, e.g., journals and conferences, not abbreviations only understood in the context of a specific discipline. -->

<!-- Mention (if applicable) a representative set of past or ongoing research projects using the software and recent scholarly publications enabled by it. -->

## Acknowledgements

<!-- Acknowledgement of any financial support.  -->
Project lead Andy Thomason received a grant from the R-consortium
[@consortiumConsortiumFundedProject2023].

We would like to acknowledge Jeroen Ooms for his [hellorust](https://github.com/r-rust/hellorust) [@hellorust_cran], and continuous maintenance of this manual embedding
of Rust in R proof of concept.


<!-- - v8 (javascript bindings): <https://github.com/jeroen/V8> -->

<!-- - Rcpp: <https://www.jstatsoft.org/article/view/v040i08> -->

<!-- - cpp11: <https://cpp11.r-lib.org/articles/internals.html> -->

<!-- - writing R extensions <https://cran.r-project.org/doc/manuals/R-exts.html> -->

<!-- - <https://raw.githubusercontent.com/dbdahl/cargo-framework/main/cargo/inst/doc/Writing_R_Extensions_in_Rust.pdf> -->

# References
