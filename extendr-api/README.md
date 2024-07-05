# `extendr-api`

`extendr-api` is an opinionated, ergonomic, and safe interface to R API.

## Installation

Simply add this line to the `[dependencies]` section of your `Cargo.toml`.
You will then be able to call R code from Rust.

```toml
[dependencies]
extendr-api = "0.7"
```

## About

On the [extendr homepage](https://extendr.github.io/) there is a [comprehensive user-guide](https://extendr.github.io/user-guide/).

The [API documentation on doc.rs](https://docs.rs/extendr-api/latest/extendr_api/), and for
[development API documentation](https://extendr.github.io/extendr/extendr_api/).

## Overview

See `Robj` for much of the content of this crate.
`Robj` provides a safe wrapper for the R object type.

Use attributes and macros to export to R.

For a module named `mymodule` (typically in a file named `mymodule.rs`)

```rust
use extendr_api::prelude::*;
// Export a function or impl to R.
#[extendr]
fn fred(a: i32) -> i32 {
    a + 1
}

// define exports using extendr_module
extendr_module! {
   mod mymodule;
   fn fred;
}
```

In R:

```rust
result <- fred(1)
```

`Robj` is a wrapper for R objects.
The `r!()` and `R!()` macros let you build R objects
using Rust and R syntax respectively.

```rust
use extendr_api::prelude::*;
test! {
    // An R object with a single string "hello"
    let character = r!("hello");
    let character = r!(["hello", "goodbye"]);

    // An R integer object with a single number 1L.
    // Note that in Rust, 1 is an integer and 1.0 is a real.
    let integer = r!(1);

    // An R real object with a single number 1.
    // Note that in R, 1 is a real and 1L is an integer.
    let real = r!(1.0);

    // An R real vector.
    let real_vector = r!([1.0, 2.0]);
    let real_vector = &[1.0, 2.0].iter().collect_robj();
    let real_vector = r!(vec![1.0, 2.0]);

    // An R function object.
    let function = R!("function(x, y) { x + y }")?;

    // A named list using the list! macro.
    let list = list!(a = 1, b = 2);

    // An unnamed list (of R objects) using the List wrapper.
    let list = r!(List::from_values(vec![1, 2, 3]));
    let list = r!(List::from_values(vec!["a", "b", "c"]));
    let list = r!(List::from_values(&[r!("a"), r!(1), r!(2.0)]));

    // A symbol
    let sym = sym!(wombat);

    // A R vector using collect_robj()
    let vector = (0..3).map(|x| x * 10).collect_robj();
}
```

In Rust, we prefer to use iterators rather than loops.

```rust
use extendr_api::prelude::*;
test! {
    // 1 ..= 100 is the same as 1:100
    let res = r!(1 ..= 100);
    assert_eq!(res, R!("1:100")?);

    // Rust arrays are zero-indexed so it is more common to use 0 .. 100.
    let res = r!(0 .. 100);
    assert_eq!(res.len(), 100);

    // Using map is a super fast way to generate vectors.
    let iter = (0..3).map(|i| format!("fred{}", i));
    let character = iter.collect_robj();
    assert_eq!(character, r!(["fred0", "fred1", "fred2"]));
}
```

To index a vector, first convert it to a slice and then
remember to use 0-based indexing. In Rust, going out of bounds
will cause and error (a panic) unlike C++ which may crash.

```rust
use extendr_api::prelude::*;
test! {
    let vals = r!([1.0, 2.0]);
    let slice = vals.as_real_slice().ok_or("expected slice")?;
    let one = slice[0];
    let two = slice[1];
    // let error = slice[2];
    assert_eq!(one, 1.0);
    assert_eq!(two, 2.0);
}
```

Much slower, but more general are these methods:

```rust
use extendr_api::prelude::*;
test! {
    let vals = r!([1.0, 2.0, 3.0]);

    // one-based indexing [[i]], returns an object.
    assert_eq!(vals.index(1)?, r!(1.0));

    // one-based slicing [x], returns an object.
    assert_eq!(vals.slice(1..=2)?, r!([1.0, 2.0]));

    // $ operator, returns an object
    let list = list!(a = 1.0, b = "xyz");
    assert_eq!(list.dollar("a")?, r!(1.0));
}
```

The `R!` macro lets you embed R code in Rust
and takes Rust expressions in `{{ }}` pairs.

The `Rraw!` macro will not expand the `{{ }}` pairs.

```rust
use extendr_api::prelude::*;
test! {
    // The text "1 + 1" is parsed as R source code.
    // The result is 1.0 + 1.0 in Rust.
    assert_eq!(R!("1 + 1")?, r!(2.0));

    let a = 1.0;
    assert_eq!(R!("1 + {{a}}")?, r!(2.0));

    assert_eq!(R!(r"
        x <- {{ a }}
        x + 1
    ")?, r!(2.0));

    assert_eq!(R!(r#"
        x <- "hello"
        x
    "#)?, r!("hello"));

    // Use the R meaning of {{ }} and do not expand.
    assert_eq!(Rraw!(r"
        x <- {{ 1 }}
        x + 1
    ")?, r!(2.0));
}
```

The `r!` macro converts a rust object to an R object
and takes parameters.

```rust
use extendr_api::prelude::*;
test! {
    // The text "1.0+1.0" is parsed as Rust source code.
    let one = 1.0;
    assert_eq!(r!(one+1.0), r!(2.0));
}
```

You can call R functions and primitives using the `call!` macro.

```rust
use extendr_api::prelude::*;
test! {

    // As one R! macro call
    let confint1 = R!("confint(lm(weight ~ group - 1, PlantGrowth))")?;

    // As many parameterized calls.
    let formula = call!("~", sym!(weight), lang!("-", sym!(group), 1))?;
    let plant_growth = global!(PlantGrowth)?;
    let model = call!("lm", formula, plant_growth)?;
    let confint2 = call!("confint", model)?;

    assert_eq!(confint1.as_real_vector(), confint2.as_real_vector());
}
```

Rust has a concept of "Owned" and "Borrowed" objects.

Owned objects, such as `Vec` and `String` allocate memory
which is released when the object lifetime ends.

Borrowed objects such as `&[i32]` and `&str` are fat pointers
to another object's memory and can't live longer than the
object they reference.

Borrowed objects are much faster than owned objects and use less
memory but are used only for temporary access.

When we take a slice of an R vector, for example, we need the
original R object to be alive or the data will be corrupted.

```rust
use extendr_api::prelude::*;
test! {
    // robj is an "Owned" object that controls the memory allocated.
    let robj = r!([1, 2, 3]);

    // Here slice is a "borrowed" reference to the bytes in robj.
    // and cannot live longer than robj.
    let slice = robj.as_integer_slice().ok_or("expected slice")?;
    assert_eq!(slice.len(), 3);
}
```

## Feature gates

extendr-api has some optional features behind these feature gates:

* `ndarray`: provides the conversion between R's matrices and [`ndarray`](https://docs.rs/ndarray/latest/ndarray/).
* `num-complex`: provides the conversion between R's complex numbers and [`num-complex`](https://docs.rs/num-complex/latest/num_complex/).
* `serde`: provides the [`serde`](https://serde.rs/) support.
* `graphics`: provides the functionality to control or implement graphics devices.
* `either`: provides implementation of type conversion traits for `Either<L, R>` from [`either`](https://docs.rs/either/latest/either/) if `L` and `R` both implement those traits.
* `faer`: provides conversion between R's matrices and [`faer`](https://docs.rs/faer/latest/faer/).

extendr-api has different encodings (conversions) of a `Result<T,E>` into an `Robj`.
In below `x_ok` represents an R variable on R side which was returned from rust via `T::into_robj()` or similar.
Likewise `x_err` was returned to R side from rust via `E::into_robj()` or similar.
extendr-api

* `result_list'` `Ok(T)` is encoded as `list(ok = x_ok, err = NULL)` and `Err` as `list(ok = NULL, err = e_err)`
* `result_condition'` `Ok(T)` is encoded as `x_ok` and `Err(E)` as `condition(msg="extendr_error", value = x_err, class=c("extendr_error", "error", "condition"))`
* Multiple of above result feature gates. Only one result feature gate will take effect, the precedence is currently [`result_list`, `result_condition`, ... ].

Finally, there are parts of R's API that are deemed non-API, in that R packages
on CRAN are recommended not to have these available in packages. If you want
to have access to them, you may use `non-api` to expose them. However, it requires
setting up `bindgen`, and specifically following setup instructions for [`libR-sys`](https://github.com/extendr/libR-sys/?tab=readme-ov-file#building-bindings-from-source-advanced).

## License

MIT
