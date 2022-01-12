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
        let r = Promise::from_parts(r!(1), global_env())?;
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
