use xshell::{Error, Shell};

pub(crate) fn run(_shell: &Shell) -> Result<(), Error> {
    // let generate_docs = cmd!(
    //     shell,
    //     "cargo doc --workspace --no-deps --document-private-items --features full-functionality"
    // )
    unimplemented!(
        "cargo doc --workspace --no-deps --document-private-items --features full-functionality"
    )
}
