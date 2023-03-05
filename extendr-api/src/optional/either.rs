use crate::{Error, Robj};
use either::Either::{self, Left, Right};

impl<'a, L, R> TryFrom<&'a Robj> for Either<L, R>
where
    L: TryFrom<&'a Robj, Error = Error>,
    R: TryFrom<&'a Robj, Error = Error>,
{
    type Error = Error;

    /// Returns the first type that matches the provided `Robj`, starting from
    /// `L`-type, and if that fails, then the `R`-type is converted.
    fn try_from(value: &'a Robj) -> Result<Self, Self::Error> {
        match L::try_from(value) {
            Ok(left) => Ok(Left(left)),
            Err(left_err) => match R::try_from(value) {
                Ok(right) => Ok(Right(right)),
                Err(right_err) => Err(Error::EitherError(Box::new(left_err), Box::new(right_err))),
            },
        }
    }
}

impl<L, R> TryFrom<Robj> for Either<L, R>
where
    for<'a> Either<L, R>: TryFrom<&'a Robj, Error = Error>,
{
    type Error = Error;

    /// Returns the first type that matches the provided `Robj`, starting from
    /// `L`-type, and if that fails, then the `R`-type is converted.
    fn try_from(value: Robj) -> Result<Self, Self::Error> {
        (&value).try_into()
    }
}

impl<L, R> From<Either<L, R>> for Robj
where
    Robj: From<L> + From<R>,
{
    fn from(value: Either<L, R>) -> Self {
        match value {
            Left(left) => Robj::from(left),
            Right(right) => Robj::from(right),
        }
    }
}
