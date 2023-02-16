use either::Either::{self, Left, Right};
use extendr_api::prelude::*;
use rstest::rstest;

#[rstest]
#[case("1L", Rint::from(1i32), "")]
#[case("1.5", Rfloat::from(1.5), Rint::default())]
#[case("\"string\"", "string", false)]
#[case("TRUE", true, 0)]
#[case("1 + 2i", Rcplx::new(1.0, 2.0), 0)]
fn expect_left<TLeft, TRight>(
    #[case] src: &'static str,
    #[case] left: TLeft,
    #[case] _right: TRight,
) where
    for<'a> TLeft: TryFrom<&'a Robj, Error = Error> + PartialEq + std::fmt::Debug,
    for<'a> TRight: TryFrom<&'a Robj, Error = Error> + PartialEq + std::fmt::Debug,
{
    test! {
        let val = eval_string(src)?;
        let val = Either::<TLeft, TRight>::try_from(&val)?;
        assert_eq!(val, Left(left));
    }
}

#[rstest]
#[case("1L", Rint::from(1i32), "")]
#[case("1.5", Rfloat::from(1.5), Rint::default())]
#[case("\"string\"", "string", false)]
#[case("TRUE", true, 0)]
#[case("1 + 2i", Rcplx::new(1.0, 2.0), 0)]
fn expect_right<TLeft, TRight>(
    #[case] src: &'static str,
    #[case] right: TRight,
    #[case] _left: TLeft,
) where
    for<'a> TLeft: TryFrom<&'a Robj, Error = Error> + PartialEq + std::fmt::Debug,
    for<'a> TRight: TryFrom<&'a Robj, Error = Error> + PartialEq + std::fmt::Debug,
{
    test! {
        let val = eval_string(src)?;
        let val = Either::<TLeft, TRight>::try_from(&val)?;
        assert_eq!(val, Right(right));
    }
}

#[test]
fn match_integers() {
    test! {
        let val = R!("1:10")?;
        let val = Either::<Integers, Doubles>::try_from(&val)?;
        assert_eq!(val, Left(Integers::from_values(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])));
    }
}

#[test]
fn match_doubles() {
    test! {
        let val = R!("c(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)")?;
        let val = Either::<Integers, Doubles>::try_from(&val)?;
        assert_eq!(val, Right(Doubles::from_values(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0])));
    }
}
