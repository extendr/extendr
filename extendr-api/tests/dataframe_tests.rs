
use extendr_api::prelude::*;

#[test]
fn test_derive_into_dataframe() {
    test! {
        use extendr_api::prelude::*;

        #[derive(Debug, IntoDataFrame)]
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
