// Tests for the #[derive(IntoList)] macro
// Testing conversion of Rust structs to R named lists with various field types

#[cfg(test)]
mod tests {
    use extendr_api::prelude::*;
    use extendr_macros::IntoList;

    #[test]
    fn test_into_list_basic_types() {
        test! {
            #[derive(IntoList, Debug)]
            struct BasicTypes {
                a: i32,
                b: f64,
                c: bool,
                d: String,
            }

            let rust_struct = BasicTypes {
                a: 42,
                b: 3.14,
                c: true,
                d: String::from("hello"),
            };

            let r_list: Robj = rust_struct.into();
            assert!(r_list.is_list());

            // Check that the list has the correct fields
            let expected = R!("list(a = 42L, b = 3.14, c = TRUE, d = 'hello')").unwrap();
            assert_eq!(r_list, expected);
        }
    }

    #[test]
    fn test_into_list_vector_types() {
        test! {
            #[derive(IntoList, Debug)]
            struct VectorTypes {
                integers: Vec<i32>,
                doubles: Vec<f64>,
                strings: Vec<String>,
                bools: Vec<bool>,
            }

            let rust_struct = VectorTypes {
                integers: vec![1, 2, 3],
                doubles: vec![1.1, 2.2, 3.3],
                strings: vec![String::from("a"), String::from("b")],
                bools: vec![true, false, true],
            };

            let r_list: Robj = rust_struct.into();
            assert!(r_list.is_list());

            // Verify individual fields
            let list = List::try_from(&r_list).unwrap();
            assert_eq!(list.len(), 4);

            // Check integers field
            let integers: Vec<i32> = list.elt(0).unwrap().try_into().unwrap();
            assert_eq!(integers, vec![1, 2, 3]);

            // Check doubles field
            let doubles: Vec<f64> = list.elt(1).unwrap().try_into().unwrap();
            assert_eq!(doubles, vec![1.1, 2.2, 3.3]);

            // Check strings field
            let strings: Vec<String> = list.elt(2).unwrap().try_into().unwrap();
            assert_eq!(strings, vec!["a", "b"]);

            // Check bools field
            let bools: Vec<bool> = list.elt(3).unwrap().try_into().unwrap();
            assert_eq!(bools, vec![true, false, true]);
        }
    }

    #[test]
    fn test_into_list_with_robj_types() {
        test! {
            #[derive(IntoList, Debug)]
            struct WithRobjTypes {
                doubles: Doubles,
                logicals: Logicals,
                strings: Strings,
                raw: Raw,
                list: List,
                robj: Robj,
            }

            let rust_struct = WithRobjTypes {
                doubles: Doubles::from_values([1.0, 2.0, 3.0]),
                logicals: Logicals::from_values([true, false, true]),
                strings: Strings::from_values(["hello", "world"]),
                raw: Raw::from_bytes(&[0x01, 0x02, 0x03]),
                list: list!(a = 1, b = 2),
                robj: r!(42),
            };

            let r_list: Robj = rust_struct.into();
            assert!(r_list.is_list());

            let list = List::try_from(&r_list).unwrap();
            assert_eq!(list.len(), 6);

            // Verify doubles field
            let doubles: Doubles = list.elt(0).unwrap().try_into().unwrap();
            assert_eq!(doubles.to_vec(), vec![1.0, 2.0, 3.0]);

            // Verify logicals field
            let logicals: Logicals = list.elt(1).unwrap().try_into().unwrap();
            assert_eq!(logicals.to_vec(), vec![true, false, true]);

            // Verify strings field
            let strings: Strings = list.elt(2).unwrap().try_into().unwrap();
            assert_eq!(strings.to_vec(), vec!["hello", "world"]);

            // Verify raw field
            let raw: Raw = list.elt(3).unwrap().try_into().unwrap();
            assert_eq!(raw.as_slice().to_vec(), vec![0x01, 0x02, 0x03]);

            // Verify list field
            let inner_list: List = list.elt(4).unwrap().try_into().unwrap();
            assert_eq!(inner_list.len(), 2);

            // Verify robj field
            let robj_val: i32 = list.elt(5).unwrap().try_into().unwrap();
            assert_eq!(robj_val, 42);
        }
    }

