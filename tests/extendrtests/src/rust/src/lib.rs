use extendr_api::prelude::*;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}

/// Do nothing.
/// @export
#[extendr]
fn do_nothing() {
}


// functions to test input/output conversion

/// Convert a double scalar to itself
/// @param x a number
/// @export
#[extendr]
fn double_scalar(x: f64) -> f64 { x }

/// Convert an int scalar to itself
/// @param x a number
/// @export
#[extendr]
fn int_scalar(x: i32) -> i32 { x }

/// Convert a bool scalar to itself
/// @param x a number
/// @export
#[extendr]
fn bool_scalar(x: bool) -> bool { x }

/// Convert a string to itself
/// @param x a string
/// @export
#[extendr]
fn char_scalar(x: String) -> String { x }

/// Convert a vector of strings to itself
/// @param x a vector of strings
/// @export
#[extendr]
fn char_vec(x: Vec<String>) -> Vec<String> {x}

// Class for testing
#[derive(Default, Debug)]
struct MyClass {
    a: i32,
}

/// Class for testing (exported)
/// @examples
/// x <- MyClass$new()
/// x$a()
/// x$set_a(10)
/// x$a()
/// @export
#[extendr]
impl MyClass {
    /// Method for making a new object.
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

// Class for testing (unexported)
#[derive(Default, Debug)]
struct MyClassUnexported {
    a: i32,
}

/// Class for testing (unexported)
#[extendr]
impl MyClassUnexported {
    /// Method for making a new object.
    fn new() -> Self {
        Self { a: 22 }
    }
    
    /// Method for getting stuff.
    fn a(&self) -> i32 {
        self.a
    }
}


// Macro to generate exports
extendr_module! {
    mod extendrtests;
    fn hello_world;
    fn do_nothing;

    fn double_scalar;
    fn int_scalar;
    fn bool_scalar;
    fn char_scalar;
    fn char_vec;
    
    impl MyClass;
    impl MyClassUnexported;
}
