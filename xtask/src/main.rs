use xshell::Shell;

use crate::cli::RCmdCheckArg;

mod cli;
mod commands;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::parse();
    let shell = Shell::new()?;

    // xtask
    // shell.change_dir(std::env::var("CARGO_MANIFEST_DIR")?);
    // dbg!(&shell.current_dir());
    dbg! {&cli};

    let result = match cli.command {
        cli::Commands::CheckFmt => commands::cargo_fmt_check::run(&shell)?,
        cli::Commands::RCmdCheck(RCmdCheckArg { no_build_vignettes }) => {
            commands::r_cmd_check::run(&shell, no_build_vignettes)?
        }
        cli::Commands::Doc => commands::generate_docs::run(&shell)?,
        cli::Commands::Msrv => commands::cargo_msrv::run(&shell)?,
        cli::Commands::DevtoolsTest => commands::devtools_test::run(&shell)?,
    };

    Ok(result)
}
