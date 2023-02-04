#[doc(hidden)]
use ndarray::prelude::*;
use ndarray::{Data, ShapeBuilder};

use crate::prelude::dim_symbol;
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

make_array_view_2!(Rbool, "Not a logical matrix.");
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::FromRobj;
    use ndarray::array;
    use rstest::rstest;

    #[rstest]
    // Scalars
    #[case(
        "1.0",
        ArrayView1::<f64>::from(&[1.][..])
    )]
    #[case(
        "1L",
        ArrayView1::<i32>::from(&[1][..])
    )]
    #[case(
        "TRUE",
        ArrayView1::<Rbool>::from(&[TRUE][..])
    )]
    // Matrices
    #[case(
       "matrix(c(1, 2, 3, 4, 5, 6, 7, 8), ncol=2, nrow=4)",
        <Array2<f64>>::from_shape_vec((4, 2).f(), vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]).unwrap()
    )]
    #[case(
        // Testing the memory layout is Fortran
        "matrix(c(1, 2, 3, 4, 5, 6, 7, 8), ncol=2, nrow=4)[, 1]",
        <Array2<f64>>::from_shape_vec((4, 2).f(), vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]).unwrap().column(0).to_owned()
    )]
    #[case(
        "matrix(c(1L, 2L, 3L, 4L, 5L, 6L, 7L, 8L), ncol=2, nrow=4)",
        <Array2<i32>>::from_shape_vec((4, 2).f(), vec![1, 2, 3, 4, 5, 6, 7, 8]).unwrap()
    )]
    #[case(
        "matrix(c(T, T, T, T, F, F, F, F), ncol=2, nrow=4)",
        <Array2<Rbool>>::from_shape_vec((4, 2).f(), vec![true.into(), true.into(), true.into(), true.into(), false.into(), false.into(), false.into(), false.into()]).unwrap()
    )]
    fn test_from_robj<DataType, DimType>(
        #[case] left: &'static str,
        #[case] right: ArrayBase<DataType, DimType>,
    ) where
        DataType: Data,
        for<'a> ArrayView<'a, <DataType as ndarray::RawData>::Elem, DimType>: FromRobj<'a>,
        DimType: Dimension,
        <DataType as ndarray::RawData>::Elem: PartialEq + std::fmt::Debug,
    {
        // Tests for the R → Rust conversion
        test! {
            let left_robj = eval_string(left).unwrap();
            let left_array = <ArrayView<DataType::Elem, DimType>>::from_robj(&left_robj).unwrap();
            assert_eq!( left_array, right );
        }
    }

    #[rstest]
    #[case(
        // An empty array should still convert to an empty R array with the same shape
        &Array4::<i32>::zeros((0, 1, 2, 3).f()),
        "array(integer(), c(0, 1, 2, 3))"
    )]
    #[case(
        &array![1., 2., 3.],
        "array(c(1, 2, 3))"
    )]
    #[case(
        // We give both R and Rust the same 1d vector and tell them both to read it as a matrix in C order.
        // Therefore these arrays should be the same.
        &Array::from_shape_vec((2, 3), vec![1., 2., 3., 4., 5., 6.]).unwrap(),
        "matrix(c(1, 2, 3, 4, 5, 6), nrow=2, byrow=TRUE)"
    )]
    #[case(
        // We give both R and Rust the same 1d vector and tell them both to read it as a matrix
        // in fortran order. Therefore these arrays should be the same.
        &Array::from_shape_vec((2, 3).f(), vec![1., 2., 3., 4., 5., 6.]).unwrap(),
        "matrix(c(1, 2, 3, 4, 5, 6), nrow=2, byrow=FALSE)"
    )]
    #[case(
        // We give both R and Rust the same 1d vector and tell them both to read it as a 3d array
        // in fortran order. Therefore these arrays should be the same.
        &Array::from_shape_vec((1, 2, 3).f(), vec![1, 2, 3, 4, 5, 6]).unwrap(),
        "array(1:6, c(1, 2, 3))"
    )]
    #[case(
        // We give R a 1d vector and tell it to read it as a 3d vector
        // Then we give Rust the equivalent vector manually split out.
        &array![[[1, 5], [3, 7]], [[2, 6], [4, 8]]],
        "array(1:8, dim=c(2, 2, 2))"
    )]
    fn test_to_robj<T>(#[case] array: T, #[case] r_expr: &str)
    where
        Robj: TryFrom<T>,
        <Robj as TryFrom<T>>::Error: std::fmt::Debug,
    {
        // Tests for the Rust → R conversion, so we therefore perform the
        // comparison in R
        test! {
            assert_eq!(
                &(Robj::try_from(array).unwrap()),
                &eval_string(r_expr).unwrap()
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
}
