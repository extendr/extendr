//! This test module contains tests for [RArray], [RMatrix], etc.
//!
use extendr_api::prelude::*;
use extendr_engine::with_r;

#[test]
fn flat_index_test() {
    with_r(|| {
        let example = R!("matrix(as.numeric(1:9), byrow=TRUE, ncol = 3, nrow = 3)").unwrap();
        let test_array = RArray::<f64, ()>::try_from(&example).unwrap();
        let test_matrix = RMatrix::<f64>::try_from(&example).unwrap();
        // 1 2 3
        // 4 5 6
        // 7 8 9

        assert_eq!(test_array[3], 2.);
        assert_eq!(test_matrix[[0, 1]], test_array[3]);
    })
}
