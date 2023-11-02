use std::error::Error;
use std::path::Path;

use toml_edit::{Document, InlineTable, Value};
use xshell::Shell;

use crate::extendrtests::path_helper::RCompatiblePath;

pub(crate) const R_FOLDER_PATH: &str = "tests/extendrtests";

const RUST_FOLDER_PATH: &str = "tests/extendrtests/src/rust";
const CARGO_TOML: &str = "Cargo.toml";

#[derive(Debug)]
pub(crate) struct DocumentHandle<'a> {
    document: Vec<u8>,
    shell: &'a Shell,
}

impl<'a> Drop for DocumentHandle<'a> {
    fn drop(&mut self) {
        let _rust_folder = self.shell.push_dir(RUST_FOLDER_PATH);
        self.shell
            .write_file(CARGO_TOML, &self.document)
            .expect("Failed to restore Cargo.toml");
    }
}

pub(crate) fn swap_extendr_api_path(shell: &Shell) -> Result<DocumentHandle, Box<dyn Error>> {
    let current_path = shell.current_dir();
    let _rust_folder = shell.push_dir(RUST_FOLDER_PATH);

    let original_cargo_toml_bytes = read_file_with_line_ending(shell, CARGO_TOML)?;

    let original_cargo_toml: Document = std::str::from_utf8(&original_cargo_toml_bytes)?.parse()?;

    let mut cargo_toml = original_cargo_toml.clone();

    let extendr_api_entry =
        get_extendr_api_entry(&mut cargo_toml).ok_or("`extendr-api` not found in Cargo.toml")?;

    let mut replacement = InlineTable::new();

    let item = Value::from(get_replacement_path(&current_path));
    replacement.entry("path").or_insert(item);
    *extendr_api_entry = Value::InlineTable(replacement);

    shell.write_file(CARGO_TOML, cargo_toml.to_string())?;
    Ok(DocumentHandle {
        document: original_cargo_toml_bytes,
        shell,
    })
}

fn get_replacement_path(path: &Path) -> String {
    let path = path.adjust_for_r();

    format!("{path}/extendr-api")
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
) -> Result<Vec<u8>, Box<dyn Error>> {
    let file_contents = shell.read_binary_file(path)?;
    Ok(file_contents)
}
