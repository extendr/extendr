use extendr_api::prelude::*;

#[extendr(use_try_from = true)]
fn test_i32(val: i32) -> i32 {
    val
}

#[extendr(use_try_from = true)]
fn test_i16(val: i16) -> i16 {
    val
}

#[extendr(use_try_from = true)]
fn test_option_i32(val: Option<i32>) -> i32 {
    if let Some(i) = val {
        i
    } else {
        -1
    }
}

#[extendr(use_try_from = true)]
fn test_option_f64(val: Option<f64>) -> f64 {
    if let Some(i) = val {
        i
    } else {
        -1.0
    }
}

#[extendr(use_try_from = true)]
fn test_option_i16(val: Option<i16>) -> i16 {
    if let Some(i) = val {
        i
    } else {
        -1
    }
}

#[test]
fn tests_with_successful_outcomes() {
    unsafe {
        test! {
            // Matching integer.
            assert_eq!(new_owned(wrap__test_i32(r!(1).get())), r!(1));

            // i32 takes any numeric.
            assert_eq!(new_owned(wrap__test_i32(r!(1.0).get())), r!(1));


            // Matching integer.
            assert_eq!(new_owned(wrap__test_option_i32(r!(1).get())), r!(1));

            // Option<i32> takes any numeric.
            assert_eq!(new_owned(wrap__test_option_i32(r!(1.0).get())), r!(1));

            // NA input.
            assert_eq!(new_owned(wrap__test_option_i32(r!(NA_REAL).get())), r!(-1));

            // NA input.
            assert_eq!(new_owned(wrap__test_option_i32(r!(NA_INTEGER).get())), r!(-1));


            // Matching integer.
            assert_eq!(new_owned(wrap__test_option_i16(r!(1).get())), r!(1));

            // Option<i16> takes any numeric.
            assert_eq!(new_owned(wrap__test_option_i16(r!(1.0).get())), r!(1));

            // NA input.
            assert_eq!(new_owned(wrap__test_option_i16(r!(NA_REAL).get())), r!(-1));

            // NA input.
            assert_eq!(new_owned(wrap__test_option_i16(r!(NA_INTEGER).get())), r!(-1));


            // Matching integer.
            assert_eq!(new_owned(wrap__test_option_f64(r!(1).get())), r!(1.0));

            // Option<f64> takes any numeric.
            assert_eq!(new_owned(wrap__test_option_f64(r!(1.0).get())), r!(1.0));

            // NA input.
            assert_eq!(new_owned(wrap__test_option_f64(r!(NA_REAL).get())), r!(-1.0));

            // NA input.
            assert_eq!(new_owned(wrap__test_option_f64(r!(NA_INTEGER).get())), r!(-1.0));
        }
    }
}

// Win32 does not support catch_unwind.
#[cfg(not(target_arch = "x86"))]
#[test]
fn tests_with_unsuccessful_outcomes() {
    unsafe {
        test! {
            // These should throw R errors.
            // They may cause stack traces, but this is harmless.
            assert!(catch_r_error(|| wrap__test_i32(r!("xyz").get())).is_err());
            assert!(catch_r_error(|| wrap__test_i32(r!(pairlist!(x=1)).get())).is_err());
            assert!(catch_r_error(|| wrap__test_i32(r!(list!(1, 2, 3)).get())).is_err());

            // TODO: check for overflow.
            // assert!(catch_r_error(|| wrap__test_i16(r!(1234567890).get())).is_err());
        }
    }
}

#[test]
fn test_call_macro() {
    test! {
        let vec = call!("c", 1.0, 2.0, 3.0).unwrap();
        assert_eq!(vec, r!([1., 2., 3.]));

        let list = call!("list", a=1, b=2).unwrap();
        assert_eq!(list.len(), 2);

        let three = call!("`+`", 1, 2).unwrap();
        assert_eq!(three, r!(3));
    }
}