    #[test]
    fn test_into_list_with_function() {
        test! {
            #[derive(IntoList, Debug)]
            struct WithFunction {
                name: String,
                func: Function,
            }

            let rust_struct = WithFunction {
                name: String::from("sum"),
                func: R!("sum").unwrap().try_into().unwrap(),
            };

            let r_list: Robj = rust_struct.into();
            assert!(r_list.is_list());

            let list = List::try_from(&r_list).unwrap();
            assert_eq!(list.len(), 2);

            // Verify the function field is present
            let func: Function = list.elt(1).unwrap().try_into().unwrap();
            assert!(func.is_function());
        }
    }

    #[test]
    fn test_into_list_with_pairlist() {
        test! {
            #[derive(IntoList, Debug)]
            struct WithPairlist {
                data: Pairlist,
                count: i32,
            }

            let rust_struct = WithPairlist {
                data: pairlist!(a = 1, b = 2),
                count: 2,
            };

            let r_list: Robj = rust_struct.into();
            assert!(r_list.is_list());

            let list = List::try_from(&r_list).unwrap();
            assert_eq!(list.len(), 2);

            // Verify pairlist field
            let pairlist: Pairlist = list.elt(0).unwrap().try_into().unwrap();
            assert_eq!(pairlist.len(), 2);
        }
    }

    #[test]
    fn test_into_list_with_environment() {
        test! {
            #[derive(IntoList, Debug)]
            struct WithEnvironment {
                env: Environment,
                name: String,
            }

            let env = Environment::new_with_parent(global_env());
            env.set_local(sym!(x), 42);

            let rust_struct = WithEnvironment {
                env,
                name: String::from("my_env"),
            };

            let r_list: Robj = rust_struct.into();
            assert!(r_list.is_list());

            let list = List::try_from(&r_list).unwrap();
            assert_eq!(list.len(), 2);

            // Verify environment field
            let env: Environment = list.elt(0).unwrap().try_into().unwrap();
            assert!(env.is_environment());
            let x_val: i32 = env.local(sym!(x)).unwrap().try_into().unwrap();
            assert_eq!(x_val, 42);
        }
    }

    #[test]
    fn test_into_list_with_ignore_attribute() {
        test! {
            #[derive(IntoList, Debug)]
            struct WithIgnored {
                public_field: String,
                visible_count: i32,
                #[into_list(ignore)]
                internal_ptr: *const u8,
                #[into_list(ignore)]
                private_data: Vec<u8>,
            }

            let rust_struct = WithIgnored {
                public_field: String::from("visible"),
                visible_count: 42,
                internal_ptr: std::ptr::null(),
                private_data: vec![1, 2, 3],
            };

            // Verify the ignored fields exist in the Rust struct
            assert_eq!(rust_struct.internal_ptr, std::ptr::null());
            assert_eq!(rust_struct.private_data, vec![1, 2, 3]);

            let r_list: Robj = rust_struct.into();
            assert!(r_list.is_list());

            let list = List::try_from(&r_list).unwrap();
            // Should only have 2 fields (the ignored ones are excluded)
            assert_eq!(list.len(), 2);

            // Verify the visible fields
            let public_field: String = list.elt(0).unwrap().try_into().unwrap();
            assert_eq!(public_field, "visible");

            let visible_count: i32 = list.elt(1).unwrap().try_into().unwrap();
            assert_eq!(visible_count, 42);

            // Verify the ignored fields are NOT present by accessing them with dollar()
            // In R, accessing a non-existent list element returns NULL, not an error
            let internal_ptr_result = r_list.dollar("internal_ptr").unwrap();
            assert!(internal_ptr_result.is_null(), "internal_ptr should return NULL");

            let private_data_result = r_list.dollar("private_data").unwrap();
            assert!(private_data_result.is_null(), "private_data should return NULL");
        }
    }

