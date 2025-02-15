//! Module metadata
//!
//! This data is returned by get_module_metadata()
//! which is generated by [extendr_module!].
use crate::*;
use std::io::Write;

/// Metadata function argument.
#[derive(Debug, PartialEq)]
pub struct Arg {
    pub name: &'static str,
    pub arg_type: &'static str,
    pub default: Option<&'static str>,
}

/// Metadata function.
#[derive(Debug, PartialEq)]
pub struct Func {
    pub doc: &'static str,
    pub rust_name: &'static str,
    pub mod_name: &'static str,
    pub r_name: &'static str,
    pub args: Vec<Arg>,
    pub return_type: &'static str,
    pub func_ptr: *const u8,
    pub hidden: bool,
}

/// Metadata Impl.
#[derive(Debug, PartialEq)]
pub struct Impl {
    pub doc: &'static str,
    pub name: &'static str,
    pub methods: Vec<Func>,
    pub methods_only: bool,
}

/// Module metadata.
#[derive(Debug, PartialEq)]
pub struct Metadata {
    pub name: &'static str,
    pub functions: Vec<Func>,
    pub impls: Vec<Impl>,
}

struct RArg {
    name: String,
    default: Option<&'static str>,
}

impl RArg {
    fn is_self(&self) -> bool {
        self.name == "self"
    }

    fn to_actual_arg(&self) -> String {
        self.name.clone()
    }

    fn to_formal_arg(&self) -> String {
        match self.default {
            Some(default_val) => format!("{} = {}", self.name, default_val),
            None => self.name.clone(),
        }
    }
}

impl From<&Arg> for RArg {
    fn from(arg: &Arg) -> Self {
        Self {
            name: sanitize_identifier(arg.name),
            default: arg.default,
        }
    }
}

impl From<Arg> for Robj {
    fn from(val: Arg) -> Self {
        use crate as extendr_api;
        let mut result = List::from_values(&[r!(val.name), r!(val.arg_type)]);
        result
            .set_names(&["name", "arg_type"])
            .expect("From<Arg> failed");
        result.into()
    }
}

impl From<Func> for Robj {
    fn from(val: Func) -> Self {
        use crate as extendr_api;
        let mut result = List::from_values(&[
            r!(val.doc),
            r!(val.rust_name),
            r!(val.mod_name),
            r!(val.r_name),
            r!(List::from_values(val.args)),
            r!(val.return_type),
            r!(val.hidden),
        ]);
        result
            .set_names(&[
                "doc",
                "rust_name",
                "mod_name",
                "r_name",
                "args",
                "return.type",
                "hidden",
            ])
            .expect("From<Func> failed");
        result.into()
    }
}

impl From<Impl> for Robj {
    fn from(val: Impl) -> Self {
        use crate as extendr_api;
        let mut result = List::from_values(&[
            r!(val.doc),
            r!(val.name),
            r!(List::from_values(val.methods)),
        ]);
        result
            .set_names(&["doc", "name", "methods"])
            .expect("From<Impl> failed");
        result.into()
    }
}

impl From<Metadata> for Robj {
    fn from(val: Metadata) -> Self {
        use crate as extendr_api;
        let mut result = List::from_values(&[
            r!(val.name),
            r!(List::from_values(val.functions)),
            r!(List::from_values(val.impls)),
        ]);
        result
            .set_names(&["name", "functions", "impls"])
            .expect("From<Metadata> failed");
        result.into()
    }
}

fn write_doc(w: &mut Vec<u8>, doc: &str) -> std::io::Result<()> {
    if !doc.is_empty() {
        write!(w, "#'")?;
        for c in doc.chars() {
            if c == '\n' {
                write!(w, "\n#'")?;
            } else {
                write!(w, "{}", c)?;
            }
        }
        writeln!(w)?;
    }
    Ok(())
}

/// Wraps invalid R identifiers, like `_function_name`, into backticks.
/// Removes raw identifiers (`r#`).
fn sanitize_identifier(name: &str) -> String {
    if name.starts_with('_') {
        format!("`{}`", name)
    } else if name.starts_with("r#") {
        name.strip_prefix("r#").unwrap().into()
    } else {
        name.to_string()
    }
}

fn join_str(input: impl Iterator<Item = String>, sep: &str) -> String {
    input.collect::<Vec<String>>().join(sep)
}

/// Generate a wrapper for a non-method function.
fn write_function_wrapper(
    w: &mut Vec<u8>,
    func: &Func,
    package_name: &str,
    use_symbols: bool,
) -> std::io::Result<()> {
    if func.hidden {
        return Ok(());
    }

    write_doc(w, func.doc)?;

    let r_args: Vec<RArg> = func.args.iter().map(Into::into).collect();
    let actual_args = r_args.iter().map(|a| a.to_actual_arg());
    let formal_args = r_args.iter().map(|a| a.to_formal_arg());

    if func.return_type == "()" {
        write!(
            w,
            "{} <- function({}) invisible(.Call(",
            sanitize_identifier(func.r_name),
            join_str(formal_args, ", ")
        )?;
    } else {
        write!(
            w,
            "{} <- function({}) .Call(",
            sanitize_identifier(func.r_name),
            join_str(formal_args, ", ")
        )?;
    }

    if use_symbols {
        write!(w, "wrap__{}", func.mod_name)?;
    } else {
        write!(w, "\"wrap__{}\"", func.mod_name)?;
    }

    if !func.args.is_empty() {
        write!(w, ", {}", join_str(actual_args, ", "))?;
    }

    if !use_symbols {
        write!(w, ", PACKAGE = \"{}\"", package_name)?;
    }

    if func.return_type == "()" {
        writeln!(w, "))\n")?;
    } else {
        writeln!(w, ")\n")?;
    }

    Ok(())
}

