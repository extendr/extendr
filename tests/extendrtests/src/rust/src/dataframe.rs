use extendr_api::prelude::*;

#[derive(Debug, IntoDataFrameRow)]
struct MyStruct {
    x: RInt,
    y: RStr,
}

#[extendr]
fn test_derive_into_dataframe() -> DataFrame<MyStruct> {
    let v = vec![
        MyStruct {
            x: 0.into(),
            y: "abc".into(),
        },
        MyStruct {
            x: 1.into(),
            y: "xyz".into(),
        },
    ];

    v.into_dataframe().unwrap()
}

#[extendr]
fn test_into_robj_dataframe() -> RObj {
    let v = vec![
        MyStruct {
            x: 0.into(),
            y: "abc".into(),
        },
        MyStruct {
            x: 1.into(),
            y: "xyz".into(),
        },
    ];

    v.into_dataframe().unwrap().into_robj()
}

// Not possible today
// https://github.com/extendr/extendr/issues/727
// #[derive(Debug, IntoDataFrameRow)]
// struct Val;

// #[extendr]
// impl Val {
//     fn new() -> Self {
//         Val {}
//     }
// }
// #[derive(IntoDataFrameRow)]
// struct Row {
//     v: Val,
// }
// #[extendr]
// fn test_use_externalptr() -> RObj {
//     vec![Row { v: Val::new() }]
//         .into_dataframe()
//         .unwrap()
//         .into_robj()
// }

// This isn't relavent right now
// thats because issue https://github.com/extendr/extendr/issues/714
// is not solved we would need an iterator for DataFrame<T> for it to
// actually be useful
// #[extendr]
// fn dataframe_conversion_try_from(_data_frame: DataFrame<Row>) -> RObj {
//     vec![Row { name: 42 }].into_dataframe().unwrap().into_robj()
// }

extendr_module! {
    mod dataframe;
    fn test_derive_into_dataframe;
    fn test_into_robj_dataframe;
}
