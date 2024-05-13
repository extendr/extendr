use super::*;

/// Wrapper for handling potentially NULL values.
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     use extendr_api::wrapper::Nullable::*;
///
///     // Plain integer.
///     let s1 = r!(1);
///     let n1 = <Nullable<i32>>::try_from(&s1)?;
///     assert_eq!(n1, NotNull(1));
///
///     // NA integer - error.
///     let sna = r!(NA_INTEGER);
///     assert_eq!(<Nullable<i32>>::try_from(&sna).is_err(), true);
///
///     // NA integer - option gives none.
///     assert_eq!(<Nullable<Option<i32>>>::try_from(&sna)?, NotNull(None));
///
///     // NULL object.
///     let snull = r!(NULL);
///     let nnull = <Nullable<i32>>::try_from(&snull)?;
///     assert_eq!(nnull, Null);
///
///     assert_eq!(r!(Nullable::<i32>::Null), r!(NULL));
///     assert_eq!(r!(Nullable::<i32>::NotNull(1)), r!(1));
/// }
/// ```
#[derive(Debug, PartialEq, Clone)]
pub enum Nullable<T> {
    NotNull(T),
    Null,
}

impl<T> TryFrom<Robj> for Nullable<T>
where
    T: TryFrom<Robj, Error = Error>,
{
    type Error = Error;

    /// Convert an object that may be null to a rust type.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let s1 = r!(1);
    ///     let n1 = <Nullable<i32>>::try_from(s1)?;
    ///     assert_eq!(n1, Nullable::NotNull(1));
    ///     let snull = r!(NULL);
    ///     let nnull = <Nullable<i32>>::try_from(snull)?;
    ///     assert_eq!(nnull, Nullable::Null);
    /// }
    /// ```
    fn try_from(robj: Robj) -> std::result::Result<Self, Self::Error> {
        if robj.is_null() {
            Ok(Nullable::Null)
        } else {
            Ok(Nullable::NotNull(robj.try_into()?))
        }
    }
}

impl<'a, T> TryFrom<&'a Robj> for Nullable<T>
where
    T: TryFrom<&'a Robj, Error = Error>,
{
    type Error = Error;

    /// Convert an object that may be null to a rust type.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let s1 = r!(1);
    ///     let n1 = <Nullable<i32>>::try_from(&s1)?;
    ///     assert_eq!(n1, Nullable::NotNull(1));
    ///     let snull = r!(NULL);
    ///     let nnull = <Nullable<i32>>::try_from(&snull)?;
    ///     assert_eq!(nnull, Nullable::Null);
    /// }
    /// ```
    fn try_from(robj: &'a Robj) -> std::result::Result<Self, Self::Error> {
        if robj.is_null() {
            Ok(Nullable::Null)
        } else {
            Ok(Nullable::NotNull(robj.try_into()?))
        }
    }
}

impl<T> From<Nullable<T>> for Robj
where
    T: Into<Robj>,
{
    /// Convert a rust object to NULL or another type.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     assert_eq!(r!(Nullable::<i32>::Null), r!(NULL));
    ///     assert_eq!(r!(Nullable::<i32>::NotNull(1)), r!(1));
    /// }
    /// ```
    fn from(val: Nullable<T>) -> Self {
        match val {
            Nullable::NotNull(t) => t.into(),
            Nullable::Null => r!(NULL),
        }
    }
}

impl<T> From<Nullable<T>> for Option<T>
where
    T: TryFrom<Robj, Error = Error>,
{
    /// Convert a Nullable type into Option
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     assert_eq!(<Option<i32>>::from(Nullable::Null), None);
    ///     assert_eq!(<Option<i32>>::from(Nullable::NotNull(42)), Some(42));
    /// }
    /// ```
    fn from(value: Nullable<T>) -> Self {
        match value {
            Nullable::NotNull(value) => Some(value),
            _ => None,
        }
    }
}

impl<'a, T> From<&'a Nullable<T>> for Option<&'a T>
where
    T: TryFrom<Robj, Error = Error>,
{
    /// Convert a Nullable reference type into Option containing reference
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     assert_eq!(<Option<&i32>>::from(&Nullable::Null), None);
    ///     assert_eq!(<Option<&i32>>::from(&Nullable::NotNull(42)), Some(&42));
    /// }
    /// ```
    fn from(value: &'a Nullable<T>) -> Self {
        match value {
            Nullable::NotNull(value) => Some(value),
            _ => None,
        }
    }
}

impl<T> From<Option<T>> for Nullable<T>
where
    T: Into<Robj>,
{
    /// Convert an Option into Nullable type
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let x : Nullable<_> = From::<Option<i32>>::from(None);
    ///     assert_eq!(x, Nullable::<i32>::Null);
    ///     let x : Nullable<_> = From::<Option<i32>>::from(Some(42));
    ///     assert_eq!(x, Nullable::<i32>::NotNull(42));
    /// }
    /// ```
    fn from(value: Option<T>) -> Self {
        match value {
            Some(value) => Nullable::NotNull(value),
            _ => Nullable::Null,
        }
    }
}

impl<T> Nullable<T>
where
    T: TryFrom<Robj, Error = Error>,
{
    /// Convert Nullable R object into `Option`
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     assert_eq!(Nullable::<Rint>::Null.into_option(), None);
    ///     assert_eq!(Nullable::<Rint>::NotNull(Rint::from(42)).into_option(), Some(Rint::from(42)));
    /// }
    /// ```
    pub fn into_option(self) -> Option<T> {
        self.into()
    }
}
impl<T> Nullable<T> {
    /// Map `Nullable<T>` into `Nullable<U>`
    ///
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     assert_eq!(Nullable::<Rfloat>::Null.map(|x| x.abs()), Nullable::<Rfloat>::Null);
    ///     assert_eq!(Nullable::<Rfloat>::NotNull(Rfloat::from(42.0)).map(|x| x.abs()), Nullable::<Rfloat>::NotNull(Rfloat::from(42.0)));
    /// }
    /// ```
    pub fn map<F, U>(self, f: F) -> Nullable<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Nullable::NotNull(value) => Nullable::NotNull(f(value)),
            _ => Nullable::Null,
        }
    }
}