    #[test]
    fn test_into_list_nested_structs() {
        test! {
            #[derive(IntoList, Debug)]
            struct Inner {
                x: i32,
                y: f64,
            }

            #[derive(IntoList, Debug)]
            struct Outer {
                name: String,
                data: List,
            }

            let inner = Inner { x: 10, y: 20.5 };
            let inner_as_robj: Robj = inner.into();
            let inner_as_list: List = inner_as_robj.try_into().unwrap();

            let outer = Outer {
                name: String::from("container"),
                data: inner_as_list,
            };

            let r_list: Robj = outer.into();
            assert!(r_list.is_list());

            let list = List::try_from(&r_list).unwrap();
            assert_eq!(list.len(), 2);

            // Verify nested structure
            let data_list: List = list.elt(1).unwrap().try_into().unwrap();
            assert_eq!(data_list.len(), 2);

            let x: i32 = data_list.elt(0).unwrap().try_into().unwrap();
            assert_eq!(x, 10);

            let y: f64 = data_list.elt(1).unwrap().try_into().unwrap();
            assert_eq!(y, 20.5);
        }
    }

    #[test]
    fn test_into_list_from_reference() {
        test! {
            #[derive(IntoList, Debug)]
            struct SimpleStruct {
                value: i32,
            }

            let rust_struct = SimpleStruct { value: 100 };

            // Test conversion from reference
            let r_list: Robj = (&rust_struct).into();
            assert!(r_list.is_list());

            let list = List::try_from(&r_list).unwrap();
            let value: i32 = list.elt(0).unwrap().try_into().unwrap();
            assert_eq!(value, 100);

            // Test conversion from owned value
            let r_list2: Robj = rust_struct.into();
            assert!(r_list2.is_list());
            assert_eq!(r_list, r_list2);
        }
    }

    #[test]
    fn test_into_list_example_from_pr() {
        test! {
            #[derive(Debug, PartialEq)]
            struct MyArg {
                name: String,
                arg_type: String,
            }

            impl From<&MyArg> for Robj {
                fn from(arg: &MyArg) -> Self {
                    list!(name = &arg.name, arg_type = &arg.arg_type).into()
                }
            }

            #[derive(Debug, PartialEq, IntoList)]
            struct MyFunc {
                doc: &'static str,
                rust_name: &'static str,
                mod_name: &'static str,
                r_name: &'static str,
                c_name: &'static str,
                args: Vec<Robj>,
                return_type: &'static str,
                #[into_list(ignore)]
                func_ptr: *const u8,
                hidden: bool,
            }

            let args = vec![
                MyArg { name: "x".to_string(), arg_type: "f64".to_string() },
                MyArg { name: "y".to_string(), arg_type: "i32".to_string() },
            ];

            let func = MyFunc {
                doc: "Test function",
                rust_name: "test_fn",
                mod_name: "test_mod",
                r_name: "testFn",
                c_name: "wrap__test_fn",
                args: args.iter().map(|a| a.into()).collect(),
                return_type: "Robj",
                func_ptr: std::ptr::null(),
                hidden: false,
            };

            let r_list: Robj = func.into();
            assert!(r_list.is_list());

            let list = List::try_from(&r_list).unwrap();
            // Should have 8 fields (func_ptr is ignored)
            assert_eq!(list.len(), 8);

            // Verify some fields
            let doc: String = list.elt(0).unwrap().try_into().unwrap();
            assert_eq!(doc, "Test function");

            let hidden: bool = list.elt(7).unwrap().try_into().unwrap();
            assert_eq!(hidden, false);

            // Verify args field is a list of lists
            let args_list: List = list.elt(5).unwrap().try_into().unwrap();
            assert_eq!(args_list.len(), 2);
        }
    }

    #[test]
    fn test_into_list_empty_collections() {
        test! {
            #[derive(IntoList, Debug)]
            struct EmptyCollections {
                empty_vec: Vec<i32>,
                empty_strings: Vec<String>,
                empty_list: List,
            }

            let rust_struct = EmptyCollections {
                empty_vec: vec![],
                empty_strings: vec![],
                empty_list: list!(),
            };

            let r_list: Robj = rust_struct.into();
            assert!(r_list.is_list());

            let list = List::try_from(&r_list).unwrap();
            assert_eq!(list.len(), 3);

            // Verify empty vector
            let empty_vec: Vec<i32> = list.elt(0).unwrap().try_into().unwrap();
            assert_eq!(empty_vec.len(), 0);

            // Verify empty strings
            let empty_strings: Vec<String> = list.elt(1).unwrap().try_into().unwrap();
            assert_eq!(empty_strings.len(), 0);

            // Verify empty list
            let empty_list: List = list.elt(2).unwrap().try_into().unwrap();
            assert_eq!(empty_list.len(), 0);
        }
    }
}
