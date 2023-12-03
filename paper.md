---
title: extendr
subtitle: extending the R langauge with the power of Rust
---

## Requirements from JOSS

Your paper should include:

A list of the authors of the software and their affiliations, using the correct format (see the example below).

A summary describing the high-level functionality and purpose of the software for a diverse, non-specialist audience.

A Statement of need section that clearly illustrates the research purpose of the software and places it in the context of related work.

A list of key references, including to other software addressing related needs. Note that the references should include full names of venues, e.g., journals and conferences, not abbreviations only understood in the context of a specific discipline.

Mention (if applicable) a representative set of past or ongoing research projects using the software and recent scholarly publications enabled by it.

Acknowledgement of any financial support.

------------------------------------------------------------------------

## Statement of need

-   r is an interface langauge. it comes with a C api to build extensions

-   over a decade ago Rcpp was relesaed revoltionizing R package development making it easy to tap into high performance library from C++

-   today we are seeing very fast growth in the adoption of Rust due to its ease of use, safety, and performance.

-   to ensure that the R ecosystem can stay on op of the maturing data science ecosystem, we need to be able to tap into Rust libraries and make bindings to them in R

    -   PyO3 serves this role for the ptyhon ecosystem and has led to wildly successful libraries such as polars

-   extendr is a rust library that provides R package developers with a way to

## Notes

The R Project for Statistical Computing, referred to simply as R, has a long history of being an interface language.

-   "Writing R extensions" discusses in detail how to create a new interface between an external library or language and R's C API.

-   R's C API is one of the reasons why it is such language. Rcpp's in 2011 (cite)

<!-- -->

-   extendr started as an R-consortium funded project by Andy Thomason.
-   interfaces with R's C API

https://www.r-consortium.org/all-projects/awarded-projects/2021-group-1#extendr+-+rust+extensions+for+r.

related software Rcpp, cpp11,

## 

## Adoption in the R ecosystem

-   rpolars

-   prqlr

-   rsgeo

-   

## References

-   v8 (javascript bindings): https://github.com/jeroen/V8

<!-- -->

-   Rcpp: https://www.jstatsoft.org/article/view/v040i08

<!-- -->

-   cpp11: https://cpp11.r-lib.org/articles/internals.html

-   writing R extensions <https://cran.r-project.org/doc/manuals/R-exts.html>