use xshell::{Error, Shell};

pub(crate) fn run(_shell: &Shell) -> Result<(), Error> {
    // {
    //     let _ = shell.push_dir(shell.current_dir().join("tests").join("extendrtests"));
    //     let extendrtests = cmd!(shell, "R -e \"devtools::test()\" ").run()?;
    // }

    unreachable!("devtools::test()")
}
