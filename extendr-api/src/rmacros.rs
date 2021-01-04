//!
//! rmacros - a set of macros to call actual R functions in a rusty way.
//!

/// Convert a rust expression to an R object.
///
/// Shorthand for Robj::from(x).
///
/// Example:
/// ```
/// use extendr_api::*;
/// extendr_engine::start_r();
/// let fred = r!(1);
/// assert_eq!(fred, Robj::from(1));
///
/// let int_array = r!([1, 2, 3]);
/// assert_eq!(int_array.len(), 3);
///
/// let numeric_array = r!([1., 2., 3.]);
/// assert_eq!(numeric_array.len(), 3);
///
/// let logical_array = r!([true, false, true]);
/// assert_eq!(logical_array.len(), 3);
///
/// let numeric_array_with_na = r!([Some(1.), None, Some(3.)]);
/// assert_eq!(numeric_array_with_na.len(), 3);
///
/// ```
#[macro_export]
macro_rules! r {
    ($e: expr) => {
        Robj::from($e)
    };
}

/// Call inline R source code from Rust.
///
/// Multiline expressions and variables with embedded "." may not work.
/// Note that the performance of this function is not good.
///
/// ```
/// use extendr_api::*;
/// extendr_engine::start_r();
///
/// let formula = R!(y ~ z + 1).unwrap();
/// assert!(formula.inherits("formula"));
///
/// let function = R!(function(x,y) x+y).unwrap();
/// assert!(function.is_function());
/// ```
#[macro_export]
macro_rules! R {
    ($($t:tt)*) => {
        Robj::eval_string(stringify!($($t)*))
    };
}

/// Get a local variable from the calling function
/// or a global variable if no such variable exists.
///
/// Variables with embedded "." may not work.
/// ```
/// use extendr_api::*;
/// extendr_engine::start_r();
///
/// current_env().set_local(sym!(myvar), 1.0);
/// assert_eq!(var!(myvar), Ok(r!(1.0)));
/// ```
#[macro_export]
macro_rules! var {
    ($($tokens: tt)*) => {{
        local_var(sym!($($tokens)*))
    }};
}

/// Get a global variable.
///
/// Variables with embedded "." may not work.
/// ```
/// use extendr_api::*;
/// test! {
///      // The "iris" dataset is a dataframe.
///      assert_eq!(global!(iris)?.is_frame(), true);
/// }
/// ```
#[macro_export]
macro_rules! global {
    ($($tokens: tt)*) => {{
        global_var(sym!($($tokens)*))
    }};
}

/// The sym! macro install symbols.
/// You should cache your symbols in variables
/// as generating them is costly.
/// ```
/// use extendr_api::*;
/// extendr_engine::start_r();
///
/// let wombat = sym!(wombat);
/// assert_eq!(wombat, r!(Symbol("wombat")));
/// ```
#[macro_export]
macro_rules! sym {
    ($($tokens: tt)*) => {
        Robj::from(Symbol(stringify!($($tokens)*)))
    };
}

/// Concatenation operator.
///
/// Example:
/// ```
/// use extendr_api::*;
/// extendr_engine::start_r();
/// let fred = c!(1, 2, 3);
/// assert_eq!(fred, r!([1, 2, 3]));
/// ```
/// Note: make sure to use rust syntax for numbers: 1 is integer, 1. is numeric.
/// For vectors of primitives, prefer to use `r!([1, 2, 3])`.
///
/// Panics on error.
#[macro_export]
macro_rules! c {
    () => {
        call!("c").unwrap()
    };
    ($($rest: tt)*) => {
        call!("c", $($rest)*).unwrap()
    };
}

/// Create a vector with repeating elements.
///
/// Example:
/// ```
/// use extendr_api::*;
/// extendr_engine::start_r();
/// let fred = rep!(1., 3);
/// assert_eq!(fred, r!([1., 1., 1.]));
/// ```
/// Note: make sure to use rust syntax for numbers: 1 is integer, 1. is numeric.
#[macro_export]
macro_rules! rep {
    ($($rest: tt)*) => {
        call!("rep", $($rest)*).unwrap()
    };
}

/// Read a CSV file.
///
/// Example:
/// ```no_run
/// use extendr_api::*;
/// extendr_engine::start_r();
/// let mydata = read_table!("mydata.csv").unwrap();
/// ```
#[macro_export]
macro_rules! read_table {
    ($($rest: tt)*) => {
        call!("read.table", $($rest)*)
    };
}

/// Create a list.
///
/// Example:
/// ```
/// use extendr_api::*;
/// extendr_engine::start_r();
/// let mylist = list!(x=1, y=2);
/// assert_eq!(mylist, r!(List(&[r!(1), r!(2)])));
/// ```
///
/// Panics on error.
#[macro_export]
macro_rules! list {
    () => {
        call!("list").unwrap()
    };
    ($($rest: tt)*) => {
        call!("list", $($rest)*).unwrap()
    };
}

/// Create a dataframe.
///
/// Example:
/// ```
/// use extendr_api::*;
/// extendr_engine::start_r();
/// let mydata = data_frame!(x=1, y=2);
/// assert_eq!(mydata, r!(List(&[r!(1), r!(2)])));
/// ```
///
/// Panics on error.
#[macro_export]
macro_rules! data_frame {
    () => {
        call!("data.frame").unwrap()
    };
    ($($rest: tt)*) => {
        call!("data.frame", $($rest)*).unwrap()
    };
}

/// Create a factor.
///
/// Example:
/// ```
/// use extendr_api::*;
/// extendr_engine::start_r();
/// let factor = factor!(vec!["abcd", "def", "fg", "fg"]);
/// assert_eq!(factor.levels().unwrap().collect::<Vec<_>>(), vec!["abcd", "def", "fg"]);
/// assert_eq!(factor.as_integer_vector().unwrap(), vec![1, 2, 3, 3]);
/// assert_eq!(factor.as_str_iter().unwrap().collect::<Vec<_>>(), vec!["abcd", "def", "fg", "fg"]);
/// ```
///
/// Panics on error.
#[macro_export]
macro_rules! factor {
    ($($rest: tt)*) => {
        call!("factor", $($rest)*).unwrap()
    };
}

/// Print via the R output stream.
///
/// Works like [`print!`] but integrates with R and respects
/// redirection with functions like `sink()` and `capture.output()`
#[macro_export]
macro_rules! rprint {
    () => {
    };
    ($($rest: tt)*) => {
        print_r_output(format!($($rest)*));
    };
}

/// Print with a newline via the R output stream.
///
/// Works like [`println!`] but integrates with R and respects
/// redirection with functions like `sink()` and `capture.output()`
#[macro_export]
macro_rules! rprintln {
    () => {
        print_r_output("\n");
    };
    ($($rest: tt)*) => {
        print_r_output(format!($($rest)*));
        print_r_output("\n");
    };
}

/// Print via the R error stream.
#[macro_export]
macro_rules! reprint {
    () => {
    };
    ($($rest: tt)*) => {
        print_r_error(format!($($rest)*));
    };
}

/// Print with a newline via the R output stream.
#[macro_export]
macro_rules! reprintln {
    () => {
        print_r_error("\n");
    };
    ($($rest: tt)*) => {
        print_r_error(format!($($rest)*));
        print_r_error("\n");
    };
}

/// Macro for running tests.
///
/// This allows us to use ? in example code instead of unwrap.
#[macro_export]
macro_rules! test {
    () => {
        test(|| Ok(()))
    };
    ($($rest: tt)*) => {
        test(|| {
            $($rest)*
            Ok(())
        })
    };
}
