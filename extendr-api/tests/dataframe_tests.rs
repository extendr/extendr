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

#[test]
fn test_into_robj_dataframe() {
    test! {
        use extendr_api::prelude::*;

        #[derive(Clone, Debug, IntoDataFrameRow)]
        struct MyStruct {
            x: Rint,
            y: Rstr,
        }

        let v = vec![MyStruct { x: 0.into(), y: "abc".into() }, MyStruct { x: 1.into(), y: "xyz".into() }];
        let df = v.into_dataframe()?;

        assert_eq!(
            df.clone().as_robj().clone(),
            df.into_robj()
        );

    }
}

#[derive(IntoDataFrameRow)]
struct Row {
    name: u32,
}

#[extendr]
fn dataframe_conversion(_data_frame: Dataframe<Row>) -> Robj {
    vec![Row { name: 42 }].into_dataframe().unwrap().into_robj()
}

#[extendr]
fn dataframe_conversion_try_from(_data_frame: Dataframe<Row>) -> Robj {
    vec![Row { name: 42 }].into_dataframe().unwrap().into_robj()
}

#[extendr]
fn return_dataframe(_data_frame: Dataframe<Row>) -> Dataframe<Row> {
    vec![Row { name: 42 }].into_dataframe().unwrap()
}

#[extendr]
fn return_dataframe_try_from(_data_frame: Dataframe<Row>) -> Dataframe<Row> {
    vec![Row { name: 42 }].into_dataframe().unwrap()
}

#[test]
fn test_storing_external_ptr_as_row() {
    struct Row {
        _id: u32,
    }

    #[extendr]
    impl Row {}

    #[extendr]
    fn use_dataframe_exptr(data_frame: Dataframe<Row>) -> Dataframe<Row> {
        data_frame
    }
}

#[test]
fn test_dataframe_row_and_externalptr() {
    #[derive(IntoDataFrameRow)]
    struct RowAndExptr {
        id: u32,
    }

    #[extendr]
    impl RowAndExptr {}

    #[extendr]
    fn use_dataframe_exptr2(data_frame: Dataframe<RowAndExptr>) -> Dataframe<RowAndExptr> {
        data_frame
    }

    #[extendr]
    fn use_dataframe_exptr3(data_frame: Dataframe<RowAndExptr>) -> Dataframe<RowAndExptr> {
        data_frame
    }
}
