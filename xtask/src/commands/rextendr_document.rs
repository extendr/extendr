//! This invokes `rextendr::document()` within `tests/extendrtests`.
//!
//! It uses the vendored `rextendr` in the repository as the source package.
//!
//! 1. Ensure that `git submodule update --init` was invoked once, as to setup
//!    the vendored `rextendr` package.
//! 2. `devtools` must be installed on system.
//!
//!
//! The idea here is to be able to develop `rextendr` alongside `extendr`,
//! as well as ease the development of extendr.
//!
use std::error::Error;

use crate::extendrtests::with_absolute_path::{swap_extendr_api_path, R_FOLDER_PATH};
use xshell::{cmd, Shell};

pub(crate) fn run(shell: &Shell) -> Result<(), Box<dyn Error>> {
    let _document_handle = swap_extendr_api_path(shell)?;

    run_rextendr_document(shell)
}

fn run_rextendr_document(shell: &Shell) -> Result<(), Box<dyn Error>> {
    let _r_path = shell.push_dir(R_FOLDER_PATH);

    let rextendr_submodule = std::path::Path::new(".../../rextendr");
    let rextendr_submodule = matches!(rextendr_submodule.try_exists(), Ok(true));
    if rextendr_submodule {
        println!("Loading vendored `{{rextendr}}`");
        cmd!(shell, "Rscript")
            .args([
                "-e",
                r#"requireNamespace("devtools")"#,
                "-e",
                r#"devtools::load_all("../../rextendr")"#,
                "-e",
                r#"rextendr::document()"#,
            ])
            .run()?;
    } else {
        // check if rextendr is installed and use that instead
        println!("Using installed `{{rextendr}}`");
        cmd!(shell, "Rscript")
            .args([
                "-e",
                r#"requireNamespace("rextendr")"#,
                "-e",
                r#"rextendr::document()"#,
            ])
            .run()?;
    }

    Ok(())
}
