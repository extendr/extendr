use xshell::{cmd, Error, Shell};

/// Generates documentation like on the site extendr.github.io
pub(crate) fn run(shell: &Shell) -> Result<(), Error> {
    cmd!(
        shell,
        "cargo doc --workspace --no-deps --document-private-items --features full-functionality"
    )
    .run()?;

    Ok(())
}
