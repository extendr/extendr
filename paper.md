---
title: "extendr: Frictionless bindings for R and Rust"
tags:
  - R
  - Rust
  - FFI
  - bindings
subtitle: "interfacing Rust code within R packages"
# <!-- This is just a suggestion, but the previous subtitle didn't read very well to me -->
authors:
  - name: "Mossa Merhi Reimert"
    orcid: 0009-0007-9297-1523
    affiliation: 1
  - name: Josiah D. Parry
    orcid: 0000-0001-9910-865X
    affiliation: 2
  - name: Claus O. Wilke
    orcid: 0000-0002-7470-9261
    affiliation: 3
  - name: Ilia Kosenkov
    orcid: 0000-0001-5563-7840
    affiliation: 4
  - name: Andy Thomason
    affiliation: 5
  - name: Michael Milton
    orcid: 0000-0002-8965-2595
    affiliation: 6
affiliations:
  - name: "Section of Animal Welfare and Disease Control, Department of Veterinary and Animal Sciences, University of Copenhagen, Denmark"
    index: 1
  - name: "Environmental Systems Research Institute (Esri), Redlands, CA, USA"
    index: 2
  - name: "Department of Integrative Biology, The University of Texas at Austin, Austin, TX, USA"
    index: 3
  - name: "MISSING AFFILIATION"
    index: 4
  - name: "MISSING AFFILIATION"
    index: 5
  - name: "Walter and Eliza Hall Institute of Medical Research"
    index: 6
date: \today
bibliography: paper.bib
---

# Summary

