/*!
Defines conversions between R objects and the [`ndarray`](https://docs.rs/ndarray/latest/ndarray/) crate, which offers native Rust array types and numerical computation routines.

To enable these conversions, you must first enable the `ndarray` feature for extendr:
```toml
[dependencies]
extendr-api = { version = "0.4", features = ["ndarray"] }
```

Specifically, extendr supports the following conversions:
* [`Robj` → `ArrayView1`](FromRobj#impl-FromRobj<%27a>-for-ArrayView1<%27a%2C%20T>), for when you have an R vector that you want to analyse in Rust:
    ```rust
    use extendr_api::prelude::*;

    #[extendr]
    fn describe_vector(vector: ArrayView1<f64>){
        println!("This R vector has length {:?}", vector.len())
    }
    ```
* [`Robj` → `ArrayView2`](FromRobj#impl-FromRobj<%27a>-for-ArrayView2<%27a%2C%20f64>), for when you have an R matrix that you want to analyse in Rust.
    ```rust
    use extendr_api::prelude::*;

    #[extendr]
    fn describe_matrix(matrix: ArrayView2<f64>){
        println!("This R matrix has shape {:?}", matrix.dim())
    }
    ```
* [`ArrayBase` → `Robj`](Robj#impl-TryFrom<ArrayBase<S%2C%20D>>-for-Robj), for when you want to return a reference to an [`ndarray`] Array from Rust back to R.
    ```rust
    use extendr_api::prelude::*;

    #[extendr]
    fn return_matrix() -> Robj {
        Array2::<f64>::zeros((4, 4)).try_into().unwrap()
    }
    ```

The item type (ie the `T` in [`Array2<T>`]) can be a variety of Rust types that can represent scalars: [`u32`], [`i32`], [`f64`] and, if you have the `num_complex` compiled feature
enabled, `Complex<f64>`. Items can also be extendr's wrapper types: [`Rbool`], [`Rint`], [`Rfloat`] and [`Rcplx`].

Note that the extendr-ndarray integration only supports accessing R arrays as [`ArrayView`], which are immutable.
Therefore, instead of directly editing the input array, it is recommended that you instead return a new array from your `#[extendr]`-annotated function, which you allocate in Rust.
It will then be copied into a new block of memory managed by R.
This is made easier by the fact that [ndarray allocates a new array automatically when performing operations on array references](ArrayBase#binary-operators-with-array-and-scalar):
```rust
use extendr_api::prelude::*;

#[extendr]
fn scalar_multiplication(matrix: ArrayView2<f64>, scalar: f64) -> Robj {
    (&matrix * scalar).try_into().unwrap()
}
```

For all array uses in Rust, refer to the [`ndarray::ArrayBase`] documentation, which explains the usage for all of the above types.
*/
#[doc(hidden)]
use ndarray::prelude::*;
use ndarray::{Data, ShapeBuilder};

use crate::prelude::{c64, dim_symbol, Rcplx, Rfloat, Rint};
use crate::*;

macro_rules! make_array_view_1 {
    ($type: ty, $error_fn: expr) => {
        impl<'a> TryFrom<&'_ Robj> for ArrayView1<'a, $type> {
            type Error = crate::Error;

            fn try_from(robj: &Robj) -> Result<Self> {
                if let Some(v) = robj.as_typed_slice() {
                    Ok(ArrayView1::<'a, $type>::from(v))
                } else {
                    Err($error_fn(robj.clone()))
                }
            }
        }

        impl<'a> TryFrom<Robj> for ArrayView1<'a, $type> {
            type Error = crate::Error;

            fn try_from(robj: Robj) -> Result<Self> {
                Self::try_from(&robj)
            }
        }
    };
}

macro_rules! make_array_view_2 {
    ($type: ty, $error_str: expr, $error_fn: expr) => {
        impl<'a> TryFrom<&'_ Robj> for ArrayView2<'a, $type> {
            type Error = crate::Error;
            fn try_from(robj: &Robj) -> Result<Self> {
                if robj.is_matrix() {
                    let nrows = robj.nrows();
                    let ncols = robj.ncols();
                    if let Some(v) = robj.as_typed_slice() {
                        // use fortran order.
                        let shape = (nrows, ncols).into_shape().f();
                        return ArrayView2::from_shape(shape, v)
                            .map_err(|err| Error::NDArrayShapeError(err));
                    } else {
                        return Err($error_fn(robj.clone()));
                    }
                }
                return Err(Error::ExpectedMatrix(robj.clone()));
            }
        }

        impl<'a> TryFrom<Robj> for ArrayView2<'a, $type> {
            type Error = crate::Error;
            fn try_from(robj: Robj) -> Result<Self> {
                Self::try_from(&robj)
            }
        }
    };
}
make_array_view_1!(Rbool, Error::ExpectedLogical);
make_array_view_1!(Rint, Error::ExpectedInteger);
make_array_view_1!(i32, Error::ExpectedInteger);
make_array_view_1!(Rfloat, Error::ExpectedReal);
make_array_view_1!(f64, Error::ExpectedReal);
make_array_view_1!(Rcplx, Error::ExpectedComplex);
make_array_view_1!(c64, Error::ExpectedComplex);
make_array_view_1!(Rstr, Error::ExpectedString);

