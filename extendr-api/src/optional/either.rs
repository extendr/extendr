/*!
Enables support for the [`either`](https://docs.rs/either/latest/either/) crate,
to allow accepting and returning `Either<L, R>` values if both `L` and `R` are convertible to/from `RObj`.

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
fn type_aware_sum(input : Either<Integers, Doubles>) -> Either<RInt, RFloat> {
    match input {
        Left(ints) => Left(ints.iter().sum::<RInt>()),
        Right(dbls) => Right(dbls.iter().sum::<RFloat>()),
    }
}
```
*/
use crate::prelude::*;
use crate::{Error, RObj, Result};

impl<'a, L, R> TryFrom<&'a RObj> for Either<L, R>
where
    L: TryFrom<&'a RObj, Error = Error>,
    R: TryFrom<&'a RObj, Error = Error>,
{
    type Error = Error;

    /// Returns the first type that matches the provided `RObj`, starting from
    /// `L`-type, and if that fails, then the `R`-type is converted.
    fn try_from(value: &'a RObj) -> Result<Self> {
        match L::try_from(value) {
            Ok(left) => Ok(Left(left)),
            Err(left_err) => match R::try_from(value) {
                Ok(right) => Ok(Right(right)),
                Err(right_err) => Err(Error::EitherError(Box::new(left_err), Box::new(right_err))),
            },
        }
    }
}

impl<L, R> TryFrom<RObj> for Either<L, R>
where
    for<'a> Either<L, R>: TryFrom<&'a RObj, Error = Error>,
{
    type Error = Error;

    /// Returns the first type that matches the provided `RObj`, starting from
    /// `L`-type, and if that fails, then the `R`-type is converted.
    fn try_from(value: RObj) -> Result<Self> {
        (&value).try_into()
    }
}

impl<L, R> From<Either<L, R>> for RObj
where
    RObj: From<L> + From<R>,
{
    fn from(value: Either<L, R>) -> Self {
        match value {
            Left(left) => RObj::from(left),
            Right(right) => RObj::from(right),
        }
    }
}
