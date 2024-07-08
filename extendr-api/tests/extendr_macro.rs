use extendr_api::call;
use extendr_api::extendr;
use extendr_api::extendr_module;
use extendr_api::Rinternals;
use extendr_api::Robj;
use extendr_api::NA_INTEGER;
use extendr_api::NA_REAL;
use extendr_api::{prelude::Rint, r, test, GetSexp, Integers};

#[extendr]
fn test_i32(val: i32) -> i32 {
    val
}

#[extendr]
fn test_i16(val: i16) -> i16 {
    val
}

#[extendr]
fn test_option_i32(val: Option<i32>) -> i32 {
    if let Some(i) = val {
        i
    } else {
        -1
    }
}

#[extendr]
fn test_option_f64(val: Option<f64>) -> f64 {
    if let Some(i) = val {
        i
    } else {
        -1.0
    }
}

#[extendr]
fn test_option_i16(val: Option<i16>) -> i16 {
    if let Some(i) = val {
        i
    } else {
        -1
    }
}

#[extendr]
fn test_rint(val: Rint) -> Rint {
    val
}

#[extendr]
fn test_integers(val: Integers) -> Integers {
    val
}

#[extendr(r_name = "test.rename.rlike", mod_name = "test_rename_mymod")]
fn test_rename() {}

extendr_module! {
    mod mymod;
    fn test_rename_mymod;
}

#[extendr]
fn test_integers2(val: Integers) -> Integers {
    val.iter().map(|i| i + 1).collect()
}

#[extendr]
fn test_integers3(val: Integers) -> Rint {
    val.iter().sum()
}

#[test]
fn tests_with_successful_outcomes() {
    unsafe {
        test! {
            // Matching integer.
            assert_eq!(Robj::from_sexp(wrap__test_i32(r!(1).get())), r!(1));

            // i32 takes any numeric.
            assert_eq!(Robj::from_sexp(wrap__test_i32(r!(1.0).get())), r!(1));


            // Matching integer.
            assert_eq!(Robj::from_sexp(wrap__test_option_i32(r!(1).get())), r!(1));

            // Option<i32> takes any numeric.
            assert_eq!(Robj::from_sexp(wrap__test_option_i32(r!(1.0).get())), r!(1));

            // NA input.
            assert_eq!(Robj::from_sexp(wrap__test_option_i32(r!(NA_REAL).get())), r!(-1));

            // NA input.
            assert_eq!(Robj::from_sexp(wrap__test_option_i32(r!(NA_INTEGER).get())), r!(-1));


            // Matching integer.
            assert_eq!(Robj::from_sexp(wrap__test_option_i16(r!(1).get())), r!(1));

            // Option<i16> takes any numeric.
            assert_eq!(Robj::from_sexp(wrap__test_option_i16(r!(1.0).get())), r!(1));

            // NA input.
            assert_eq!(Robj::from_sexp(wrap__test_option_i16(r!(NA_REAL).get())), r!(-1));

            // NA input.
            assert_eq!(Robj::from_sexp(wrap__test_option_i16(r!(NA_INTEGER).get())), r!(-1));


            // Matching integer.
            assert_eq!(Robj::from_sexp(wrap__test_option_f64(r!(1).get())), r!(1.0));

            // Option<f64> takes any numeric.
            assert_eq!(Robj::from_sexp(wrap__test_option_f64(r!(1.0).get())), r!(1.0));

            // NA input.
            assert_eq!(Robj::from_sexp(wrap__test_option_f64(r!(NA_REAL).get())), r!(-1.0));

            // NA input.
            assert_eq!(Robj::from_sexp(wrap__test_option_f64(r!(NA_INTEGER).get())), r!(-1.0));

            // Rint.
            assert_eq!(Robj::from_sexp(wrap__test_rint(r!(1).get())), r!(1));
            assert_eq!(Robj::from_sexp(wrap__test_rint(r!(1.0).get())), r!(1));
            assert_eq!(Robj::from_sexp(wrap__test_rint(r!(NA_INTEGER).get())), r!(NA_INTEGER));

            // Integers
            assert_eq!(Robj::from_sexp(wrap__test_integers(r!([1, 2]).get())), r!([1, 2]));
            assert_eq!(Robj::from_sexp(wrap__test_integers2(r!([1, 2]).get())), r!([2, 3]));
            assert_eq!(Robj::from_sexp(wrap__test_integers3(r!(0..4).get())), r!(6));
        }
    }
}

// This behavior is now handled in Rust nightly 1.81 making catch_r_error() unusable.
// For previous versions this is useful.
// See related: https://rust-lang.zulipchat.com/#narrow/stream/122651-general/topic/default.20PR.20description.20feedback
// TODO: revisit when formalized
// Win32 does not support catch_unwind.
#[cfg(not(target_arch = "x86"))]
#[test]
#[ignore = "panicking in FFI is now automatically abort instead of undefined behavior"]
fn tests_with_unsuccessful_outcomes() {
    // Using [single_threaded] here may help with sporadic test failures.

    use extendr_api::{catch_r_error, list, pairlist};
    extendr_api::single_threaded(|| unsafe {
        test! {
            let old_hook = std::panic::take_hook();

            // Suppress backtrace with a custom hook.
            std::panic::set_hook(Box::new(|_| {
            }));

            // These should throw R errors.
            // They may cause stack traces, but this is harmless.
            assert!(catch_r_error(|| wrap__test_i32(r!("xyz").get())).is_err());
            assert!(catch_r_error(|| wrap__test_i32(r!(pairlist!(x=1)).get())).is_err());
            assert!(catch_r_error(|| wrap__test_i32(r!(list!(1, 2, 3)).get())).is_err());

            assert!(catch_r_error(|| wrap__test_rint(r!([1, 2]).get())).is_err());
            assert!(catch_r_error(|| wrap__test_integers(r!([1.0, 2.0]).get())).is_err());

            assert!(catch_r_error(|| wrap__test_i16(r!(1234567890).get())).is_err());
            std::panic::set_hook(old_hook);
        }
    });
}

#[test]
fn test_call_macro() {
    use extendr_api::Length;
    use extendr_api::Operators;
    test! {
        let vec = call!("c", 1.0, 2.0, 3.0).unwrap();
        assert_eq!(vec, r!([1., 2., 3.]));

        let list = call!("list", a=1, b=2).unwrap();
        assert_eq!(list.len(), 2);

        let three = call!("`+`", 1, 2).unwrap();
        assert_eq!(three, r!(3));
    }
}

#[extendr]
fn test_metadata_1(#[default = "NULL"] val: Robj) -> i32 {
    if val.is_null() {
        1
    } else {
        0
    }
}

#[test]
fn test_metadata() {
    use extendr_api::metadata::Arg;
    use extendr_api::metadata::Func;
    let mut funcs: Vec<Func> = Vec::new();
    meta__test_metadata_1(&mut funcs);

    let args = vec![Arg {
        name: "val",
        arg_type: "Robj",
        default: Some("NULL"),
    }];

    assert_eq!(
        funcs[0],
        Func {
            doc: "",
            rust_name: "test_metadata_1",
            mod_name: "test_metadata_1",
            r_name: "test_metadata_1",
            args,
            return_type: "i32",
            func_ptr: wrap__test_metadata_1 as *const u8,
            hidden: false,
        }
    );
}
