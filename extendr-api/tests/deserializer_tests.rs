#[cfg(feature = "serde")]
mod test {
    use extendr_api::prelude::*;
    use extendr_api::deserializer::from_robj;
    use serde::Deserialize;

    ////////////////////////////////////////////////////////////////////////////////

    #[test]
    fn test_deserialize_robj() {
        test! {
            #[derive(Deserialize, PartialEq, Debug)]
            struct Null;
            assert_eq!(from_robj::<Null>(r!(NULL)), Ok(Null));
            assert_eq!(from_robj::<Null>(r!(1)), Err(Error::ExpectedNull(r!(1))));

            #[derive(Deserialize, PartialEq, Debug)]
            struct Int(i32);
            assert_eq!(from_robj::<Int>(r!(1)), Ok(Int(1)));
            assert_eq!(from_robj::<Int>(r!(1.0)), Ok(Int(1)));
            assert_eq!(from_robj::<Int>(r!(NULL)), Err(Error::ExpectedNonZeroLength(r!(NULL))));

            #[derive(Deserialize, PartialEq, Debug)]
            struct VInt(Vec<i32>);
            assert_eq!(from_robj::<VInt>(r!(list!(1, 2))), Ok(VInt(vec![1, 2])));
        }
    }
}
