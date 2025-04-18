/*!
Defines conversions between R objects and the [`nalgebra`](https://docs.rs/nalgebra/latest/nalgebra/) crate.

To enable these conversions, you must first enable the `nalgebra` feature for extendr:
```toml
[dependencies]
extendr-api = { version = "0.8.0", features = ["nalgebra"] }
```

Specifically, extendr supports the following conversions between dynamic nalgebra matrices (`DMatrix<f64>`) and `RMatrix`:

TODO: Complete documentation
*/

use crate::*;
use crate::scalar::Rfloat;
use nalgebra::{DMatrix, Matrix};
use nalgebra::base::storage::Storage;
use nalgebra::Dim;
use std::convert::TryFrom;

/// Convert any nalgebra `Matrix<f64, R, C, S>` into an R numeric matrix.
impl<R: Dim, C: Dim, S> From<Matrix<f64, R, C, S>> for RMatrix<f64>
where
    S: Storage<f64, R, C>,
{
    fn from(value: Matrix<f64, R, C, S>) -> Self {
        let nrows = value.nrows();
        let ncols = value.ncols();
        RMatrix::new_matrix(nrows, ncols, |i, j| value[(i, j)])
    }
}

/// Convert any nalgebra `Matrix<f64, R, C, S>` into an `Robj` (via `RMatrix<f64>`).
impl<R: Dim, C: Dim, S> From<Matrix<f64, R, C, S>> for Robj
where
    S: Storage<f64, R, C>,
{
    fn from(value: Matrix<f64, R, C, S>) -> Self {
        RMatrix::<f64>::from(value).into()
    }
}

/// Convert any nalgebra `Matrix<f64, R, C, S>` into an R numeric matrix (`Rfloat`).
impl<R: Dim, C: Dim, S> From<Matrix<f64, R, C, S>> for RMatrix<Rfloat>
where
    S: Storage<f64, R, C>,
{
    fn from(value: Matrix<f64, R, C, S>) -> Self {
        let nrows = value.nrows();
        let ncols = value.ncols();
        RMatrix::new_matrix(nrows, ncols, |i, j| value[(i, j)].into())
    }
}

/// Convert an R real matrix into a dynamic nalgebra matrix (`DMatrix<f64>`).
impl TryFrom<&Robj> for DMatrix<f64> {
    type Error = Error;
    fn try_from(robj: &Robj) -> Result<Self> {
        let rmat = RMatrix::<f64>::try_from(robj)?;
        let nrows = rmat.nrows();
        let ncols = rmat.ncols();
        if let Some(slice) = robj.as_real_slice() {
            Ok(DMatrix::from_column_slice(nrows, ncols, slice))
        } else {
            Err(Error::ExpectedReal(robj.clone()))
        }
    }
}

/// Infallible conversion from `Robj` into `DMatrix<f64>` (panics on conversion error).
impl From<Robj> for DMatrix<f64> {
    fn from(robj: Robj) -> Self {
        DMatrix::try_from(&robj).expect("Failed to convert R object to nalgebra DMatrix")
    }
}

/// Convert an R integer matrix into a dynamic nalgebra matrix (`DMatrix<f64>`).
impl TryFrom<&RMatrix<i32>> for DMatrix<f64> {
    type Error = Error;
    fn try_from(rm: &RMatrix<i32>) -> Result<Self> {
        let nrows = rm.nrows();
        let ncols = rm.ncols();
        let slice = rm
            .as_integer_slice()
            .ok_or_else(|| Error::ExpectedInteger((*rm).clone()))?;
        let vec = slice.iter().map(|&x| x as f64).collect::<Vec<_>>();
        Ok(DMatrix::from_column_slice(nrows, ncols, &vec))
    }
}

/// Convert an R numeric matrix into a dynamic nalgebra matrix (`DMatrix<f64>`).
impl TryFrom<&RMatrix<f64>> for DMatrix<f64> {
    type Error = Error;
    fn try_from(rm: &RMatrix<f64>) -> Result<Self> {
        let nrows = rm.nrows();
        let ncols = rm.ncols();
        let slice = rm
            .as_real_slice()
            .ok_or_else(|| Error::ExpectedReal((*rm).clone()))?;
        Ok(DMatrix::from_column_slice(nrows, ncols, slice))
    }
}

#[cfg(test)]
mod test {
    use crate as extendr_api;
    use crate::*;
    use nalgebra::DMatrix;
    use std::convert::TryFrom;

    #[test]
    fn test_rmatrix_to_nalgebra_matrix() {
        test! {
            let values = [
                [1.0, 5.0, 9.0],
                [2.0, 6.0, 10.0],
                [3.0, 7.0, 11.0],
                [4.0, 8.0, 12.0],
            ];
            let a = DMatrix::from_fn(4, 3, |i, j| values[i][j]);
            let rmatrix = RMatrix::new_matrix(4, 3, |i, j| values[i][j]);
            let b = DMatrix::try_from(&rmatrix).unwrap();
            assert_eq!(a, b);
        }
    }

    #[test]
    fn test_nalgebra_matrix_to_rmatrix() {
        test! {
            let values = [
                [1.0, 5.0, 9.0],
                [2.0, 6.0, 10.0],
                [3.0, 7.0, 11.0],
                [4.0, 8.0, 12.0],
            ];
            let a = DMatrix::from_fn(4, 3, |i, j| values[i][j]);
            let rmatrix: RMatrix<f64> = a.clone().into();
            let expected = (1..=12).map(|x| x as f64).collect::<Vec<_>>();
            assert_eq!(rmatrix.as_real_slice().unwrap(), &expected[..]);
        }
    }

    #[test]
    fn test_integer_rmatrix_to_nalgebra_matrix() {
        test! {
            let values = [
                [1, 5, 9],
                [2, 6, 10],
                [3, 7, 11],
                [4, 8, 12],
            ];
            let a = DMatrix::from_fn(4, 3, |i, j| values[i][j] as f64);
            let rmatrix = RMatrix::new_matrix(4, 3, |i, j| values[i][j]);
            let b = DMatrix::try_from(&rmatrix).unwrap();
            assert_eq!(a, b);
        }
    }
    
    #[test]
    fn test_robj_to_dmatrix() {
        test! {
            let values = [
                [1.0, 5.0, 9.0],
                [2.0, 6.0, 10.0],
                [3.0, 7.0, 11.0],
                [4.0, 8.0, 12.0],
            ];
            let a = DMatrix::from_fn(4, 3, |i, j| values[i][j]);
            let rmatrix = RMatrix::new_matrix(4, 3, |i, j| values[i][j]);
            let robj = Robj::from(rmatrix.clone());
            let b: DMatrix<f64> = robj.into();
            assert_eq!(a, b);
        }
    }
}
