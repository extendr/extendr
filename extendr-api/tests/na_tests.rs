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
        let na_r = Rfloat::new(f64::from_bits(na_bits));
        let na_f64 = f64::from_bits(na_bits);
        assert!(na_r.is_na());
        assert!(na_f64.is_na());
    }
}

#[test]
fn test_float_not_na_is_not_na() {
    test! {
        assert!(!Rfloat::new(42f64).is_na());
        assert!(!Rfloat::new(f64::NAN).is_na());
        assert!(!Rfloat::new(f64::INFINITY).is_na());
        assert!(!Rfloat::new(f64::NEG_INFINITY).is_na());
        assert!(!Rfloat::new(f64::MAX).is_na());
        assert!(!Rfloat::new(f64::MIN).is_na());
        assert!(!Rfloat::new(f64::MIN_POSITIVE).is_na());

        assert!(!42f64.is_na());
        assert!(!f64::NAN.is_na());
        assert!(!f64::INFINITY.is_na());
        assert!(!f64::NEG_INFINITY.is_na());
        assert!(!f64::MAX.is_na());
        assert!(!f64::MIN.is_na());
        assert!(!f64::MIN_POSITIVE.is_na());
    }
}
