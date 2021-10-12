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
