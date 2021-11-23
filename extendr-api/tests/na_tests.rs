use extendr_api::prelude::*;

#[test]
fn test_float_na_is_na() {
    test! {
        assert!(Rfloat::na().is_na());
        assert!(f64::na().is_na());
        let na_val = Rfloat::na().inner();
        let na_val_u64 = unsafe {std::mem::transmute::<f64, u64>(na_val)};
        let expected_na_val = 0x7ff00000u64 << 32 | 1954;
        assert_eq!(na_val_u64, expected_na_val);
    }
}