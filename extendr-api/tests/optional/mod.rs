//! Tests for the optional (additive) features of `extendr-api`
//!
//!
//! These are invoked via
//!
//! ```sh
//! cargo test --test features --feature <features required>
//! ```
//!
//! For this purpose, the feature `full-functionality` ought to represent all
//! the additive features to `extendr-api`, i.e.
//!
//! ```sh
//! cargo test --test features --feature full-functionality
//! ```
//!
//! should run all the tests defined here.
//!

#[cfg(feature = "either")]
mod either;

#[cfg(feature = "ndarray")]
mod ndarray;

#[cfg(feature = "serde")]
mod serde;

#[cfg(feature = "non-api")]
mod non_api;
