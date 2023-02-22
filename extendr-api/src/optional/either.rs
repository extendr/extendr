use crate::{Error, Robj};
use either::Either::{self, Left, Right};

impl<'a, TLeft, TRight> TryFrom<&'a Robj> for Either<TLeft, TRight>
where
    TLeft: TryFrom<&'a Robj, Error = Error>,
    TRight: TryFrom<&'a Robj, Error = Error>,
{
    type Error = Error;

    fn try_from(value: &'a Robj) -> Result<Self, Self::Error> {
        match TLeft::try_from(value) {
            Ok(left) => Ok(Left(left)),
            Err(left_err) => match TRight::try_from(value) {
                Ok(right) => Ok(Right(right)),
                Err(right_err) => Err(Error::EitherError(Box::new(left_err), Box::new(right_err))),
            },
        }
    }
}

impl<TLeft, TRight> TryFrom<Robj> for Either<TLeft, TRight>
where
    for<'a> Either<TLeft, TRight>: TryFrom<&'a Robj, Error = Error>,
{
    type Error = Error;

    fn try_from(value: Robj) -> Result<Self, Self::Error> {
        (&value).try_into()
    }
}

impl<TLeft, TRight> From<Either<TLeft, TRight>> for Robj
where
    Robj: From<TLeft> + From<TRight>,
{
    fn from(value: Either<TLeft, TRight>) -> Self {
        match value {
            Left(left) => Robj::from(left),
            Right(right) => Robj::from(right),
        }
    }
}
