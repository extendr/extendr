use clap::{Args, ValueEnum};

use crate::commands::r_cmd_check::RCmdCheckErrorOn;

#[derive(Args, Debug)]
pub(crate) struct RCmdCheckArg {
    #[arg(long, default_value = "false", help = "Passed to R CMD check")]
    pub(crate) no_build_vignettes: bool,
    #[arg(
        long,
        default_value = "warning",
        help = "Determines which R CMD check errors to fail on"
    )]
    pub(crate) error_on: ErrorOn,
    #[arg(long, help = "Defaults to a temporary directory")]
    pub(crate) check_dir: Option<String>,
}

#[derive(ValueEnum, Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum ErrorOn {
    Never,
    Note,
    Warning,
    Error,
}

impl From<ErrorOn> for RCmdCheckErrorOn {
    fn from(val: ErrorOn) -> Self {
        match val {
            ErrorOn::Never => RCmdCheckErrorOn::Never,
            ErrorOn::Note => RCmdCheckErrorOn::Note,
            ErrorOn::Warning => RCmdCheckErrorOn::Warning,
            ErrorOn::Error => RCmdCheckErrorOn::Error,
        }
    }
}
