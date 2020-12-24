use extendr_api::*;

#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}

// functions to test input/output conversion
#[extendr]
fn double_scalar(x: f64) -> f64 { x }
#[extendr]
fn int_scalar(x: i32) -> i32 { x }
#[extendr]
fn bool_scalar(x: bool) -> bool { x }
#[extendr]
fn char_scalar(x: String) -> String { x }


// Macro to generate exports
extendr_module! {
    mod extendrtests;
    fn hello_world;

    fn double_scalar;
    fn int_scalar;
    fn bool_scalar;
    fn char_scalar;
}
