# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Branch convention — work on `main-claude`, not `main`

This local clone uses a long-lived branch called **`main-claude`** as Claude's working trunk. Treat it the way you'd normally treat `origin/main`:

- **All Claude-authored commits land on `main-claude`** (or on feature branches forked from `main-claude`). Never commit directly to `main`.
- `main` stays in lockstep with `origin/main` (the real `extendr/extendr` upstream). It's only updated by `git pull` (or `git fetch && git merge --ff-only`) — Claude does not touch it.
- **Feature branches that will become PRs MUST be forked from `main` (not `main-claude`)**, or rebased onto `origin/main` before they are pushed. `main-claude` carries this `CLAUDE.md`, scratch notes, and tooling files that must NOT appear in proposed upstream PRs.
- Before opening or updating a PR, verify with `git log --oneline origin/main..HEAD` — every commit listed must be one you intended to propose. If `CLAUDE.md` or other helper commits show up, rebase onto `origin/main` with `git rebase --onto origin/main main-claude <branch>`.
- To resync: `git switch main && git pull --ff-only && git switch main-claude && git merge main` (fast-forward where possible; otherwise rebase deliberately). Resolve any conflicts on `main-claude`.
- If `git status` reports the current branch is `main`, switch back before doing any work: `git switch main-claude`.
- The sibling `.rextendr/` clone uses the same `main-claude` / `main` separation. The same no-PR-pollution rule applies there.

This separation exists so Claude's experimental commits, scratch notes, and tooling files (`issues.md`, `OPEN-PR.md`, `scripts/`, regenerated wrappers, etc.) can accumulate without contaminating the upstream-tracking branch — and so they never accidentally land in a PR.

## What's in this directory

Two related but independent repositories live side-by-side here:

1. **`/` (this directory, branch `main`)** — the `extendr` **Rust workspace**: four crates (`extendr-ffi`, `extendr-macros`, `extendr-api`, `extendr-engine`) plus the `tests/extendrtests/` integration R package.
2. **`.rextendr/`** — a **separate git checkout** of the `extendr/rextendr` repository (the R package `{rextendr}` that scaffolds, compiles, and loads extendr-powered R code). It is *not* a submodule and *not* part of the Cargo workspace; it has its own `.git`, `DESCRIPTION`, `justfile`, CI, and release cycle.

The two are coupled at runtime: `{rextendr}` builds R packages that depend on `extendr-api`, and `tests/extendrtests/` uses `{rextendr}` to regenerate R wrappers. The `justfile` recipe `just document` (in this workspace) deliberately loads `.rextendr/` if present via `devtools::load_all("../../rextendr")` so an in-progress rextendr can be exercised against the local extendr.

When the user asks about "rextendr" the function/feature, the answer lives in `.rextendr/R/`; when they ask about extendr-api / Robj / `#[extendr]`, the answer lives in the Rust crates.

---

## Part 1 — Rust workspace (`/`)

### Workspace layout

