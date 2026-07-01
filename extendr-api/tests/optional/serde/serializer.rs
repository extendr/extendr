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
        assert_eq!(to_robj(&test).unwrap(), RObj::from(expected));
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
        struct Null(RObj);
        let s = Null(r!(NULL));
        let expected = r!(NULL);
        assert_eq!(to_robj(&s).unwrap(), expected);

        #[derive(Serialize)]
        struct Sym(Symbol);
        let s = Sym(sym!(xyz).try_into()?);
        let expected = r!("xyz");
        assert_eq!(to_robj(&s).unwrap(), expected);

        #[derive(Serialize)]
        struct Plist(PairList);
        let s = Plist(pairlist!(a=1, b=2));
        let expected = list!(a=1, b=2);
        assert_eq!(to_robj(&s).unwrap(), RObj::from(expected));

        #[derive(Serialize)]
        struct RStr1(RStr);
        let s = RStr1(RStr::from("xyz"));
        let expected = r!("xyz");
        assert_eq!(to_robj(&s).unwrap(), expected);

        #[derive(Serialize)]
        struct Int(Integers);
        let s = Int(Integers::from_values([1]));
        let expected = r!(1);
        assert_eq!(to_robj(&s).unwrap(), expected);

        #[derive(Serialize)]
        struct Int2(Integers);
        let s = Int2(Integers::from_values([1, 2]));
        let expected = r!(list![1, 2]);
        assert_eq!(to_robj(&s).unwrap(), expected);

        #[derive(Serialize)]
        struct Dbl2(Doubles);
        let s = Dbl2(Doubles::from_values([1.0, 2.0]));
        let expected = r!(list![1.0, 2.0]);
        assert_eq!(to_robj(&s).unwrap(), expected);

        // BUG! Will probably be fixed by "better-debug"
        //
        // #[derive(Serialize)]
        // struct List1(List);
        // let s = List1(list!(a=1, b=2));
        // let expected = r!(list!(a=1, b=2));
        // assert_eq!(to_robj(&s).unwrap(), RObj::from(expected));

        // #[derive(Serialize)]
        // struct List2(List);
        // let s = List2(list!(1, 2));
        // let expected = r!(list!(1, 2));
        // assert_eq!(to_robj(&s).unwrap(), RObj::from(expected));

        #[derive(Serialize)]
        struct Raw1(Raw);
        let s = Raw1(Raw::from_bytes(&[1, 2, 3]));
        let expected = r!(Raw::from_bytes(&[1, 2, 3]));
        assert_eq!(to_robj(&s).unwrap(), expected);

        #[derive(Serialize)]
        struct RInt1(RInt);
        let s = RInt1(RInt::from(1));
        let expected = r!(1);
        assert_eq!(to_robj(&s).unwrap(), expected);

        #[derive(Serialize)]
        struct RInt2(RInt);
        let s = RInt2(RInt::na());
        let expected = r!(());
        assert_eq!(to_robj(&s).unwrap(), expected);

        #[derive(Serialize)]
        struct RFloat1(RFloat);
        let s = RFloat1(RFloat::from(1.0));
        let expected = r!(1.0);
        assert_eq!(to_robj(&s).unwrap(), expected);

        #[derive(Serialize)]
        struct RFloat2(RFloat);
        let s = RFloat2(RFloat::na());
        let expected = r!(());
        assert_eq!(to_robj(&s).unwrap(), expected);

        #[derive(Serialize)]
        struct RBool1(RBool);
        let s = RBool1(RBool::from(true));
        let expected = r!(true);
        assert_eq!(to_robj(&s).unwrap(), expected);

        #[derive(Serialize)]
        struct RBool2(RBool);
        let s = RBool2(RBool::na());
        let expected = r!(());
        assert_eq!(to_robj(&s).unwrap(), expected);
    }
}
