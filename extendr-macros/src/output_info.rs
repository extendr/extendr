use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;
use syn::FnArg;

const OUTPUT_FILE_NAME: &str = "extendr_wrappers.json";

fn find_target_dir() -> Result<PathBuf, &'static str> {
    if let Some(manifest_dir) = env::var_os("CARGO_MANIFEST_DIR") {
        let target_dir = PathBuf::from(manifest_dir).join("target");
        return Ok(target_dir);
    }
    Err("Unable to determine cargo target directory")
}

fn find_output_file() -> Result<PathBuf, &'static str> {
    Ok(find_target_dir()?.join(OUTPUT_FILE_NAME))
}

#[derive(Serialize, Deserialize)]
struct WrapperFn {
    pub name: String,
    pub is_void: bool,
    pub arguments: Vec<WrapperFnArg>,
}

#[derive(Serialize, Deserialize)]
struct WrapperFnArg {
    pub name: String,
}

/// Extract info about wrapper function and write it to target directory.
pub fn output_wrapper_info(wrapper_fn_name: &str, args: Vec<FnArg>) {
    let is_void = false; // TODO
    let mut func = WrapperFn {
        name: wrapper_fn_name.to_owned(),
        is_void,
        arguments: Vec::new(),
    };

    for arg in args {
        if let FnArg::Typed(pat_type) = arg {
            if let syn::Pat::Ident(pat_ident) = *pat_type.pat {
                func.arguments.push(WrapperFnArg {
                    name: pat_ident.ident.to_string(),
                })
            }
        }
    }

    write_wrapper_info(func).expect("Unable to write wrapper info file to target directory");
}
/// Write info about wrapper function to the output info file in the target directory.
fn write_wrapper_info(wrapper_fn: WrapperFn) -> Result<(), Box<dyn Error>> {
    let output_file_path = find_output_file().unwrap();

    // Read previously emitted wrapper functions from file
    let mut wrapper_fns: Vec<WrapperFn> = match output_file_path.exists() {
        true => {
            let file = File::open(&output_file_path)?;
            let reader = BufReader::new(file);
            serde_json::from_reader(reader)?
        }
        false => Vec::new(),
    };

    wrapper_fns.push(wrapper_fn);

    let file = File::create(&output_file_path)?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, &wrapper_fns)?;
    writer.flush()?;

    Ok(())
}
