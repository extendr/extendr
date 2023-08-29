use extendr_api::prelude::*;

#[derive(Debug, PartialEq)]
struct Success {}

impl TryFrom<&Robj> for Success {
    type Error = Error;

    fn try_from(_: &Robj) -> std::result::Result<Self, Self::Error> {
        Ok(Success {})
    }
}

#[derive(Debug, PartialEq)]
struct Failure {}

impl TryFrom<&Robj> for Failure {
    type Error = Error;

    fn try_from(_: &Robj) -> std::result::Result<Self, Self::Error> {
        Err(Error::Other(String::new()))
    }
}

#[test]
fn try_from_one_match() {
    test! {
        let robj = r!(());

        let left_match = <Either<Success, Failure> as TryFrom<&Robj>>::try_from(&robj);
        let right_match = <Either<Failure, Success> as TryFrom<&Robj>>::try_from(&robj);

        assert_eq!(left_match, Ok(Left(Success{})));
        assert_eq!(right_match, Ok(Right(Success{})));
    }
}

#[test]
fn try_from_both_match_return_left() {
    test! {
        let robj = r!(());

        let both_match = <Either<Success, Success> as TryFrom<&Robj>>::try_from(&robj);

        assert_eq!(both_match, Ok(Left(Success{})));
    }
}

#[test]
fn try_from_none_match_return_error() {
    test! {
        let robj = r!(());

        let none_match = <Either<Failure, Failure> as TryFrom<&Robj>>::try_from(&robj);

        assert_eq!(
            none_match,
            Err(
                Error::EitherError(
                    Box::new(Error::Other(String::new())),
                    Box::new(Error::Other(String::new())))
            )
        );
    }
}
