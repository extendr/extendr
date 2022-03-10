use extendr_api::prelude::*;
use extendr_macros::{IntoDataframe};

#[test]
fn test_derive_into_dataframe() {

    #[derive(Debug)]
    struct MyStruct {
        x: i32,
        y: String,
    }

    trait IntoDataframe {
        fn into_dataframe(&self) -> extendr_api::wrapper::List;
    }

    impl<I> IntoDataframe for I where I : ExactSizeIterator<Item=MyStruct> + Clone {
        fn into_dataframe(&self) -> extendr_api::wrapper::List
        where
            I : ExactSizeIterator<Item=MyStruct> + Clone,
        {
            let len = self.len();
            data_frame!(
                x = Integers::from_values(self.clone().map(|r| r.x)),
                y = Strings::from_values(self.map(|r| r.y))
            )
        }
    }

    let v = vec![MyStruct { x: 0, y: "xyz".into() }, MyStruct { x: 1, y: "xyz".into() }];
    let df = v.iter().into_dataframe();

    assert_eq!(df.class(), ["data.frame"]);
}
