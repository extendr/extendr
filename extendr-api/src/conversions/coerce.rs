use crate::error::Result;
use crate::robj::{Rinternals, Types};
use crate::{Error, Robj};
use extendr_ffi::SEXPTYPE;

/// Best-effort conversion wrapper that asks R to coerce values to primitives first.
///
/// `Coerce<T>` first attempts the standard `TryFrom<&Robj>` conversion for `T`.
/// If that fails, it retries after letting R coerce the input into a handful of
/// primitive vector types (`LGLSXP`, `INTSXP`, `REALSXP`, `RAWSXP`). This lets
/// callers lean on R's own coercion rules (e.g. `TRUE` to `1L`, `1.9` to `1L`,
/// integers to raw bytes) before applying the regular extendr conversions. The
/// backing [`Robj`] is retained so borrowed outputs stay alive for as long as
/// the `Coerce` wrapper is kept around.
pub struct Coerce<T> {
    value: T,
    backing: Robj,
}

impl<T> Coerce<T> {
    /// Extract the inner coerced value.
    ///
    /// For borrowed outputs keep the returned [`Robj`] from [`into_parts`] or
    /// hold on to the `Coerce` wrapper so the backing allocation stays alive.
    pub fn into_inner(self) -> T {
        self.value
    }

    /// Split into the coerced value and the backing [`Robj`].
    pub fn into_parts(self) -> (T, Robj) {
        (self.value, self.backing)
    }

    /// Get a shared reference to the coerced value.
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Get the [`Robj`] that was used to produce this value.
    pub fn backing(&self) -> &Robj {
        &self.backing
    }
}

impl<T> From<Coerce<T>> for T {
    fn from(value: Coerce<T>) -> Self {
        value.value
    }
}

impl<T> std::ops::Deref for Coerce<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> TryFrom<Robj> for Coerce<T>
where
    T: for<'a> TryFrom<&'a Robj, Error = Error>,
{
    type Error = Error;

    fn try_from(robj: Robj) -> Result<Self> {
        try_coerce_owned(robj)
    }
}

impl<'a, T> TryFrom<&'a Robj> for Coerce<T>
where
    T: for<'b> TryFrom<&'b Robj, Error = Error>,
{
    type Error = Error;

    fn try_from(robj: &'a Robj) -> Result<Self> {
        try_coerce_owned(robj.clone())
    }
}

fn try_coerce_owned<T>(robj: Robj) -> Result<Coerce<T>>
where
    T: for<'a> TryFrom<&'a Robj, Error = Error>,
{
    let mut attempts = Vec::new();

    match T::try_from(&robj) {
        Ok(value) => {
            return Ok(Coerce {
                value,
                backing: robj,
            })
        }
        Err(err) => attempts.push(format!("original: {err}")),
    }

    for (sexptype, label) in primitive_targets() {
        let coerced = robj.coerce_vector(*sexptype);
        match T::try_from(&coerced) {
            Ok(value) => {
                return Ok(Coerce {
                    value,
                    backing: coerced,
                })
            }
            Err(err) => attempts.push(format!("{label}: {err}")),
        }
    }

    Err(Error::Other(format!(
        "Could not coerce {:?} into target type; attempts failed: {}",
        robj.rtype(),
        attempts.join(", ")
    )))
}

const fn primitive_targets() -> &'static [(SEXPTYPE, &'static str)] {
    &[
        (SEXPTYPE::LGLSXP, "logical"),
        (SEXPTYPE::INTSXP, "integer"),
        (SEXPTYPE::REALSXP, "double"),
        (SEXPTYPE::RAWSXP, "raw"),
    ]
}

#[cfg(test)]
mod tests {
    use super::Coerce;
    use crate as extendr_api;
    use crate::prelude::*;

    #[test]
    fn coerce_non_integerish_real_into_int() {
        test! {
            let source = r!(1.7);
            let coerced: Coerce<i32> = source.try_into()?;
            assert_eq!(coerced.into_inner(), 1);
        }
    }

    #[test]
    fn coerce_logical_into_int() {
        test! {
            let source = r!(TRUE);
            let coerced: Coerce<i32> = source.try_into()?;
            assert_eq!(coerced.into_inner(), 1);
        }
    }

    #[test]
    fn coerce_integers_into_raw_bytes() {
        test! {
            let source = r!([1, 2, 255]);
            let coerced: Coerce<Vec<u8>> = source.try_into()?;
            assert_eq!(coerced.into_inner(), vec![1, 2, 255]);
        }
    }

    #[test]
    fn coerce_stores_backing_for_borrowed_outputs() {
        test! {
            let source = r!(TRUE);
            let coerced: Coerce<&[i32]> = source.try_into()?;
            assert_eq!(coerced.len(), 1);
            assert_eq!(coerced[0], 1);
        }
    }
}
