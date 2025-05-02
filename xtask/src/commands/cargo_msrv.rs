use xshell::{cmd, Error, Shell};

pub(crate) fn run(shell: &Shell, features: Option<Vec<String>>) -> Result<(), Error> {
    let features_arg = features.map(|x| x.join(",")).unwrap_or_default();

    let msrv = cmd!(
        shell,
        "cargo-msrv --path extendr-api verify -- cargo check --features={features_arg}"
    )
    .run();
    if msrv.is_err() {
        println!(
            "Cannot perform `cargo-msrv` check\nInstall `cargo-msrv` by `cargo install cargo-msrv`"
        );
    }
    msrv
}
