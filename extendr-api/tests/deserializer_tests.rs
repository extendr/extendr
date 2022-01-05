#[cfg(feature = "serde")]
mod test {
    use extendr_api::deserializer::from_robj;
    use extendr_api::prelude::*;
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
            assert_eq!(from_robj::<Null>(&r!(NULL)), Ok(Null));
            assert_eq!(from_robj::<Null>(&r!(1)), Err(Error::ExpectedNull(r!(1))));

            #[derive(Deserialize, PartialEq, Debug)]
            struct Int(i32);
            assert_eq!(from_robj::<Int>(&r!(1)), Ok(Int(1)));
            assert_eq!(from_robj::<Int>(&r!(1.0)), Ok(Int(1)));
            assert_eq!(from_robj::<Int>(&r!(NULL)).is_err(), true);

            #[derive(Deserialize, PartialEq, Debug)]
            struct RInt(Rint);
            assert_eq!(from_robj::<RInt>(&r!(1)), Ok(RInt(1.into())));
            assert_eq!(from_robj::<RInt>(&r!(1.0)), Ok(RInt(1.into())));
            assert_eq!(from_robj::<RInt>(&r!(Rint::na())).is_err(), true);
            assert_eq!(from_robj::<RInt>(&r!(NULL)).is_err(), true);

            #[derive(Deserialize, PartialEq, Debug)]
            struct Float(f64);
            assert_eq!(from_robj::<Float>(&r!(1)), Ok(Float(1.0)));
            assert_eq!(from_robj::<Float>(&r!(1.0)), Ok(Float(1.0)));
            assert_eq!(from_robj::<Float>(&r!(NULL)).is_err(), true);

            #[derive(Deserialize, PartialEq, Debug)]
            struct RFloat(Rfloat);
            assert_eq!(from_robj::<RFloat>(&r!(1)), Ok(RFloat(1.0.into())));
            assert_eq!(from_robj::<RFloat>(&r!(1.0)), Ok(RFloat(1.0.into())));
            assert_eq!(from_robj::<RFloat>(&r!(Rfloat::na())).is_err(), true);
            assert_eq!(from_robj::<RFloat>(&r!(NULL)).is_err(), true);

            #[derive(Deserialize, PartialEq, Debug)]
            struct Bool(bool);
            assert_eq!(from_robj::<Bool>(&r!(TRUE)), Ok(Bool(true)));
            assert_eq!(from_robj::<Bool>(&r!(FALSE)), Ok(Bool(false)));
            assert_eq!(from_robj::<Bool>(&r!(NULL)).is_err(), true);

            #[derive(Deserialize, PartialEq, Debug)]
            struct RBool(Rbool);
            assert_eq!(from_robj::<RBool>(&r!(TRUE)), Ok(RBool(TRUE)));
            assert_eq!(from_robj::<RBool>(&r!(FALSE)), Ok(RBool(FALSE)));
            assert_eq!(from_robj::<RBool>(&r!(Rbool::na())).is_err(), true);
            assert_eq!(from_robj::<RBool>(&r!(NULL)).is_err(), true);

            #[derive(Deserialize, PartialEq, Debug)]
            struct VInt(Vec<i32>);
            assert_eq!(from_robj::<VInt>(&r!(list!(1, 2))), Ok(VInt(vec![1, 2])));
            assert_eq!(from_robj::<VInt>(&r!([1, 2])), Ok(VInt(vec![1, 2])));
            assert_eq!(from_robj::<VInt>(&r!([1, 2, i32::na()])).is_err(), true);

            // Any integer type will do.
            #[derive(Deserialize, PartialEq, Debug)]
            struct VInt16(Vec<i16>);
            assert_eq!(from_robj::<VInt16>(&r!(list!(1, 2))), Ok(VInt16(vec![1, 2])));
            assert_eq!(from_robj::<VInt16>(&r!([1, 2])), Ok(VInt16(vec![1, 2])));

            #[derive(Deserialize, PartialEq, Debug)]
            struct VFloat64(Vec<f64>);
            assert_eq!(from_robj::<VFloat64>(&r!(list!(1, 2))), Ok(VFloat64(vec![1., 2.])));
            assert_eq!(from_robj::<VFloat64>(&r!([1, 2])), Ok(VFloat64(vec![1., 2.])));

            #[derive(Deserialize, PartialEq, Debug)]
            struct VBool(Vec<bool>);
            assert_eq!(from_robj::<VBool>(&r!(list!(TRUE, FALSE))), Ok(VBool(vec![true, false])));
            assert_eq!(from_robj::<VBool>(&r!([TRUE, FALSE])), Ok(VBool(vec![true, false])));

            #[derive(Deserialize, PartialEq, Debug)]
            struct Str(String);
            assert_eq!(from_robj::<Str>(&r!("xyz")), Ok(Str("xyz".into())));

            #[derive(Deserialize, PartialEq, Debug)]
            struct StrSlice<'a>(&'a str);
            assert_eq!(from_robj::<StrSlice>(&r!("xyz")), Ok(StrSlice("xyz")));

            // Structs are mapped to named lists.
            #[derive(Deserialize, PartialEq, Debug)]
            struct Struct { a: i32, b: f64 }
            assert_eq!(from_robj::<Struct>(&r!(list!(a=1, b=2))), Ok(Struct{ a: 1, b: 2.0 }));

            // Enums are mapped to named lists of lists.
            #[derive(Deserialize, PartialEq, Debug)]
            enum Enum {
                Unit,
                Newtype(u32),
                Tuple(u32, u32),
                Struct { a: u32 },
            }

            let j = r!("Unit");
            let expected = Enum::Unit;
            assert_eq!(expected, from_robj(&j).unwrap());

            let j = r!(list!(Newtype=1));
            let expected = Enum::Newtype(1);
            assert_eq!(expected, from_robj(&j).unwrap());

            let j = r!(list!(Tuple=list!(1, 2)));
            let expected = Enum::Tuple(1, 2);
            assert_eq!(expected, from_robj(&j).unwrap());

            let j = r!(list!(Struct=list!(a=1)));
            let expected = Enum::Struct { a: 1 };
            assert_eq!(expected, from_robj(&j).unwrap());
        }
    }
}
