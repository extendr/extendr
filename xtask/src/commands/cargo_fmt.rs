use xshell::{cmd, Error, Shell};

pub(crate) fn run(shell: &Shell) -> Result<(), Error> {
    // extendr-api, extendr-macros, extendr-engine, xtask
    cmd!(shell, "cargo fmt --all").run()?;
    // extendrtests
    cmd!(
        shell,
        "cargo fmt --all --manifest-path tests/extendrtests/src/rust/Cargo.toml"
    )
    .run()?;

    Ok(())
}
