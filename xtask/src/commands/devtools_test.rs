use std::error::Error;

use crate::extendrtests::with_absolute_path::swap_extendr_api_path;
use xshell::{cmd, Shell};

const R_FOLDER_PATH: &str = "tests/extendrtests";

pub(crate) fn run(shell: &Shell) -> Result<(), Box<dyn Error>> {
    let _document_handle = swap_extendr_api_path(shell)?;

    run_tests(shell)?;

    Ok(())
}

fn run_tests(shell: &Shell) -> Result<(), Box<dyn Error>> {
    let _r_path = shell.push_dir(R_FOLDER_PATH);
    cmd!(shell, "Rscript -e devtools::test()").run()?;

    Ok(())
}
