use extendr_api::*;

#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}

// functions to test input/output conversion

/// convert a double scalar to itself
/// @param x a number
#[extendr]
fn double_scalar(x: f64) -> f64 { x }

/// convert an int scalar to itself
/// @param x a number
#[extendr]
fn int_scalar(x: i32) -> i32 { x }

#[extendr]
fn bool_scalar(x: bool) -> bool { x }
#[extendr]
fn char_scalar(x: String) -> String { x }

#[derive(Default, Debug)]
struct MyClass {
    a: i32,
}

#[extendr]
impl MyClass {
    fn new() -> Self {
        Self { a: 0 }
    }
    
    fn set_a(& mut self, x: i32) {
        self.a = x;
    }
    
    fn a(&self) -> i32 {
        self.a
    }
    
    fn me(&self) -> &Self {
        self
    }
}

// Macro to generate exports
extendr_module! {
    mod extendrtests;
    fn hello_world;

    fn double_scalar;
    fn int_scalar;
    fn bool_scalar;
    fn char_scalar;
    
    impl MyClass;
}
