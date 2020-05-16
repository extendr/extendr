# extendr - A safe and user friendly R extension interface using Rust.

Low-level R library bindings

[![Travis Build Status](https://api.travis-ci.org/extendr/extendr-api.svg?branch=master)](https://travis-ci.org/extendr/extendr-api)
[![Crates.io](http://meritbadge.herokuapp.com/extendr-api)](https://crates.io/crates/extendr-api)

[API Documentation](https://extendr.github.io/extendr-api/master/extendr-api/index.html)

This library aims to provide an interface that will be familiar to
first-time users of Rust or indeed any compiled language.

Anyone who knows the R library should be able to write R extensions.


This library is just being born, but goals are:

A macro-based interface to R internal functions and language
features.

Example:

```
let v = c!(1, 2, 3);
let l = list!(a=1, b=2);
print!(paste0!("v=", v, " l=", l));
```

Provide a safe wrapper for r objects with error handlng
and panic-free execution.

Example:

```
let s = r!("hello");
let i = r!(1);
let r = r!(1.0);
```

Provide iterator support for creation and consumption of r vectors.

Example:

```
let res = (1..=100).iter().collect::<RObj>();
for x in res.as_i32_slice() {
    println!("{}", x)?;
}
```

Provide a procedural macro to adapt Rust functions to R

Example:

```
#[r_bindgen]
fn fred(a: i32) -> i32 {
    a + 1
}
```

In R:

```

result <- .Call("fred", 1)

```

## extendr roadmap

### Basic
- [x] Be able to build simple rust extensions for R.
- [x] Wrap the R SEXP object safely (Robj)
- [ ] Iterator support for matrices and vectors.
- [ ] Class support.

### Documentation
- [x] Begin documentation.
- [ ] Begin book-form documentation.
- [ ] Paper for Bioinformatics.
- [ ] Build and publish CRAN R package.
- [ ] Publish Use R! series book.

### Automation
- [ ] Auto-generate binding wrappers.
- [ ] Auto-generate NAMESPACE and lib.R.

### Features
- [ ] Feature-gated support for ndarray.
- [ ] Feature-gated support for rayon.

### R packages
- [ ] Bindings for rust-bio

