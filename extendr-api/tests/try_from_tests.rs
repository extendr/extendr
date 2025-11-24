//! See https://github.com/extendr/extendr/issues/369
//!

#[test]
fn test_try_from() {
    use extendr_api::scalar::{Rbool, Rcplx, Rfloat, Rint};
    use extendr_api::{r, test, Robj, TryFrom};
    // use extendr_api::wrapper::{Integers, Doubles, Strings};

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
            assert!(u8::try_from($value).is_ok() == $int_ok);
            assert!(i8::try_from($value).is_ok() == $int_ok);
            assert!(u16::try_from($value).is_ok() == $int_ok);
            assert!(i16::try_from($value).is_ok() == $int_ok);
            assert!(u32::try_from($value).is_ok() == $int_ok);
            assert!(i32::try_from($value).is_ok() == $int_ok);
            assert!(u64::try_from($value).is_ok() == $int_ok);
            assert!(i64::try_from($value).is_ok() == $int_ok);
            // assert!(usize::try_from($value).is_ok() == $int_ok);
            // assert!(isize::try_from($value).is_ok() == $int_ok);
            assert!(f32::try_from($value).is_ok() == $float_ok);
            assert!(f64::try_from($value).is_ok() == $float_ok);
            assert!(Rint::try_from($value).is_ok() == $int_ok);
            assert!(Rfloat::try_from($value).is_ok() == $float_ok);
            assert!(Rcplx::try_from($value).is_ok() == $cplx_ok);
            // assert!(<&Rstr>::try_from($value).is_ok() == $str_ok);
            assert!(Rbool::try_from($value).is_ok() == $bool_ok);
            assert!(bool::try_from($value).is_ok() == $bool_ok);
            assert!(<&str>::try_from($value).is_ok() == $str_ok);
            assert!(String::try_from($value).is_ok() == $str_ok);
            assert!(<&[i32]>::try_from($value).is_ok() == $int_slice_ok);
            assert!(<&[f64]>::try_from($value).is_ok() == $float_slice_ok);
            // assert!(<&Robj>::try_from($value).is_ok() == true);
            assert!(<Robj>::try_from($value).is_ok() == true);
            // assert!(<&[Rint]>::try_from($value).is_ok() == $int_slice_ok);
            // assert!(<&[Rfloat]>::try_from($value).is_ok() == $float_slice_ok);
            // assert!(<&[Rcplx]>::try_from($value).is_ok() == $cplx_slice_ok);
            // assert!(<&[Rbool]>::try_from($value).is_ok() == $bool_slice_ok);
            // assert!(<&[Rstr]>::try_from($value).is_ok() == $str_ok);
            // assert!(<&[Robj]>::try_from($value).is_ok() == $list_ok);
            // assert!(<&Integers>::try_from($value).is_ok() == $int_ok);
            // assert!(<&Doubles>::try_from($value).is_ok() == $float_ok);
            // assert!(<&List>::try_from($value).is_ok() == $list_ok);

            // assert_eq!(<Option<u8>::try_from($value).is_ok(), $int_ok);
            // assert_eq!(<Option<i8>::try_from($value).is_ok(), $int_ok);
            // assert_eq!(<Option<u16>::try_from($value).is_ok(), $int_ok);
            // assert_eq!(<Option<i16>::try_from($value).is_ok(), $int_ok);
            // assert_eq!(<Option<u32>::try_from($value).is_ok(), $int_ok);
            // assert_eq!(<Option<i32>::try_from($value).is_ok(), $int_ok);
            // assert_eq!(<Option<u64>::try_from($value).is_ok(), $int_ok);
            // assert_eq!(<Option<i64>::try_from($value).is_ok(), $int_ok);
            // assert_eq!(<Option<usize>::try_from($value).is_ok(), $int_ok);
            // assert_eq!(<Option<isize>::try_from($value).is_ok(), $int_ok);
            // assert_eq!(<Option<Rint>::try_from($value).is_ok(), $int_ok);
            // assert_eq!(<Option<f32>::try_from($value).is_ok(), $float_ok);
            // assert_eq!(<Option<f64>::try_from($value).is_ok(), $float_ok);
            // assert_eq!(<Option<Rfloat>::try_from($value).is_ok(), $float_ok);
            // assert_eq!(<Option<Rcplx>::try_from($value).is_ok(), $float_ok);
            // assert_eq!(<Option<bool>::try_from($value).is_ok(), $bool_ok);
            // assert_eq!(<Option<&str>>::try_from($value).is_ok(), $str_ok);
            // assert_eq!(<Option<String>::try_from($value).is_ok(), $str_ok);
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

        // let integers = Integers::from_values([1]);
        // test_matrix!(
        //     integer.clone(),
        //     int_ok : true,
        //     float_ok : true,
        //     cplx_ok: true,
        //     bool_ok : false,
        //     str_ok: false,
        //     int_slice_ok: true,
        //     float_slice_ok: false,
        // );
        // test_matrix!(integers.clone(), int_ok : true, float_ok : true, bool_ok : false, str_ok: false);

        // let doubles = Doubles::from_values([1.0]);
        // test_matrix!(doubles.clone(), int_ok : true, float_ok : true, bool_ok : false, str_ok: false);

        // let strings = Strings::from_values(["1"]);
        // test_matrix!(strings.clone(), int_ok : true, float_ok : true, bool_ok : false, str_ok: false);
    }
}
