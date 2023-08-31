use xshell::{cmd, Shell};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let shell = Shell::new()?;
    // xtask
    // shell.change_dir(std::env::var("CARGO_MANIFEST_DIR")?);
    // dbg!(&shell.current_dir());
    let generate_docs = cmd!(
        shell,
        "cargo doc --workspace --no-deps --document-private-items --features full-functionality"
    )
    .run()?;
    {
        let _ = shell.push_dir(shell.current_dir().join("tests").join("extendrtests"));
        let extendrtests = cmd!(shell, "R -e \"devtools::test()\" ").run()?;
    }

    //TODO: Add option for `--no-build-vignettes`

    let rcmdcheck_extendrtests =
        cmd!(shell, "R CMD check --no-manual --as-cran --force-multiarch tests/extendrtests").run()?;

    let fmt_check = cmd!(shell, "cargo fmt -- --check").run()?;

    let msrv = cmd!(shell, "cargo-msrv --path extendr-api/ verify");
    match msrv.run() {
        Ok(_) => {}
        Err(error) => {
            //FIXME: Displays badly
            return Err(format!(
                "{}\n\nInstall `cargo-msrv` by `cargo install cargo-msrv`",
                error.to_string()
            )
            .into());
        }
    }

    Ok(())
}
