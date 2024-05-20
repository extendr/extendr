#[cfg(feature = "ndarray")]
mod ndarray_try_from_tests {
    use extendr_api::prelude::*;

    #[test]
    fn test_1d_view_from_robj() {
        test! {
            let robj = R!("c(1L, 2L, 3L, 4L, 5L)")?;
            let view = <ArrayView1<Rint>>::try_from(robj)?;
            assert_eq!(view.dim(), 5);
        }
    }

    #[test]
    fn integer_matrix_2d() {
        test! {
             let robj = R!("matrix(c(1L, 2L, 3L, 4L, 5L, 6L, 7L, 8L), ncol = 2, nrow = 4, byrow = TRUE)")?;

             let view = <ArrayView2<Rint>>::try_from(&robj)?;
             assert_eq!(view.dim(), (4, 2));

             for (&mapped, expected) in view.iter().zip(1..=8) {
                 assert_eq!(mapped, expected);
             }

             let view = <ArrayView2<i32>>::try_from(&robj)?;
             assert_eq!(view.dim(), (4, 2));

             for (&mapped, expected) in view.iter().zip(1..=8) {
                 assert_eq!(mapped, expected);
             }

             for (&mapped, expected) in view.iter().zip(1..=8) {
                 assert_eq!(mapped, expected);
             }
        }
    }

    #[test]
    fn real_matrix_2d() {
        test! {
             let robj = R!("matrix(c(1, 2, 3, 4, 5, 6, 7, 8), ncol = 2, nrow = 4, byrow = TRUE)")?;

             let view = <ArrayView2<Rfloat>>::try_from(&robj)?;
             assert_eq!(view.dim(), (4, 2));
             for (&mapped, expected) in view.iter().zip(1..=8) {
                 assert_eq!(mapped, expected as f64);
             }

             let view = <ArrayView2<f64>>::try_from(&robj)?;
             assert_eq!(view.dim(), (4, 2));
             for (&mapped, expected) in view.iter().zip(1..=8) {
                 assert_eq!(mapped, expected as f64);
             }
        }
    }

    #[test]
    fn logical_matrix_2d() {
        test! {
             let robj = R!("matrix(c(TRUE, FALSE, TRUE, FALSE, FALSE, TRUE), ncol = 2, nrow = 3, byrow = TRUE)")?;

             let view = <ArrayView2<Rbool>>::try_from(&robj)?;
             assert_eq!(view.dim(), (3, 2));
             for (&mapped, expected) in view.iter().zip(vec![true, false, true, false, false, true]) {
                 assert_eq!(mapped, expected);
             }
        }
    }

    #[test]
    fn complex_matrix_2d() {
        test! {
             let robj = R!("matrix(c(1, 2, 3i, 4i, 5 + 5i, 6 - 6i), ncol = 2, nrow = 3, byrow = TRUE)")?;

             let expected = vec![1, 2, 0, 0, 5, 6].iter().zip(vec![0, 0, 3, 4, 5, -6]).map(|(&re, im)| <c64>::new(re as f64, im as f64))
             .collect::<Vec<_>>();

             let view = <ArrayView2<Rcplx>>::try_from(&robj)?;
             assert_eq!(view.dim(), (3, 2));

             for (&mapped, &expected) in view.iter().zip(expected.iter()) {
                 assert_eq!(mapped, <Rcplx>::from(expected));
             }

             let view = <ArrayView2<c64>>::try_from(&robj)?;
             assert_eq!(view.dim(), (3, 2));
             for (&mapped, &expected) in view.iter().zip(expected.iter()) {
                 assert_eq!(mapped, expected);
             }
        }
    }

    #[test]
    fn character_matrix_2d() {
        test! {
             let robj = R!("matrix(c(\"Hello\", \"World\"), ncol = 1, nrow = 2, byrow = TRUE)")?;

             let view = <ArrayView2<Rstr>>::try_from(&robj)?;
             assert_eq!(view.dim(), (2, 1));
             for (mapped, expected) in view.iter().zip(vec!["Hello", "World"]) {
                 assert_eq!(mapped, expected);
             }
        }
    }

    #[test]
    fn integer_matrix_1d() {
        test! {
             let robj = R!("matrix(c(1L, 2L, 3L, 4L, 5L, 6L, 7L, 8L), ncol = 1, nrow = 8)")?;

             let view = <ArrayView1<Rint>>::try_from(&robj)?;
             assert_eq!(view.dim(), 8);
             for (&mapped, expected) in view.iter().zip(1..=8) {
                 assert_eq!(mapped, expected);
             }

             let view = <ArrayView1<i32>>::try_from(&robj)?;
             assert_eq!(view.dim(), 8);
             for (&mapped, expected) in view.iter().zip(1..=8) {
                 assert_eq!(mapped, expected);
             }
        }
    }

