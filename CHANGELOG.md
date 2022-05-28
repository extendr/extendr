# Changelog

## 0.3.0

### Added

- `Function` type that wraps an R function, which can be invoked using the `call()` method [[#188]](https://github.com/extendr/extendr/pull/188)
- `pairlist!` macro for generating `Pairlist` objects, e.g. for use in function calls [[#202]](https://github.com/extendr/extendr/pull/202)
- `use_try_from` option for the `extendr` macro, which allows the use of any type that implements `TryInto<Robj>`/`TryFrom<Robj>`, e.g. `#[extendr(use_try_from = true)]` [[#222]](https://github.com/extendr/extendr/pull/222)
- Large Rust integer types (`u32`, `u64` and `i64`) can now be converted to R's `numeric` type, which can handle large integer values [[#242]](https://github.com/extendr/extendr/pull/242)
- `call!` macro, which can be used to call an R function whose name is provided as a string [[#238]](https://github.com/extendr/extendr/pull/238)
- Implemented `TryFrom<Robj>` for a large number of Rust types [[#249]](https://github.com/extendr/extendr/pull/249), [[#258]](https://github.com/extendr/extendr/pull/258)
- Support for `ALTREP` [[#250]](https://github.com/extendr/extendr/pull/250)
- An `S4` struct, which wraps an S4 class in R [[#268]](https://github.com/extendr/extendr/pull/268)
- Implemented `TryFrom<&ArrayBase> for Robj`, allowing `extendr`-annotated functions to return Arrays from the `ndarray` crate and have them automatically converted to R arrays. Note: even if 1D arrays are returned they will not be returned as vectors. [[#275]](https://github.com/extendr/extendr/pull/275)
- `Rint`, `Rdouble`, `Rbool` and `Rcplx`: `ALTREP` wrapper classes for R vectors, which can be used without copying [[#274]](https://github.com/extendr/extendr/pull/274), [[#284]](https://github.com/extendr/extendr/pull/284), [[#301]](https://github.com/extendr/extendr/pull/301), [[#338]](https://github.com/extendr/extendr/pull/338), [[#350]](https://github.com/extendr/extendr/pull/350)
- `Integers`, `Doubles`, `Strings`, `Logicals` and `Complexes`: `ALTREP` wrappers for vectors that allows for much larger vectors than native R [[#274]](https://github.com/extendr/extendr/pull/274), [[#284]](https://github.com/extendr/extendr/pull/284), [[#301]](https://github.com/extendr/extendr/pull/301), [[#338]](https://github.com/extendr/extendr/pull/338), [[#350]](https://github.com/extendr/extendr/pull/350)
- `ExternalPtr`, a wrapper class for creating R objects containing any Rust object. [[#260]](https://github.com/extendr/extendr/pull/260)
- Support for R graphics and graphics devices. These are locked behind the `graphics` feature flag, which is disabled by default. [[#279]](https://github.com/extendr/extendr/pull/279), [[#360]](https://github.com/extendr/extendr/pull/360), [[#373]](https://github.com/extendr/extendr/pull/373), [[#379]](https://github.com/extendr/extendr/pull/379), [[#380]](https://github.com/extendr/extendr/pull/380), [[#389]](https://github.com/extendr/extendr/pull/389)
- Implemented `Deref` for vector types (Rint/Rfloat/Rbool/Rstr/Robj) to appropriately typed Rust slices [[#327]](https://github.com/extendr/extendr/pull/327)
- Added a `default` option for `extendr`-annotated functions, allowing them to have default values, e.g. `fn fred(#[default="NULL"] x: Option><i32>) { }` [[#334]](https://github.com/extendr/extendr/pull/334)
- The `r_name` option for `extendr`-annotated functions, allowing the generated R function to have a different name. e.g.
    ```rust
    #[extendr(
        use_try_from = true,
        r_name = "test.rename.rlike",
        mod_name = "test_rename_mymod"
    )]
    fn test_rename() { }
    ```
    [[#335]](https://github.com/extendr/extendr/pull/335)
- Implemented `serde::Serialize` for R types [[#305]](https://github.com/extendr/extendr/pull/305), [[#355]](https://github.com/extendr/extendr/pull/355)
- `Rany` type and the `as_any` conversion method. [[#320]](https://github.com/extendr/extendr/pull/320)
- Implemented `std::fmt::Debug` for wrapper types [[#345]](https://github.com/extendr/extendr/pull/345)
- Implemented the `[[` operator to function on Rust classes, where it functions the same the `$` operator. [[#359]](https://github.com/extendr/extendr/pull/359)
- `Load` and `Save`, traits that implement the ability to load and save R data to the RDS format. These traits are implemented for all `Robj` [[#363]](https://github.com/extendr/extendr/pull/363)
- Support for R version 4.2 [[#399]](https://github.com/extendr/extendr/pull/399)
- The `Dataframe` wrapper struct [[#393]](https://github.com/extendr/extendr/pull/393)
- The `IntoDataFrame` trait, which can be derived to allow arbitrary Rust structs to be converted to rows of a data frame [[#393]](https://github.com/extendr/extendr/pull/393)

### Changed

- `Strings::elt` now returns an `Rstr` [[#345]](https://github.com/extendr/extendr/pull/345)
- Renamed `RType` to `Rtype`  [[#345]](https://github.com/extendr/extendr/pull/345)
- Wrapper types now contain `Robj` fields [[#190]](https://github.com/extendr/extendr/pull/190)
- The `R!` macro now accepts strings that contain R code. This is now the recommended way of using the macro. [[#203]](https://github.com/extendr/extendr/pull/203)
- Better error handling for `<&str>::try_from(Robj)` [[#226]](https://github.com/extendr/extendr/pull/226)
- `SymPair::sym_pair()` now returns `(Option<Robj>, Robj)` [[#225]](https://github.com/extendr/extendr/pull/225)
- Better error messages when converting Rust integer types to R [[#243]](https://github.com/extendr/extendr/pull/243)
- `Character` is now called `Rstr` [[#273]](https://github.com/extendr/extendr/pull/273)
- Bump `ndarray` to 0.15.3. Under [RFC 1977](https://github.com/rust-lang/rfcs/pull/1977) this is a "public dependency" change, and therefore can be considered a breaking change, as consumers of `extendr` that use an older version of `ndarray` will no longer be compatible until they also bump `ndarray` to a compatible version. [[#275]](https://github.com/extendr/extendr/pull/275)
- `IsNA` trait has been renamed to `CanBeNA` [[#288]](https://github.com/extendr/extendr/pull/288)
- `list!` has been rewritten, and how returns a `List` struct [[#303]](https://github.com/extendr/extendr/pull/303)

### Deprecated
- Calling the `R!` macro with non-string types (e.g. `R!(1)`) is now deprecated. [[#203]](https://github.com/extendr/extendr/pull/203)

### Removed
- `Real`, `Int`, `Bool` and the redundant trait `SliceIter`, which should be replaced with `Doubles`, `Integers`, and `Logicals` respectively [[#304]](https://github.com/extendr/extendr/pull/304), [[#338]](https://github.com/extendr/extendr/pull/338)
- `TryFrom` conversions between `Robj` and `HashMap` for consistency. `List::into_hashmap()` and `List::from_hashmap()` should be used instead.

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
