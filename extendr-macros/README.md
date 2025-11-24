# extendr-macros

A procedural macro crate for extendr-api.

This crate implements macros such as the `#[extendr]` function
markup and the `extendr_module!` macro. See `extendr-api` for
more details.

## For Developers

This crate uses [`trybuild`](https://docs.rs/trybuild/) for testing procedural macros.
Trybuild compiles test cases in `tests/cases/` and `tests/extendr_impl/` and verifies that the compiler produces expected error messages for invalid code.

### Running Tests

To run the trybuild tests:

```bash
cargo test --package extendr-macros --test trybuild
```

### Updating Expected Outputs

When you intentionally change error messages or add new test cases, you need to update the expected output files (`.stderr` files). Use the `TRYBUILD=overwrite` environment variable to automatically update them:

```bash
TRYBUILD=overwrite cargo test --package extendr-macros --test trybuild
```

This will regenerate the `.stderr` files in the `tests/cases/` directory to match the current compiler output.
