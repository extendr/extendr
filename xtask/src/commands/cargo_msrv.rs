use xshell::{Error, Shell};

pub(crate) fn run(_shell: &Shell) -> Result<(), Error> {
    // let msrv = cmd!(shell, "cargo-msrv --path extendr-api/ verify");
    // match msrv.run() {
    //     Ok(_) => {}
    //     Err(error) => {
    //         //FIXME: Displays badly
    //         return Err(format!(
    //             "{}\n\nInstall `cargo-msrv` by `cargo install cargo-msrv`",
    //             error.to_string()
    //         )
    //         .into());
    //     }
    // }
    //
    unimplemented!("cargo-msrv --path extendr-api")
}
