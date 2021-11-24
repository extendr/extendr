use extendr_api::prelude::*;

#[test]
fn test_float_na_is_na() {
    test! {
        assert!(f64::na().is_na());
        assert!(Rfloat::na().is_na());
    }
}

#[test]
fn test_float_from_bits_is_na() {
    test! {
        let na_bits = 0x7ff00000u64 << 32 | 1954;
        let na_r = Rfloat(unsafe {std::mem::transmute(na_bits)});
        let na_f64 : f64 = unsafe {std::mem::transmute(na_bits)};
        assert!(na_r.is_na());
        assert!(na_f64.is_na());
    }
}

// https://github.com/extendr/extendr/issues/321
#[test]
#[cfg(all(windows, target_arch = "x86"))]
fn test_float_na_is_na_ignore_signalling_bit_win_x86() {
    test! {
        let correct_na_bits = 0x7ff00000u64 << 32 | 1954;
        // correct_na_bits & (1u64 << 51)
        let quietened_na_bits = 0x7ff80000u64 << 32 | 1954;
        let correct_na = Rfloat(unsafe {std::mem::transmute(correct_na_bits)});
        let quiet_na = Rfloat(unsafe {std::mem::transmute(quietened_na_bits)});

        assert!(correct_na.is_na());
        assert!(quiet_na.is_na());
    }
}