use libR_sys::*;
use ndarray::prelude::*;

use crate::logical::Bool;
use crate::robj::{AsTypedSlice, FromRobj, Robj};

/// Input Numeric vector parameter.
/// Note we don't accept mutable R objects as parameters
/// but you can make this behaviour using unsafe code.
impl<'a, T> FromRobj<'a> for ArrayView1<'a, T>
where
    Robj: AsTypedSlice<T>,
{
    fn from_robj(robj: &'a Robj) -> Result<Self, &'static str> {
        if let Some(v) = robj.as_typed_slice() {
            Ok(ArrayView1::<'a, T>::from(v))
        } else {
            Err("not a floating point vector")
        }
    }
}

macro_rules! make_array_view_2 {
    ($type: ty, $fn: tt, $error_str: tt, $($sexp: tt),* ) => {
        impl<'a> FromRobj<'a> for ArrayView2<'a, $type> {
            fn from_robj(robj: &'a Robj) -> Result<Self, &'static str> {
                match robj.sexptype() {
                    $( $sexp )|* => unsafe {
                        let ptr = $fn(robj.get()) as *const $type;
                        let ncols = Rf_ncols(robj.get()) as usize;
                        let nrows = Rf_nrows(robj.get()) as usize;

                        Ok(ArrayView2::from_shape_ptr((nrows, ncols).f(), ptr))
                    },
                    _ => Err($error_str),
                }
            }
        }
    }
}

make_array_view_2!(Bool, INTEGER, "not a logical matrix", LGLSXP);
make_array_view_2!(i32, INTEGER, "not a integer matrix", INTSXP);
make_array_view_2!(f64, REAL, "not a floating point matrix", REALSXP);
make_array_view_2!(u8, RAW, "not a raw matrix", RAWSXP);

#[test]
fn test_from_robj() {
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
    assert_eq!(
        <ArrayView2<f64>>::from_robj(&Robj::from(1.)),
        Ok(ArrayView2::<f64>::from_shape((1, 1), &[1.][..]).unwrap())
    );
    assert_eq!(
        <ArrayView2<i32>>::from_robj(&Robj::from(1)),
        Ok(ArrayView2::<i32>::from_shape((1, 1), &[1][..]).unwrap())
    );
    assert_eq!(
        <ArrayView2<Bool>>::from_robj(&Robj::from(true)),
        Ok(ArrayView2::<Bool>::from_shape((1, 1), &[Bool(1)][..]).unwrap())
    );

    assert_eq!(
        <ArrayView2<f64>>::from_robj(
            &Robj::eval_string("matrix(c(1, 2, 3, 4, 5, 6, 7, 8), ncol=2, nrow=4, byrow=T)")
                .unwrap()
        ),
        Ok(ArrayView2::<f64>::from_shape(
            (4, 2),
            &[1f64, 2f64, 3f64, 4f64, 5f64, 6f64, 7f64, 8f64][..]
        )
        .unwrap())
    );
}
