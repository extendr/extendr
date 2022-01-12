#[cfg(feature = "serde")]
mod test {
    use extendr_api::prelude::*;
    use extendr_api::serializer::to_robj;
    use serde::Serialize;

    ////////////////////////////////////////////////////////////////////////////////

    #[test]
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
            assert_eq!(to_robj(&u).unwrap(), r!(expected));

            let n = E::Newtype(1);
            let expected = list!(Newtype=1);
            assert_eq!(to_robj(&n).unwrap(), r!(expected));

            let t = E::Tuple(1, 2);
            let expected = list!(Tuple=list!(1, 2));
            assert_eq!(to_robj(&t).unwrap(), r!(expected));

            let s = E::Struct { a: 1 };
            let expected = list!(Struct=list!(a=1));
            assert_eq!(to_robj(&s).unwrap(), r!(expected));
        }
    }

    #[test]
    fn test_serialize_robj() {
        test! {
            let s = r!(());
            assert_eq!(to_robj(&s).unwrap(), r!(()));
            let s = r!(sym!(xyz));
            assert_eq!(to_robj(&s).unwrap(), r!("xyz"));
            let s = r!(pairlist!(x=1, y=2));
            assert_eq!(to_robj(&s).unwrap(), r!(list!(x=1, y=2)));
            let s = R!("function (x) 1").unwrap();
            assert_eq!(to_robj(&s).unwrap(), r!(()));
            let s = r!(Environment::new_with_parent(global_env()));
            assert_eq!(to_robj(&s).unwrap(), r!(()));
            let s = r!(Promise::from_parts(r!(()), global_env()));
            assert_eq!(to_robj(&s).unwrap(), r!(()));
            let s = r!(Language::from_values([r!(())]));
            assert_eq!(to_robj(&s).unwrap(), r!(()));
            let s = r!(Primitive::from_string("+"));
            assert_eq!(to_robj(&s).unwrap(), r!(".Primitive(\"+\")"));
            let s = r!(Primitive::from_string("if"));
            assert_eq!(to_robj(&s).unwrap(), r!(".Primitive(\"if\")"));
            let s = r!(Rstr::from_string("xyz"));
            assert_eq!(to_robj(&s).unwrap(), r!("xyz"));
            let s = r!([TRUE, FALSE, NA_LOGICAL]);
            assert_eq!(to_robj(&s).unwrap(), r!(list!(TRUE, FALSE, ())));
            let s = r!([1, 2, 3]);
            assert_eq!(to_robj(&s).unwrap(), r!(list![1, 2, 3]));
            let s = r!([1., 2., 3.]);
            assert_eq!(to_robj(&s).unwrap(), r!(list![1., 2., 3.]));
            // Rany::Complexes(_complex) => serializer.serialize_unit(),
            let s = r!(["1", "2", "3"]);
            assert_eq!(to_robj(&s).unwrap(), r!(list!["1", "2", "3"]));
            // Rany::Dot(_dot) => serializer.serialize_unit(),
            // Rany::Any(_any) => serializer.serialize_unit(),
            let s = r!(list![1, 2., "3"]);
            assert_eq!(to_robj(&s).unwrap(), r!(list![1, 2., "3"]));
            let s = r!(list![a=1, b=2., c="3"]);
            assert_eq!(to_robj(&s).unwrap(), r!(list![a=1, b=2., c="3"]));
            let s = r!(parse("1 + 2"));
            assert_eq!(to_robj(&s).unwrap(), r!("expression(1 + 2)"));
            // Rany::Bytecode(_bytecode) => serializer.serialize_unit(),
            // Rany::ExternalPtr(_externalptr) => serializer.serialize_unit(),
            // Rany::WeakRef(_weakref) => serializer.serialize_unit(),
            let s = r!(Raw::from_bytes(&[1, 2, 3]));
            assert_eq!(to_robj(&s).unwrap(), r!(Raw::from_bytes(&[1, 2, 3])));
            // Rany::S4(value) => value.serialize(serializer),
            // Rany::Unknown(_unknown) => serializer.serialize_unit(),
        }
    }
}
