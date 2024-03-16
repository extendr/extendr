use std::error::Error;

use xshell::{cmd, Shell};

use crate::{
    cli::devtools_test::DevtoolsTestArg,
    extendrtests::with_absolute_path::{swap_extendr_api_path, R_FOLDER_PATH},
};

pub(crate) fn run(shell: &Shell, args: DevtoolsTestArg) -> Result<(), Box<dyn Error>> {
    let _document_handle = swap_extendr_api_path(shell)?;

    run_tests(shell, args)?;

    Ok(())
}

fn run_tests(shell: &Shell, args: DevtoolsTestArg) -> Result<(), Box<dyn Error>> {
    let _r_path = shell.push_dir(R_FOLDER_PATH);
    if args.snapshot_accept {
        cmd!(
            shell,
            "Rscript -e testthat::snapshot_accept(\"macro-snapshot\")"
        )
        .run()?;
    }
    cmd!(shell, "Rscript -e devtools::test()").run()?;

    Ok(())
}
