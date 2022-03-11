// use core::slice::SlicePattern;

use extendr_api::{prelude::*};
// use extendr_macros::IntoDataframe;

#[test]
fn test_derive_into_dataframe() {

    #[derive(Debug)]
    struct MyStruct<'a> {
        x: i32,
        y: &'a str,
    }

    trait IntoDataframe<'a> : ExactSizeIterator<Item=&'a MyStruct<'a>> + std::fmt::Debug + Sized {
        fn into_dataframe(self) -> extendr_api::wrapper::List {
            let len = self.len();
            let mut x = Vec::with_capacity(len);
            let mut y = Vec::with_capacity(len);
            for record in self {
                x.push(record.x);
                y.push(record.y);
            }
            data_frame!(
                x = Integers::from_values(x),
                y = Strings::from_values(y)
            ).try_into().unwrap()
        }
    }

    impl<'a, I> IntoDataframe<'a> for I where I : ExactSizeIterator<Item=&'a MyStruct<'a>> + std::fmt::Debug  {}

    let v = vec![MyStruct { x: 0, y: "xyz" }, MyStruct { x: 1, y: "xyz" }];
    let df = v.iter().into_dataframe();

    assert!(df.inherits("data.frame"));
}
