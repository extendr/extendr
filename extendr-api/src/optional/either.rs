use crate::{Error, IntoRobj, Robj};
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

impl<TLeft, TRight> IntoRobj for Either<TLeft, TRight>
where
    TLeft: IntoRobj,
    TRight: IntoRobj,
{
    fn into_robj(self) -> Robj {
        match self {
            Left(left) => left.into_robj(),
            Right(right) => right.into_robj(),
        }
    }
}