- `extendr-ffi` — Hand-curated low-level FFI to `libR.so/dll`. Has a `build.rs` that locates `R_HOME`/`R_INCLUDE_DIR`, parses `Rversion.h`, sets `cargo:rustc-cfg` flags (`r_4_4`, `r_4_5`), and emits `links = "R"`. Downstream crates read R version via `DEP_R_*` env vars. Hosts backports for symbols moving to non-API status. (Historically also emitted `use_r_ge_version_15/16/17` for the graphics engine; the graphics module is being removed in PR #1079, which also drops those cfgs and the `extendr-ffi/src/graphics.rs` FFI surface.)
- `extendr-macros` — Proc-macro crate (`#[extendr]`, `extendr_module!`, `R!`, `Rraw!`, `IntoDataFrameRow`, etc.). Modules split by item kind: `extendr_function.rs`, `extendr_impl.rs`, `extendr_module.rs`, `extendr_conversion.rs`, plus wrapper-generation helpers in `wrappers.rs`. Macro test fixtures live in `extendr-macros/tests/cases/` and `tests/extendr_impl/` and run through `trybuild`.
- `extendr-api` — The user-facing safe API. Re-exports macros via `prelude`. Key submodules: `robj/` (the central `Robj` wrapper), `wrapper/` (typed wrappers like `Integers`, `Doubles`, `List`, `Dataframe`, `Function`, `Environment`, `ExternalPtr`, `Altrep`, `RMatrix`), `conversions/` (`IntoRobj`/`TryFrom<Robj>`), `scalar/` (NA-aware `Rint`/`Rfloat`/`Rbool`/`Rcplx`), `io/`, `optional/` (feature-gated `ndarray`/`faer`/`either`/`serde` interop), `conditions/`, plus `iter.rs`, `thread_safety.rs`, `ownership.rs`, `serializer.rs`/`deserializer.rs`, `metadata.rs`. (`graphics/` and the `graphics` Cargo feature are being removed in PR #1079.)
- `extendr-engine` — Embeds an R interpreter (`start_r`, `with_r`) for use only in tests/binaries. Declares non-API usage; must stay in `[dev-dependencies]` for downstream R packages.

`tests/extendrtests/` is a real R package built against the local workspace via `[patch.crates-io]` and exercised through `R CMD check`. It is *the* integration test for the whole stack — wrappers are generated by `rextendr::document()` and exercised from `testthat` files under `tests/extendrtests/tests/testthat/`. Snapshot tests live there too (accept with `just devtools-test SNAPSHOT=1`).

### Common commands (use `just`, not raw `cargo`)

The `justfile` exists because most commands must run twice — once over the workspace and once against `tests/extendrtests/src/rust/Cargo.toml` — and because `just test` parses an extra `--` separator for test-binary args.

- `just fmt` / `just fmt-check` — rustfmt across workspace AND `tests/extendrtests/src/rust`.
- `just check` / `just build` / `just clippy` — same dual sweep.
- `just test [cargo-args] -- [test-args]` — `cargo test --workspace --no-fail-fast --features=full-functionality` with `--test-threads=1` (R is single-threaded and `extendr-engine` mutates a global `Once`). Pass test-name filter after `--`.
- `cargo test -p extendr-macros --test trybuild` — proc-macro trybuild suite. `TRYBUILD=overwrite` regenerates expected `.stderr`.
- `just doc` / `just doc-check` — requires `cargo +nightly` (uses `--document-private-items`).
- `just msrv [FEATURES=…]` — `cargo msrv verify` against the pinned MSRV (`rust-version = "1.77"` in root `Cargo.toml`).
- `just document` — regenerates R wrappers in `tests/extendrtests/` via `rextendr::document()`; prefers vendored `../../rextendr` (i.e. the sibling `.rextendr/` here) over the installed package.
- `just devtools-test [FILTER=…] [SNAPSHOT=1]` — `devtools::test()` for `extendrtests`.
- `just r-cmd-check [ERROR_ON=warning|error] [CHECK_DIR=…] [NO_VIGNETTES=1]` — full `R CMD check` of `extendrtests`. The recipe rewrites the `[patch.crates-io]` path in `tests/extendrtests/src/rust/Cargo.toml` to an absolute path for the duration of the check and restores it on exit (via `trap`).

Direct `cargo` is fine for one-crate-only operations (`cargo test -p extendr-api scalar_tests`); anything touching `extendrtests` must go through `just` so the second manifest is included.

### Feature flags that matter

`extendr-api` has aggregate test features the CI matrix uses: `tests-minimal`, `tests`, `tests-all`, plus the umbrella `full-functionality` (enables `either`, `faer`, `ndarray`, `num-complex`, `serde`). `result_list` and `result_condition` change how `Result<T,E>` is encoded back to R — only one takes effect at a time (precedence: `result_list`, then `result_condition`). The `non-api` feature pulls in R internals CRAN packages must avoid; enable only deliberately. (Before PR #1079, there was also a `graphics` feature and a `tests-graphics` aggregate, plus a `libc` optional dep gated to it — all removed when #1079 lands.)

### R version cfg

`extendr-ffi/build.rs` is the single source of truth for which R version is targeted. When adding code that depends on an R API change, gate it with the existing `r_4_4` / `r_4_5` cfgs, and if a symbol moved to non-API, follow the backport pattern in `extendr-ffi/README.md` (declare both old and new `extern "C"` with cfg gates, expose an `unsafe` inline wrapper).

### Conventions

- PR titles follow Conventional Commits (`feat:`, `fix:`, `chore:`, `feat!:` for breaking). See `git log` and `CHANGELOG.md` for cadence.
- The PR template requires `just fmt`, `just test`, integration tests for new features, and a `CHANGELOG.md` entry.
- New R-visible API surface typically needs: (1) Rust impl in `extendr-api`, (2) macro/wrapper support if needed in `extendr-macros`, (3) exercising R-side test in `tests/extendrtests/`, with wrappers regenerated via `just document`.

---

## Part 2 — `.rextendr/` (the `{rextendr}` R package)

A separate-repo, full R package (`rextendr` 0.5.0, on CRAN). Provides the developer-side tooling: scaffolding (`use_extendr()`), interactive Rust eval (`rust_function()`, `rust_source()`, `rust_eval()`), the `extendr`/`extendrsrc` knitr engines (registered in `R/zzz.R`), wrapper regeneration (`document()`, now mostly delegated to `devtools::document()` because newer scaffolds emit a `document` Rust binary that writes `R/extendr-wrappers.R` during `cargo build`).

### Layout

- `R/` — All package source. Notable files: `source.R` (the big one — `rust_source`/`rust_function`), `eval.R` (`rust_eval`), `use_extendr.R` (scaffolds a Rust-powered R package), `knitr_engine.R` (the `extendr` chunk engine), `rust_sitrep.R` (toolchain diagnostic), `register_extendr.R`, `make_module_macro.R`, `rextendr_document.R` (deprecated wrapper around `devtools::document`), `toml_serialization.R`, `cran-compliance.R`, `use_crate.R`, `use_msrv.R`, `use_vscode.R`. Imports vendored: `standalone-purrr.R`, `import-standalone-obj-type.R`, `import-standalone-types-check.R` — *don't edit* (kept synced upstream; `.lintr` and `jarl.toml` exclude them).
- `inst/templates/` — Files written by `use_extendr()` into a new package: `Cargo.toml`, `lib.rs`, `entrypoint.c`, `document.rs`, `Makevars.in` / `Makevars.win.in`, `configure` / `configure.win`, `cleanup` scripts, `tools/msrv.R`, `tools/config.R`. Editing these changes what every newly scaffolded extendr package looks like.
- `inst/rstudio/templates/` — RStudio "New Project" template hook.
- `tests/testthat/` — testthat 3rd edition, `parallel: true`. Helpers in `helper.R`: `expect_rextendr_error()`, `local_package()`, `local_temp_dir()`, `skip_if_cargo_unavailable()`, `mask_any_version()`. `setup.R` sets `rextendr.extendr_deps` to the github extendr-api for CI. `REXTENDR_SKIP_DEV_TESTS=TRUE` skips the slowest dev tests.
- `vignettes/` — `package.Rmd`, `rmarkdown.Rmd`, `setting_up_rust.Rmd`.
- `principles.md` — Internal coding conventions (read this before adding user-facing messages or errors).

### Common commands

Use the local `justfile` (commands run from inside `.rextendr/`):

- `just test` — `R --quiet -e "devtools::test()"`.
- `just update-snaps` — `testthat::snapshot_accept()`.
- `just check` — `devtools::check()`.
- `just doc` — `devtools::document()` to regenerate `man/` and `NAMESPACE` from roxygen.
- `just lint` / `just lint-fix` — uses `jarl` (linter); honors `.lintr` (line length 120, ignores `import-standalone-obj-type.R`) and `jarl.toml` (excludes `standalone-purrr.R`).
- `just fmt` — `air format R/`.

The `justfile` deliberately scrubs VS Code/Positron env vars before invoking R. **Always run tests from the command line, not the IDE**: per `CONTRIBUTING.md`, IDE runs leak `.vscode/` etc. into temp package fixtures used by `local_package()` and break snapshot tests/CI.

CI: `R-CMD-check.yaml` runs against R release / devel / oldrel on ubuntu, mac (release only), and windows with rust toolchains via `dtolnay/rust-toolchain`. `error-on: '"note"'` — notes fail the build. `REXTENDR_SKIP_DEV_TESTS=TRUE` is set globally in CI.

### Conventions (rextendr-specific, from `principles.md` and `CONTRIBUTING.md`)

- **User messages** go through `cli::cli_alert_success / _info / _warning / _danger`, `cli::cli_ul`. Honor `quiet` via the `local_quiet_cli(quiet)` helper.
- **Errors** use `cli::cli_abort(..., class = "rextendr_error")` with `cli` named-vector bullets (`"x" = ...`, `"*" = ...`). Tests assert via `expect_rextendr_error()`.
- **Style** — tidyverse style guide; `air format R/` formats; roxygen2 with markdown; testthat 3rd edition.
- **CRAN posture** — this package targets CRAN release; `cran-comments.md` and `cran-compliance.R` are kept up to date. Don't introduce CRAN-disallowed patterns.

### Coupling to the Rust workspace next door

- `.onLoad` (in `R/zzz.R`) queries crates.io for the latest stable `extendr-api` version, falling back to `"*"`; sets `options(rextendr.extendr_deps = list(\`extendr-api\` = <version>))` and `rextendr.extendr_dev_deps = list(\`extendr-api\` = list(git = "https://github.com/extendr/extendr"))`.
- Tests override this via `setup.R` to use the github version.
- Downstream in *this* workspace, `tests/extendrtests/README.md` shows how to point `rextendr.patch.crates_io` at local paths so `rust_function()` builds against the *local* `extendr-api` — this is the mechanism for testing changes here against in-progress rextendr features there.
- `just document` (Rust workspace) calls `devtools::load_all("../../rextendr")` so that running it from this directory uses the sibling `.rextendr/` you're editing.
