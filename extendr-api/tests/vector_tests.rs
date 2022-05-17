use extendr_api::prelude::*;

#[test]
fn test_strings() {
    test! {
        let s = Strings::new(10);
        assert_eq!(s.len(), 10);
        assert_eq!(s.rtype(), Rtype::Strings);

        let mut s = Strings::from_values(["x", "y", "z"]);
        assert_eq!(s.len(), 3);
        assert_eq!(s.rtype(), Rtype::Strings);
        assert_eq!(s.elt(0), "x");
        assert_eq!(s.elt(1), "y");
        assert_eq!(s.elt(2), "z");
        assert_eq!(s.elt(3), <&str>::na());

        let v = s.as_slice().iter().map(|c| c.as_str()).collect::<String>();
        assert_eq!(v, "xyz");

        s.set_elt(1, Rstr::from("q"));
        assert_eq!(s.elt(1), "q");

        let s : Strings = ["x", "y", "z"].iter().collect();
        let v = s.iter().map(|c| c.as_str()).collect::<String>();
        assert_eq!(v, "xyz");
        assert_eq!(&*s, &["x", "y", "z"]);

        // Strings supports methods of &[Rstr] via Deref.
        assert_eq!(s.contains(&"x".into()), true);

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
        assert_eq!(s.rtype(), Rtype::List);

        let mut s = List::from_values(["x", "y", "z"]);
        assert_eq!(s.len(), 3);
        assert_eq!(s.rtype(), Rtype::List);
        assert_eq!(s.elt(0)?, r!("x"));
        assert_eq!(s.elt(1)?, r!("y"));
        assert_eq!(s.elt(2)?, r!("z"));
        assert_eq!(s.elt(3).is_err(), true);

        let v = s.as_slice().iter().collect::<Vec<_>>();
        assert_eq!(v, vec![&r!("x"), &r!("y"), &r!("z")]);

        s.set_elt(1, r!("q"))?;
        assert_eq!(s.elt(1)?, r!("q"));

        let s : List = ["x", "y", "z"].iter().collect();
        assert_eq!(s, list!("x", "y", "z"));

        let v = list!(a="x", b="y", c="z").iter().collect::<Vec<_>>();
        assert_eq!(v, vec![("a", r!("x")), ("b", r!("y")), ("c", r!("z"))]);

        let s = List::from_values(["x", <&str>::na(), "z"]);
        assert_eq!(s.elt(1)?.is_na(), true);
        assert_eq!(s.as_slice().iter().any(|s| s.is_na()), true);

        // Deref allows all the immutable methods from slice.
        let v = s.as_slice().iter().collect::<Vec<_>>();
        assert_eq!(v, vec![&r!("x"), &r!(<&str>::na()), &r!("z")]);
        assert_eq!(v[0], "x");
        assert_eq!(v[1].is_na(), true);
        assert_eq!(v.contains(&&r!("x")), true);
        assert_eq!(s.as_slice().iter().any(Robj::is_na), true);
    }
}

