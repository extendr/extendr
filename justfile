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

doc:
    cargo +nightly doc --workspace --no-deps --document-private-items --features full-functionality
    cargo +nightly doc --document-private-items --features full-functionality --manifest-path=tests/extendrtests/src/rust/Cargo.toml
