# `R` <--> `Rust` marshaling rules

This document outlines conversion and validation rules between `R` and `Rust` types.
The aim is to maintain type safety on the `Rust` side without sacrificing usability on the `R` side.


# Conversion and validation configuration
[comment01](https://github.com/extendr/extendr/pull/261#discussion_r690303432)

Configurations described below cover all possible cases. However, some of the combinations may not be needed in a real-life application. Suggested approach is to implement type-based configuration, which depends on the argument types defined in the user code. Here are some (top of the head) examples:

- `Vec<i32>` triggers `NA` validation, altrep unfolding, and type coercion if compatible (so `1.0` converts to `1L`). Heavy on overhead and memory allocation, good for prototypes and testing things out.
- `&[Rint]` is a responsible approach, which for array-based vectors of type `integer()` results in no allocations and no validation (zero overhead in the wrapper). **Unclear if type coercion should be performed**
- `Integer` is an obscure wrapper of either `&[Rint]` or some `AltRepInt`. Acts as iterator with indexing capabilities, items are of type `Rint`, providing correct `NA` handling. Preferred way to handle vectors, mimics that of `{cpp11}`. **Unclear if type coercion should be performed**
- (**Undecided**) `Numeric` is a discriminated union of `Integer | Real | Complex`. Accepts all numeric inputs, but leaves it up to the user to decipher what exactly was received from `R`. No runtime validation, no extra allocation, altreps remain altreps. 


The following configurations can be applied to each exported `Rust` function using the `#[extendr(...)]` syntax:
- Validation (**Suggested implementation -- type-based**):
  - `Strict` -- no compromises, disallows usage of built-in types like `i32`, which mistreat `NA` values;
  - `Relaxed` -- allows usage of built-in types, user is responsible for correctly handling input data. Maximizes performance as there are no extra checks or conversions;
  - `Runtime (default)` -- allows usage of built-in types, but performs runtime validation of the input, panicking if `NA` is detected. Introduces some overhead;
- ALTREP handling (**Suggested implementation -- type-based**)
  - `UnfoldToVec (default)` -- allocates memory for ALTREP vectors if parameter type is a `&[T]` or `Vec<T>`. Can potentially waste memory;
  - `IteratorOnly` -- panics if parameter type is not an iterator;
- Type coercion (**Undecided**)
  - `NoCoercion` -- exact `R` <--> `Rust` type matches. If input is `c(1, 2, 3)` and parameter type is `&[Rint]`, panics because of the type mismatch;
  - `SafeCoercion (default)` -- inspired by [`{vctrs}`](https://vctrs.r-lib.org/reference/theory-faq-coercion.html), allows coercion `logical` --> `integer` --> `double` (and possibly --> `complex`) with no restrictions. The reverse coercion happens only if the coerced value falls within the value range of the target type. `1 + 0i` can be coerced to `1.0`, then to `1L`, and finally to `TRUE`. Otherwise, panics. Introduces overhead;
  - (optionally) `RCoercion` -- relies on `R` coercion rules (`as._` methods). Produces unpredictable results (e.g., `as.logical("cat") == NA`).

<details>
<summary>Allowed types as function of configuration</summary>

| Validation |   ALTREP       | Allowed `Rust` types |
| ---------- | ---------      | ------------ |
| `Strict`   | `UnfoldToVec`  | `Vec<Rint>`; `&[Rint]`; `Integer` |
| `Strict`   | `IteratorOnly` | `Integer` |
| `Relaxed`  | `UnfoldToVec`  | `Vec<Rint>`; `&[Rint]`; `Integer`; `Vec<i32>`; `&[i32]` |
| `Relaxed`  | `IteratorOnly` | `Integer` ; `Integer`|
| `Runtime`  | `UnfoldToVec`  | Any |
| `Runtime`  | `IteratorOnly` | `Integer` |

</details>

----------------------------------------------------------------------------

# Vector Types
## Terminology
A 'vector' is a primitive type used in `R`. Vectors are designed to behave as a strongly typed 1D array of objects. There are two different implementations of vector types: one is basically a pointer to a contiguous block of memory with known length (and some additional metadata), another is an iterator deigned to store rules for generating sequences of elements (instead of storing potentially very large vectors in memory). Array-based vectors shall be referred to as 'plain old data' (POD), iterators -- as ALTREP.

`R` recognizes the following vector types that are directly exposed to the user:
 - `logical (i32)`
 - `integer (i32)`
 - `real (f64)`
 - `complex (f64, f64)`
 - `raw (u8)`
 - `character (usize)` (collection of pointers to character arrays)

Each vector can contain special `NA` values. None of the primitive types have built-in support for `NA` (including `f64`, which has notion of `NaN`, a different thing), so `R` treats one value from the range of allowed values as `NA`. For instance, `NA_integer_` is `i32::MIN`, which is `1i32 << 31 = -2147483648`. As a result, `x <- -2147483648L` results in an error in `R`.

## `Rust` counterparts

`R` objects passed to `Rust` require additional validation and transformation. Let us define the following types:
- `struct Rint(i32)`
- `struct Rbool(i32)`
- `struct Rfloat(f64)`
- `struct Rbyte(u8)`
- `struct Rcmpl(f64, f64)`
- `struct Rstr(usize)` (?)

Each of these types is binary compatible with their underlying type. An array of, say `i32`, represented by a `*i32` pointer and length, can be reinterpreted as `*Rint` of the safe length. 
This can be the preferred solution when dealing with `R` plain vectors.

[comment01](https://github.com/extendr/extendr/pull/261#issuecomment-901096354):
Prefer `Rt` over `T` types. Parameters that use `T`-derived types will require runtime `NA` validation, introducing implicit overhead.

Suggested type conversion traits for `Rt` are:
- `Into<Option<T>>` (this is always a valid conversion)
- `TryInto<T>`, errors on `NA`
- `TryFrom<T>`, errors if provided argument equals to the value reserved for `NA`
- `TryFrom<Option<T>>` for the same reason, as `Some(i32::MIN)` is invalid `Rint` value (instead, `None` should be used)

These conversions can be grouped in a trait `CanBeNA<T>` or `Rtype<T>` (**name can be discussed**), which exposes conversions `Rt` <--> `T` mentioned above, as well as some `is_na() -> bool` method (and perhaps some other useful ones).



### ALTREP
**TODO: make a detailed description of ALTREP**

[comment01](https://github.com/extendr/extendr/pull/261#discussion_r690781040); 
[comment02](https://github.com/extendr/extendr/pull/261#discussion_r690786944)

A separate public API for altreps is not needed, there are no real use cases for a method to only accept altreps. Instead, expose the following iterator types:
- `Integer`
- `Logical`
- `Double`
- `Raw`
- `Complex`
- `Character`

These opaque iterators wrap either plain data vectors (e.g., storing pointer & length) or altreps. 
They should likely implement `std::iter::Iterator<Item = Rt>` to support `NA` validation, as well as `std::ops::Index<Output = Rt>`.
Another suggested methods:
- `len() -> usize` as both plain data and altrep know their size,
- `is_altrep() -> bool` to avoid unnecessary random access in case of altrep

*Note*: It seems `Rust` has no standard trait for collections (that is, something that has a length and an indexer).



<details>
<summary> TL;DR </summary>
Here is a set of functions with different parameter types and allowed arguments.

1. Default (aka comfortable on both ends)
```Rust
#[extendr]
fn fn_1(x : &[i32])
```
| `R` type               | Allocation  | Coercion | Error            | Validation         |
| ---------------------- | ----------- | -------- | ---------------- | ------------------ |
| `integer()`            | No          | No       |  If `NA` found   | Runtime            |
| `altrep_integer()`     | Yes         | No       |  If `NA` found   | Runtime            |
| `real()` / `complex()` | Yes         | Yes      |  If `NA` found   | Runtime |

2. Close to metal (aka performance)
```Rust
#[extendr(validation = Relaxed, altrep_handling = IteratorOnly, coercion = NoCoercion)]
fn fn_2(x : Integer)
```
| `R` type               | Allocation  | Coercion | Error            | Validation  |
| ---------------------- | ----------- | -------- | ---------------- | ----------- |
| `integer()`            | No          | No       |  No              | None        |
| `altrep_integer()`     | No          | No       |  No              | None        |

3. Reasonable 
```Rust
#[extendr(validation = Strict, altrep_handling = UnfoldToVec, coercion = SafeCoercion)]
fn fn_3(x : &[Rint])
```
| `R` type               | Allocation  | Coercion | Error               | Validation         |
| ---------------------- | ----------- | -------- | ------------------- | ------------------ |
| `integer()`            | No          | No       |  No                 | User               |
| `altrep_integer()`     | Yes         | No       |  No                 | User               |
| `real()` / `complex()` | Yes         | Yes      |  If `x != floor(x)` | Runtime & User     |

</details>