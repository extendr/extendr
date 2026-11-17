#![cfg(r_4_6)]
use extendr_api::{
    dots::{Dot, Dots},
    prelude::*,
};

#[test]
fn dots_len_and_names() {
    test! {
        let dots = Dots::try_from(eval_string("(function(...) environment())(a = 1, 2)")?)?;
        assert_eq!(dots.len(), 2);
        assert!(!dots.is_empty());
        assert_eq!(dots.names(), Some(Strings::from_values(["a", ""])));
    }
}

#[test]
fn dots_empty() {
    test! {
        let dots = Dots::try_from(eval_string("(function(...) environment())()")?)?;
        assert!(dots.is_empty());
        assert_eq!(dots.names(), None);
        assert_eq!(dots.get(0), None);
        assert_eq!(dots.into_iter().count(), 0);
    }
}

#[test]
fn dots_get_by_position_and_name() {
    test! {
        let dots = Dots::try_from(eval_string("(function(...) environment())(a = 1, 2)")?)?;
        assert_eq!(dots.get(0), dots.get("a"));
        assert_eq!(dots.get(0), dots.get(String::from("a")));
        assert!(dots.get(1).is_some());
        assert_eq!(dots.get(2), None);
        assert_eq!(dots.get("b"), None);
        assert_eq!(dots.get(""), None);
    }
}

#[test]
fn dot_types() {
    test! {
        let dots = Dots::try_from(eval_string("(function(...) environment())(1, , stop('boom'))")?)?;
        assert!(matches!(dots.get(0), Some(Dot::DelayedExpression { .. })));
        assert_eq!(dots.get(1), Some(Dot::Missing));

        let dots = Dots::try_from(eval_string("(function(...) { ..1; environment() })(1)")?)?;
        assert!(matches!(dots.get(0), Some(Dot::ForcedExpression { .. })));

        let dots = Dots::try_from(eval_string(
            "f <- function(...) environment(); compiler::cmpfun(function() f(1))()",
        )?)?;
        assert_eq!(dots.get(0), Some(Dot::Value(Robj::from(1.0))));
    }
}

#[test]
fn dot_value() {
    test! {
        let dots = Dots::try_from(eval_string("(function(...) environment())(1, , stop('boom'))")?)?;
        let Some(first) = dots.get(0) else { panic!("no first element") };
        assert_eq!(first.value()?, Some(Robj::from(1.0)));
        assert!(matches!(dots.get(0), Some(Dot::ForcedExpression { .. })));

        assert_eq!(Dot::Missing.value()?, None);

        let Some(third) = dots.get(2) else { panic!("no third element") };
        assert!(third.value().is_err());
    }
}

#[test]
fn dots_iter() {
    test! {
        let dots = Dots::try_from(eval_string("(function(...) environment())(1, 2, 3)")?)?;
        let iter = dots.into_iter();
        assert_eq!(iter.len(), 3);
        let values = iter.map(|dot| dot.value()).collect::<Result<Vec<_>, _>>()?;
        assert_eq!(values, vec![Some(Robj::from(1.0)), Some(Robj::from(2.0)), Some(Robj::from(3.0))]);
    }
}

#[test]
fn dots_try_from_and_find() {
    test! {
        assert!(Dots::try_from(eval_string("(function(...) environment())(1)")?).is_ok());
        assert!(Dots::try_from(Robj::from(1.0)).is_err());
        assert!(Dots::try_from(eval_string("new.env()")?).is_err());
        assert_eq!(Dots::find(), None);
    }
}
