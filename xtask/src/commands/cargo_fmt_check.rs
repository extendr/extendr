use xshell::{cmd, Error, Shell};

pub(crate) fn run(shell: &Shell) -> Result<(), Error> {
    let check_extendr = cmd!(shell, "cargo fmt --all -- --check").run();

    let check_extendrtests = cmd!(
        shell,
        "cargo fmt --all --manifest-path tests/extendrtests/src/rust/Cargo.toml -- --check"
    )
    .run();

    if check_extendr.is_err() || check_extendrtests.is_err() {
        println!("Please run `cargo extendr fmt`");
    } else {
        println!("Success!");
    }

    check_extendr
}
