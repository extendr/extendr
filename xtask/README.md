# `extendr` developer tools

In order to perform integration test (with R), generate documentation, etc.,
one can use this tool. In the future, `xtask` will be renamed `ci`, when it can
perform all the tasks, that our custom CI scripts can do at the moment.

```shell
cargo extendr CMD
```

Running `cargo extendr --help` yields:

```shell
Facilitates extendr-developer tasks through `cargo`

Usage: xtask <COMMAND>

Commands:
  check-fmt      Run cargo fmt on extendr
  fmt            Run `cargo fmt` on extendr crates
  r-cmd-check    Run R CMD check on {extendrtests}
  doc            Generate documentation for all features
  msrv           Check that the specified rust-version is MSRV
  devtools-test  Run devtools::test() on {extendrtests}
  document       Generate wrappers with `rextendr::document()`
  help           Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

Let's describe some of the features listed above.

## `check-fmt`

This checks if the rust code follows the specified `rustfmt`. It does not
format the code. In order to do so, please run `cargo fmt`.

## `fmt`

This command calls `cargo fmt` within the workspace, ensuring that the contents of `tests/extendrtests/src/rust` folder are formatted as well.

## `r-cmd-check`

Runs `R CMD check` tests in `tests/extendrtests`.

## `doc`

Generates documentation as seen on [/extendr.github.io](https://extendr.github.io/extendr/extendr_api/), meaning it will enable feature `full-functionality`,
which includes all the optional dependencies, plus all the additive features.

## `msrv`

Performs Minimum Supported Rust Version (MSRV) check in the repo.

## `devtools-test`

This performs `devtools::test()` in R, within the R-package `tests/extendrtests`. If this call results in messages about updating
macro snapshots, one may run `cargo extendr devtools-test -a` to accept the newly generated snapshots.

## `document`

Use `cargo extendr document` to regenerate the wrappers for the integration-test package `tests/extendrtests`. This invokes `rextendr::document()`.

## TODO

In the following are features that could be added to `xtask`.

### Embed local `libR-sys`

Clone `libR-sys` into `extendr/libR-sys`. Change the `extendr/Cargo.toml` to
use the embedded `extendr/libR-sys`.

### Copy R-headers

Copy R's C-headers to the current `extendr`-directory.

This helps with researching specific things, and mainly interesting for developers.

### Windows-specific: Add `libgcc_eh.a` and `libgcc_s.a`

On Windows, and due to specific Rust internal settings, we need to generate
empty `libgcc_eh.a`, and `libgcc_s.a` files in order to satisfy the linker.

Hang up is that once a linker is set, and the linker on Windows needing
the presence of certain files to work, then the `xtask` doesn't compile,
thus unable to provide those necessary files.

## Credits

Following [`xtask`](https://github.com/matklad/cargo-xtask) template.
