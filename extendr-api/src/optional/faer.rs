use faer::{mat, Mat, MatRef};

use crate::scalar::Rfloat;
use crate::*;

/// Convert a `faer::Mat<f64>` into an `RMatrix<f64>` which is not NA aware.
impl From<Mat<f64>> for RMatrix<f64> {
    fn from(value: Mat<f64>) -> Self {
        RMatrix::new_matrix(value.nrows(), value.ncols(), |i, j| value.read(i, j))
    }
}

impl From<Mat<f64>> for Robj {
    fn from(value: Mat<f64>) -> Self {
        RMatrix::<f64>::from(value).into()
    }
}

/// Convert a `faer::Mat<f64>` into an `RMatrix<f64>` which is not NA aware.
impl From<MatRef<'_, f64>> for RMatrix<f64> {
    /// Convert a faer MatRef<f64> into Robj.
    fn from(value: MatRef<'_, f64>) -> Self {
        RMatrix::new_matrix(value.nrows(), value.ncols(), |i, j| value.read(i, j))
    }
}

impl From<MatRef<'_, f64>> for Robj {
    fn from(value: MatRef<'_, f64>) -> Self {
        RMatrix::<f64>::from(value).into_robj()
    }
}

impl From<Mat<f64>> for RMatrix<Rfloat> {
    fn from(value: Mat<f64>) -> Self {
        RMatrix::new_matrix(value.nrows(), value.ncols(), |i, j| value.read(i, j).into())
    }
}

impl From<MatRef<'_, f64>> for RMatrix<Rfloat> {
    fn from(value: MatRef<f64>) -> Self {
        RMatrix::new_matrix(value.nrows(), value.ncols(), |i, j| {
            Rfloat::from(value.read(i, j))
        })
    }
}

impl From<RMatrix<f64>> for Mat<f64> {
    fn from(value: RMatrix<f64>) -> Self {
        let nrow = value.nrows();
        let ncol = value.ncols();
        let slice = value.as_real_slice().expect("RMatrix should be doubles");
        Mat::from_fn(nrow, ncol, |i, j| slice[i + j * nrow])
    }
}

impl<'a> From<&'_ RMatrix<f64>> for MatRef<'a, f64> {
    fn from(value: &RMatrix<f64>) -> Self {
        let nrow = value.nrows();
        let ncol = value.ncols();
        let slice = value.as_typed_slice().expect("RMatrix should be doubles");
        let mat_ref = faer::mat::from_column_major_slice::<f64>(slice, nrow, ncol);
        mat_ref
    }
}

impl TryFrom<&Robj> for Mat<f64> {
    type Error = Error;

    fn try_from(robj: &Robj) -> Result<Self> {
        let rmat = &RMatrix::<f64>::try_from(robj)?;
        let nrow = rmat.nrows();
        let ncol = rmat.ncols();

        if let Some(slice) = robj.as_real_slice() {
            let fmat = Mat::from_fn(nrow, ncol, |i, j| slice[i + j * nrow]);
            Ok(fmat)
        } else {
            Err(Error::ExpectedReal(robj.clone()))
        }
    }
}

impl<'a> TryFrom<&'_ Robj> for MatRef<'a, f64> {
    type Error = Error;

    fn try_from(robj: &Robj) -> Result<Self> {
        let rmat = &RMatrix::<f64>::try_from(robj)?;
        let nrows = rmat.nrows();
        let ncols = rmat.ncols();

        if let Some(slice) = robj.as_typed_slice() {
            let fmat = mat::from_column_major_slice::<f64>(slice, nrows, ncols);
            Ok(fmat)
        } else {
            Err(Error::ExpectedReal(robj.clone()))
        }
    }
}

impl TryFrom<Robj> for Mat<f64> {
    type Error = crate::Error;

    fn try_from(robj: Robj) -> Result<Self> {
        Self::try_from(&robj)
    }
}

impl<'a> TryFrom<Robj> for MatRef<'a, f64> {
    type Error = crate::Error;

    fn try_from(robj: Robj) -> Result<Self> {
        Self::try_from(&robj)
    }
}

impl From<RMatrix<i32>> for Mat<f64> {
    fn from(value: RMatrix<i32>) -> Self {
        let nrow = value.nrows();
        let ncol = value.ncols();
        let slice = value
            .as_integer_slice()
            .expect("RMatrix should be integers");
        Mat::from_fn(nrow, ncol, |i, j| slice[i + j * nrow] as f64)
    }
}

#[cfg(test)]
mod test {
    use crate as extendr_api;
    use crate::*;
    use faer::{mat, Mat, MatRef};

