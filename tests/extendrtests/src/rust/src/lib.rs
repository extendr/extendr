use extendr_api::{graphics::*, prelude::*};

mod submodule;

mod optional_ndarray;

mod graphic_device;

mod optional_either;

mod raw_identifiers;

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
fn check_rfloat_na(x: Rfloat) -> bool {
    x.is_na()
}

#[extendr(use_try_from = true)]
fn check_rint_na(x: Rint) -> bool {
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
fn get_logicals_element(x: Logicals, i: i32) -> Rbool {
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
fn complexes_square(input: Complexes) -> Complexes {
    let mut result = Complexes::new(input.len());

    for (x, y) in result.iter_mut().zip(input.iter()) {
        *x = Rcplx::from((y.re() * y.re(), 0.0.into()));
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
fn logicals_not(input: Logicals) -> Logicals {
    let mut result = Logicals::new(input.len());

    for (x, y) in result.iter_mut().zip(input.iter()) {
        *x = !y;
    }

    result
}

// Parsing

#[extendr(use_try_from = true)]
fn check_default(#[default = "NULL"] x: Robj) -> bool {
    x.is_null()
}

// Weird behavior of parameter descriptions:
// first passes tests as is, second -- only in backquotes.
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

#[extendr(
    use_try_from = true,
    r_name = "test.rename.rlike",
    mod_name = "test_rename_mymod"
)]
fn test_rename() -> i32 {
    1
}

#[extendr]
fn get_default_value(#[default = "42"] x: i32) -> i32 {
    x
}

#[extendr(use_try_from = true)]
fn add_5_if_not_null(x: Nullable<Rint>) -> Nullable<Rint> {
    x.map(|y| y + 5)
}

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

    // https://github.com/extendr/extendr/issues/431
    fn restore_from_robj(robj: Robj) -> Self {
        let res: ExternalPtr<MyClass> = robj.try_into().unwrap();
        Self { a: res.a }
    }

    // https://github.com/extendr/extendr/issues/435
    fn get_default_value(#[default = "42"] x: i32) -> i32 {
        x
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

/// Create a new device.
///
/// @param welcome_message A warm message to welcome you.
/// @export
#[extendr]
fn my_device(welcome_message: String) {
    let device_driver = graphic_device::MyDevice {
        welcome_message: welcome_message.as_str(),
    };

    let device_descriptor = DeviceDescriptor::new();
    device_driver.create_device::<graphic_device::MyDevice>(device_descriptor, "my device");
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
    fn get_logicals_element;

    fn doubles_square;
    fn complexes_square;
    fn integers_square;
    fn logicals_not;

    fn check_default;

    fn try_rfloat_na;
    fn try_rint_na;

    fn check_rfloat_na;
    fn check_rint_na;

    fn special_param_names;
    fn __00__special_function_name;

    // Note that this uses an alternative name.
    fn test_rename_mymod;

    fn get_default_value;

    fn add_5_if_not_null;

    impl MyClass;
    impl __MyClass;
    impl MyClassUnexported;

    fn my_device;

    use submodule;
    use optional_ndarray;
    use optional_either;
    use raw_identifiers;
}
