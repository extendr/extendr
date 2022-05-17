use extendr_api::prelude::*;

#[test]
fn test_derive_into_dataframe() {
    test! {
        use extendr_api::prelude::*;

        #[derive(Debug, IntoDataFrameRow)]
        struct MyStruct {
            x: Rint,
            y: Rstr,
        }

        let v = vec![MyStruct { x: 0.into(), y: "abc".into() }, MyStruct { x: 1.into(), y: "xyz".into() }];
        let df = v.into_dataframe()?;

        assert!(df.inherits("data.frame"));

        let list : List = df.as_list().unwrap();
        assert_eq!(list[0], r!([0, 1]));
        assert_eq!(list[1], r!(["abc", "xyz"]));

        // Note the odd RFC 2451 (I,). If you can fix this, I'll be a happy bunny.
        let df3 = ((0..2).map(|i| MyStruct { x: i.into(), y: i.to_string().into() }),).into_dataframe()?;
        assert!(df3.inherits("data.frame"));

        let list : List = df3.as_list().unwrap();
        assert_eq!(list[0], r!([0, 1]));
        assert_eq!(list[1], r!(["0", "1"]));
    }
}
