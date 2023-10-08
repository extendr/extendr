use xshell::{cmd, Error, Shell};

pub(crate) fn run(shell: &Shell) -> Result<(), Error> {
    cmd!(shell, "cargo fmt -- --check").run()
}