    #[test]
    fn test_rmatrix_to_faer_mat() {
        test! {
            let values = [
                [1.0, 5.0, 9.0],
                [2.0, 6.0, 10.0],
                [3.0, 7.0, 11.0],
                [4.0, 8.0, 12.0f64]
            ];
            let a = Mat::<f64>::from_fn(4, 3, |i, j| values[i][j] as f64);

            let rmatrix = RMatrix::new_matrix(4, 3, |i, j| values[i][j]);
            let b = Mat::<f64>::try_from(rmatrix);
            assert_eq!(a, b.expect("matrix to be converted"));
        }
    }

    #[test]
    fn test_rmatrix_to_faer_mat_with_nan() {
        test! {
            let values = [
                [1.0, 5.0, 9.0],
                [2.0, 6.0, 10.0],
                [3.0, 7.0, 11.0],
                [f64::NAN, 8.0, 12.0f64]
            ];

            let rmatrix = RMatrix::new_matrix(4, 3, |i, j| values[i][j]);
            let b = Mat::<f64>::try_from(rmatrix);
            assert!(b.expect("matrix to be converted").read(3, 0).is_nan());
        }
    }

    #[test]
    fn test_rmatrix_to_faer_mat_ref() {
        test! {
            let values = [
                [1.0, 5.0, 9.0],
                [2.0, 6.0, 10.0],
                [3.0, 7.0, 11.0],
                [4.0, 8.0, 12.0f64]
            ];
            let mat = Mat::<f64>::from_fn(4, 3, |i, j| values[i][j] as f64);
            let a = mat.as_ref();

            let rmatrix = RMatrix::new_matrix(4, 3, |i, j| values[i][j]);
            let b = MatRef::<f64>::try_from(&rmatrix);
            assert_eq!(a, b.expect("matrix to be converted"));
        }
    }

    #[test]
    fn test_faer_mat_to_rmatrix() {
        test! {
            let vec: Vec<f64> = (1..13).map(f64::from).collect();
            let a = mat![
                [1.0, 5.0, 9.0],
                [2.0, 6.0, 10.0],
                [3.0, 7.0, 11.0],
                [4.0, 8.0, 12.0f64],
            ];
            let rmatrix: RMatrix<f64> = a.into();
            assert_eq!(rmatrix.as_real_slice().expect("slice"), &vec);
        }
    }

    #[test]
    fn test_faer_mat_ref_to_rmatrix() {
        test! {
            let vec: Vec<f64> = (1..13).map(f64::from).collect();
            let a = mat![
                [1.0, 5.0, 9.0],
                [2.0, 6.0, 10.0],
                [3.0, 7.0, 11.0],
                [4.0, 8.0, 12.0f64],
            ];
            let rmatrix: RMatrix<f64> = a.as_ref().into();
            assert_eq!(rmatrix.as_real_slice().expect("slice"), &vec);
        }
    }

    #[test]
    fn test_try_from_robj_to_faer_mat() {
        test! {
            let values = [
                [1.0, 5.0, 9.0],
                [2.0, 6.0, 10.0],
                [3.0, 7.0, 11.0],
                [4.0, 8.0, 12.0f64]
            ];
            let a = Mat::<f64>::from_fn(4, 3, |i, j| values[i][j] as f64);

            let rmatrix = RMatrix::new_matrix(4, 3, |i, j| values[i][j]);
            let b = Mat::<f64>::try_from(&Robj::from(rmatrix));
            assert_eq!(a, b.expect("matrix to be converted"));
        }
    }

    #[test]
    fn test_try_from_robj_to_faer_mat_ref() {
        test! {
            let values = [
                [1.0, 5.0, 9.0],
                [2.0, 6.0, 10.0],
                [3.0, 7.0, 11.0],
                [4.0, 8.0, 12.0f64]
            ];
            let mat = Mat::<f64>::from_fn(4, 3, |i, j| values[i][j] as f64);
            let a = mat.as_ref();

            let rmatrix = RMatrix::new_matrix(4, 3, |i, j| values[i][j]);
            let robj = Robj::from(rmatrix);
            let b = MatRef::<f64>::try_from(&robj);
            assert_eq!(a, b.expect("matrix to be converted"));
        }
    }

    #[test]
    fn test_int_rmatrix_to_faer_mat() {
        test! {
            let values = [
                [1, 5, 9],
                [2, 6, 10],
                [3, 7, 11],
                [4, 8, 12]
            ];
            let a = Mat::<f64>::from_fn(4, 3, |i, j| values[i][j] as f64);

            let rmatrix = RMatrix::new_matrix(4, 3, |i, j| values[i][j]);
            let b = Mat::<f64>::try_from(rmatrix);
            assert_eq!(a, b.expect("matrix to be converted"));
        }
    }
}
