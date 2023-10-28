use crate::commands::r_cmd_check::RCmdCheckErrorOn;
use clap::{Args, ValueEnum};

#[derive(Args, Debug)]
pub(crate) struct RCmdCheckArg {
    #[arg(long, default_value = "false", help = "Passed to R CMD check")]
    pub(crate) no_build_vignettes: bool,
    #[arg(long, short, default_value = "warning", help = "Passed to R CMD check")]
    pub(crate) error_on: ErrorOn,
}

#[derive(ValueEnum, Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum ErrorOn {
    Never,
    Note,
    Warning,
    Error,
}

impl Into<RCmdCheckErrorOn> for ErrorOn {
    fn into(self) -> RCmdCheckErrorOn {
        match self {
            ErrorOn::Never => RCmdCheckErrorOn::Never,
            ErrorOn::Note => RCmdCheckErrorOn::Note,
            ErrorOn::Warning => RCmdCheckErrorOn::Warning,
            ErrorOn::Error => RCmdCheckErrorOn::Error,
        }
    }
}