make_array_view_2!(Rbool, "Not a logical matrix.", Error::ExpectedLogical);
make_array_view_2!(Rint, "Not an integer matrix.", Error::ExpectedInteger);
make_array_view_2!(i32, "Not an integer matrix.", Error::ExpectedInteger);
make_array_view_2!(Rfloat, "Not a floating point matrix.", Error::ExpectedReal);
make_array_view_2!(f64, "Not a floating point matrix.", Error::ExpectedReal);
make_array_view_2!(
    Rcplx,
    "Not a complex number matrix.",
    Error::ExpectedComplex
);
make_array_view_2!(c64, "Not a complex number matrix.", Error::ExpectedComplex);
make_array_view_2!(Rstr, "Not a string matrix.", Error::ExpectedString);

impl<A, S, D> TryFrom<&ArrayBase<S, D>> for Robj
where
    S: Data<Elem = A>,
    A: Copy + ToVectorValue,
    D: Dimension,
{
    type Error = Error;

    /// Converts a reference to an ndarray Array into an equivalent R array.
    /// The data itself is copied.
    fn try_from(value: &ArrayBase<S, D>) -> Result<Self> {
        // Refer to https://github.com/rust-ndarray/ndarray/issues/1060 for an excellent discussion
        // on how to convert from `ndarray` types to R/fortran arrays
        // This thread has informed the design decisions made here.

        // In general, transposing and then iterating an ndarray in C-order (`iter()`) is exactly
        // equivalent to iterating that same array in Fortan-order (which `ndarray` doesn't currently support)
        let mut result = value
            .t()
            .iter()
            // Since we only have a reference, we have to copy all elements so that we can own the entire R array
            .copied()
            .collect_robj();
        result.set_attrib(
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
        )?;
        Ok(result)
    }
}

impl<A, S, D> TryFrom<ArrayBase<S, D>> for Robj
where
    S: Data<Elem = A>,
    A: Copy + ToVectorValue,
    D: Dimension,
{
    type Error = Error;

    /// Converts an ndarray Array into an equivalent R array.
    /// The data itself is copied.
    fn try_from(value: ArrayBase<S, D>) -> Result<Self> {
        Robj::try_from(&value)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate as extendr_api;
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
    fn test_from_robj<DataType, DimType, Error>(
        #[case] left: &'static str,
        #[case] right: ArrayBase<DataType, DimType>,
    ) where
        DataType: Data,
        Error: std::fmt::Debug,
        for<'a> ArrayView<'a, <DataType as ndarray::RawData>::Elem, DimType>:
            TryFrom<&'a Robj, Error = Error>,
        DimType: Dimension,
        <DataType as ndarray::RawData>::Elem: PartialEq + std::fmt::Debug,
        Error: std::fmt::Debug,
    {
        // Tests for the R → Rust conversion
        test! {
            let left_robj = eval_string(left).unwrap();
            let left_array = <ArrayView<DataType::Elem, DimType>>::try_from(&left_robj).unwrap();
            assert_eq!( left_array, right );
        }
    }

    #[rstest]
    #[case(
        // An empty array should still convert to an empty R array with the same shape
        Array4::<i32>::zeros((0, 1, 2, 3).f()),
        "array(integer(), c(0, 1, 2, 3))"
    )]
    #[case(
        array![1., 2., 3.],
        "array(c(1, 2, 3))"
    )]
    #[case(
        // We give both R and Rust the same 1d vector and tell them both to read it as a matrix in C order.
        // Therefore these arrays should be the same.
        Array::from_shape_vec((2, 3), vec![1., 2., 3., 4., 5., 6.]).unwrap(),
        "matrix(c(1, 2, 3, 4, 5, 6), nrow=2, byrow=TRUE)"
    )]
    #[case(
        // We give both R and Rust the same 1d vector and tell them both to read it as a matrix
        // in fortran order. Therefore these arrays should be the same.
        Array::from_shape_vec((2, 3).f(), vec![1., 2., 3., 4., 5., 6.]).unwrap(),
        "matrix(c(1, 2, 3, 4, 5, 6), nrow=2, byrow=FALSE)"
    )]
    #[case(
        // We give both R and Rust the same 1d vector and tell them both to read it as a 3d array
        // in fortran order. Therefore these arrays should be the same.
        Array::from_shape_vec((1, 2, 3).f(), vec![1, 2, 3, 4, 5, 6]).unwrap(),
        "array(1:6, c(1, 2, 3))"
    )]
    #[case(
        // We give R a 1d vector and tell it to read it as a 3d vector
        // Then we give Rust the equivalent vector manually split out.
        array![[[1, 5], [3, 7]], [[2, 6], [4, 8]]],
        "array(1:8, dim=c(2, 2, 2))"
    )]
    fn test_to_robj<ElementType, DimType>(
        #[case] array: Array<ElementType, DimType>,
        #[case] r_expr: &str,
    ) where
        Robj: TryFrom<Array<ElementType, DimType>>,
        for<'a> Robj: TryFrom<&'a Array<ElementType, DimType>>,
        <robj::Robj as TryFrom<Array<ElementType, DimType>>>::Error: std::fmt::Debug,
        for<'a> <robj::Robj as TryFrom<&'a Array<ElementType, DimType>>>::Error: std::fmt::Debug,
    {
        // Tests for the Rust → R conversion, so we therefore perform the
        // comparison in R
        test! {
            // Test for borrowed array
            assert_eq!(
                &(Robj::try_from(&array).unwrap()),
                &eval_string(r_expr).unwrap()
            );
            // Test for owned array
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
                let rust_arr= <ArrayView2<i32>>::try_from(&rval).unwrap();
                let r_arr: Robj = (&rust_arr).try_into().unwrap();
                assert_eq!(
                    rval,
                    r_arr
                );
            }
        }
    }
}
