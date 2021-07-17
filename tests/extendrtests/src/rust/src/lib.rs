use extendr_api::prelude::*;
mod submodule;
use submodule::*;
use std::collections::HashMap;

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

// Conversions

// From
// functions to test input/output conversion
// atomic types
// TODO Vec<bool>, HashMap<String, Robj>

/// Convert a double scalar to itself
/// @param x a number
/// @export
#[extendr]
fn double_scalar(x: f64) -> f64 { x }

/// Convert an int scalar to itself
/// @param x an integer
/// @export
#[extendr]
fn int_scalar(x: i32) -> i32 { x }

/// Convert a bool scalar to itself
/// @param x a logical
/// @export
#[extendr]
fn bool_scalar(x: bool) -> bool { x }

/// Convert a length-one character type in R to String and back
/// @param x a length-one character type in R
/// @export
#[extendr]
fn char_scalar(x: String) -> String { x }

/// Convert a numeric vector to itself
/// @param x a numeric vector
/// @export
#[extendr]
fn double_vec(x: Vec<f64>) -> Vec<f64> {x}

/// Convert an integer vector to itself
/// @param x an integer vector
/// @export
#[extendr]
fn int_vec(x: Vec<i32>) -> Vec<i32> {x}

/// Convert a character vector in R to Vec<String> and back
/// @param x a character vector
/// @export
#[extendr]
fn char_string_vec(x: Vec<String>) -> Vec<String> {x}

/// Convert a character vector in R to Vec<String> and back
/// @param x a character vector
/// @export
#[extendr]
fn char_str_vec() {
    todo!("FromRobj not found for Vec<&str>")
}

/// Convert a logical vector to itself
/// @param x a logical vector
/// @export
#[extendr]
fn bool_vec() {
    todo!("FromRobj not found for Vec<bool>")
}


// From slices, matrices and arrays

/// Convert a numeric vector to itself
/// @param x a numeric vector
/// @export
#[extendr]
fn double_slice(x: &[f64]) -> &[f64] {x}

/// Convert a numeric matrix RArray RArray<&[f64], [usize;2]> and back
/// @param x a numeric matrix
/// @export
#[extendr]
fn double_mat() {
    todo!("Robj in RArray<T, D> doesn't satisfy AsTypedSlice, in extendr-api/src/wrapper/matrix.rs")
}

// Non-atomic types

/// Convert a list to a HashMap<&str, Robj> in rust and back. Does not preserve list order
/// @param x a list
/// @export
#[extendr]
fn list_str_hash(x: HashMap<&str, Robj>) -> HashMap<&str, Robj> {x}

/// Convert a list to a HashMap<String, Robj> in rust and back. Does not preserve list order
/// @param x a list
/// @export
#[extendr]
fn list_string_hash() {
    // into_robj.rs missing From<HashMap<String, Robj>> for Robj
    todo!("trait bound `HashMap<String, Robj>: From<HashMap<String, Robj>>` is not satisfied")
}


// TryFrom
// functions to test input/output conversion
// atomic types

/// Convert a double scalar to itself
/// @param x a number
/// @export
#[extendr(use_try_from = true)]
fn try_double_scalar(x: f64) -> f64 { x }

/// Convert an int scalar to itself
/// @param x a number
/// @export
#[extendr(use_try_from = true)]
fn try_int_scalar(x: i32) -> i32 { x }

/// Convert a bool scalar to itself
/// @param x a number
/// @export
#[extendr(use_try_from = true)]
fn try_bool_scalar(x: bool) -> bool { x }

/// Convert a string to itself
/// @param x a string
/// @export
#[extendr(use_try_from = true)]
fn try_char_scalar(x: String) -> String { x }

/// Convert a vector of doubles to itself
/// @param x a vector of doubles
/// @export
#[extendr(use_try_from = true)]
fn try_double_vec(x: Vec<f64>) -> Vec<f64> {x}

/// Convert a vector of ints to itself
/// @param x a vector of ints
/// @export
#[extendr(use_try_from = true)]
fn try_int_vec(x: Vec<i32>) -> Vec<i32> {x}

/// Convert a vector of strings to itself
/// @param x a vector of strings
/// @export
#[extendr(use_try_from = true)]
fn try_char_vec(x: Vec<String>) -> Vec<String> {x}

/// Convert a logical vector to itself
/// @param x a logical vector
/// @export
#[extendr(use_try_from = true)]
fn try_bool_vec() {
    todo!("`From<Robj>` is not implemented for `Vec<bool>`")
}


// Non-atomic types

/// Convert a list to a HashMap<&str, Robj> in rust and back. Does not preserve list order
/// @param x a list
/// @export
#[extendr(use_try_from = true)]
fn try_list_str_hash(x: HashMap<&str, Robj>) -> HashMap<&str, Robj> {x}

/// Convert a list to a HashMap<String, Robj> in rust and back. Does not preserve list order
/// @param x a list
/// @export
#[extendr(use_try_from = true)]
fn try_list_string_hash() {
    todo!("trait bound `HashMap<String, Robj >: From<HashMap<String, Robj>>` is not satisfied")
}



// Weird behavior of parameter descriptions:
// first passes tests as is, second -- only in backqutoes.

/// Test whether `_arg` parameters are treated correctly in R
/// Executes \code{`_x` - `_y`}
/// @param _x an integer scalar, ignored
/// @param `_y` an integer scalar, ignored
/// @export
#[extendr]
fn special_param_names(_x : i32, _y : i32) -> i32 { _x - _y }

/// Test wrapping of special function name
/// @name f__00__special_function_name
/// @export
#[extendr]
#[allow(non_snake_case)]
fn __00__special_function_name() {}

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

// Class for testing special names
#[derive(Default, Debug)]
struct __MyClass {
}

// Class for testing special names
// Unexported because of documentation conflict
#[extendr]
impl __MyClass {
    /// Method for making a new object.
    fn new() -> Self {
        Self {}
    }
    /// Method with special name unsupported by R
    fn __name_test(&self) {}
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
    
    fn double_vec;
    fn int_vec;
    fn char_string_vec;
    fn char_str_vec;
    fn bool_vec;
    
    fn double_slice;
    fn double_mat;

    fn list_str_hash;
    fn list_string_hash;

    fn try_double_scalar;
    fn try_int_scalar;
    fn try_bool_scalar;
    fn try_char_scalar;
    
    fn try_double_vec;
    fn try_int_vec;   
    fn try_char_vec;
    fn try_bool_vec;

    fn try_list_str_hash;
    fn try_list_string_hash;

    fn special_param_names;
    fn __00__special_function_name;

    impl MyClass;
    impl __MyClass;
    impl MyClassUnexported;
    
    use submodule;
}
