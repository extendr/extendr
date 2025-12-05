# https://just.systems

default:
    echo 'Hello, world!'

alias cargo-clean := clean
clean *cargo_flags:
    cargo clean -p extendr-api {{cargo_flags}}
    cargo clean -p extendr-macros {{cargo_flags}}
    cargo clean -p extendr-ffi {{cargo_flags}}
    cargo clean --manifest-path=tests/extendrtests/src/rust/Cargo.toml {{cargo_flags}}

alias cargo-check := check
check *cargo_flags:
    cargo check --features full-functionality --workspace {{cargo_flags}}
    cargo check --features full-functionality --manifest-path=tests/extendrtests/src/rust/Cargo.toml {{cargo_flags}}

alias cargo-build := build
build *cargo_flags:
    cargo build --features full-functionality --workspace {{cargo_flags}}
    cargo build --features full-functionality --manifest-path=tests/extendrtests/src/rust/Cargo.toml {{cargo_flags}}

alias cargo-clippy := clippy
clippy *cargo_flags:
    cargo clippy --features full-functionality --workspace {{cargo_flags}}
    cargo clippy --features full-functionality --manifest-path=tests/extendrtests/src/rust/Cargo.toml {{cargo_flags}}

alias cargo-doc-check := doc-check
doc-check *cargo_flags:
    cargo +nightly doc --no-deps --document-private-items --features full-functionality --workspace {{cargo_flags}}
    cargo +nightly doc --no-deps --document-private-items --features full-functionality --manifest-path=tests/extendrtests/src/rust/Cargo.toml {{cargo_flags}}

alias cargo-doc := doc
doc *cargo_flags:
    cargo +nightly doc --document-private-items --features full-functionality --workspace {{cargo_flags}}
    cargo +nightly doc --document-private-items --features full-functionality --manifest-path=tests/extendrtests/src/rust/Cargo.toml {{cargo_flags}}

alias cargo-fmt-check := fmt-check
fmt-check *cargo_flags:
    cargo fmt --all {{cargo_flags}} -- --check
    cargo fmt --all --manifest-path=tests/extendrtests/src/rust/Cargo.toml {{cargo_flags}} -- --check

alias cargo-fmt := fmt
fmt *cargo_flags:
    cargo fmt --all {{cargo_flags}}
    cargo fmt --all --manifest-path=tests/extendrtests/src/rust/Cargo.toml {{cargo_flags}}

alias cargo-test := test
test *args:
    cargo_flags="" \
    && test_args="" \
    && sep=0 \
    && for arg in {{args}}; do \
      if [ "$arg" = "--" ]; then sep=1; continue; fi; \
      if [ "$sep" = "0" ]; then cargo_flags="$cargo_flags $arg"; else test_args="$test_args $arg"; fi; \
    done \
    && cargo test --workspace --no-fail-fast --features=full-functionality $cargo_flags -- --no-capture --test-threads=1 $test_args \
    && cargo test --manifest-path=tests/extendrtests/src/rust/Cargo.toml --no-fail-fast --features=full-functionality $cargo_flags -- --no-capture --test-threads=1 $test_args

alias cargo-tree := tree
tree *cargo_flags:
  cargo tree --features=full-functionality --workspace {{cargo_flags}}
  cargo tree --features=full-functionality --manifest-path=tests/extendrtests/src/rust/Cargo.toml {{cargo_flags}}

alias cargo-expand := expand
expand *cargo_flags:
    cargo expand --features=full-functionality -p extendr-api {{cargo_flags}}
    cargo expand -p extendr-macros {{cargo_flags}}
    cargo expand -p extendr-ffi {{cargo_flags}}
    cargo expand --features=full-functionality --manifest-path=tests/extendrtests/src/rust/Cargo.toml {{cargo_flags}}

# Verify MSRV with optional comma-separated FEATURES (empty means default features)
msrv FEATURES="":
    if [ -z "{{FEATURES}}" ]; then \
      cargo msrv --path extendr-api verify -- cargo check; \
    else \
      cargo msrv --path extendr-api verify -- cargo check --features {{FEATURES}}; \
    fi

