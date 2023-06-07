use extendr_api::prelude::*;

#[extendr]
fn leak_implicit_strings(x: Strings) -> String {
    x.len().to_string()
}

#[extendr]
fn leak_implicit_doubles(x: Doubles) -> String {
    x.len().to_string()
}

#[extendr(use_try_from = true)]
fn leak_arg2_try_implicit_strings(_y: Doubles, x: Strings) -> String {
    x.len().to_string()
}

#[extendr(use_try_from = true)]
fn leak_arg2_try_implicit_doubles(_y: Doubles, x: Doubles) -> String {
    x.len().to_string()
}

#[extendr]
fn leak_unwrap_strings(x: Robj) -> String {
    let x = x.as_string_vector().ok_or("ERROR").unwrap();
    x.len().to_string()
}

#[extendr]
fn leak_unwrap_doubles(x: Robj) -> String {
    x.as_real_vector().ok_or("ERROR").unwrap().len().to_string()
}

#[extendr]
fn leak_positive_control(x: Robj) {
    std::mem::forget(x);
}

#[extendr]
fn leak_negative_control(x: Robj) {
    drop(x)
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod memory_leaks;

    fn leak_implicit_strings;
    fn leak_implicit_doubles;
    fn leak_arg2_try_implicit_strings;
    fn leak_arg2_try_implicit_doubles;
    fn leak_unwrap_strings;
    fn leak_unwrap_doubles;
    fn leak_positive_control;
    fn leak_negative_control;

}
