use extendr_api::prelude::*;

#[test]
fn test_debug() {
    test! {
        let r = r!(());
        assert_eq!(format!("{:?}", r), "()");
        let r : Symbol = sym!("xyz").try_into().unwrap();
        assert_eq!(format!("{:?}", r), "sym!(\"xyz\")");
        let r : Pairlist = pairlist!(x=1);
        assert_eq!(format!("{:?}", r), "pairlist!(x=1)");
        let r : Function = R!("function() 1").unwrap().try_into().unwrap();
        assert_eq!(format!("{:?}", r), "function () 1");
        let r = global_env();
        assert_eq!(format!("{:?}", r), "global_env()");
        let r = empty_env();
        assert_eq!(format!("{:?}", r), "empty_env()");
        let r = base_env();
        assert_eq!(format!("{:?}", r), "base_env()");
        let r = Environment::new_with_parent(global_env());
        assert_eq!(format!("{:?}", r), "<environment>");
        #[cfg(feature = "non-api")]
        let r = Promise::from_parts(r!(1), global_env())?;
        #[cfg(feature = "non-api")]
        assert_eq!(format!("{:?}", r), "Promise { code: 1, environment: global_env() }");
        let r : Language = lang!("x").try_into()?;
        assert_eq!(format!("{:?}", r), "lang!(sym!(x))");
        let r : Language = lang!("x", 1, 2).try_into()?;
        assert_eq!(format!("{:?}", r), "lang!(sym!(x), 1, 2)");
        let r : Primitive = R!("`+`")?.try_into()?;
        assert_eq!(format!("{:?}", r), "\".Primitive(\\\"+\\\")\"");
        let r : Primitive  = R!("`if`")?.try_into()?;
        assert_eq!(format!("{:?}", r), "\".Primitive(\\\"if\\\")\"");
        let r : Rstr = Rstr::from_string("xyz");
        assert_eq!(format!("{:?}", r), "\"xyz\"");
        let r : Logicals = Logicals::from_values([TRUE]);
        assert_eq!(format!("{:?}", r), "TRUE");
        let r : Logicals = Logicals::from_values([TRUE, FALSE, NA_LOGICAL]);
        assert_eq!(format!("{:?}", r), "[TRUE, FALSE, NA_LOGICAL]");
        let r : Integers = Integers::new(1);
        assert_eq!(format!("{:?}", r), "0");
        let r : Integers = Integers::new(2);
        assert_eq!(format!("{:?}", r), "[0, 0]");
        let r : Doubles = Doubles::new(1);
        assert_eq!(format!("{:?}", r), "0.0");
        let r : Doubles = Doubles::new(2);
        assert_eq!(format!("{:?}", r), "[0.0, 0.0]");
        let r : Strings = Strings::from_values(["xyz"]);
        assert_eq!(format!("{:?}", r), "\"xyz\"");
        let r : Strings = Strings::from_values(["xyz", "abc"]);
        assert_eq!(format!("{:?}", r), "[\"xyz\", \"abc\"]");
        let r : List = list!(a=1, b=2);
        assert_eq!(format!("{:?}", r), "list!(a=1, b=2)");
        let r : List = list!(1, 2);
        assert_eq!(format!("{:?}", r), "list!(1, 2)");
        let r : List = list!(1, b=2);
        assert_eq!(format!("{:?}", r), "list!(1, b=2)");
        let r : Expressions = parse("1 + 2").unwrap();
        assert_eq!(format!("{:?}", r), "Expressions { values: [lang!(sym!(+), 1.0, 2.0)] }");
        let r : Raw = Raw::new(2);
        assert_eq!(format!("{:02x?}", r), "Raw[00, 00]");
        S4::set_class("fred", pairlist!(x="numeric"), r!(()))?;
        let r : S4 = S4::new("fred")?;
        assert_eq!(format!("{:?}", r), "S4");
    }
}

#[test]
fn test_debug_scalar() {
    test! {
        let test_data = vec![(true, "TRUE"), (false, "FALSE")];
        for (val, dbg_str) in test_data {
            assert_eq!(format!("{:?}", Rbool::from(val)), dbg_str);
        }
        assert_eq!(format!("{:?}", Rbool::na()), "NA_LOGICAL");

        let test_data = vec![42, -42, 0];
        for val in test_data {
            assert_eq!(format!("{:?}", Rint::from(val)), format!("{:?}", val),);
        }
        assert_eq!(format!("{:?}", Rint::na()), "NA_INTEGER");

        let test_data = vec![
            42.,
            -42.,
            0.,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::NAN,
            3.141592653589793,
            -3.141592653589793,
        ];
        for val in test_data {
            assert_eq!(format!("{:?}", Rfloat::from(val)), format!("{:?}", val));
        }
        assert_eq!(format!("{:?}", Rfloat::na()), "NA_REAL");

        let test_data = vec![
            (42., 42., "42.0 + 42.0i"),
            (-42., 42., "-42.0 + 42.0i"),
            (42., -42., "42.0 - 42.0i"),
            (-42., -42., "-42.0 - 42.0i"),
            (0., 0., "0.0 + 0.0i"),
        ];
        for (re, im, dbg_str) in test_data {
            assert_eq!(format!("{:?}", Rcplx::new(re, im)), dbg_str);
        }
        assert_eq!(format!("{:?}", Rcplx::na()), "NA_COMPLEX");

        let test_data = vec!["Hello", "World"];
        for str in test_data {
            assert_eq!(format!("{:?}", Rstr::from(str)), format!("{:?}", str));
        }
        assert_eq!(format!("{:?}", Rstr::na()), "NA_CHARACTER");
    }
}

#[test]
fn test_debug_vectors() {
    test! {
        let r: Logicals = Logicals::from_values([TRUE]);
        assert_eq!(format!("{:?}", r), "TRUE");
        let r: Logicals = Logicals::from_values([TRUE, FALSE, NA_LOGICAL]);
        assert_eq!(format!("{:?}", r), "[TRUE, FALSE, NA_LOGICAL]");

        let r: Integers = Integers::new(1);
        assert_eq!(format!("{:?}", r), "0");
        let r: Integers = Integers::from_values([Rint::from(0), Rint::from(0), Rint::na()]);
        assert_eq!(format!("{:?}", r), "[0, 0, NA_INTEGER]");

        let r: Doubles = Doubles::new(1);
        assert_eq!(format!("{:?}", r), "0.0");
        let r: Doubles = Doubles::from_values([Rfloat::from(0.0), Rfloat::from(0.0), Rfloat::na()]);
        assert_eq!(format!("{:?}", r), "[0.0, 0.0, NA_REAL]");

        let r: Strings = Strings::from_values(["xyz"]);
        assert_eq!(format!("{:?}", r), "\"xyz\"");
        let r: Strings = Strings::from_values([Rstr::from("xyz"), Rstr::from("abc"), Rstr::na()]);
        assert_eq!(format!("{:?}", r), "[\"xyz\", \"abc\", NA_CHARACTER]");

        let r: Complexes = Complexes::from_values([Rcplx::new(42.0, -42.0)]);
        assert_eq!(format!("{:?}", r), "42.0 - 42.0i");
        let r: Complexes =
            Complexes::from_values([Rcplx::new(42.0, -42.0), Rcplx::new(0.0, 0.0), Rcplx::na()]);
        assert_eq!(format!("{:?}", r), "[42.0 - 42.0i, 0.0 + 0.0i, NA_COMPLEX]");
    }
}
