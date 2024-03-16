//! This invokes `rextendr::document()` within `tests/extendrtests`.
//! 
//! It uses the vendored `rextendr` in the repository as the source package.
//! 
//! 1. Ensure that `git submodule update --init` was invoked once, as to setup
//! the vendored `rextendr` package.
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

    //FIXME: check if `../../rextendr` is available, and report back
    // if it is not, then it is due to lack of `git submodule update --init`
    // which should be
    //FIXME: test if `devtools` is available, and report back if not

    cmd!(shell, "Rscript")
        .args([
            "-e",
            r#"devtools::load_all("../../rextendr")"#,
            "-e",
            r#"rextendr::document()"#,
        ])
        .run()?;
    Ok(())
}
