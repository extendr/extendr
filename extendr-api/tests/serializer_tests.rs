#[cfg(feature = "serde")]
mod test {
    use extendr_api::prelude::*;
    use extendr_api::serializer::to_robj;
    use serde::Serialize;

    #[test]
    fn test_serialize_struct() {
        test! {
            #[derive(Serialize)]
            struct Test<'a> {
                int: i32,
                seq: Vec<&'a str>,
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
            let expected = r!("Unit");
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
            #[derive(Serialize)]
            struct Null(Robj);
            let s = Null(r!(NULL));
            let expected = r!(NULL);
            assert_eq!(to_robj(&s).unwrap(), Robj::from(expected));

            #[derive(Serialize)]
            struct Sym(Symbol);
            let s = Sym(sym!(xyz).try_into()?);
            let expected = r!("xyz");
            assert_eq!(to_robj(&s).unwrap(), Robj::from(expected));

            #[derive(Serialize)]
            struct Plist(Pairlist);
            let s = Plist(pairlist!(a=1, b=2));
            let expected = list!(a=1, b=2);
            assert_eq!(to_robj(&s).unwrap(), Robj::from(expected));

            #[derive(Serialize)]
            struct Rstr1(Rstr);
            let s = Rstr1(Rstr::from("xyz"));
            let expected = r!("xyz");
            assert_eq!(to_robj(&s).unwrap(), Robj::from(expected));

            #[derive(Serialize)]
            struct Int(Integers);
            let s = Int(Integers::from_values([1]));
            let expected = r!(1);
            assert_eq!(to_robj(&s).unwrap(), Robj::from(expected));

            #[derive(Serialize)]
            struct Int2(Integers);
            let s = Int2(Integers::from_values([1, 2]));
            let expected = r!(list![1, 2]);
            assert_eq!(to_robj(&s).unwrap(), Robj::from(expected));

            #[derive(Serialize)]
            struct Dbl2(Doubles);
            let s = Dbl2(Doubles::from_values([1.0, 2.0]));
            let expected = r!(list![1.0, 2.0]);
            assert_eq!(to_robj(&s).unwrap(), Robj::from(expected));

            // BUG! Will probably be fixed by "better-debug"
            //
            // #[derive(Serialize)]
            // struct List1(List);
            // let s = List1(list!(a=1, b=2));
            // let expected = r!(list!(a=1, b=2));
            // assert_eq!(to_robj(&s).unwrap(), Robj::from(expected));

            // #[derive(Serialize)]
            // struct List2(List);
            // let s = List2(list!(1, 2));
            // let expected = r!(list!(1, 2));
            // assert_eq!(to_robj(&s).unwrap(), Robj::from(expected));

            #[derive(Serialize)]
            struct Raw1(Raw);
            let s = Raw1(Raw::from_bytes(&[1, 2, 3]));
            let expected = r!(Raw::from_bytes(&[1, 2, 3]));
            assert_eq!(to_robj(&s).unwrap(), Robj::from(expected));

            #[derive(Serialize)]
            struct Rint1(Rint);
            let s = Rint1(Rint::from(1));
            let expected = r!(1);
            assert_eq!(to_robj(&s).unwrap(), Robj::from(expected));

            #[derive(Serialize)]
            struct Rint2(Rint);
            let s = Rint2(Rint::na());
            let expected = r!(());
            assert_eq!(to_robj(&s).unwrap(), Robj::from(expected));

            #[derive(Serialize)]
            struct Rfloat1(Rfloat);
            let s = Rfloat1(Rfloat::from(1.0));
            let expected = r!(1.0);
            assert_eq!(to_robj(&s).unwrap(), Robj::from(expected));

            #[derive(Serialize)]
            struct Rfloat2(Rfloat);
            let s = Rfloat2(Rfloat::na());
            let expected = r!(());
            assert_eq!(to_robj(&s).unwrap(), Robj::from(expected));

            #[derive(Serialize)]
            struct Rbool1(Rbool);
            let s = Rbool1(Rbool::from(true));
            let expected = r!(true);
            assert_eq!(to_robj(&s).unwrap(), Robj::from(expected));

            #[derive(Serialize)]
            struct Rbool2(Rbool);
            let s = Rbool2(Rbool::na());
            let expected = r!(());
            assert_eq!(to_robj(&s).unwrap(), Robj::from(expected));
        }
    }
}
