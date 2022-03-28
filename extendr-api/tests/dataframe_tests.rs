#[test]
fn dataframe_tests() {
    use extendr_api::prelude::*;
    use extendr_api::wrapper::Dataframe;
    test! {
        let x = [(1_i32, 1.0_f64), (2, 2.0), (3, 3.0)];

        let df : Dataframe<(i32, f64)> = Dataframe::try_from_values(x).unwrap();
        let df2 = data_frame!(a=[1, 2, 3], b=[1., 2., 3.]);
        assert_eq!(df.as_robj(), &df2);
    }
}

// use core::slice::SlicePattern;

use extendr_api::prelude::*;
// use extendr_macros::IntoDataframe;

#[test]
fn test_derive_into_dataframe() {
    test! {
        use extendr_api::prelude::*;

        #[derive(Debug, IntoDataframe)]
        struct MyStruct {
            x: i32,
            y: String,
        }

        let v = vec![MyStruct { x: 0, y: "abc".into() }, MyStruct { x: 1, y: "xyz".into() }];
        let df = v.into_dataframe()?;

        assert!(df.inherits("data.frame"));
        assert_eq!(df[0], r!([0, 1]));
        assert_eq!(df[1], r!(["abc", "xyz"]));

        // Note the odd RFC 2451 (I,). If you can fix this, I'll be a happy bunny.
        let df3 = ((0..2).map(|i| MyStruct { x: i, y: i.to_string() }),).into_dataframe()?;
        assert!(df3.inherits("data.frame"));
        assert_eq!(df3[0], r!([0, 1]));
        assert_eq!(df3[1], r!(["0", "1"]));
    }
}
