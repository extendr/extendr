use xshell::{cmd, Error, Shell};

pub(crate) fn run(shell: &Shell, features: Option<Vec<String>>) -> Result<(), Error> {
    let msrv = match features {
        Some(feats) if !feats.is_empty() => {
            let features = feats.join(",");
            cmd!(
                shell,
                "cargo-msrv --path extendr-api verify -- cargo check --features {features}"
            )
            .run()
        }
        _ => cmd!(shell, "cargo-msrv --path extendr-api verify -- cargo check").run(),
    };

    if let Err(ref e) = msrv {
        eprintln!(
            "Cannot perform `cargo-msrv` check: {e}\n\
             Install `cargo-msrv` with `cargo install cargo-msrv`"
        );
    }

    msrv
}
