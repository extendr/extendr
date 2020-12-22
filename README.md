# extendr - A safe and user friendly R extension interface using Rust.

Low-level R library bindings

[![Github Actions Build Status](https://github.com/extendr/extendr/workflows/Tests/badge.svg)](https://github.com/extendr/extendr/actions)
[![Crates.io](http://meritbadge.herokuapp.com/extendr-api)](https://crates.io/crates/extendr-api)
[![Documentation](https://docs.rs/extendr-api/badge.svg)](https://docs.rs/extendr-api)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![Logo](https://github.com/extendr/extendr/raw/master/extendr-logo-256.png)](https://github.com/extendr/extendr/raw/master/extendr-logo-256.png)

## Installation - Rust

Extendr is available on [crates.io](https://crates.io/crates/extendr-api).

Simply add this line to the `[dependencies]` section of a rust crate.
You will then be able to call R code from Rust.

```toml
[dependencies]
extendr-api = "0.1.10"
```
## Installation - R

There are two ways you can use the extendr API from R. First, you can use the [rextendr](https://extendr.github.io/rextendr/) package to call individual Rust functions from an R session. Second, you can write an R package that uses compiled Rust code, see the [helloextendr](https://github.com/extendr/helloextendr) repo for a minimal example.

## Overview

Extendr is a Rust extension mechanism for R

It is intended to be easier to use than the C interface and
Rcpp as Rust gives type safety and freedom from segfaults.

The following code illustrates a simple structure trait
which is written in Rust. The data is defined in the `struct`
declaration and the methods in the `impl`.

```rust
use extendr_api::*;

struct Person {
    pub name: String,
}

#[extendr]
impl Person {
    fn new() -> Self {
        Self { name: "".to_string() }
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

#[extendr]
fn aux_func() {
}


// Macro to generate exports
extendr_module! {
    mod classes;
    impl Person;
    fn aux_func;
}
```

The `#[extendr]` attribute causes the compiler to generate
wrapper and registration functions for R which are called
when the package is loaded.

The `extendr_module!` macro lists the module name and exported functions
and interfaces.

This library aims to provide an interface that will be familiar to
first-time users of Rust or indeed any compiled language.

Anyone who knows the R library should be able to write R extensions.

## Goals of the project

Instead of wrapping R objects, we convert to Rust native objects
on entry to a function. This makes the wrapped code clean and dependency
free. The ultimate goal is to allow the wrapping of existing 
Rust libraries without markup, but in the meantime, the markup
is as light as possible.

```rust
#[extendr]
pub fn my_sum(v: &[f64]) -> f64 {
    v.iter().sum()
}
```

You can interact in more detail with R objects using the RObj
type which wraps the native R object type. This supports a large
subset of the R internals functions, but wrapped to prevent
accidental segfaults and failures.

## extendr roadmap

### Basic
- [x] Be able to build simple rust extensions for R.
- [x] Wrap the R SEXP object safely (Robj)
- [x] Iterator support for matrices and vectors.
- [x] Class support.

### Documentation
- [x] Begin documentation.
- [ ] Begin book-form documentation.
- [ ] Paper for Bioinformatics.
- [ ] Build and publish CRAN R package.
- [ ] Publish Use R! series book.

### Automation
- [x] Auto-generate binding wrappers.
- [ ] Auto-generate NAMESPACE and lib.R.

### Features
- [ ] Feature-gated support for ndarray.
- [ ] Feature-gated support for rayon.

### R packages
- [ ] Bindings for rust-bio

## Publishing the crate

### Prerequisite

Install [cargo-workspaces](https://github.com/pksunkara/cargo-workspaces)

``` sh
cargo install cargo-workspaces
```

### Bump version without publish

We use `cargo ws version` for this. This command does:

1. Bump all the versions of the crates in this workspace.
2. Commit.
3. Tag with the bumped version.
4. Push to the repo immediately. (Be careful! If you want to review the change manually before pushing, add `--no-git-push` option as well.)

``` sh
cargo ws version --force='*' --no-individual-tags --pre-id alpha prerelease
```

The meanings of the options and arguments are

* `--force='*'`: By default, `cargo ws version` skips the crates unchange since the last version. This option makes them included in the targets. 
* `--no-individual-tags`: By default, `cargo ws version` creates a tag for each crates (e.g. `crateA@v0.0.1`) in addition to the usual version tag (e.g. `v0.0.1`). This option skips the individual tags.
* `--pre-id alpha`: Specify the identifier prepended to the version.
* `prerelease`: Increase the version with prerelease identifier (e.g. `v0.1.10 -> v0.1.10-alpha.0`, `v0.1.10-alpha.0 -> v0.1.10-alpha.1`). We can also specify `patch`, `minor`, or `major` to increment the corresponding part of the version. Alternatively, we can omit this and choose the version interactively.

<details>

<summary>console output</summary>

If you are asked to select a new version interactively, move the cursor with <kbd>↑</kbd><kbd>↓</kbd>, and press <kbd>Enter</kbd> to choose.

``` console
info looking for changes since v0.1.10
info current common version 0.1.11
? Select a new version (currently 0.1.11) ›
❯ Patch (0.1.12)
  Minor (0.2.0)
  Major (1.0.0)
  Prepatch (0.1.12-alpha.0)
  Preminor (0.2.0-alpha.0)
  Premajor (1.0.0-alpha.0)
  Custom Prerelease
  Custom Version
```

Then, you will be asked to confirm the change. Press `y` to proceed.

``` console
Changes:
 - extendr-api: 0.1.12-alpha.0 => 0.1.12-alpha.1
 - extendr-engine: 0.1.12-alpha.0 => 0.1.12-alpha.1
 - extendr-macros: 0.1.12-alpha.0 => 0.1.12-alpha.1

? Are you sure you want to create these versions? (y/N) › no
```

</details>

### Bump version and publish

When we publish, we use `cargo ws publish`. This command does:

1. Run `cargo ws version`.
2. Publish all the crates within the workspace.

``` sh
# publish
cargo ws publish --force='*' --no-individual-tags patch

# change the version for further development
cargo ws version --force='*' --no-individual-tags --pre-id alpha prerelease
```

This command will publish to Crates.io immediately.
If we want to review the change manually before publishing, we can do it step by step.

``` sh
# bump version
cargo ws version --force='*' --no-individual-tags --no-git-push patch

# review the changes
git show HEAD^

# push all the changes
git push --tags

# publish without modifying the current version
cargo ws publish --force='*' --from-git

# change the version for further development
cargo ws version --force='*' --no-individual-tags --pre-id alpha prerelease
```

## Contributing

We are happy about any contributions!

To get started you can take a look at our [Github issues](https://github.com/extendr/extendr/issues).

You can also get in contact via our [Discord server](https://discord.gg/7hmApuc)!

### Development

The documentation for the latest development version is available here: <https://extendr.github.io/extendr/extendr_api/>
