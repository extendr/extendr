use either::Either::{self, Left, Right};
use extendr_api::optional::either::*;
use extendr_api::prelude::*;
use rstest::rstest;

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
