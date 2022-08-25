#[doc(hidden)]
use ndarray::prelude::*;
use ndarray::{Data, ShapeBuilder};

use crate::prelude::dim_symbol;
use crate::*;

#[cfg(feature = "num-complex")]
use num_complex::Complex;

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

make_array_view_2!(Rbool, "Not a logical matrix.");
make_array_view_2!(i32, "Not an integer matrix.");
make_array_view_2!(f64, "Not a floating point matrix.");

#[cfg(feature = "num-complex")]
make_array_view_2!(Complex<f64>, "Not a complex matrix.");
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
impl<A, S, D> TryFrom<&ArrayBase<S, D>> for Robj
where
    S: Data<Elem = A>,
    A: Copy + ToVectorValue,
    D: Dimension,
{
    type Error = Error;

    fn try_from(value: &ArrayBase<S, D>) -> Result<Self> {
        // Refer to https://github.com/rust-ndarray/ndarray/issues/1060 for an excellent discussion
        // on how to convert from `ndarray` types to R/fortran arrays
        // This thread has informed the design decisions made here.

        // In general, transposing and then iterating an ndarray in C-order (`iter()`) is exactly
        // equivalent to iterating that same array in Fortan-order (which `ndarray` doesn't currently support)
        value
            .t()
            .iter()
            // Since we only have a reference, we have to copy all elements so that we can own the entire R array
            .copied()
            .collect_robj()
            .set_attrib(
                dim_symbol(),
                value
                    .shape()
                    .iter()
                    .map(|x| i32::try_from(*x))
                    .collect::<std::result::Result<Vec<i32>, <i32 as TryFrom<usize>>::Error>>()
                    .map_err(|_err| {
                        Error::Other(String::from(
                            "One or more array dimensions were too large to be handled by R.",
                        ))
                    })?,
            )
    }
}

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
            <ArrayView1<Rbool>>::from_robj(&Robj::from(true)),
            Ok(ArrayView1::<Rbool>::from(&[TRUE][..]))
        );

        #[cfg(feature = "num-complex")]
        {
        assert_eq!(
            <ArrayView1<Complex<f64>>>::from_robj(&Robj::from(Complex::<f64>::new(1., 1.))),
            Ok(ArrayView1::<Complex<f64>>::from(&[Complex::<f64>::new(1., 1.)][..]))
        );
        }

        let robj = R!("matrix(c(1, 2, 3, 4, 5, 6, 7, 8), ncol=2, nrow=4)")?;
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
        let robj = R!("matrix(c(1L, 2L, 3L, 4L, 5L, 6L, 7L, 8L), ncol=2, nrow=4)")?;
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
        let robj = R!("matrix(c(T, T, T, T, F, F, F, F), ncol=2, nrow=4)")?;
        let mx = <ArrayView2<Rbool>>::from_robj(&robj)?;
        assert_eq!(mx[[0, 0]], TRUE);
        assert_eq!(mx[[1, 0]], TRUE);
        assert_eq!(mx[[2, 0]], TRUE);
        assert_eq!(mx[[3, 0]], TRUE);
        assert_eq!(mx[[0, 1]], FALSE);
        assert_eq!(mx[[1, 1]], FALSE);
        assert_eq!(mx[[2, 1]], FALSE);
        assert_eq!(mx[[3, 1]], FALSE);
       
        // check complex matrices
        #[cfg(feature = "num-complex")]
        {
        let robj = R!("matrix(c(1+1i, 2+2i, 3+3i, 4+4i, 5+5i, 6+6i, 7+7i, 8+8i), ncol=2, nrow=4)")?;
        let mx = <ArrayView2<Complex<f64>>>::from_robj(&robj)?;
        assert_eq!(mx[[0, 0]], Complex::<f64>::new(1., 1.));
        assert_eq!(mx[[1, 0]], Complex::<f64>::new(2., 2.));
        assert_eq!(mx[[2, 0]], Complex::<f64>::new(3., 3.));
        assert_eq!(mx[[3, 0]], Complex::<f64>::new(4., 4.));
        assert_eq!(mx[[0, 1]], Complex::<f64>::new(5., 5.));
        assert_eq!(mx[[1, 1]], Complex::<f64>::new(6., 6.));
        assert_eq!(mx[[2, 1]], Complex::<f64>::new(7., 7.));
        assert_eq!(mx[[3, 1]], Complex::<f64>::new(8., 8.));
        }

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

#[test]
fn test_to_robj() {
    test! {
        // An empty array should still convert to an empty R array with the same shape
        assert_eq!(
            &Robj::try_from(&Array4::<i32>::zeros((0, 1, 2, 3).f()))?,
            &R!("array(integer(), c(0, 1, 2, 3))")?
        );

        assert_eq!(
            &Robj::try_from(&array![1., 2., 3.])?,
            &R!("array(c(1, 2, 3))")?
        );

        // We give both R and Rust the same 1d vector and tell them both to read it as a matrix
        // in C order. Therefore these arrays should be the same.
        assert_eq!(
            &Robj::try_from(&Array::from_shape_vec((2, 3), vec![1., 2., 3., 4., 5., 6.]).unwrap())?,
            &R!("matrix(c(1, 2, 3, 4, 5, 6), nrow=2, byrow=TRUE)")?
        );

        // We give both R and Rust the same 1d vector and tell them both to read it as a matrix
        // in fortran order. Therefore these arrays should be the same.
        assert_eq!(
            &Robj::try_from(&Array::from_shape_vec((2, 3).f(), vec![1., 2., 3., 4., 5., 6.]).unwrap())?,
            &R!("matrix(c(1, 2, 3, 4, 5, 6), nrow=2, byrow=FALSE)")?
        );

        // We give both R and Rust the same 1d vector and tell them both to read it as a 3d array
        // in fortran order. Therefore these arrays should be the same.
        assert_eq!(
            &Robj::try_from(&Array::from_shape_vec((1, 2, 3).f(), vec![1, 2, 3, 4, 5, 6]).unwrap())?,
            &R!("array(1:6, c(1, 2, 3))")?
        );

        // We give R a 1d vector and tell it to read it as a 3d vector
        // Then we give Rust the equivalent vector manually split out.
        assert_eq!(
            &Robj::try_from(&array![[[1, 5], [3, 7]], [[2, 6], [4, 8]]])?,
            &R!("array(1:8, dim=c(2, 2, 2))")?
        );
    }
}

#[test]
fn test_round_trip() {
    test! {
        let rvals = [
            R!("matrix(c(1L, 2L, 3L, 4L, 5L, 6L), nrow=2)"),
            R!("array(1:8, c(4, 2))")
        ];
        for rval in rvals {
            let rval = rval.unwrap();
            let rust_arr= <ArrayView2<i32>>::from_robj(&rval).unwrap();
            let r_arr: Robj = (&rust_arr).try_into().unwrap();
            assert_eq!(
                rval,
                r_arr
            );
        }
    }
}