The programming language [Rust](https://www.rust-lang.org) continues to gain popularity with
developers due to a strong emphasis on safety, performance, and productivity [@perkelWhyScientistsAre2020].
As a general-purpose, low-level programming language, Rust has a wide variety of potential uses
in both commercial and research applications where performance is important. Commercial examples
include web development and game development, and in the research domain Rust is increasingly being
used in a wide range of contexts including change point detection [@JMLR:v24:22-0512], high-performance
GIF encoding [@gifski_cran], and agent-based models of disease spread [@eval_rust_for_custom_abm; @epirust_paper; @forth_african_2022-1].
<!-- Other references/examples of Rust usage in research would be useful! -->

However, typical workflows in research domains including disease modelling are frequently based on higher-level
programming languages, due to lower barriers to usage and therefore wider adoption within these
communities compared to low-level languages such as C++ and Rust. The statistical programming language
[R](https://www.r-project.org) is one of the most widely used
high-level languages in research. R's official interpreter is written in C, and it provides a C-API as well as
tools for building dynamic libraries in Fortran/C/C++ natively.
<!-- On Windows, the R-project provides
a toolchain, Rtools, which bundles a developer environment for Fortran, C and C++. -->
<!-- The R-project provides documentation for developing extensions in [R-internals](https://cran.r-project.org/doc/manuals/R-ints.html)
and [R-extenstions](https://cran.r-project.org/doc/manuals/R-exts.html). -->
The Extending R book [@chambers2017extending] also describes interfacing with other languages
such as Python and Julia.

A strength of R is the ecosystem of packages hosted at [CRAN](https://cran.r-project.org),
which are primarily written by research scientists,
specialists, and professionals. An important use case is as a front-end to working with other
languages, for which automated tooling providing scaffolding / boilerplate code for integrating external languages
are widely used. For example, embedding C++ code is a good way to resolve performance bottlenecks
within R packages, and it can be easily accomplished using cpp11 [@cpp11] and Rcpp [@rcpp_jss].
Rust has similar performance to C++ but also has other beneficial features such as declarative memory management,
providing compile-time guarantees for memory safety without the need for a garbage collector.
<!-- Rust and R also share support for WebAssembly [webR](https://docs.r-wasm.org/webr/latest/). -->
We note that other scientific computing communities
have introduced plug-ins for Rust, including Python via [PyO3](https://github.com/PyO3/pyo3),
and Julia via [jlrs](https://github.com/Taaitaaiger/jlrs).
However, to the authors' knowledge there is currently no fully featured mechanism for integrating Rust within an R package
in a similar manner to that provided by Rcpp for C++.

This paper introduces a collection of four Rust crates and an R package that collectively make up the extendr project.
The goal of this project is to provide (automatic) binding of Rust to R, using an opinionated and
ergonomics-focused suite of tools that facilitate the use of Rust code within R packages.
This is achieved by providing an emulation of the R data model within Rust, integration
of Rust tooling in the R-package build systems, a rust developer experience in
R, and functions for preparing publishing of Rust-powered R-packages to CRAN.
A webpage with an overview of the extendr crates/packages and access to comprehensive
API documentation is provided at [extendr.github.io](https://extendr.github.io/).

# Statement of Need

R provides tools to compile and embed Fortran, C, and C++ code, with binding
through R's C-API. However, these raw bindings are not easy for users to navigate, so
frameworks to facilitate interfacing other languages to R are extremely popular, for example
Rcpp [@rcpp_cran] and cpp11 [@cpp11] for C++, Java via rJava [@rJava], Python
with `reticulate` [@reticulate_cran], and JavaScript on the V8 runtime and the
V8 R-package [@v8_cran]. In contrast, bindings between Rust and R, such as
[`gifski`](https://crates.io/crates/gifski) [@gifski_cran],
are currently mostly written by hand.

We note that there are other software packages providing bindings between R and Rust. The
Rust crate / R-package resp `roxido` / [`cargo`](https://github.com/dbdahl/cargo-framework)
provides a mechanism of embedding Rust code within R packages.
<!-- Looks like this should be cited:  David B. Dahl (2021), Writing R Extensions in Rust, arXiv:2108.07179 [cs.PL], URL https://arxiv.org/abs/2108.07179. -->
The [savvy](https://github.com/yutannihilation/savvy) interface was also written as a
simplified offshoot of extendr. However, these implementations differ to extendr in
that extendr aims to provide an opinionated API, with a focus on an
ergonomic API design inspired by features from Rcpp and cpp11.

There are a number of existing examples of projects using extendr. The DataFrame library
[Polars](https://pola.rs/) has bindings to python (via [`py-polars`](https://github.com/pola-rs/polars/tree/main/py-polars))
and to R via [`polars`](https://github.com/pola-rs/r-polars), where the latter is built with extendr.
The CRAN package [`rsgeo`](https://cran.r-project.org/web/packages/rsgeo/) provides
bindings to [`geo-rust`](https://crates.io/crates/geo), to allow R users to take advantage of
highly performant geometric primitives and algorithms in Rust.
<!-- Are there preferred citation methods for rsgeo? -->
A further example of scientific work enabled by extendr is the [`changeforest`](https://github.com/mlondschien/changeforest/)
package [@JMLR:v24:22-0512].

  <!-- - prqlr which are bindings to the prql crust compiler library that generates sql queries. -->

  <!-- - rsgeo are bindings to geo-rust geometry primitives and algorithms which are very performant -->

# Design

## Overview

The extendr project provides a suite of software packages, where the aim is to provide a mechanism
for interfacing Rust to R that is comparable in scope to the R/C++ interfaces provided by Rcpp and cpp11.
It consists of the following components:

- extendr-api: a Rust crate integrating R's data model in Rust, which underlies the functionality of extendr
- extendr-macros: a Rust crate responsible for auto-generating R wrappers for embedding Rust within R code
- extendr-engine: a Rust crate that enables launching R sessions from within Rust code, similar to `RInside` [@rinside_cran]
- rextendr: an R package that simplifies the process of embedding Rust code within an R package, including helping the user to adhere to CRAN rules for publishing Rust-powered R packages
- libR-sys: a Rust crate providing auto-generated Rust bindings to R's C-API

<!-- I adapted this from a combination of https://extendr.github.io and the older text below -->

<!--
It includes `libR-sys`, which is a crate providing Rust bindings to R's C-API.
The main three crates are `extendr-macros`, `extendr-api`, and `extendr-engine`.
These crates are responsible for providing a automatic wrapper generation,
an R data model in Rust, and embedding of R interpreter in Rust (for testing purposes) resp.
Lastly, extendr provides a R-based Rust developer environment through an R-package `rextendr`.
These crates provide an R data model in Rust, embedding of R in Rust code.

## Features

Features that extendr aims towards

- Integrate R's data model within Rust through `extendr-api`
- Auto-generate R wrappers for embedded Rust code, via `extendr-macros`
- Embed R inside of Rust for use in unit-testing, integration testing, etc. through `extendr-engine`
- Integrate Rust's packaging in R and its package build system, see `rextendr`
- Tools to help adhere to CRAN's extensive rules for publishing Rust-powered R-packages
-->
Using extendr requires both Rust and R to be installed, but no further dependencies are required.
API documentation for all the extendr packages are available at [extendr.github.io](https://extendr.github.io/),
and the repositories for extendr-packages are freely available from GitHub [github.com/extendr](https://github.com/extendr/),
under an MIT license.  All hardware/software platforms supported by Rust and R are also supported by extendr.
<!-- Check this is true! -->

## Technical details

Most R data is vector-based, including scalar values (which are length-1 vectors). These vectors
are represented in Rust as slices `&[T]` / `&mut [T]`. R data may be allocated
in Rust, but these are invisible to R's garbage collector, and thus have to
be protected. `extendr-api` registers R allocated data in an internal hash-map / dictionary,
that stores a reference count for Rust allocated R data.

A C-function is callable in R if it returns an `SEXP` and all of its arguments
are `SEXP` - these are opaque pointers to an internal R representation of data.
These are callable in R via `.Call`. A Rust function that is exported to R must
have all of its arguments and return values convertible to `SEXP`. Annotating
it with `#[extendr]` will add a callable C-function in R, that converts the
custom data types into `SEXP` types.

The `rextendr` package also provides R-level functions `rust_source`, which allows
arbitrary Rust code to be evaluated returning the last value in the block, and
`rust_function`, which compiles, wraps and returns arbitrary Rust functions as
callable R functions. These two functions are very similar in scope to the
`evalCpp` and `cppFunction` functions provided by Rcpp, and are very versatile,
as they can also be used to include 3rd party crates.

## Creating Rust-powered R packages

The `rextendr::use_extendr()` function can be used to auto-edit an existing user-specified
R package (for example created using `usethis::create_package()`) to include all of the
details necessary to embed Rust code within the package. This includes Makevars files
that adapt the compilation process of the R package to generate the embedded Rust binary
using R's internal build system.
<!-- Actually, we _could_ do a little better job with that, but this
  part is very sparsely documented by R-core...
 -->
<!-- TODO: add rust_siterep? it is really useful and in spirit of the whole thing -->
This should then be followed by calling `rextendr::document()`, which provides R wrapper functions,
within which the Rust functions are invoked via the `.Call` foreign function interface.
<!-- I assume this is used by e.g. RStudio in place of roxygen2::document, and perhaps also
does Roxygen stuff at the same time?  A brief statement would be useful... -->

<!-- The surrounding R-package needs to know about the exported functions, their
expected signature, and provide the R wrapping code, that calls them via the `.Call`
interface. This wrapper code is generated by `rextendr::document()`. -->
<!-- the type information is stored _in_ the generated rust library... -->

For many R package authors, being able to publish their code on CRAN is essential.
However, CRAN has strict rules for publishing packages, including that the number
of threads that a package uses at build & testing must not exceed 2. Uniquely, Rust
<!-- Is this really unique?  What about reticulate and Python packages (or libraries or whatever they are called)? -->
has a package manager, which means that R packages have 3rd party dependencies external to R and CRAN.
These must be vendored to ensure package stability (see ["Using Rust in CRAN packages
"](https://cran.r-project.org/web/packages/using_rust.html)). The `rextendr::use_cran_defaults()`
and `rextendr::vendor_pkgs()` will ensure that dependencies are built entirely offline and from vendored sources,
which ensures that the resulting R package is fully CRAN-compliant.

# Getting started

Ensure that both [R](https://www.r-project.org) and [Rust](https://www.rust-lang.org/tools/install) are installed.
Then in an R terminal, the rextendr package can be installed via:

```r
install.packages("rextendr") 
```

Or, for the latest development version:

```r
remotes::install_github("extendr/rextendr") # installs latest dev-version
```

Then, an R-package should be constructed - optionally using the `usethis` R-package [@usethis_cran], which inspires
the design principles of `rextendr`:

```r
usethis::create_package("exampleRustRpkg")
```
<!-- Note: usethis is only a suggests for rextendr, so the user would also have to install usethis separately -->

Finally, the function `use_extendr` should be used to set up the necessary boilerplate for compiling
Rust code within an R package, and `document` used to refresh the R function wrappers (this augments
a call to `devtools::document()`).

```r
rextendr::use_extendr()
rextendr::document()
```
<!-- Rust project is in `exampleRustRpkg/src/rust/`. -->

# Acknowledgements

<!-- Acknowledgement of any financial support.  -->

Project lead Andy Thomason received a grant from the R-consortium
[@consortiumConsortiumFundedProject2023].

Mossa Merhi Reimert received funding from the Danish Food and Veterinary Administration for his PhD project.

Claus O. Wilke acknowledges funding from The University of Texas at Austin (Reeder Centennial Fellowship in Systematic and Evolutionary Biology, Blumberg Centennial Professor in Molecular Evolution).

The authors acknowledge the assistance of Matt Denwood and Maya Gussmann with writing this article.

We would like to acknowledge Jeroen Ooms for his [hellorust](https://github.com/r-rust/hellorust) [@hellorust_cran], and continuous maintenance of a hand-written embedding of Rust in an R proof-of-concept project.
Their [github.com/r-rust](https://github.com/r-rust) contains several examples
of hand-crafted bindings to Rust packages for R, such as `gifski` [@gifski_cran].

# References
