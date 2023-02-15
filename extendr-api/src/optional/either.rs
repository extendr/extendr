use crate::{IntoRobj, Robj};
use either::Either::{self, Left, Right};

impl<'a, TLeft, TRight> TryFrom<&'a Robj> for Either<TLeft, TRight>
where
    TLeft: TryFrom<&'a Robj, Error = crate::Error>,
    TRight: TryFrom<&'a Robj, Error = crate::Error>,
{
    type Error = crate::Error;

    fn try_from(value: &'a Robj) -> Result<Self, Self::Error> {
        if let Ok(left) = TLeft::try_from(value) {
            Ok(Left(left))
        } else if let Ok(right) = TRight::try_from(value) {
            Ok(Right(right))
        } else {
            Err(crate::Error::Other("todo".to_string()))
        }
    }
}

impl<TLeft, TRight> TryFrom<Robj> for Either<TLeft, TRight>
where
    for<'a> Either<TLeft, TRight>: TryFrom<&'a Robj, Error = crate::Error>,
{
    type Error = crate::Error;

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
