# `R` <--> `Rust` marshaling rules

This document outlines conversion and validation rules between `R` and `Rust` types.
The aim is to maintain type safety on the `Rust` side without sacrificing usability on the `R` side.


# Conversion and validation 

`extendr` takes into account the following problems when converting `R` object to `Rust` object:

- `NA` handling
  - If `Rust` type is `NA`-aware, `NA`-validation falls onto the user code
  - If `Rust` type is `NA`-oblivious (e.g., vector of basic type), `extendr` performs `NA` validation at runtime and `panic!`s if `NA` value is found (introduces overhead)
- ALTREP handling
  - If `extendr`-provided iterator is used, then ALTREP is exposed as an obscure type with iteration and indexing capabilities
  - If `Rust` type has no notion of ALTREP, then `extendr` unfolds ALTREP vector into an array, allocating memory (introduces overhead)
- Type conversion (applicable to `Vec<T>`)
  - `logical()`, `raw()`, `character()` are treated as-is. If there is a `Rust` - `R` type mismatch, `extendr` wrapper `panic!`s
  - `integer()` can be passed to functions that expect `double()` or `complex()`. `extendr` performs type cast, allocating memory for the new vector
  - `double()` can be safely passed as `complex()` 
  - `double()` can sometimes be passed as `integer()`, if its values are representable by `i32`
  - `complex()` can sometimes be passed as `double()` or `integer()` (see reasoning above)
  - Whenever a numeric type-mismatch happens, a guaranteed allocation occurs
  - An obscure iterator that accepts one of `integer()`, `double()` (and maybe `complex()`) handles both vectors and ALTREPs, does not allocate and offloads all validation and type checks onto user


Here is a list of examples:
- `Vec<i32>` triggers `NA` validation, altrep unfolding, and type coercion if compatible (so `1.0` or `1.0 + 0i` convert to `1L`). Heavy on overhead and memory allocation, good for prototypes and testing things out.

- `Integer` is an obscure wrapper of either `&[Rint]` or some `AltRepInt`. Acts as iterator with indexing capabilities, items are of type `Rint`, providing correct `NA` handling. Preferred way to handle vectors, mimics that of `{cpp11}`. 

- `Numeric` is a discriminated union of `Integer | Real`. Accepts all numeric inputs, but leaves it up to the user to decipher what exactly was received from `R`. No runtime validation, no extra allocation, ALTREPS remain unfolded. 
- `ComplexNumeric` represents either `Complex` or `Numeric`


----------------------------------------------------------------------------

# Underlying vector types
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
- `struct Rcomplex((f64, f64))`
- `struct Rstr(usize)` (?)

Note: `complex` is an `(f64, f64)` struct

Each of these types is binary compatible with their underlying type. An array of, say `i32`, represented by a `*i32` pointer and length, can be viewed as `*Rint` of the same length. 
This can be the preferred solution when dealing with `R` plain vectors.

For each supported primitive type `T` `Rt` would be its minimal wrapper. E.g., for `T = i32`, `Rt = Rint`.
`extendr` prefers `Rt` over `T` types. Parameters that use `T`-derived types will require runtime `NA` validation, introducing implicit overhead.

Type conversion traits for `Rt` are:
- `Into<Option<T>>` (this is always a valid conversion)
- `TryInto<T>`, errors on `NA`
- `TryFrom<T>`, errors if provided argument equals to the value reserved for `NA`
- `TryFrom<Option<T>>` for the same reason, as `Some(i32::MIN)` is invalid `Rint` value (instead, `None` should be used)

These conversions can be grouped in a trait `Rtype<T>`, which exposes conversions `Rt` <--> `T` mentioned above, as well as some `is_na() -> bool` method (and perhaps some other useful ones).

A limited number of binary-incompatible type conversions is also allowed. These rules are required to support common use scenarios on `R` side.

For `Rint` the following is allowed:
- `Into<Rfloat>`, this is always correct (all `i32` are within `f64` with no loss of accuracy)
- `Into<Rcomplex>`, for the same reason
  
For `Rfloat`
- `Into<Rcomplex>`, (`Real(f64)` are within `(Real(f64), Imaginary(f64))`)
- `TryInto<Rint>`; this conversion succeeds only when `f64` can be precisely represented as `i32` (lossless), e.g. `1.0f64` convert to `1i32`

For `Rcomplex` (see reasoning above)
- `TryInto<Rfloat>`
- `TryInto<Rint>`

Other primitive types are treated as-is and any type conversion should be performed by extracting the underlying value (or `NA`) and casting/converting it to another type manually.

### ALTREP

A separate public API for ALTREPs is not needed, there are no real use cases for a method to only accept ALTREPs. Instead, expose the following iterator types:
- `Integer`
- `Logical`
- `Double`
- `Raw`
- `Complex`
- `Character`


These opaque iterators wrap either plain data vectors (e.g., storing pointer & length) or ALTREPs. 
They should implement `std::iter::Iterator<Item = Rt>` to support `NA` validation, as well as `std::ops::Index<Output = Rt>`.

Another suggested methods:
- `len() -> usize` as both plain data and ALTREP know their size,
- `is_altrep() -> bool` to avoid unnecessary random access in case of ALTREP

*Note*: It seems `Rust` has no standard trait for collections (that is, something that has a length and an indexer).


The iterators are enriched with the following discriminated unions:
- `Numeric = Integer | Double`
- `ComplexNumeric = Integer | Double | Complex`

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
| `real()` / `complex()` | Yes         | Yes      |  If `NA` found   | Runtime            |

2. Close to metal (aka performance)
```Rust
#[extendr]
fn fn_2(x : ComplexNumeric)
```
| `R` type               | Allocation  | Coercion | Error            | Validation  |
| ---------------------- | ----------- | -------- | ---------------- | ----------- |
| `integer()`            | No          | No       |  No              | User        |
| `altrep_integer()`     | No          | No       |  No              | User        |
| `double()`             | No          | No       |  No              | User        |
| `altrep_double()`      | No          | No       |  No              | User        |
| `complex()`            | No          | No       |  No              | User        |
| `altrep_complex()`     | No          | No       |  No              | User        |

3. Reasonable 
```Rust
#[extendr]
fn fn_3(x : &[Rint])
```
| `R` type               | Allocation  | Coercion | Error               | Validation         |
| ---------------------- | ----------- | -------- | ------------------- | ------------------ |
| `integer()`            | No          | No       |  No                 | User               |
| `altrep_integer()`     | Yes         | No       |  No                 | User               |
| `(altrep_)real()`      | Yes         | Yes      |  If `x != floor(x)` | Runtime & User     |
| `(altrep_)complex()`   | Yes         | Yes      |  If `x != floor(x)` | Runtime & User     |


</details>

# Return type conversions
The procedure is reversed. The preferred way it so return a `Vec<Rt>`, which is correctly encodes `NA`s. If `Vec<T>` is returned, then validation is performed by the wrapper, and `panic!` occurs if an invalid value is found (i.e., if `Vec<i32>` contains `i32::MIN`, which is an invalid value in `R`).
