use super::*;

/// Wrapper for handling potentially NULL values.
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     use extendr_api::wrapper::Nullable::*;
///
///     // Plain integer.
///     let s1 = r!(1);
///     let n1 = <Nullable<i32>>::from_robj(&s1)?;
///     assert_eq!(n1, NotNull(1));
///
///     // NA integer - error.
///     let sna = r!(NA_INTEGER);
///     assert_eq!(<Nullable<i32>>::from_robj(&sna).is_err(), true);
///
///     // NA integer - option gives none.
///     assert_eq!(<Nullable<Option<i32>>>::from_robj(&sna)?, NotNull(None));
///
///     // NULL object.
///     let snull = r!(NULL);
///     let nnull = <Nullable<i32>>::from_robj(&snull)?;
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

impl<'a, T> FromRobj<'a> for Nullable<T>
where
    T: FromRobj<'a>,
{
    /// Convert an object that may be null to a rust type.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let s1 = r!(1);
    ///     let n1 = <Nullable<i32>>::from_robj(&s1)?;
    ///     assert_eq!(n1, Nullable::NotNull(1));
    ///     let snull = r!(NULL);
    ///     let nnull = <Nullable<i32>>::from_robj(&snull)?;
    ///     assert_eq!(nnull, Nullable::Null);
    /// }
    /// ```
    fn from_robj(robj: &'a Robj) -> std::result::Result<Self, &'static str> {
        if robj.is_null() {
            Ok(Nullable::Null)
        } else {
            Ok(Nullable::NotNull(<T>::from_robj(robj)?))
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
