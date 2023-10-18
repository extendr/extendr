use std::error::Error;
use std::path::Path;

use toml_edit::Document;
use xshell::Shell;

pub(crate) fn run(shell: &Shell) -> Result<(), Box<dyn Error>> {
    // {
    //     let _ = shell.push_dir(shell.current_dir().join("tests").join("extendrtests"));
    //     let extendrtests = cmd!(shell, "R -e \"devtools::test()\" ").run()?;
    // }

    {
        let _ = shell.push_dir("tests/extendrtests/src/rust");

        let (cargo_toml, is_crlf) = read_file_with_line_ending(&shell, "Cargo.toml")?;

        let cargo_toml: Document = cargo_toml.parse()?;

        write_file_preserve_line_ending(&shell, "Cargo.toml", &cargo_toml, is_crlf)?;
    }
    unreachable!("devtools::test()")
}

fn read_file_with_line_ending<P: AsRef<Path>>(
    shell: &Shell,
    path: P,
) -> Result<(String, bool), Box<dyn Error>> {
    let file_contents = shell.read_binary_file(path)?;
    let file_contents = String::from_utf8(file_contents)?;
    let is_crlf = file_contents.contains("\r\n");
    Ok((file_contents, is_crlf))
}

fn write_file_preserve_line_ending<P: AsRef<Path>>(
    shell: &Shell,
    path: P,
    contents: &Document,
    is_crlf: bool,
) -> Result<(), Box<dyn Error>> {
    let mut file_contents = contents.to_string();
    if is_crlf {
        file_contents = file_contents.replace("\n", "\r\n");
    }
    shell.write_file(path, file_contents)?;
    Ok(())
}