    #[test]
    fn real_matrix_1d() {
        test! {
             let robj = R!("matrix(c(1, 2, 3, 4, 5, 6, 7, 8), ncol = 1, nrow = 8)")?;

             let view = <ArrayView1<Rfloat>>::try_from(&robj)?;
             assert_eq!(view.dim(), 8);
             for (&mapped, expected) in view.iter().zip(1..=8) {
                 assert_eq!(mapped, expected as f64);
             }

             let view = <ArrayView1<f64>>::try_from(&robj)?;
             assert_eq!(view.dim(), 8);
             for (&mapped, expected) in view.iter().zip(1..=8) {
                 assert_eq!(mapped, expected as f64);
             }
        }
    }

    #[test]
    fn logical_matrix_1d() {
        test! {
             let robj = R!("matrix(c(TRUE, FALSE, TRUE, FALSE, FALSE, TRUE), ncol = 1, nrow = 6)")?;

             let view = <ArrayView1<Rbool>>::try_from(&robj)?;
             assert_eq!(view.dim(), 6);
             for (&mapped, expected) in view.iter().zip(vec![true, false, true, false, false, true]) {
                 assert_eq!(mapped, expected);
             }
        }
    }

    #[test]
    fn complex_matrix_1d() {
        test! {
             let robj = R!("matrix(c(1, 2, 3i, 4i, 5 + 5i, 6 - 6i), ncol = 1, nrow = 6)")?;

             let expected = vec![1, 2, 0, 0, 5, 6].iter().zip(vec![0, 0, 3, 4, 5, -6]).map(|(&re, im)| <c64>::new(re as f64, im as f64))
             .collect::<Vec<_>>();

             let view = <ArrayView1<Rcplx>>::try_from(&robj)?;
             assert_eq!(view.dim(), 6);
             for (&mapped, &expected) in view.iter().zip(expected.iter()) {
                 assert_eq!(mapped, <Rcplx>::from(expected));
             }

             let view = <ArrayView1<c64>>::try_from(&robj)?;
             assert_eq!(view.dim(), 6);
             for (&mapped, &expected) in view.iter().zip(expected.iter()) {
                 assert_eq!(mapped, expected);
             }
        }
    }

    #[test]
    fn character_matrix_1d() {
        test! {
             let robj = R!("matrix(c(\"Hello\", \"World\"), ncol = 1, nrow = 2)")?;

             let view = <ArrayView1<Rstr>>::try_from(&robj)?;
             assert_eq!(view.dim(), 2);
             for (mapped, expected) in view.iter().zip(vec!["Hello", "World"]) {
                 assert_eq!(mapped, expected);
             }
        }
    }

    #[test]
    fn integer_matrix_1d_type_mismatch() {
        test! {
             let robj = R!("matrix(c(1L, 2L, 3L, 4L, 5L, 6L, 7L, 8L), ncol = 1, nrow = 8)")?;

             assert!(<ArrayView1<Rfloat>>::try_from(&robj).is_err());
             assert!(<ArrayView1<f64>>::try_from(&robj).is_err());

             assert!(<ArrayView1<Rcplx>>::try_from(&robj).is_err());
             assert!(<ArrayView1<c64>>::try_from(&robj).is_err());

             assert!(<ArrayView1<Rbool>>::try_from(&robj).is_err());
             assert!(<ArrayView1<Rstr>>::try_from(&robj).is_err());
        }
    }

    #[test]
    fn real_matrix_1d_type_mismatch() {
        test! {
            let robj = R!("matrix(c(1, 2, 3, 4, 5, 6, 7, 8), ncol = 1, nrow = 8)")?;

             assert!(<ArrayView1<Rint>>::try_from(&robj).is_err());
             assert!(<ArrayView1<i32>>::try_from(&robj).is_err());

             assert!(<ArrayView1<Rcplx>>::try_from(&robj).is_err());
             assert!(<ArrayView1<c64>>::try_from(&robj).is_err());

             assert!(<ArrayView1<Rbool>>::try_from(&robj).is_err());
             assert!(<ArrayView1<Rstr>>::try_from(&robj).is_err());
        }
    }

    #[test]
    fn logical_matrix_1d_type_mismatch() {
        test! {
             let robj = R!("matrix(c(TRUE, FALSE, TRUE, FALSE, FALSE, TRUE), ncol = 1, nrow = 6)")?;

             assert!(<ArrayView1<Rint>>::try_from(&robj).is_err());
             assert!(<ArrayView1<i32>>::try_from(&robj).is_err());

             assert!(<ArrayView1<Rfloat>>::try_from(&robj).is_err());
             assert!(<ArrayView1<f64>>::try_from(&robj).is_err());

             assert!(<ArrayView1<Rcplx>>::try_from(&robj).is_err());
             assert!(<ArrayView1<c64>>::try_from(&robj).is_err());

             assert!(<ArrayView1<Rstr>>::try_from(&robj).is_err());
        }
    }

    #[test]
    fn complex_matrix_1d_type_mismatch() {
        test! {
             let robj = R!("matrix(c(1, 2, 3i, 4i, 5 + 5i, 6 - 6i), ncol = 1, nrow = 6)")?;

             assert!(<ArrayView1<Rint>>::try_from(&robj).is_err());
             assert!(<ArrayView1<i32>>::try_from(&robj).is_err());

             assert!(<ArrayView1<Rfloat>>::try_from(&robj).is_err());
             assert!(<ArrayView1<f64>>::try_from(&robj).is_err());

             assert!(<ArrayView1<Rstr>>::try_from(&robj).is_err());
             assert!(<ArrayView1<Rbool>>::try_from(&robj).is_err());
        }
    }

