# https://just.systems

default:
    echo 'Hello, world!'

check *cargo_flags:
    cargo check --workspace {{cargo_flags}}
    cargo check --manifest-path=tests/extendrtests/src/rust/Cargo.toml {{cargo_flags}}


build *cargo_flags:
    cargo build --workspace {{cargo_flags}}
    cargo build --manifest-path=tests/extendrtests/src/rust/Cargo.toml {{cargo_flags}}

clippy *cargo_flags:
    cargo clippy --workspace {{cargo_flags}}
    cargo clippy --manifest-path=tests/extendrtests/src/rust/Cargo.toml {{cargo_flags}}

doc-check *cargo_flags:
    cargo +nightly doc --no-deps --document-private-items --features full-functionality --workspace {{cargo_flags}}
    cargo +nightly doc --no-deps --document-private-items --features full-functionality --manifest-path=tests/extendrtests/src/rust/Cargo.toml {{cargo_flags}}

doc *cargo_flags:
    cargo +nightly doc --document-private-items --features full-functionality --workspace {{cargo_flags}}
    cargo +nightly doc --document-private-items --features full-functionality --manifest-path=tests/extendrtests/src/rust/Cargo.toml --open {{cargo_flags}}

fmt-check *cargo_flags:
    cargo fmt --all {{cargo_flags}} -- --check
    cargo fmt --all --manifest-path=tests/extendrtests/src/rust/Cargo.toml {{cargo_flags}} -- --check

fmt *cargo_flags:
    cargo fmt --all {{cargo_flags}}
    cargo fmt --all --manifest-path=tests/extendrtests/src/rust/Cargo.toml {{cargo_flags}}

test *cargo_flags:
    cargo test --workspace --no-fail-fast --features=full-functionality -- --no-capture --test-threads=1 {{cargo_flags}}
    cargo test --manifest-path=tests/extendrtests/src/rust/Cargo.toml --no-fail-fast -- --no-capture --test-threads=1 {{cargo_flags}}

tree *cargo_flags:
  cargo tree --workspace {{cargo_flags}}
  cargo tree --manifest-path=tests/extendrtests/src/rust/Cargo.toml {{cargo_flags}}

expand *cargo_flags:
    cargo expand -p extendr-api {{cargo_flags}}
    cargo expand -p extendr-macros {{cargo_flags}}
    cargo expand --manifest-path=tests/extendrtests/src/rust/Cargo.toml {{cargo_flags}}

# Verify MSRV with optional comma-separated FEATURES (empty means default features)
msrv FEATURES="":
    if [ "{{FEATURES}}" = "" ]; then \
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
    if [ "{{FILTER}}" = "" ]; then \
      Rscript -e 'devtools::test()'; \
    else \
      Rscript -e 'devtools::test(filter = "{{FILTER}}")'; \
    fi

# Run R CMD check on extendrtests; set NO_VIGNETTES=1 or ERROR_ON=warning|error; optional CHECK_DIR
r-cmd-check NO_VIGNETTES="0" ERROR_ON="warning" CHECK_DIR="":
    CHECK_DIR_ARG="NULL" && \
    if [ -n "{{CHECK_DIR}}" ]; then \
      if [ "{{CHECK_DIR}}" = /* ]; then \
        CHECK_DIR_ARG="'{{CHECK_DIR}}'"; \
      else \
        CHECK_DIR_ARG="'$$(cd '{{CHECK_DIR}}' 2>/dev/null && pwd || realpath '{{CHECK_DIR}}')'"; \
      fi; \
    fi && \
    cd tests/extendrtests && \
    ARGS="'--as-cran','--no-manual'" && \
    if [ "{{NO_VIGNETTES}}" = "1" ]; then ARGS="$${ARGS},'--no-build-vignettes'"; fi; \
    Rscript -e "rcmdcheck::rcmdcheck(args = c($${ARGS}), error_on = '{{ERROR_ON}}', check_dir = $${CHECK_DIR_ARG})"
