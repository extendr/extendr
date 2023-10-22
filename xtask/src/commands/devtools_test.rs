use std::error::Error;
use std::path::{Path, PathBuf};

use toml_edit::{Document, InlineTable, Value};
use xshell::Shell;

const RUST_FOLDER_PATH: &str = "tests/extendrtests/src/rust";
const R_FOLDER_PATH: &str = "tests/extendrtests";
const CARGO_TOML: &str = "Cargo.toml";

pub(crate) fn run(shell: &Shell) -> Result<(), Box<dyn Error>> {
    let _document_handle = swap_extendr_api_path(&shell)?;

    run_tests(&shell)?;

    Ok(())
}

#[derive(Debug, Clone)]
struct DocumentHandle<'a> {
    document: Document,
    is_crlf: bool,
    shell: &'a Shell,
}

impl<'a> Drop for DocumentHandle<'a> {
    fn drop(&mut self) {
        let _rust_folder = self.shell.push_dir(RUST_FOLDER_PATH);
        write_file_preserve_line_ending(&self.shell, CARGO_TOML, &self.document, self.is_crlf)
            .expect("Failed to restore Cargo.toml");
    }
}

fn run_tests(shell: &Shell) -> Result<(), Box<dyn Error>> {
    let _r_path = shell.push_dir(R_FOLDER_PATH);
    shell
        .cmd("Rscript")
        .arg("-e")
        .arg("devtools::test()")
        .run()?;

    Ok(())
}

fn swap_extendr_api_path(shell: &Shell) -> Result<DocumentHandle, Box<dyn Error>> {
    let current_path = shell.current_dir();
    let _rust_folder = shell.push_dir(RUST_FOLDER_PATH);

    let (original_cargo_toml, is_crlf) = read_file_with_line_ending(&shell, CARGO_TOML)?;

    let original_cargo_toml: Document = original_cargo_toml.parse()?;

    let mut cargo_toml = original_cargo_toml.clone();

    let extendr_api_entry =
        get_extendr_api_entry(&mut cargo_toml).ok_or("`extendr-api` not found in Cargo.toml")?;

    let mut replacement = InlineTable::new();

    let item = Value::from(get_replacement_path(&current_path));
    replacement.entry("path").or_insert(item);
    *extendr_api_entry = Value::InlineTable(replacement);

    write_file_preserve_line_ending(&shell, CARGO_TOML, &cargo_toml, is_crlf)?;
    Ok(DocumentHandle {
        document: original_cargo_toml,
        is_crlf,
        shell,
    })
}

fn get_replacement_path(path: &PathBuf) -> String {
    let path = path.to_string_lossy();
    let path = if cfg!(target_os = "windows") && path.starts_with(r"\\?\") {
        path[4..].replace("\\", "/")
    } else {
        path.to_string()
    };

    format!("{}/extendr-api", path)
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
