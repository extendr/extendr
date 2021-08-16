# `R` <--> `Rust` marshaling rules

This document outlines conversion and validation rules between `R` and `Rust` types.
The aim is to maintain type safety on the `Rust` side without sacrificing usability on the `R` side.


# Conversion and validation configuration
The following configurations can be applied to each exported `Rust` function using the `#[extendr(...)]` syntax:
- Validation:
  - `Strict` -- no compromises, disallows usage of built-in types like `i32`, which mistreat `NA` values;
  - `Relaxed` -- allows usage of built-in types, user is responsible for correctly handling input data. Maximizes performance as there are no extra checks or conversions;
  - `Runtime (default)` -- allows usage of built-in types, but performs runtime validation of the input, panicking if `NA` is detected. Introduces some overhead;
- ALTREP handling
  - `UnfoldToVec (default)` -- allocates memory for ALTREP vectors if parameter type is a `&[T]` or `Vec<T>`. Can potentially waste memory;
  - `IteratorOnly` -- panics if parameter type is not an iterator;
- Type coercion
  - `NoCoercion` -- exact `R` <--> `Rust` type matches. If input is `c(1, 2, 3)` and parameter type is `&[Rint]`, panics because of the type mismatch;
  - `SafeCoercion (default)` -- inspired by [`{vctrs}`](https://vctrs.r-lib.org/reference/theory-faq-coercion.html), allows coercion `logical` --> `integer` --> `double` (and possibly --> `complex`) with no restrictions. The reverse coercion happens only if the coerced value falls within the value range of the target type. `1 + 0i` can be coerced to `1.0`, then to `1L`, and finally to `TRUE`. Otherwise, panics. Introduces overhead;
  - (optionally) `RCoercion` -- relies on `R` coercion rules (`as._` methods). Produces unpredictable results (e.g., `as.logical("cat") == NA`).

| Validation |   ALTREP       | Allowed `Rust` types |
| ---------- | ---------      | ------------ |
| `Strict`   | `UnfoldToVec`  | `Vec<Rint>`; `&[Rint]`; `Either<&[Rint], AltRepInt>` |
| `Strict`   | `IteratorOnly` | `Either<&[Rint], AltRepInt>` |
| `Relaxed`  | `UnfoldToVec`  | `Vec<Rint>`; `&[Rint]`; `Either<&[Rint], AltRepInt>`; `Vec<i32>`; `&[i32]`; `Either<&[i32], AltRepInt>` |
| `Relaxed`  | `IteratorOnly` | `Either<&[Rint], AltRepInt>` ; `Either<&[i32], AltRepInt>`|
| `Runtime`  | `UnfoldToVec`  | Any |
| `Runtime`  | `IteratorOnly` | `Either<&[Rint], AltRepInt>` ; `Either<&[i32], AltRepInt>` |

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

### ALTREP
**TODO: make a detailed description of ALTREP**
Assume there are ALTREP wrappers for each type
- `AltRepInt`
- `AltRepBool`
- `AltRepFloat`
- `AltRepByte`
- `AltRepCmpl`
- `AltRepStr`

Altreps should be iterators of types `Rxxx`, so `AltRepInt` should produce `Rint`.
If altreps cannot produce `NA`s, then they can iterate over primitive types like `i32` (i.e. with no `NA` validation mechanism).

**Bare ALTREP types should not be allowed as function parameters, but may be used as return types**.

### Handling both plain and ALTREP vectors
To enable direct consumption of ALTREP vector wrappers (without explicit memory allocations), a simple approach can be used. The monad `Either` is an excellent solution for representing either plain vector or its ALTREP wrapper.
The following example illustrates this case:
```Rust
fn accept_vector_no_alloc(vec : Either<&[Rint], AltRepInt>)
```
If the input is a plain vector, `Either` wraps pointer using `&[Rint]` (no allocation),
If otherwise, an ALTREP wrapper struct is created.
This type covers **all** possible vector inputs (disallowing `NULL`) of `R` type `integer`, and correctly handles `NA`s. 






## TL;DR
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
fn fn_2(x : Either<&[i32], AltRepInt>)
```
| `R` type               | Allocation  | Coercion | Error            | Validation                |
| ---------------------- | ----------- | -------- | ---------------- | ------------------------- |
| `integer()`            | No          | No       |  No              | None (`NA` is `i32::MIN`) |
| `altrep_integer()`     | No          | No       |  No              | None                      |

3. Reasonable 
```Rust
#[extendr(validation = Strict, altrep_handling = UnfoldToVec, coercion = SafeCoercion)]
fn fn_3(x : &[Rint])
```
| `R` type               | Allocation  | Coercion | Error               | Validation         |
| ---------------------- | ----------- | -------- | ------------------- | ------------------ |
| `integer()`            | No          | No       |  No                 | User               |
| `altrep_integer()`     | Yes         | No       |  No                 | User               |
| `real()` / `complex()` | Yes         | Yes      |  If `x != floor(x)` | Runtime & User               |

