//! Tests the ability to use R to generate random numbers.
//!
//! The `extendr` attribute macro now has a `use_rng` argument.
//! If this is used, then a `GetRNGstate()` is invoked before the function is called,
//! and a `PutRNGstate()` is invoked after, even if said method panics.
//!
//! These tests has to check two things:
//!
//! * That the seed is set and affects the sampling functions on the rust-side
//! in the same way, as on the r-side.
//! * That the resulting sampled vector is not equal 0, as it would be that
//! if `GetRNGstate()` is not called prior to sampling.
//!
//!
use extendr_api::prelude::*;

#[extendr(use_rng = true)]
fn generate_big_random_vec() -> Vec<f64> {
    let n = 100;
    let param = 10e3;
    (0..n)
        .map(|_| single_threaded(|| unsafe { libR_sys::R_unif_index(param) }))
        .collect()
}

#[test]
fn test_extendr_rng() {
    test! {
        R!(r#"set.seed(20230205)"#).unwrap();
        let x = generate_big_random_vec();
        R!(r#"set.seed(20230205)"#).unwrap();
        let y = generate_big_random_vec();
        assert_eq!(y, x);
        // if the rng state isn't retrieved, then distribution is
        // 0 always
        assert!(
            !x.iter().all(|&x| x == 0f64)
        );
    }
}