# Generate documentation (R wrappers) via rextendr::document()
document:
    cd tests/extendrtests && \
    if [ -d ../../rextendr ]; then \
      echo "Loading vendored {rextendr}" && \
      Rscript -e 'requireNamespace("devtools")' \
              -e 'devtools::load_all("../../rextendr")' \
              -e 'rextendr::document()'; \
    else \
      echo "Using installed {rextendr}" && \
      Rscript -e 'requireNamespace("rextendr")' \
              -e 'rextendr::document()'; \
    fi

# Run devtools::test() for extendrtests; set FILTER or SNAPSHOT=1 to accept snapshots
devtools-test FILTER="" SNAPSHOT="0":
    cd tests/extendrtests && \
    if [ "{{SNAPSHOT}}" = "1" ]; then \
      Rscript -e 'testthat::snapshot_accept("macro-snapshot")'; \
    fi; \
    if [ -z "{{FILTER}}" ]; then \
      Rscript -e 'devtools::test()'; \
    else \
      Rscript -e 'devtools::test(filter = "{{FILTER}}")'; \
    fi

alias rcmdcheck := r-cmd-check
# Run R CMD check on extendrtests; accepts NO_VIGNETTES=1, ERROR_ON=warning|error, CHECK_DIR=path
r-cmd-check *args:
    NO_VIGNETTES="0" \
    ERROR_ON="warning" \
    CHECK_DIR="" \
    ROOT_DIR="$(pwd)" \
    PATCH_ROOT="$(pwd)" \
    CARGO_TOML="$(pwd)/tests/extendrtests/src/rust/Cargo.toml" \
    && for arg in {{args}}; do \
      case "$arg" in \
        NO_VIGNETTES=*) NO_VIGNETTES="${arg#NO_VIGNETTES=}" ;; \
        ERROR_ON=*) ERROR_ON="${arg#ERROR_ON=}" ;; \
        CHECK_DIR=*) CHECK_DIR="${arg#CHECK_DIR=}" ;; \
        *) echo "Ignoring unknown arg '$arg'" ;; \
      esac; \
    done \
    && CHECK_DIR_ARG="NULL" \
    && if [ -n "$CHECK_DIR" ]; then \
      case "$CHECK_DIR" in \
        /*) CHECK_DIR_ARG="'$CHECK_DIR'" ;; \
        *)  CHECK_DIR_ARG="'$(pwd)/$CHECK_DIR'" ;; \
      esac; \
    fi \
    && case "$(uname -s)" in \
      MINGW*|MSYS*|CYGWIN*) PATCH_ROOT="$(cygpath -m "$PATCH_ROOT" 2>/dev/null || pwd -W 2>/dev/null || echo "$PATCH_ROOT")" ;; \
    esac \
    && TMP_CARGO_TOML="$(mktemp)" \
    && cp "$CARGO_TOML" "$TMP_CARGO_TOML" \
    && cleanup() { mv "$TMP_CARGO_TOML" "$CARGO_TOML"; } \
    && trap cleanup EXIT \
    && TMP_EDIT="$(mktemp)" \
    && awk -v root="$PATCH_ROOT" ' \
      /^[[:space:]]*#/ {print; next} \
      /^\[patch\.crates-io\]/ {in_patch=1; print; next} \
      in_patch && done==0 && /^[[:space:]]*extendr-api[[:space:]]*=/ { \
        gsub(/path[[:space:]]*=[[:space:]]*"[^"]+"/, "path = \"" root "/extendr-api\""); \
        done=1; \
      } \
      {print} \
      END { if (done==0) exit 1 } \
    ' "$CARGO_TOML" > "$TMP_EDIT" \
    && mv "$TMP_EDIT" "$CARGO_TOML" \
    && cd tests/extendrtests \
    && ARGS="'--as-cran','--no-manual'" \
    && if [ "$NO_VIGNETTES" = "1" ]; then ARGS="${ARGS},'--no-build-vignettes'"; fi \
    && Rscript -e "rcmdcheck::rcmdcheck(args = c(${ARGS}), error_on = '${ERROR_ON}', check_dir = ${CHECK_DIR_ARG})"
