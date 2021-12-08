use extendr_api::prelude::*;
mod submodule;
use submodule::*;

// Return string `"Hello world!"` to R.
#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}

// Do nothing.
#[extendr]
fn do_nothing() {}

// From: input/output conversion

// Atomic types

// Convert a double scalar to itself
// x a number
#[extendr]
fn double_scalar(x: f64) -> f64 {
    x
}

// Convert an int scalar to itself
// x a number
#[extendr]
fn int_scalar(x: i32) -> i32 {
    x
}

// Convert a bool scalar to itself
// x a number
#[extendr]
fn bool_scalar(x: bool) -> bool {
    x
}

// Convert a string to itself
// x a string
#[extendr]
fn char_scalar(x: String) -> String {
    x
}

// Convert a vector of strings to itself
// x a vector of strings
#[extendr]
fn char_vec(x: Vec<String>) -> Vec<String> {
    x
}

// Convert a numeric vector to itself
// x a numeric vector
#[extendr]
fn double_vec(x: Vec<f64>) -> Vec<f64> {
    x
}

// NA-related tests
#[extendr(use_try_from = true)]
fn try_rfloat_na() -> Rfloat {
    Rfloat::na()
}

#[extendr(use_try_from = true)]
fn try_rint_na() -> Rint {
    Rint::na()
}

#[extendr(use_try_from = true)]
fn check_rfloat_na(x : Rfloat) -> bool {
    x.is_na()
}

#[extendr(use_try_from = true)]
fn check_rint_na(x : Rint) -> bool {
    x.is_na()
}

// Non-atomic types
// TODO

// TryFrom: conversions

// Atomic types

// Convert a vector of doubles to itself
// x a vector of doubles
#[extendr(use_try_from = true)]
fn try_double_vec(x: Vec<f64>) -> Vec<f64> {
    x
}

// Non-atomic types
// TODO

// Vector wrappers
#[extendr(use_try_from = true)]
fn get_doubles_element(x: Doubles, i: i32) -> Rfloat {
    x.elt(i as usize)
}

#[extendr(use_try_from = true)]
fn get_integers_element(x: Integers, i: i32) -> Rint {
    x.elt(i as usize)
}

#[extendr(use_try_from = true)]
fn doubles_square(input: Doubles) -> Doubles {
    let mut result = Doubles::new(input.len());

    for (x, y) in result.iter_mut().zip(input.iter()) {
        *x = y * y;
    }

    result
}

#[extendr(use_try_from = true)]
fn integers_square(input: Integers) -> Integers {
    let mut result = Integers::new(input.len());

    for (x, y) in result.iter_mut().zip(input.iter()) {
        *x = y * y;
    }

    result
}

#[extendr(use_try_from = true)]
fn check_default(#[default="NULL"] x: Robj) -> bool {
    x.is_null()
}

// Parsing

// Weird behavior of parameter descriptions:
// first passes tests as is, second -- only in backqutoes.
/// Test whether `_arg` parameters are treated correctly in R
/// Executes \code{`_x` - `_y`}
/// @param _x an integer scalar, ignored
/// @param `_y` an integer scalar, ignored
/// @export
#[extendr]
fn special_param_names(_x: i32, _y: i32) -> i32 {
    _x - _y
}

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
    fn set_a(&mut self, x: i32) {
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
struct __MyClass {}

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

    fn char_vec;
    fn double_vec;

    fn try_double_vec;

    fn get_doubles_element;
    fn get_integers_element;

    fn doubles_square;
    fn integers_square;

    fn check_default;

    fn try_rfloat_na;
    fn try_rint_na;

    fn check_rfloat_na;
    fn check_rint_na;

    fn special_param_names;
    fn __00__special_function_name;

    impl MyClass;
    impl __MyClass;
    impl MyClassUnexported;

    use submodule;
}
