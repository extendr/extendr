use xshell::{cmd, Error, Shell};

pub(crate) fn run(shell: &Shell) -> Result<(), Error> {
    let msrv = cmd!(shell, "cargo-msrv --path extendr-api verify").run();
    if msrv.is_err() {
        println!(
            "Cannot perform `cargo-msrv` check\nInstall `cargo-msrv` by `cargo install cargo-msrv`"
        );
    }
    msrv
}
