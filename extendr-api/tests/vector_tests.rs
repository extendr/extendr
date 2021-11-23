use extendr_api::prelude::*;

#[test]
fn test_strings() {
    test! {
        let s = Strings::new(10);
        assert_eq!(s.len(), 10);
        assert_eq!(s.rtype(), RType::String);

        let mut s = Strings::from_values(["x", "y", "z"]);
        assert_eq!(s.len(), 3);
        assert_eq!(s.rtype(), RType::String);
        assert_eq!(s.elt(0), "x");
        assert_eq!(s.elt(1), "y");
        assert_eq!(s.elt(2), "z");
        assert_eq!(s.elt(3), <&str>::na());

        let v = s.as_slice().iter().map(|c| c.as_str()).collect::<String>();
        assert_eq!(v, "xyz");

        s.set_elt(1, "q");
        assert_eq!(s.elt(1), "q");

        let s : Strings = ["x", "y", "z"].iter().collect();
        let v = s.iter().map(|c| c.as_str()).collect::<String>();
        assert_eq!(v, "xyz");

        let s = Strings::from_values(["x", <&str>::na(), "z"]);
        assert_eq!(s.elt(1).is_na(), true);

        let robj = r!("xyz");
        let s = Strings::try_from(robj)?;
        assert_eq!(s.len(), 1);
        assert_eq!(s.elt(0), "xyz");
    }
}

#[test]
fn test_list() {
    test! {
        let s = List::new(10);
        assert_eq!(s.len(), 10);
        assert_eq!(s.rtype(), RType::List);

        let mut s = List::from_values(["x", "y", "z"]);
        assert_eq!(s.len(), 3);
        assert_eq!(s.rtype(), RType::List);
        assert_eq!(s.elt(0)?, r!("x"));
        assert_eq!(s.elt(1)?, r!("y"));
        assert_eq!(s.elt(2)?, r!("z"));
        assert_eq!(s.elt(3).is_err(), true);

        let v = s.as_slice().iter().map(|c| c).collect::<Vec<_>>();
        assert_eq!(v, vec![&r!("x"), &r!("y"), &r!("z")]);

        s.set_elt(1, r!("q"))?;
        assert_eq!(s.elt(1)?, r!("q"));

        let s : List = ["x", "y", "z"].iter().collect();
        assert_eq!(s, list!("x", "y", "z"));

        let v = list!(a="x", b="y", c="z").iter().collect::<Vec<_>>();
        assert_eq!(v, vec![("a", r!("x")), ("b", r!("y")), ("c", r!("z"))]);

        let s = List::from_values(["x", <&str>::na(), "z"]);
        assert_eq!(s.elt(1)?.is_na(), true);
    }
}
