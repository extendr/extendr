#[doc(hidden)]
use ndarray::prelude::*;

use crate::*;

impl<'a, T> FromRobj<'a> for ArrayView1<'a, T>
where
    Robj: AsTypedSlice<'a, T>,
{
    /// Convert an R object to a `ndarray` ArrayView1.
    fn from_robj(robj: &'a Robj) -> std::result::Result<Self, &'static str> {
        if let Some(v) = robj.as_typed_slice() {
            Ok(ArrayView1::<'a, T>::from(v))
        } else {
            Err("Not a vector of the correct type.")
        }
    }
}

macro_rules! make_array_view_2 {
    ($type: ty, $error_str: expr) => {
        impl<'a> FromRobj<'a> for ArrayView2<'a, $type> {
            /// Convert an R object to a `ndarray` ArrayView2.
            fn from_robj(robj: &'a Robj) -> std::result::Result<Self, &'static str> {
                if robj.is_matrix() {
                    let nrows = robj.nrows();
                    let ncols = robj.ncols();
                    if let Some(v) = robj.as_typed_slice() {
                        // use fortran order.
                        let shape = (nrows, ncols).into_shape().f();
                        if let Ok(res) = ArrayView2::from_shape(shape, v) {
                            return Ok(res);
                        }
                    }
                }
                return Err($error_str);
            }
        }
    };
}

make_array_view_2!(Bool, "Not a logical matrix.");
make_array_view_2!(i32, "Not an integer matrix.");
make_array_view_2!(f64, "Not a floating point matrix.");
//make_array_view_2!(u8, "Not a raw matrix.");

// impl<'a, T> From<ArrayView2<'a, T>> for Robj
// where
//     T : ToVectorValue
// {
//     fn from(array: ArrayView2<T>) -> Self {
//         let dims = array.dim();
//         let slice : &[T] = array.as_slice().unwrap();
//         let mx = Matrix::new(slice, dims.0, dims.1);
//         r!(mx)
//     }
// }

#[test]
fn test_from_robj() {
    test! {
        assert_eq!(
            <ArrayView1<f64>>::from_robj(&Robj::from(1.)),
            Ok(ArrayView1::<f64>::from(&[1.][..]))
        );
        assert_eq!(
            <ArrayView1<i32>>::from_robj(&Robj::from(1)),
            Ok(ArrayView1::<i32>::from(&[1][..]))
        );
        assert_eq!(
            <ArrayView1<Bool>>::from_robj(&Robj::from(true)),
            Ok(ArrayView1::<Bool>::from(&[Bool(1)][..]))
        );

        let robj = R!(matrix(c(1, 2, 3, 4, 5, 6, 7, 8), ncol=2, nrow=4))?;
        let mx = <ArrayView2<f64>>::from_robj(&robj)?;
        assert_eq!(mx[[0, 0]], 1.0);
        assert_eq!(mx[[1, 0]], 2.0);
        assert_eq!(mx[[2, 0]], 3.0);
        assert_eq!(mx[[3, 0]], 4.0);
        assert_eq!(mx[[0, 1]], 5.0);
        assert_eq!(mx[[1, 1]], 6.0);
        assert_eq!(mx[[2, 1]], 7.0);
        assert_eq!(mx[[3, 1]], 8.0);

        // check basic logic of fortran-order matrices.
        let col0 = mx.column(0);
        assert_eq!(col0[0], 1.0);
        assert_eq!(col0[1], 2.0);
        assert_eq!(col0[2], 3.0);
        assert_eq!(col0[3], 4.0);

        // check integer matrices
        let robj = R!(matrix(c(1L, 2L, 3L, 4L, 5L, 6L, 7L, 8L), ncol=2, nrow=4))?;
        let mx = <ArrayView2<i32>>::from_robj(&robj)?;
        assert_eq!(mx[[0, 0]], 1);
        assert_eq!(mx[[1, 0]], 2);
        assert_eq!(mx[[2, 0]], 3);
        assert_eq!(mx[[3, 0]], 4);
        assert_eq!(mx[[0, 1]], 5);
        assert_eq!(mx[[1, 1]], 6);
        assert_eq!(mx[[2, 1]], 7);
        assert_eq!(mx[[3, 1]], 8);

        // check logical matrices
        let robj = R!(matrix(c(T, T, T, T, F, F, F, F), ncol=2, nrow=4))?;
        let mx = <ArrayView2<Bool>>::from_robj(&robj)?;
        assert_eq!(mx[[0, 0]], TRUE);
        assert_eq!(mx[[1, 0]], TRUE);
        assert_eq!(mx[[2, 0]], TRUE);
        assert_eq!(mx[[3, 0]], TRUE);
        assert_eq!(mx[[0, 1]], FALSE);
        assert_eq!(mx[[1, 1]], FALSE);
        assert_eq!(mx[[2, 1]], FALSE);
        assert_eq!(mx[[3, 1]], FALSE);

        // check raw matrices
        // let robj = r!(Matrix::new(vec![1_u8, 2, 3, 4, 5, 6, 7, 8], 4, 2));
        // let mx = <ArrayView2<u8>>::from_robj(&robj)?;
        // assert_eq!(mx[[0, 0]], 1);
        // assert_eq!(mx[[1, 0]], 2);
        // assert_eq!(mx[[2, 0]], 3);
        // assert_eq!(mx[[3, 0]], 4);
        // assert_eq!(mx[[0, 1]], 5);
        // assert_eq!(mx[[1, 1]], 6);
        // assert_eq!(mx[[2, 1]], 7);
        // assert_eq!(mx[[3, 1]], 8);
    }
}