    #[test]
    fn character_matrix_1d_type_mismatch() {
        test! {
             let robj = R!("matrix(c(\"Hello\", \"World\"), ncol = 1, nrow = 2)")?;

             assert!(<ArrayView1<Rint>>::try_from(&robj).is_err());
             assert!(<ArrayView1<i32>>::try_from(&robj).is_err());

             assert!(<ArrayView1<Rfloat>>::try_from(&robj).is_err());
             assert!(<ArrayView1<f64>>::try_from(&robj).is_err());

             assert!(<ArrayView1<Rcplx>>::try_from(&robj).is_err());
             assert!(<ArrayView1<c64>>::try_from(&robj).is_err());

             assert!(<ArrayView1<Rbool>>::try_from(&robj).is_err());
        }
    }

    #[test]
    fn integer_matrix_2d_type_mismatch() {
        test! {
             let robj = R!("matrix(c(1L, 2L, 3L, 4L, 5L, 6L, 7L, 8L), ncol = 2, nrow = 4, byrow = TRUE)")?;

             assert!(<ArrayView2<Rfloat>>::try_from(&robj).is_err());
             assert!(<ArrayView2<f64>>::try_from(&robj).is_err());

             assert!(<ArrayView2<Rcplx>>::try_from(&robj).is_err());
             assert!(<ArrayView2<c64>>::try_from(&robj).is_err());

             assert!(<ArrayView1<Rbool>>::try_from(&robj).is_err());
             assert!(<ArrayView1<Rstr>>::try_from(&robj).is_err());
        }
    }

    #[test]
    fn real_matrix_2d_type_mismatch() {
        test! {
             let robj = R!("matrix(c(1, 2, 3, 4, 5, 6, 7, 8), ncol = 2, nrow = 4, byrow = TRUE)")?;

             assert!(<ArrayView2<Rint>>::try_from(&robj).is_err());
             assert!(<ArrayView2<i32>>::try_from(&robj).is_err());

             assert!(<ArrayView2<Rcplx>>::try_from(&robj).is_err());
             assert!(<ArrayView2<c64>>::try_from(&robj).is_err());

             assert!(<ArrayView1<Rbool>>::try_from(&robj).is_err());
             assert!(<ArrayView1<Rstr>>::try_from(&robj).is_err());
        }
    }

    #[test]
    fn logical_matrix_2d_type_mismatch() {
        test! {
             let robj = R!("matrix(c(TRUE, FALSE, TRUE, FALSE, FALSE, TRUE), ncol = 2, nrow = 3, byrow = TRUE)")?;

             assert!(<ArrayView2<Rint>>::try_from(&robj).is_err());
             assert!(<ArrayView2<i32>>::try_from(&robj).is_err());

             assert!(<ArrayView2<Rfloat>>::try_from(&robj).is_err());
             assert!(<ArrayView2<f64>>::try_from(&robj).is_err());

             assert!(<ArrayView2<Rcplx>>::try_from(&robj).is_err());
             assert!(<ArrayView2<c64>>::try_from(&robj).is_err());

             assert!(<ArrayView1<Rstr>>::try_from(&robj).is_err());
        }
    }

    #[test]
    fn complex_matrix_2d_type_mismatch() {
        test! {
             let robj = R!("matrix(c(1, 2, 3i, 4i, 5 + 5i, 6 - 6i), ncol = 2, nrow = 3, byrow = TRUE)")?;

             assert!(<ArrayView2<Rint>>::try_from(&robj).is_err());
             assert!(<ArrayView2<i32>>::try_from(&robj).is_err());

             assert!(<ArrayView2<Rfloat>>::try_from(&robj).is_err());
             assert!(<ArrayView2<f64>>::try_from(&robj).is_err());

             assert!(<ArrayView1<Rstr>>::try_from(&robj).is_err());
             assert!(<ArrayView1<Rbool>>::try_from(&robj).is_err());
        }
    }

    #[test]
    fn character_matrix_2d_type_mismatch() {
        test! {
             let robj = R!("matrix(c(\"Hello\", \"World\"), ncol = 1, nrow = 2, byrow = TRUE)")?;

             assert!(<ArrayView2<Rint>>::try_from(&robj).is_err());
             assert!(<ArrayView2<i32>>::try_from(&robj).is_err());

             assert!(<ArrayView2<Rfloat>>::try_from(&robj).is_err());
             assert!(<ArrayView2<f64>>::try_from(&robj).is_err());

             assert!(<ArrayView2<Rcplx>>::try_from(&robj).is_err());
             assert!(<ArrayView2<c64>>::try_from(&robj).is_err());

             assert!(<ArrayView1<Rbool>>::try_from(&robj).is_err());
        }
    }
}
