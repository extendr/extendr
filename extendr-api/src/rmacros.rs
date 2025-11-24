//!
//! rmacros - a set of macros to call actual R functions in a rusty way.
//!

/// Convert a rust expression to an R object.
///
/// Shorthand for `Robj::from(x)`.
///
/// Example:
/// ```
/// use extendr_api::prelude::*;
/// test! {
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
/// }
/// ```
#[macro_export]
macro_rules! r {
    ($e: expr) => {
        extendr_api::Robj::from($e)
    };
}

/// Get a local variable from the calling function
/// or a global variable if no such variable exists.
///
/// Variables with embedded "." may not work.
#[macro_export]
macro_rules! var {
    ($($tokens: tt)*) => {{
        local_var(sym!($($tokens)*))
    }};
}

/// Get a global variable.
///
/// Variables with embedded "." may not work.
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
/// use extendr_api::prelude::*;
/// test! {
///
/// let wombat = sym!(wombat);
/// assert_eq!(wombat, r!(Symbol::from_string("wombat")));
/// }
/// ```
#[macro_export]
macro_rules! sym {
    ($($tokens: tt)*) => {
        Robj::from(Symbol::from_string(stringify!($($tokens)*)))
    };
}

/// Create a dataframe.
///
/// Example:
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let mydata = data_frame!(x=1, y=2);
///     assert_eq!(mydata.inherits("data.frame"), true);
///     //assert_eq!(mydata, r!(List::from_pairs(vec![("x", r!(1)), ("y", r!(2))])).set_class(&["data.frame"])?);
/// }
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
/// use extendr_api::prelude::*;
/// test! {
///     let factor = factor!(vec!["abcd", "def", "fg", "fg"]);
///     assert_eq!(factor.levels().unwrap().collect::<Vec<_>>(), vec!["abcd", "def", "fg"]);
///     assert_eq!(factor.as_integer_vector().unwrap(), vec![1, 2, 3, 3]);
///     assert_eq!(factor.as_str_iter().unwrap().collect::<Vec<_>>(), vec!["abcd", "def", "fg", "fg"]);
/// }
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
/// This starts up the underlying [`extendr_engine`] so that interactions with R will work.
/// Additionally, this allows us to use `?` in example code instead of `unwrap()`.
///
/// **Note:** This macro is meant to be used in test code (annotated with
/// `#[cfg(test)]`) or in doc strings. If it is used in library code that
/// gets incorporated into an R package, R CMD check will complain about
/// non-API calls.
///
/// [`extendr_engine`]: https://extendr.github.io/extendr/extendr_engine/
#[macro_export]
macro_rules! test {
    () => {
        test(|| Ok(()))
    };
    ($($rest: tt)*) => {
        {
            use extendr_engine;

            // this helper function must reside in the macro so it doesn't get compiled
            // unless the macro actually gets used (e.g., in testing code)
            fn test<F: FnOnce() -> extendr_api::Result<()>>(f: F) {
                extendr_engine::start_r();
                f().unwrap();
            }

            test(|| {
                $($rest)*
                Ok(())
            })
        }
    };
}
