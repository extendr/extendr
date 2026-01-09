//! See https://github.com/extendr/extendr/issues/369
//!

#[test]
fn test_try_from() {
    use extendr_api::scalar::{Rbool, Rcplx, Rfloat, Rint};
    use extendr_api::{r, test, Robj, TryFrom};

    macro_rules! test_matrix {
        (
            $value : expr,
            int_ok: $int_ok : expr,
            float_ok: $float_ok : expr,
            cplx_ok: $cplx_ok : expr,
            bool_ok: $bool_ok : expr,
            str_ok: $str_ok : expr,
            int_slice_ok: $int_slice_ok : expr,
            float_slice_ok: $float_slice_ok : expr,
        ) => {
            let robj = $value.clone();
            assert!(u8::try_from(&robj).is_ok() == $int_ok);
            assert!(i8::try_from(&robj).is_ok() == $int_ok);
            assert!(u16::try_from(&robj).is_ok() == $int_ok);
            assert!(i16::try_from(&robj).is_ok() == $int_ok);
            assert!(u32::try_from(&robj).is_ok() == $int_ok);
            assert!(i32::try_from(&robj).is_ok() == $int_ok);
            assert!(u64::try_from(&robj).is_ok() == $int_ok);
            assert!(i64::try_from(&robj).is_ok() == $int_ok);
            assert!(f32::try_from(&robj).is_ok() == $float_ok);
            assert!(f64::try_from(&robj).is_ok() == $float_ok);
            assert!(Rint::try_from(&robj).is_ok() == $int_ok);
            assert!(Rfloat::try_from(&robj).is_ok() == $float_ok);
            assert!(Rcplx::try_from(&robj).is_ok() == $cplx_ok);
            assert!(Rbool::try_from(&robj).is_ok() == $bool_ok);
            assert!(bool::try_from(&robj).is_ok() == $bool_ok);
            assert!(<&str>::try_from(&robj).is_ok() == $str_ok);
            assert!(String::try_from(&robj).is_ok() == $str_ok);
            assert!(<&[i32]>::try_from(&robj).is_ok() == $int_slice_ok);
            assert!(<&[f64]>::try_from(&robj).is_ok() == $float_slice_ok);
            assert!(<Robj>::try_from($value).is_ok() == true);
        };
    }

    test! {
        let integer = r!(1);
        test_matrix!(
            integer.clone(),
            int_ok : true,
            float_ok : true,
            cplx_ok: true,
            bool_ok : false,
            str_ok: false,
            int_slice_ok: true,
            float_slice_ok: false,
        );

        let double = r!(1.0);
        test_matrix!(
            double.clone(),
            int_ok : true,
            float_ok : true,
            cplx_ok: true,
            bool_ok : false,
            str_ok: false,
            int_slice_ok: false,
            float_slice_ok: true,
        );

        let null = r!(());
        test_matrix!(
            null.clone(),
            int_ok : false,
            float_ok : false,
            cplx_ok: false,
            bool_ok : false,
            str_ok: false,
            int_slice_ok: false,
            float_slice_ok: false,
        );

        let string = r!("1");
        test_matrix!(
            string.clone(),
            int_ok : false,
            float_ok : false,
            cplx_ok: false,
            bool_ok : false,
            str_ok: true,
            int_slice_ok: false,
            float_slice_ok: false,
        );
    }
}
