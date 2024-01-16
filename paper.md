---
title: "extendr"
tags:
  - R
  - Rust
  - FFI
  - bindings
subtitle: "extending the R language with the power of Rust"
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
format:
  pdf:
    documentclass: scrartcl
    papersize: a4paper
---

# Summary


# Statement of Need

One of the key strengths of the R programming language is its ability to integrate with other languages and tools. It does so through its C API as described in Writing R Extensions [@r_cite]. In 2006, the R package Rcpp was released on CRAN enabling R developers to build packages that harnessed the power of C++ (Rcpp [@rcpp_cran]). Since then, Rcpp has grown to be the most imported R package across the Comprehensive R Archive Network (CRAN) due to its use in low-level libraries that are used by the vast majority of R packages ([@vriesAndriePagerank2021]). In addition to Rcpp there are many other packages that enable cross-langauge communication. The package cpp11 provides similar bindings to R via C++ ([@cpp11]). Bindings to Java are provided by rJava, to JavaScript via V8, and to python via reticulate ([@rJava], [@v8_cran], [@reticulate_cran]).

[@chambers2017extending]

Today, we are seeing the proliferation of the Rust programming language. According to StackOverflow, Rust is the most admired programming language for many years running—and for good reason (<https://survey.stackoverflow.co/2023/>). Rust provides similar performance such as C and C++ while also being far more ergonomic ([@perkelWhyScientistsAre2020]). But most importantly, Rust provides guarantees memory that make exceptionally safe. For all of these reasons and more, providing R package developers a way to integrate Rust and R is necessary for the continued growth of the R ecosystem. The extendr Rust library and its companion R package `{rextendr}` make the process of marrying R and Rust simple.

## Implementation

extendr utilizes R's C API via the libR-sys library crate. libR-sys utilizes the Rust library bindgen to automatically create foreign function interface (FFI) bindings for all major distributions (<https://github.com/rust-lang/rust-bindgen>). The bindings provided by libR-sys create Rust representations of the structs defined in R's C API. On top of these exceptionally low-level bindings, extendr is built. extendr defines a number of user friendly Rust structs that can be be passed directly to and from R and Rust.

### Scalar types

| R type      | Extendr wrapper                        | Deref type: `&*object` |
|------------------|----------------------------------|---------------------|
| `Any`       | `extendr_api::robj::Robj`              | N/A                    |
| `character` | `extendr_api::wrapper::Rstr`           | N/A                    |
| `integer`   | `extendr_api::wrapper::Rint`           | N/A                    |
| `double`    | `extendr_api::wrapper::Rfloat`         | N/A                    |
| `complex`   | `extendr_api::wrapper::Rcplx`          | N/A                    |
| `extptr`    | `extendr_api::wrapper::ExternalPtr<T>` | `&T`                   |

### Vector types

| R type       | Extendr wrapper                      | Deref type: `&*object` |
|------------------|---------------------------------|---------------------|
| `integer`    | `extendr_api::wrapper::Integer`      | `&[Rint]`              |
| `double`     | `extendr_api::wrapper::Doubles`      | `&[Rfloat]`            |
| `logical`    | `extendr_api::wrapper::Logical`      | `&[Rbool]`             |
| `complex`    | `extendr_api::wrapper::Complexes`    | `&[Rcplx]`             |
| `string`     | `extendr_api::wrapper::Strings`      | `&[Rstr]`              |
| `list`       | `extendr_api::wrapper::List`         | `&[Robj]`              |
| `data.frame` | `extendr_api::wrapper::Dataframe<T>` | `&[Robj]`              |
| `expression` | `extendr_api::wrapper::Expression`   | `&[Lang]`              |

### Linked list types

| R type     | Extendr wrapper                  | Deref type: `&*object` |
|------------|----------------------------------|------------------------|
| `pairlist` | `extendr_api::wrapper::Pairlist` | N/A                    |
| `lang`     | `extendr_api::wrapper::Lang`     | N/A                    |

## Example usage

## Case studies

- rpolars

- prqlr

- rsgeo

## Related work

Integration of Rust and R has been explored in other libraries. The `roxido` library crate and accompanying R package `cargo` by David B. Dahl are an alternative approach to creating Rust bindings ([@cargo_cran]). An offshoot of extendr, [savvy](https://github.com/yutannihilation/savvy), is an "unfriendly" lower-level approach to generating Rust and R bindings in which "ergonomics is not included.".

## Notes

- r is an interface langauge. it comes with a C api to build extensions

- over a decade ago Rcpp was released revoltionizing R package development making it easy to tap into high performance library from C++

  - cpp11 is a fairly recent take on the same objective by the folks from the r-lib team

  - V8 is another take on interfacing with another language enabling R users to call javaScript via V8

- today we are seeing very fast growth in the adoption of Rust due to its ease of use, safety, and performance.

- to ensure that the R ecosystem can stay on op of the maturing data science ecosystem, we need to be able to tap into Rust libraries and make bindings to them in R

  - PyO3 serves this role for the ptyhon ecosystem and has led to wildly successful libraries such as polars

- extendr is a rust library that provides R package developers with a way to create R packages that utilize the power and safety of Rust

- it creates bindings to R's C API via the low-level Rust crate libR-sys that supports extendr.

- extendr comes with a companion R package called {rextendr}

  - rextendr is a user friendly package that is used for creating the scaffolding of a rust-enabled R package

  - it documents Rust functions and creats wrappers to rust functions that are then exported to R via the `.Call()` function interface

- extendr works by creating a staticlib that is called by R

- extendr has already seen a fair amount of adoption in the R ecosystem. Notably it has been used to develop the R package {rpolars} which are R bindings to polars rust data frame library.

  - prqlr which are bindings to the prql crust compiler library that generates sql queries.

  - rsgeo are bindings to geo-rust geometry primitives and algorithms which are very performant

- extendr is extensible meaning that other rust-crates can be developed to integrate external rust libraries with extendr and thus R

  - a recent example is the arrow-extendr library crate which enables conversion from from R's arrow and nanoarrow R packages to Apache Arrow arrow-rs rust implementation.

The R Project for Statistical Computing, referred to simply as R, has a long history of being an interface language.

- "Writing R extensions" discusses in detail how to create a new interface between an external library or language and R's C API.

- R's C API is one of the reasons why it is such language. Rcpp's in 2011 [@rcpp_jss]

- extendr started as an R-consortium funded project by Andy Thomason.
- interfaces with R's C API

<https://www.r-consortium.org/all-projects/awarded-projects/2021-group-1#extendr+-+rust+extensions+for+r>.

related software Rcpp, cpp11,

## Adoption in the R ecosystem

- rpolars
- prqlr
- rsgeo
- [`changeforest`](https://github.com/mlondschien/changeforest/tree/main) [@JMLR:v24:22-0512]

Acknowledge [hellorust](https://github.com/r-rust/hellorust) [@hellorust_cran]

## Requirements from JOSS

Your paper should include:

A list of the authors of the software and their affiliations, using the correct format (see the example below).

A summary describing the high-level functionality and purpose of the software for a diverse, non-specialist audience.

A Statement of need section that clearly illustrates the research purpose of the software and places it in the context of related work.

A list of key references, including to other software addressing related needs. Note that the references should include full names of venues, e.g., journals and conferences, not abbreviations only understood in the context of a specific discipline.

Mention (if applicable) a representative set of past or ongoing research projects using the software and recent scholarly publications enabled by it.

Acknowledgement of any financial support. Project lead Andy Thomason received a grant from the R-consortium
[@consortiumConsortiumFundedProject2023]

<!-- - v8 (javascript bindings): <https://github.com/jeroen/V8> -->

<!-- - Rcpp: <https://www.jstatsoft.org/article/view/v040i08> -->

<!-- - cpp11: <https://cpp11.r-lib.org/articles/internals.html> -->

- writing R extensions <https://cran.r-project.org/doc/manuals/R-exts.html>

<!-- - <https://raw.githubusercontent.com/dbdahl/cargo-framework/main/cargo/inst/doc/Writing_R_Extensions_in_Rust.pdf> -->

# References
