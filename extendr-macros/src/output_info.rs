use lazy_static::lazy_static;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::string::ToString;
use std::sync::Mutex;
use syn::{FnArg, Ident, Pat, ReturnType};

lazy_static! {
    static ref WRAPPER_FNS: Mutex<Vec<WrapperFn>> = Mutex::new(Vec::new());
}

const OUTPUT_FILE_NAME: &str = "extendr_wrappers.R";

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

struct WrapperFn {
    pub name: String,
    pub wrapper_name: String,
    pub is_void: bool,
    pub arguments: Vec<WrapperFnArg>,
}

impl WrapperFn {
    fn to_r_wrapper(&self) -> String {
        let seperated_args = self
            .arguments
            .iter()
            .map(|arg| arg.name.as_ref())
            .collect::<Vec<_>>()
            .join(", ");
        let mut inner_invocation = format!(
            ".Call(\"{name}\"{leading_comma}{args})",
            name = &self.wrapper_name,
            leading_comma = match self.arguments.len() {
                0 => "",
                _ => ", ",
            },
            args = seperated_args,
        );
        if self.is_void {
            inner_invocation = format!("invisible({})", inner_invocation);
        }

        let function_signature = format!(
            "{name} <- function({args})",
            name = &self.name,
            args = seperated_args,
        );

        format!(
            r#"
            {signature} {{
                {invocation}
            }}
            "#,
            signature = function_signature,
            invocation = inner_invocation
        )
    }
}

struct WrapperFnArg {
    pub name: String,
}

/// Extract info about wrapper function and write it to target directory.
pub fn output_wrapper_info(
    fn_name: &Ident,
    wrapper_fn_name: &str,
    args: Vec<FnArg>,
    return_type: &ReturnType,
) {
    let is_void = match return_type {
        ReturnType::Default => true,
        _ => false,
    };
    let mut func = WrapperFn {
        name: fn_name.to_string(),
        wrapper_name: wrapper_fn_name.to_owned(),
        is_void,
        arguments: Vec::new(),
    };

    for arg in args {
        if let FnArg::Typed(pat_type) = arg {
            if let Pat::Ident(pat_ident) = *pat_type.pat {
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

    // Write fn to singleton to keep track of all functions across macro invocations
    WRAPPER_FNS
        .lock()
        .expect("Could not aquire lock to WRAPPER_FNS singleton")
        .push(wrapper_fn);

    let file = File::create(&output_file_path)?;
    let mut writer = BufWriter::new(file);
    let wrapper_lock = WRAPPER_FNS
        .lock()
        .expect("Could not aquire lock to WRAPPER_FNS singleton");
    for wrapper_fn in wrapper_lock.iter() {
        writer
            .write_all(wrapper_fn.to_r_wrapper().as_bytes())
            .expect("Could not write R wrapper function to file");
    }

    Ok(())
}
