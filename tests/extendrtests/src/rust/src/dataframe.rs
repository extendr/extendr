use extendr_api::prelude::*;

#[derive(Debug, IntoDataFrameRow)]
struct MyStruct {
    x: Rint,
    y: Rstr,
}

#[extendr]
fn test_derive_into_dataframe() -> Dataframe<MyStruct> {
    let mut v = Vec::with_capacity(2);
    v.push(MyStruct {
        x: 0.into(),
        y: "abc".into(),
    });
    v.push(MyStruct {
        x: 1.into(),
        y: "xyz".into(),
    });
    let v = v;

    v.into_dataframe().unwrap()
}

#[extendr]
fn test_into_robj_dataframe() -> Robj {
    let mut v = Vec::with_capacity(2);
    v.push(MyStruct {
        x: 0.into(),
        y: "abc".into(),
    });
    v.push(MyStruct {
        x: 1.into(),
        y: "xyz".into(),
    });
    let v = v;

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
// fn test_use_externalptr() -> Robj {
//     vec![Row { v: Val::new() }]
//         .into_dataframe()
//         .unwrap()
//         .into_robj()
// }

// This isn't relavent right now
// thats because issue https://github.com/extendr/extendr/issues/714
// is not solved we would need an iterator for Dataframe<T> for it to
// actually be useful
// #[extendr]
// fn dataframe_conversion_try_from(_data_frame: Dataframe<Row>) -> Robj {
//     vec![Row { name: 42 }].into_dataframe().unwrap().into_robj()
// }

extendr_module! {
    mod dataframe;
    fn test_derive_into_dataframe;
    fn test_into_robj_dataframe;
}
