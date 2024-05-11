use std::error::Error;
use std::path::{Path, PathBuf};

use xshell::Shell;

use crate::extendrtests::path_helper::RCompatiblePath;
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
    let check_dir = match check_dir {
        Some(cd) => Some(construct_check_dir_path(cd, initial_path)?),
        _ => None,
    };

    let _document_handle = swap_extendr_api_path(shell)?;

    run_r_cmd_check(shell, no_build_vignettes, error_on, check_dir)
}

fn construct_check_dir_path<S: AsRef<str>, P: AsRef<Path>>(
    check_dir: S,
    initial_path: P,
) -> Result<String, Box<dyn Error>> {
    let mut path = PathBuf::from(check_dir.as_ref());
    if !path.is_absolute() {
        let str_rep = path.to_string_lossy();
        if str_rep.starts_with("./") {
            path = PathBuf::from(str_rep.trim_start_matches("./"));
        } else if str_rep.starts_with(r".\\") {
            path = PathBuf::from(str_rep.trim_start_matches(r".\\"));
        }
        path = initial_path.as_ref().canonicalize()?.join(path);
    }
    Ok(path.adjust_for_r())
}

#[derive(Debug)]
enum RCmdCheckError {
    MissingRPackages,
}
impl std::fmt::Display for RCmdCheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RCmdCheckError::MissingRPackages => {
                write!(f, "Missing required R-packages, please install them.")
            }
        }
    }
}
impl Error for RCmdCheckError {}

fn run_r_cmd_check(
    shell: &Shell,
    no_build_vignettes: bool,
    error_on: RCmdCheckErrorOn,
    check_dir: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let _r_path = shell.push_dir(R_FOLDER_PATH);
    let mut args = vec!["'--as-cran'", "'--no-manual'"];
    if no_build_vignettes {
        args.push("'--no-build-vignettes'");
    }

    let args = format!("c({0})", args.join(", "));

    let error_on = error_on.get_error_on();

    let check_dir = match check_dir {
        Some(cd) => format!("'{}'", cd),
        _ => "NULL".to_string(),
    };

    let has_prerequisites = shell
        .cmd("Rscript")
        .args([
            "-e",
            r#"requireNamespace("devtools");\
            requireNamespace("rcmdcheck");\
            requireNamespace("patrick");\
            requireNamespace("lobstr");\
            requireNamespace("rextendr")"#,
        ])
        .run()
        .is_ok();

    if !has_prerequisites {
        println!(
            r#"R installation is missing necessary packages.
RScript -e 'options(repos = list(CRAN="http://cran.rstudio.com/"))'
        -e 'install.packages("devtools")'
        -e 'install.packages("rcmdcheck")'
        -e 'install.packages("patrick")'
        -e 'install.packages("lobstr")'
        -e 'install.packages("rextendr")'

Alternatively, install development version on rextendr
Rscript -e 'options(repos = list(CRAN="http://cran.rstudio.com/"))'
        -e 'remotes::install_github("extendr/rextendr")'"#
        );
        return Err(RCmdCheckError::MissingRPackages.into());
    }

    shell
        .cmd("Rscript")
        .arg("-e")
        .arg(format!(
            "rcmdcheck::rcmdcheck(args = {args}, error_on = {error_on}, check_dir = {check_dir})"
        ))
        .run()?;

    Ok(())
}
