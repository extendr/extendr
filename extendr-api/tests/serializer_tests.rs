#[cfg(feature = "serde")]
mod test {
    use extendr_api::prelude::*;
    use extendr_api::serializer::to_robj;
    use serde::Serialize;

    ////////////////////////////////////////////////////////////////////////////////

    #[test]
    #[cfg(feature = "serde")]
    fn test_serialize_struct() {
        test! {
            #[derive(Serialize)]
            struct Test {
                int: i32,
                seq: Vec<&'static str>,
            }

            let test = Test {
                int: 1,
                seq: vec!["a", "b"],
            };

            let expected = list!(int=1, seq=list!("a", "b"));
            assert_eq!(to_robj(&test).unwrap(), Robj::from(expected));
        }
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_serialize_enum() {
        test! {
            #[derive(Serialize)]
            enum E {
                Unit,
                Newtype(i32),
                Tuple(i32, i32),
                Struct { a: i32 },
            }

            let u = E::Unit;
            let expected = list!(Unit=NULL);
            assert_eq!(to_robj(&u).unwrap(), expected);

            let n = E::Newtype(1);
            let expected = list!(Newtype=1);
            assert_eq!(to_robj(&n).unwrap(), expected);

            let t = E::Tuple(1, 2);
            let expected = list!(Tuple=list!(1, 2));
            assert_eq!(to_robj(&t).unwrap(), expected);

            let s = E::Struct { a: 1 };
            let expected = list!(Struct=list!(a=1));
            assert_eq!(to_robj(&s).unwrap(), expected);
        }
    }
}