#[test]
fn test_doubles() {
    test! {
        let s = Doubles::new(10);
        assert_eq!(s.len(), 10);
        assert_eq!(s.rtype(), Rtype::Doubles);

        let mut s = Doubles::from_values([1.0, 2.0, 3.0]);
        assert_eq!(s.len(), 3);
        assert_eq!(s.rtype(), Rtype::Doubles);
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
fn test_complexes() {
    test! {
        let s = Complexes::new(10);
        assert_eq!(s.len(), 10);
        assert_eq!(s.rtype(), Rtype::Complexes);

        let s = Complexes::from_values([1.0, 2.0, 3.0]);
        assert_eq!(s.len(), 3);
        assert_eq!(s.rtype(), Rtype::Complexes);
        assert_eq!(s.elt(0), 1.0);
        assert_eq!(s.elt(1), 2.0);
        assert_eq!(s.elt(2), 3.0);
        assert!(s.elt(3).is_na());

        let v = s.iter().collect::<Vec<Rcplx>>();
        assert_eq!(v, [1.0, 2.0, 3.0]);

        // s.set_elt(1, 5.0.into());
        // assert_eq!(s.elt(1), 5.0);

        let s : Complexes = [1.0, 2.0, 3.0].iter().map(|i| Rcplx::from(*i)).collect();
        let v = s.iter().collect::<Complexes>();
        assert_eq!(v, Complexes::from_values([1.0, 2.0, 3.0]));

        // Bug: from_values should be Into<Rcplx>
        //let s = Complexes::from_values([Rint::from(1), Rint::na(), Rint::from(3)]);
        //assert_eq!(s.elt(1).is_na(), true);

        // let robj = r!([Rcplx::from(1.0), Rcplx::from(2.0), Rcplx::from(3.0)]);
        let robj = r!([(1.0, 0.0), (2.0, 0.0), (3.0, 0.0)]);
        let s = Complexes::try_from(robj)?;
        assert_eq!(s.len(), 3);
        assert_eq!(s.elt(0), 1.0);

        // Test Deref and DerefMut.
        let mut s = Complexes::from_values([1.0, 2.0, 3.0]);
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
        assert_eq!(s.rtype(), Rtype::Integers);

        let mut s = Integers::from_values([1, 2, 3]);
        assert_eq!(s.len(), 3);
        assert_eq!(s.rtype(), Rtype::Integers);
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

#[test]
fn test_rstr() {
    test! {
        let x = Rstr::from_string("xyz");
        // All methods of &str are usable on Rstr.
        assert_eq!(x.contains('y'), true);
        assert_eq!(x.starts_with("xy"), true);
        assert_eq!(x.len(), 3);

        let x : Rstr = "xyz".into();
        assert_eq!(x, "xyz");
    }
}

#[test]
fn test_doubles_from_iterator() {
    test! {
        let vec : Doubles = (0..3).map(|i| (i as f64).into()).collect();
        assert_eq!(vec, Doubles::from_values([0.0, 1.0, 2.0]));
    }
}
#[test]
fn test_doubles_iter_mut() {
    test! {
        let mut vec = Doubles::from_values([0.0, 1.0, 2.0, 3.0]);
        vec.iter_mut().for_each(|v| *v += 1.0);
        assert_eq!(vec, Doubles::from_values([1.0, 2.0, 3.0, 4.0]));
    }
}

#[test]
fn test_doubles_iter() {
    test! {
        let vec = Doubles::from_values([0.0, 1.0, 2.0, 3.0]);
        assert_eq!(vec.iter().sum::<Rfloat>(), 6.0);
    }
}

#[test]
fn test_doubles_from_values_short() {
    test! {
        let vec = Doubles::from_values((0..3).map(|i| 2.0 - i as f64));
        assert_eq!(vec.is_altrep(), false);
        assert_eq!(r!(vec.clone()), r!([2.0, 1.0, 0.0]));
        assert_eq!(vec.elt(1), 1.0);
        let mut dest = [0.0.into(); 2];
        vec.get_region(1, &mut dest);
        assert_eq!(dest, [1.0, 0.0]);
    }
}
#[test]
fn test_doubles_from_values_altrep() {
    test! {
        let vec = Doubles::from_values_altrep((0..1000000000).map(|x| x as f64));
        assert_eq!(vec.is_altrep(), true);
        assert_eq!(vec.elt(12345678), 12345678.0);
        let mut dest = [0.0.into(); 2];
        vec.get_region(12345678, &mut dest);
        assert_eq!(dest, [12345678.0, 12345679.0]);
    }
}

#[test]
fn test_doubles_new() {
    test! {
        let vec = Doubles::new(10);
        assert_eq!(vec.is_real(), true);
        assert_eq!(vec.len(), 10);
    }
}

#[cfg(feature = "num-complex")]
mod num_complex {
    use extendr_api::prelude::*;

    #[test]
    fn from_iterator() {
        test! {
            let vec : Complexes = (0..3).map(|i| (i as f64).into()).collect();
            assert_eq!(vec, Complexes::from_values([0.0, 1.0, 2.0]));
        }
    }
    #[test]
    fn iter_mut() {
        test! {
            let mut vec = Complexes::from_values([0.0, 1.0, 2.0, 3.0]);
            vec.iter_mut().for_each(|v| *v = *v + Rcplx::from(1.0));
            assert_eq!(vec, Complexes::from_values([1.0, 2.0, 3.0, 4.0]));
        }
    }

    #[test]
    fn iter() {
        test! {
            let vec = Complexes::from_values([0.0, 1.0, 2.0, 3.0]);
            assert_eq!(vec.iter().sum::<Rcplx>(), Rcplx::from(6.0));
        }
    }

    #[test]
    fn from_values_short() {
        test! {
            // Short (<64k) vectors are allocated.
            let vec = Complexes::from_values((0..3).map(|i| 2.0 - i as f64));
            assert_eq!(vec.is_altrep(), false);
            assert_eq!(r!(vec.clone()), r!([Rcplx::from(2.0), Rcplx::from(1.0), Rcplx::from(0.0)]));
            assert_eq!(vec.elt(1), Rcplx::from(1.0));
            let mut dest = [0.0.into(); 2];
            vec.get_region(1, &mut dest);
            assert_eq!(dest, [Rcplx::from(1.0), Rcplx::from(0.0)]);
        }
    }
    #[test]
    fn from_values_long() {
        test! {
            // Long (>=64k) vectors are lazy ALTREP objects.
            let vec = Complexes::from_values_altrep((0..1000000000).map(|x| x as f64));
            assert_eq!(vec.is_altrep(), true);
            assert_eq!(vec.elt(12345678), Rcplx::from(12345678.0));
            let mut dest = [0.0.into(); 2];
            vec.get_region(12345678, &mut dest);
            assert_eq!(dest, [Rcplx::from(12345678.0), Rcplx::from(12345679.0)]);
        }
    }

    #[test]
    fn new() {
        test! {
            let vec = Complexes::new(10);
            assert_eq!(vec.is_complex(), true);
            assert_eq!(vec.len(), 10);
        }
    }
}
