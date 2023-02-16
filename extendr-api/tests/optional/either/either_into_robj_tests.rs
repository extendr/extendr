use either::Either::{self, Left, Right};
use extendr_api::prelude::*;
use rstest::rstest;

#[rstest]
#[case("1L", Rint::from(1i32), "")]
#[case("1.5", Rfloat::from(1.5), Rint::default())]
#[case("\"string\"", "string", false)]
#[case("TRUE", true, 0)]
#[case("1 + 2i", Rcplx::new(1.0, 2.0), 0)]
fn return_left<TLeft, TRight>(
    #[case] expected: &'static str,
    #[case] left: TLeft,
    #[case] _right: TRight,
) where
    TLeft: IntoRobj,
    TRight: IntoRobj,
{
    test! {
        let expected = eval_string(expected)?;
        let val : Either<TLeft, TRight> = Left(left);
        let robj = val.into_robj();
        assert_eq!(expected, robj);
    }
}

#[rstest]
#[case("1L", Rint::from(1i32), "")]
#[case("1.5", Rfloat::from(1.5), Rint::default())]
#[case("\"string\"", "string", false)]
#[case("TRUE", true, 0)]
#[case("1 + 2i", Rcplx::new(1.0, 2.0), 0)]
fn return_right<TLeft, TRight>(
    #[case] expected: &'static str,
    #[case] right: TRight,
    #[case] _left: TLeft,
) where
    TLeft: IntoRobj,
    TRight: IntoRobj,
{
    test! {
        let expected = eval_string(expected)?;
        let val : Either<TLeft, TRight> = Right(right);
        let robj = val.into_robj();
        assert_eq!(expected, robj);
    }
}
#[test]
fn return_integers() {
    test! {
        let val : Either<Integers, Doubles> = Left(
            Integers::from_values(
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
            )
        );
        let robj = val.into_robj();
        assert_eq!(eval_string("1:10")?, robj);
    }
}

#[test]
fn return_doubles() {
    test! {
        let val : Either<Integers, Doubles> = Right(
            Doubles::from_values(
                vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]
            )
        );
        let robj = val.into_robj();
        assert_eq!(eval_string("c(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)")?, robj);
    }
}
