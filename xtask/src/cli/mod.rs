use clap::{Parser, Subcommand};

pub(crate) mod devtools_test;
pub(crate) mod r_cmd_check;

use devtools_test::DevtoolsTestArg;
use r_cmd_check::RCmdCheckArg;

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
    #[command(about = "Run `cargo fmt` on extendr crates")]
    Fmt,
    #[command(about = "Run R CMD check on {extendrtests}")]
    RCmdCheck(RCmdCheckArg),
    #[command(about = "Generate documentation for all features")]
    Doc,
    #[command(about = "Check that the specified rust-version is MSRV")]
    Msrv,
    #[command(about = "Run devtools::test() on {extendrtests} and generate snapshots")]
    DevtoolsTest(DevtoolsTestArg),
    #[command(about = "Generate wrappers by `rextendr::document()`")]
    Document,
}

pub(crate) fn parse() -> Cli {
    Cli::parse()
}