/// Generate a wrapper for a method.
fn write_method_wrapper(
    w: &mut Vec<u8>,
    func: &Func,
    package_name: &str,
    use_symbols: bool,
    class_name: &str,
) -> std::io::Result<()> {
    if func.hidden {
        return Ok(());
    }

    let r_args: Vec<RArg> = func.args.iter().map(Into::into).collect();
    let actual_args = r_args.iter().map(|a| a.to_actual_arg());

    // Skip a leading "self" argument.
    // This is supplied by the environment.
    let formal_args = r_args
        .iter()
        .skip_while(|a| a.is_self())
        .map(|a| a.to_formal_arg());

    // Both `class_name` and `func.name` should be processed
    // because they are exposed to R
    if func.return_type == "()" {
        write!(
            w,
            "{}${} <- function({}) invisible(.Call(",
            sanitize_identifier(class_name),
            sanitize_identifier(func.r_name),
            join_str(formal_args, ", ")
        )?;
    } else {
        write!(
            w,
            "{}${} <- function({}) .Call(",
            sanitize_identifier(class_name),
            sanitize_identifier(func.r_name),
            join_str(formal_args, ", ")
        )?;
    }

    // Here no processing is needed because of `wrap__` prefix
    if use_symbols {
        write!(w, "wrap__{}__{}", class_name, func.mod_name)?;
    } else {
        write!(w, "\"wrap__{}__{}\"", class_name, func.mod_name)?;
    }

    if actual_args.len() != 0 {
        write!(w, ", {}", join_str(actual_args, ", "))?;
    }

    if !use_symbols {
        write!(w, ", PACKAGE = \"{}\"", package_name)?;
    }

    if func.return_type == "()" {
        writeln!(w, "))\n")?;
    } else {
        writeln!(w, ")\n")?;
    }

    Ok(())
}

/// Generate a wrapper for an implementation block.
fn write_impl_wrapper(
    w: &mut Vec<u8>,
    imp: &Impl,
    package_name: &str,
    use_symbols: bool,
) -> std::io::Result<()> {
    let exported = imp.doc.contains("@export");

    write_doc(w, imp.doc)?;

    let imp_name_fixed = sanitize_identifier(imp.name);

    if !imp.methods_only {
        // Using fixed name because it is exposed to R
        writeln!(w, "{} <- new.env(parent = emptyenv())\n", imp_name_fixed)?;
    }

    for func in &imp.methods {
        // write_doc(& mut w, func.doc)?;
        // `imp.name` is passed as is and sanitized within the function
        write_method_wrapper(w, func, package_name, use_symbols, imp.name)?;
    }

    if exported {
        writeln!(w, "#' @rdname {}", imp.name)?;
        writeln!(w, "#' @usage NULL")?;
    }

    if !imp.methods_only {
        // This is needed no matter whether the user added `@export` or
        // not; even if we don't export the class itself and its
        // initializers, we always export the `$` method so the method is
        // correctly added to the NAMESPACE.
        writeln!(w, "#' @export")?;

        // LHS with dollar operator is wrapped in ``, so pass name as is,
        // but in the body `imp_name_fixed` is called as valid R function,
        // so we pass preprocessed value
        writeln!(w, "`$.{}` <- function (self, name) {{ func <- {}[[name]]; environment(func) <- environment(); func }}\n", imp.name, imp_name_fixed)?;

        writeln!(w, "#' @export")?;
        writeln!(w, "`[[.{}` <- `$.{}`\n", imp.name, imp.name)?;
    }

    Ok(())
}

impl Metadata {
    pub fn make_r_wrappers(
        &self,
        use_symbols: bool,
        package_name: &str,
    ) -> std::io::Result<String> {
        let mut w = Vec::new();

        writeln!(
            w,
            r#"# Generated by extendr: Do not edit by hand
#
# This file was created with the following call:
#   .Call("wrap__make_{}_wrappers", use_symbols = {}, package_name = "{}")
"#,
            self.name,
            if use_symbols { "TRUE" } else { "FALSE" },
            package_name
        )?;

        if use_symbols {
            writeln!(w, "#' @usage NULL")?;
            writeln!(w, "#' @useDynLib {}, .registration = TRUE", package_name)?;
            writeln!(w, "NULL")?;
            writeln!(w)?;
        }

        for func in &self.functions {
            write_function_wrapper(&mut w, func, package_name, use_symbols)?;
        }

        for imp in &self.impls {
            write_impl_wrapper(&mut w, imp, package_name, use_symbols)?;
        }
        unsafe { Ok(String::from_utf8_unchecked(w)) }
    }
}
