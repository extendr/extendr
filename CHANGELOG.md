# Changelog

## Unreleased

### Added

- New optional `faer` feature which enables conversion between `faer` matrix and `RMatrix<f64>` [[#706]](https://github.com/extendr/extendr/pull/706)
- Adds `TryFrom<Robj>` and `<TryFrom<&Robj>` for `impl` blocks marked with `#[extendr]` macro allowing falliable conversion to `&Self` `&mut Self` 
- Adds `From<T> for Robj` for impl blocks marked with `#[extendr]` macro
- The new `ExpectedExternalNonNullPtr` error variant provides a more informative error when a null pointer is accessed
- `RArray::data_mut` provides a mutable slice to the underlying array data [[#657]](https://github.com/extendr/extendr/pull/657)
- Implements the `Attributes` trait for all R vector wrapper structs (e.g. `Integers` , `Doubles`, `Strings`, etc.), allowing for easy access and setting of the attributes of an R object [[#745]](https://github.com/extendr/extendr/pull/745). This comes with breaking changes. See below.
- feature `non-api` that gives access to non-API items; Requires compile-time generation of bindings
[[#754]](https://github.com/extendr/extendr/pull/754)
- `TryFrom<&Robj>` for `StrIter`, `HashMap<K, Robj>` for `K = String` and `K = &str` [[#759]](https://github.com/extendr/extendr/pull/759)

### Changed

- [**Breaking change**]: the `Attributes` trait now returns a mutable reference
  to `Self`. [[#745]](https://github.com/extendr/extendr/pull/745). Previously `.set_attrib()` would modify an object in
  place, and then return an untyped owned pointer (Robj). Instead, now we return `&mut Self`.
- [_Potentially breaking_]: `RArray::from_parts` no longer requires a pointer to the underlying data
  vector [[#657]](https://github.com/extendr/extendr/pull/657)
- `#[extendr(use_try_from = true)` is now the default setting, therefore the option has `use_try_from` has been removed [[#759]](https://github.com/extendr/extendr/pull/759)

#### Breaking changes

- In `AltRep` the `unserialize_ex`, `set_parent`, `set_envflags` are
now hidden behind the feature flag `non-api`. Also, `Promise::from_parts` is marked as non-API.
- Floating point numbers with decimal part can no longer be converted to integer types via
  rounding [[#757]](https://github.com/extendr/extendr/pull/757)
- You can no longer create an `Robj` from a reference `&T`, where `T` is an `extendr`-impl. [[#759]](https://github.com/extendr/extendr/pull/759)
- You can no longer use `from_robj`, as the trait `FromRobj` as been removed. Instead, use `try_from`.
- It is no longer possible to access an R integer vector as a `&[u32]` [[#767]](https://github.com/extendr/extendr/pull/767)

### Fixed

- returning `&Self` or `&mut Self` from a method in an `#[extendr]`-impl would
result in unintended cloning  [[#614]](https://github.com/extendr/extendr/issues/614)
- `TryFrom<&Robj>` and `FromRobj` for integer scalars now correctly handles conversions
  from `f64` [[#757]](https://github.com/extendr/extendr/pull/757)

## 0.6.0

### Added

- `ALTLIST` support allowing users to represent structs as R list objects
- [**either**] `TryFrom<&Robj> for Either<T, R>` and `From<Either<T, R>> for Robj` if `T` and `R` are themselves implement these traits. This unblocks scenarios like accepting any numeric vector from R via `Either<Integers, Doubles>` without extra memory allocation [[#480]](https://github.com/extendr/extendr/pull/480)
- `PartialOrd` trait implementation for `Rfloat`, `Rint` and `Rbool`. `Rfloat` and `Rint` gained `min()` and `max()` methods [[#573]](https://github.com/extendr/extendr/pull/573)
- `use_rng` option for the `extendr` attribute macro, which enables the use of
random number sampling methods from R, e.g. `#[extendr(use_rng = true)` [[#476]](https://github.com/extendr/extendr/pull/476)
- `[T; N]` conversions to `Robj` [[#594]](https://github.com/extendr/extendr/pull/594/)
- `ToVectorValue` for `Rfloat`, `Rint` and `Rbool` [[#593]](https://github.com/extendr/extendr/pull/593)
- `TryFrom<_>` on `Vec<_>` for `Integers` (`i32`), `Complexes` (`c64`), `Doubles` (`f64`), and `Logicals` (`bool` / `i32`). [[#593]](https://github.com/extendr/extendr/pull/593)
- `Rstr` can now be constructed from `Option<String>`  [[#630]](https://github.com/extendr/extendr/pull/630)

### Fixed

- You can now create `ArrayView1` from `&Robj` as well as `Robj`
    [[#501]](https://github.com/extendr/extendr/pull/501)
- Raw literals from Rust can be used for function and argument names. e.g.
    `fn r#type()` in Rust is converted to `type()` in R.
    [[#531]](https://github.com/extendr/extendr/pull/531)
- Fix memory leaks on errors and panics
    [[#555]](https://github.com/extendr/extendr/pull/555)
- Fixed error when collecting too many objects into `List`, etc.
    [[#540]](https://github.com/extendr/extendr/pull/540)

## 0.4.0

### Added

- Support for setting the default value of arguments to struct methods, using `#[default = "..."]` [[#436]](https://github.com/extendr/extendr/pull/436)
- [**ndarray**] `TryFrom<&Robj>` for `ArrayView1<T>` and `ArrayView2<T>`, where `T` is `i32`, `f64`, `c64`, `Rint`, `Rfloat`, `Rcplx`, `Rstr`, `Rbool` [[#443]](https://github.com/extendr/extendr/pull/443)
- `Debug` trait implementation for `Rcplx` and `Complexes` [[#444]](https://github.com/extendr/extendr/pull/444)
- `TryFrom<Robj>`, `From<Option<T>>`, `Into<Option<T>>` and their variations for `Nullable<T>` [[#446]](https://github.com/extendr/extendr/pull/446)
- `Nullable<T>::map()` that acts on not null value and propagates `NULL` [[#446]](https://github.com/extendr/extendr/pull/446)
- [**ndarray**] Conversion from owned arrays (ie `ndarray::Array`) into `Robj` [[#450]](https://github.com/extendr/extendr/pull/450)
- [**ndarray**][**docs**] Documentation for the `robj_ndarray` module [[#450]](https://github.com/extendr/extendr/pull/450)
- `Sum` for scalars like `Rint`, `Rfloat` and `Rcplx`, which accept `Iterator<Item = &Rtype>` [[#454]](https://github.com/extendr/extendr/pull/454)
- A new `collect_rarray` method that can be used to collect arbitrary iterables into an R matrix
 [[#466]](https://github.com/extendr/extendr/pull/466)
- [**docs**] Documentation for `RArray::new_matrix()` [[#466]](https://github.com/extendr/extendr/pull/466)

### Changed

- [**docs**] Use bindgen on docs.rs, to ensure newer R features will still be documented [[#426]](https://github.com/extendr/extendr/pull/426)
- Unify the tagging mechanism used to identify Rust types inside `ExternalPtr`. This allows `#[extendr]`-annotated functions to directly accept `ExternalPtr<MyStruct>` as well as `MyStruct` [[#433]](https://github.com/extendr/extendr/pull/433)
- `Nullable<T>` is now part of `extendr_api::prelude` [[#446]](https://github.com/extendr/extendr/pull/446)
- Bump the Rust edition from 2018 to 2021 [[#458]](https://github.com/extendr/extendr/pull/458)
- When converted to `STRSXP`, strings are now correctly marked as UTF-8 even on non-UTF-8 platforms (i.e., R < 4.2 on Windows), which shouldn't matter for most of the users [[#467]](https://github.com/extendr/extendr/pull/467)

### Fixed

- The R CMD check note "Found non-API calls to R" by moving `use extendr_engine;` inside `test!` macro [[#424]](https://github.com/extendr/extendr/pull/424)
- The clippy lint "this public function might dereference a raw pointer but is not marked `unsafe`" [[#451]](https://github.com/extendr/extendr/pull/451)
- A bug where importing a submodule via `use some_module;` inside the `extendr_module!` macro wasn't working [[#469]](https://github.com/extendr/extendr/pull/469)

## 0.3.0

### Added

- `Function` type that wraps an R function, which can be invoked using the `call()` method. [[#188]](https://github.com/extendr/extendr/pull/188)
- `pairlist!` macro for generating `Pairlist` objects, e.g. for use in function calls. [[#202]](https://github.com/extendr/extendr/pull/202)
- `use_try_from` option for the `extendr` macro, which allows the use of any type that implements `TryInto<Robj>`/`TryFrom<Robj>`, e.g. `#[extendr(use_try_from = true)]`. [[#222]](https://github.com/extendr/extendr/pull/222)
- Support for R version 4.2. [[#235]](https://github.com/extendr/extendr/issues/235)
- `call!` macro, which can be used to call an R function whose name is provided as a string. [[#238]](https://github.com/extendr/extendr/pull/238)
- Large Rust integer types (`u32`, `u64` and `i64`) can now be converted to R's `numeric` type, which can handle large integer values. [[#242]](https://github.com/extendr/extendr/pull/242)
- `TryFrom<Robj>` for a large number of Rust types. [[#249]](https://github.com/extendr/extendr/pull/249), [[#258]](https://github.com/extendr/extendr/pull/258)
- Support for `ALTREP`. [[#250]](https://github.com/extendr/extendr/pull/250), [[#274]](https://github.com/extendr/extendr/pull/274)
- `S4` struct, which wraps an S4 class in R. [[#268]](https://github.com/extendr/extendr/pull/268)
- [**ndarray**] Implemented `TryFrom<&ArrayBase> for Robj`, allowing `extendr`-annotated functions to return Arrays from the `ndarray` crate and have them automatically converted to R arrays. [[#275]](https://github.com/extendr/extendr/pull/275)
- `Rint`, `Rdouble`, `Rbool` and `Rcplx`: `NA`-aware wrappers for scalar elements of R vectors [[#274]](https://github.com/extendr/extendr/pull/274), [[#284]](https://github.com/extendr/extendr/pull/284), [[#301]](https://github.com/extendr/extendr/pull/301), [[#338]](https://github.com/extendr/extendr/pull/338), [[#350]](https://github.com/extendr/extendr/pull/350)
- `Integers`, `Doubles`, `Strings`, `Logicals` and `Complexes`: wrappers for R vectors that deref to slices of the above types (`RInt` etc). [[#274]](https://github.com/extendr/extendr/pull/274), [[#284]](https://github.com/extendr/extendr/pull/284), [[#301]](https://github.com/extendr/extendr/pull/301), [[#338]](https://github.com/extendr/extendr/pull/338), [[#350]](https://github.com/extendr/extendr/pull/350)
- `ExternalPtr`, a wrapper class for creating R objects containing any Rust object. [[#260]](https://github.com/extendr/extendr/pull/260)
- [**graphics**] Support for R graphics and graphics devices. The `graphics` feature flag is disabled by default. [[#279]](https://github.com/extendr/extendr/pull/279), [[#360]](https://github.com/extendr/extendr/pull/360), [[#373]](https://github.com/extendr/extendr/pull/373), [[#379]](https://github.com/extendr/extendr/pull/379), [[#380]](https://github.com/extendr/extendr/pull/380), [[#389]](https://github.com/extendr/extendr/pull/389)
- `Deref` implementation for vector types (Rint/Rfloat/Rbool/Rstr/Robj) to appropriately typed Rust slices. [[#327]](https://github.com/extendr/extendr/pull/327)
- `default` option for `extendr`-annotated functions, allowing them to have default values, e.g. `fn fred(#[default="NULL"] x: Option<i32>) { }`. [[#334]](https://github.com/extendr/extendr/pull/334)
- `r_name` option for `extendr`-annotated functions, allowing the generated R function to have a different name. e.g.

    ```rust
    #[extendr(
        use_try_from = true,
        r_name = "test.rename.rlike",
        mod_name = "test_rename_mymod"
    )]
    fn test_rename() { }
    ```

    [[#335]](https://github.com/extendr/extendr/pull/335)
- `serde::Serialize` implementation for R types. [[#305]](https://github.com/extendr/extendr/pull/305), [[#355]](https://github.com/extendr/extendr/pull/355)
- `Rany` type and the `as_any` conversion method. [[#320]](https://github.com/extendr/extendr/pull/320)
- `std::fmt::Debug` implementation for wrapper types. [[#345]](https://github.com/extendr/extendr/pull/345)
- `#[derive(TryFromRobj)` and `#[derive(IntoRobj)]` which provide an automatic conversion from and to any custom Rust struct and `Robj` [[#347]](https://github.com/extendr/extendr/pull/347)
- `[[` operator that works with Rust classes. Its behavior is identical to that of the `$` operator. [[#359]](https://github.com/extendr/extendr/pull/359)
- `Load` and `Save`, traits that, once implemented, provide the ability to load and save R data in the RDS format. These traits are implemented for all `Robj`. [[#363]](https://github.com/extendr/extendr/pull/363)
- `Dataframe` wrapper struct. [[#393]](https://github.com/extendr/extendr/pull/393)
- `IntoDataFrame` trait, which can be derived to allow arbitrary Rust structs to be converted to rows of a data frame. [[#393]](https://github.com/extendr/extendr/pull/393)

### Changed

- `Strings::elt` now returns an `Rstr`. [[#345]](https://github.com/extendr/extendr/pull/345)
- Renamed `RType` to `Rtype`. [[#345]](https://github.com/extendr/extendr/pull/345)
- Wrapper types now contain `Robj` fields. [[#190]](https://github.com/extendr/extendr/pull/190)
- The `R!` macro now accepts strings that contain R code. This is now the recommended way of using the macro, especially with raw strings e.g.

  ```rust
  R!(r#"
      print("hello")
  "#);
  ```

  [[#203]](https://github.com/extendr/extendr/pull/203)
- Improved error handling for `<&str>::try_from(Robj)`. [[#226]](https://github.com/extendr/extendr/pull/226)
- `SymPair::sym_pair()` now returns `(Option<Robj>, Robj)`. [[#225]](https://github.com/extendr/extendr/pull/225)
- More detailed error messages when converting Rust integer types to R. [[#243]](https://github.com/extendr/extendr/pull/243)
- `Character` is now called `Rstr`. [[#273]](https://github.com/extendr/extendr/pull/273)
- [**ndarray**] Bumped `ndarray` to 0.15.3. Under [RFC 1977](https://github.com/rust-lang/rfcs/pull/1977) this is a "public dependency" change, and therefore can be considered a breaking change, as consumers of `extendr` that use an older version of `ndarray` will no longer be compatible until they also bump `ndarray` to a compatible version. [[#275]](https://github.com/extendr/extendr/pull/275)
- `IsNA` trait has been renamed to `CanBeNA`. [[#288]](https://github.com/extendr/extendr/pull/288)
- `list!` has been rewritten, and now returns a `List` struct. [[#303]](https://github.com/extendr/extendr/pull/303)

### Deprecated

- Calling the `R!` macro with non-string types (e.g. `R!(1)`) is now deprecated. [[#203]](https://github.com/extendr/extendr/pull/203)

### Removed

- `Real`, `Int`, `Bool` and the redundant trait `SliceIter`, which should be replaced with `Rdouble`, `Rint`, and `Rbool` respectively. [[#304]](https://github.com/extendr/extendr/pull/304), [[#338]](https://github.com/extendr/extendr/pull/338)
- `TryFrom` conversions between `Robj` and `HashMap` for consistency. `List::into_hashmap()` and `List::from_hashmap()` should be used instead. [[#254]](https://github.com/extendr/extendr/pull/254)

## extendr 0.2.0

- Added contributing guidelines and code of conduct.

- Made use of ndarray optional.

- Made #[extendr] calls panic and thread safe.

- Added NA handling to the #[extendr] macro.

- Added a separate extendr-engine crate that is needed when calling R from Rust.

- Wrapper classes for pairlists, environment, raw, symbols and others.

- More iterator support.

- Operators index, slice, dollar, double_colon, +, -, * and /`.

- Debug printing support expanded to use wrappers.

- Conversion of Robj to wrapper types.

- Multithreaded support - allows multithreaded testing using a recursive spinlock.

- Bool type extended and implemented using TRUE, FALSE and NA_BOOLEAN.

- Optional parameters to support NA handing.

- Errors thrown if input parameters without Option are NA.

- Harmonising of function names into integer, real, logical, symbol, raw, list, pairlist and env.

- Refactored robj code into several source files.

- Many functions updated to use generic types.

- R! macro for executing R source code.

- call! macro to call R code.

- sym! macro to generate symbols.

- Simplification of vector generation using collect_robj and ToVectorValue.

- Added array types `[1, 2, 3]` as `Robj::from` targets.

- Macros now mostly return errors.

## extendr 0.1.10

- Fix build on Windows and MacOS.
