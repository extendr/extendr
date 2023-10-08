use xshell::{Error, Shell};

pub(crate) fn run(shell: &Shell, no_build_vignettes: bool) -> Result<(), Error> {
    let mut cmd = shell
        .cmd("R")
        .arg("CMD")
        .arg("check")
        .arg("tests/extendrtests")
        .arg("--no-manual")
        .arg("--as-cran")
        .arg("--force-multiarch");

    if no_build_vignettes {
        cmd = cmd.arg("--no-build-vignettes");
    }
    cmd.run()
}
