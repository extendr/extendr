#[cfg(feature = "serde")]
mod test {
    use extendr_api::prelude::*;
    use extendr_api::deserializer::from_robj;
    use serde::Deserialize;

    ////////////////////////////////////////////////////////////////////////////////
    ///
    /// Deserialize from a Robj.
    /// 
    /// Like JSON, we can use a Robj as a storage format.
    /// 
    /// For example if creating vectors from a RDS file or returning a structure.
    /// 
    #[test]
    fn test_deserialize_robj() {
        test! {
            // In these tests, the wrapper is transparent and just tests the contents.
            // So Int(i32) is actually testing i32.

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

            #[derive(Deserialize, PartialEq, Debug)]
            struct Str(String);
            assert_eq!(from_robj::<Str>(r!("xyz")), Ok(Str("xyz".into())));

            #[derive(Deserialize, PartialEq, Debug)]
            struct StrSlice<'a>(&'a str);
            assert_eq!(from_robj::<StrSlice>(r!("xyz")), Ok(StrSlice("xyz")));
        }
    }
}
