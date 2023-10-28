use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    #[command(about = "Run cargo fmt on extendr")]
    CheckFmt,
    #[command(about = "Run R CMD check on {extendrtests}")]
    RCmdCheck(RCmdCheckArg),
    #[command(about = "Generate documentation for all features")]
    Doc,
    #[command(about = "Check that the specified rust-version is MSRV")]
    Msrv,
    #[command(about = "Run devtools::test() on {extendrtests}")]
    DevtoolsTest,
}

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

pub(crate) fn parse() -> Cli {
    Cli::parse()
}
