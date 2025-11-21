# https://just.systems

default:
    echo 'Hello, world!'

build:
    cargo build --workspace
    cargo build -p extendr-api
    cargo build -p extendr-macros
    cargo build -p extendr-ffi
    cargo build -p xtask
    cargo build --manifest-path=tests/extendrtests/src/rust/Cargo.toml

clippy:
    cargo clippy --workspace
    cargo clippy --manifest-path=tests/extendrtests/src/rust/Cargo.toml

doc-check:
    cargo +nightly doc --no-deps --document-private-items --features full-functionality --workspace
    cargo +nightly doc --no-deps --document-private-items --features full-functionality --manifest-path=tests/extendrtests/src/rust/Cargo.toml

doc:
    cargo +nightly doc --document-private-items --features full-functionality --workspace
    cargo +nightly doc --document-private-items --features full-functionality --manifest-path=tests/extendrtests/src/rust/Cargo.toml

fmt:
    cargo fmt -p extendr-api
    cargo fmt -p extendr-macros
    cargo fmt -p extendr-ffi
    cargo fmt -p xtask
    cargo fmt --manifest-path=tests/extendrtests/src/rust/Cargo.toml
