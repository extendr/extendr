use std::path::PathBuf;

use xshell::Shell;

use cli::r_cmd_check::RCmdCheckArg;

mod cli;
mod commands;
mod extendrtests;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::parse();
    let shell = Shell::new()?;
    let original_path = shell.current_dir();

    let path: PathBuf = std::env::var("CARGO_MANIFEST_DIR")?.parse()?;

    shell.change_dir(
        path.parent()
            .ok_or("Failed to get parent dir")?
            .canonicalize()?,
    );
    match cli.command {
        cli::Commands::Fmt => commands::cargo_fmt::run(&shell)?,
        cli::Commands::CheckFmt => commands::cargo_fmt_check::run(&shell)?,
        cli::Commands::RCmdCheck(RCmdCheckArg {
            no_build_vignettes,
            error_on,
            check_dir,
        }) => commands::r_cmd_check::run(
            &shell,
            no_build_vignettes,
            error_on.into(),
            check_dir,
            original_path,
        )?,
        cli::Commands::Doc => commands::generate_docs::run(&shell)?,
        cli::Commands::Msrv => commands::cargo_msrv::run(&shell)?,
        cli::Commands::DevtoolsTest => commands::devtools_test::run(&shell)?,
        // TODO: Accept snapshots from command-line
        // Rscript -e "testthat::snapshot_accept(path = 'tests/extendrtests/tests/testthat', 'macro-snapshot')" 
    };

    Ok(())
}
