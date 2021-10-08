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

    #[test]
    fn test_serialize_robj() {
        test! {
            #[derive(Serialize)]
            struct Test {
                null: Robj,
                symbol: Symbol,
                pairlist: Pairlist,
                function: Function,
                // environment: Environment,
                // promise: Promise,
                // language: Language,
                // special: Primitive,
                // builtin: Primitive,
                rstr: Rstr,
                // logical: Robj,
                integer: Integers,
                real: Doubles,
                // complex: Robj,
                // string: Robj,
                // dot: Dot,
                // any: Any,
                list: List,
                // expression: Expression,
                // bytecode: Bytecode,
                // externalptr: ExternalPtr,
                // weakref: WeakRef,
                raw: Raw,
                // s4: S4,
            }

            let test = Test {
                null: r!(()),
                symbol: sym!("xyz").try_into()?,
                pairlist: pairlist!(x=1),
                function: R!("function() 1")?.try_into()?,
                // environment: Environment::new_with_parent(global_env()),
                // promise: Promise::from_parts(r!(1), global_env())?,
                // language: lang!("x").try_into()?,
                // special: r!(()),
                // builtin: r!(()),
                rstr: Rstr::from_string("xyz"),
                // logical: r!(TRUE),
                integer: Integers::new(10),
                real: Doubles::new(10),
                // complex: R!("complex(10)")?,
                // string: r!("xyz"),
                // dot: Dot,
                // any: Any,
                list: list!(a=1, b=2).try_into()?,
                // expression: Expression,
                // bytecode: Bytecode,
                // externalptr: ExternalPtr,
                // weakref: WeakRef,
                raw: Raw::new(10),
                // s4: S4,
            };

            let expected = list!(int=1, seq=list!(null = (), symbol = (), pairlist = (), function = (), rstr = (), integer = (), real = (), list = (), raw = ()));
            // assert_eq!(to_robj(&test).unwrap(), Robj::from(expected));
        }
    }
}
