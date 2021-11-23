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

#[test]
fn test_doubles() {
    test! {
        let s = Doubles::new(10);
        assert_eq!(s.len(), 10);
        assert_eq!(s.rtype(), RType::Real);

        let mut s = Doubles::from_values([1, 2, 3]);
        assert_eq!(s.len(), 3);
        assert_eq!(s.rtype(), RType::Real);
        assert_eq!(s.elt(0), 1.0);
        assert_eq!(s.elt(1), 2.0);
        assert_eq!(s.elt(2), 3.0);
        assert!(s.elt(3).is_na());

        let v = s.iter().collect::<Vec<Rfloat>>();
        assert_eq!(v, [1.0, 2.0, 3.0]);

        s.set_elt(1, 5.0.into());
        assert_eq!(s.elt(1), 5.0);

        let s : Doubles = [1.0, 2.0, 3.0].iter().map(|i| Rfloat::from(*i)).collect();
        let v = s.iter().collect::<Doubles>();
        assert_eq!(v, Doubles::from_values([1.0, 2.0, 3.0]));

        // Bug: from_values should be Into<Rfloat>
        //let s = Doubles::from_values([Rint::from(1), Rint::na(), Rint::from(3)]);
        //assert_eq!(s.elt(1).is_na(), true);

        let robj = r!([1.0, 2.0, 3.0]);
        let s = Doubles::try_from(robj)?;
        assert_eq!(s.len(), 3);
        assert_eq!(s.elt(0), 1.0);

        // Test Deref and DerefMut.
        let mut s = Doubles::from_values([1.0, 2.0, 3.0]);
        assert_eq!(s[0], 1.0);
        assert_eq!(s[1], 2.0);
        assert_eq!(s[2], 3.0);

        s[0] = 4.0.into();
        s[1] = 5.0.into();
        s[2] = 6.0.into();
        assert_eq!(s[0], 4.0);
        assert_eq!(s[1], 5.0);
        assert_eq!(s[2], 6.0);
    }
}

#[test]
fn test_integers() {
    test! {
        let s = Integers::new(10);
        assert_eq!(s.len(), 10);
        assert_eq!(s.rtype(), RType::Integer);

        let mut s = Integers::from_values([1, 2, 3]);
        assert_eq!(s.len(), 3);
        assert_eq!(s.rtype(), RType::Integer);
        assert_eq!(s.elt(0), 1);
        assert_eq!(s.elt(1), 2);
        assert_eq!(s.elt(2), 3);
        assert!(s.elt(3).is_na());

        let v = s.iter().collect::<Vec<Rint>>();
        assert_eq!(v, [1, 2, 3]);

        s.set_elt(1, 5.into());
        assert_eq!(s.elt(1), 5);

        let s : Integers = [1, 2, 3].iter().map(|i| Rint::from(*i)).collect();
        let v = s.iter().collect::<Integers>();
        assert_eq!(v, Integers::from_values([1, 2, 3]));

        // Bug: from_values should be Into<Rint>
        //let s = Integers::from_values([Rint::from(1), Rint::na(), Rint::from(3)]);
        //assert_eq!(s.elt(1).is_na(), true);

        let robj = r!([1, 2, 3]);
        let s = Integers::try_from(robj)?;
        assert_eq!(s.len(), 3);
        assert_eq!(s.elt(0), 1);

        // Test Deref and DerefMut.
        let mut s = Integers::from_values([1, 2, 3]);
        assert_eq!(s[0], 1);
        assert_eq!(s[1], 2);
        assert_eq!(s[2], 3);

        s[0] = 4.into();
        s[1] = 5.into();
        s[2] = 6.into();
        assert_eq!(s[0], 4);
        assert_eq!(s[1], 5);
        assert_eq!(s[2], 6);
    }
}

