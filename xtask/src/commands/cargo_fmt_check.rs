use xshell::{cmd, Error, Shell};

pub(crate) fn run(shell: &Shell) -> Result<(), Error> {
    let check_result = cmd!(shell, "cargo fmt -- --check").run();
    if check_result.is_err() {
        println!("Please run `cargo fmt --all`");
    } else {
        println!("Success!");
    }

    check_result
}
