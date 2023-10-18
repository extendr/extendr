use std::error::Error;
use std::path::Path;

use toml_edit::{Document, InlineTable, Value};
use xshell::Shell;

pub(crate) fn run(shell: &Shell) -> Result<(), Box<dyn Error>> {
    // {
    //     let _ = shell.push_dir(shell.current_dir().join("tests").join("extendrtests"));
    //     let extendrtests = cmd!(shell, "R -e \"devtools::test()\" ").run()?;
    // }

    {
        let _rust_folder = shell.push_dir("tests/extendrtests/src/rust");

        let (original_cargo_toml, is_crlf) = read_file_with_line_ending(&shell, "Cargo.toml")?;

        let original_cargo_toml: Document = original_cargo_toml.parse()?;

        let mut cargo_toml = original_cargo_toml.clone();

        let extendr_api_entry = get_extendr_api_entry(&mut cargo_toml)
            .ok_or("`extendr-api` not found in Cargo.toml")?;

        let mut replacement = InlineTable::new();
        let item = Value::from(
            shell
                .current_dir()
                .into_os_string()
                .to_string_lossy()
                .to_string()
                .replace("\\", "/"),
        );
        replacement.entry("path").or_insert(item);
        *extendr_api_entry = Value::InlineTable(replacement);

        write_file_preserve_line_ending(&shell, "Cargo.toml", &cargo_toml, is_crlf)?;
    }
    unreachable!("devtools::test()")
}

fn get_extendr_api_entry(document: &mut Document) -> Option<&mut Value> {
    document
        .get_mut("patch")?
        .get_mut("crates-io")?
        .get_mut("extendr-api")?
        .as_value_mut()
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
