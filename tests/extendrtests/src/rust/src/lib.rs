use extendr_api::*;

/// Say hello
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

/// convert a bool scalar to itself
/// @param x a number
#[extendr]
fn bool_scalar(x: bool) -> bool { x }

/// convert a string to itself
/// @param x a number
#[extendr]
fn char_scalar(x: String) -> String { x }

/// Class for testing
#[derive(Default, Debug)]
struct MyClass {
    a: i32,
}

/// Class for testing
#[extendr]
impl MyClass {
    /// Method for making new object.
    fn new() -> Self {
        Self { a: 0 }
    }
    
    /// Method for setting stuff.
    /// @param x a number
    fn set_a(& mut self, x: i32) {
        self.a = x;
    }
    
    /// Method for getting stuff.
    fn a(&self) -> i32 {
        self.a
    }
    
    /// Method for getting one's self.
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
