/*!
Enables support for the [`either`](https://docs.rs/either/latest/either/) crate,
to allow accepting and returning `Either<L, R>` values if both `L` and `R` are convertible to/from `Robj`.

`either` crate support is currently available in the dev version of `extendr-api`
and requires enabling `either` feature:

```toml
[dependencies]
extendr-api = { git = "https://github.com/extendr/extendr" , features = ["either"] }
```

```rust
use extendr_api::prelude::*;

#[extendr]
fn accept_numeric(input : Either<Integers, Doubles>) {}
```

Here is an example of `either` usage -- a type-aware sum:
```rust
use extendr_api::prelude::*;

#[extendr]
fn type_aware_sum(input : Either<Integers, Doubles>) -> Either<Rint, Rfloat> {
    match input {
        Left(ints) => Left(ints.iter().sum::<Rint>()),
        Right(dbls) => Right(dbls.iter().sum::<Rfloat>()),
    }
}
```
*/
use crate::prelude::*;
use crate::{Error, Robj};

impl<'a, L, R> TryFrom<&'a Robj> for Either<L, R>
where
    L: TryFrom<&'a Robj, Error = Error>,
    R: TryFrom<&'a Robj, Error = Error>,
{
    type Error = Error;

    /// Returns the first type that matches the provided `Robj`, starting from
    /// `L`-type, and if that fails, then the `R`-type is converted.
    fn try_from(value: &'a Robj) -> Result<Self> {
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
    fn try_from(value: Robj) -> Result<Self> {
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
