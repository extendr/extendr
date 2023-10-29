use std::error::Error;
use std::path;
use std::path::{Path, PathBuf};

use crate::extendrtests::path_helper::RCompatiblePath;
use xshell::Shell;

use crate::extendrtests::with_absolute_path::{swap_extendr_api_path, R_FOLDER_PATH};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum RCmdCheckErrorOn {
    Never,
    Note,
    Warning,
    Error,
}

impl RCmdCheckErrorOn {
    fn get_error_on(&self) -> &'static str {
        match self {
            RCmdCheckErrorOn::Never => "'never'",
            RCmdCheckErrorOn::Note => "'note'",
            RCmdCheckErrorOn::Warning => "'warning'",
            RCmdCheckErrorOn::Error => "'error'",
        }
    }
}

pub(crate) fn run<P: AsRef<Path>>(
    shell: &Shell,
    no_build_vignettes: bool,
    error_on: RCmdCheckErrorOn,
    check_dir: Option<String>,
    initial_path: P,
) -> Result<(), Box<dyn Error>> {
    if let Some(check_dir) = check_dir {
        let mut path = PathBuf::from(check_dir);
        if !path.is_absolute() {
            let str_rep = path.to_string_lossy();
            if str_rep.starts_with("./") {
                path = PathBuf::from(str_rep.trim_start_matches("./"));
            } else if str_rep.starts_with(r".\\") {
                path = PathBuf::from(str_rep.trim_start_matches(r".\\"));
            }
            path = initial_path.as_ref().canonicalize()?.join(path);
        }
        let path = path.adjust_for_r();
        dbg! {&path};
    }

    let _document_handle = swap_extendr_api_path(shell)?;

    run_r_cmd_check(shell, no_build_vignettes, error_on)
}

fn run_r_cmd_check(
    shell: &Shell,
    no_build_vignettes: bool,
    error_on: RCmdCheckErrorOn,
) -> Result<(), Box<dyn Error>> {
    let _r_path = shell.push_dir(R_FOLDER_PATH);
    let mut args = vec!["'--as-cran'", "'--no-manual'"];
    if no_build_vignettes {
        args.push("'--no-build-vignettes'");
    }

    let args = format!("c({0})", args.join(", "));

    let error_on = error_on.get_error_on();
    shell
        .cmd("Rscript")
        .arg("-e")
        .arg(format!(
            "rcmdcheck::rcmdcheck(args = {args}, error_on = {error_on})"
        ))
        .run()?;

    Ok(())
}
