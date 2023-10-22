use xshell::{Error, Shell, cmd};

pub(crate) fn run(shell: &Shell) -> Result<(), Error> {
    let _generate_docs = cmd!(
        shell,
        "cargo doc --workspace --no-deps --document-private-items --features full-functionality"
    ).run()?;

    Ok(())
}
