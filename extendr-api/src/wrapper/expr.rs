use super::*;

/// Wrapper for creating expression objects.
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let expr = r!(Expr(&[r!(1.), r!("xyz")]));
///     assert_eq!(expr.len(), 2);
/// }
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct Expr<T>(pub T);

impl<T> From<Expr<T>> for Robj
where
    T: IntoIterator,
    T::IntoIter: ExactSizeIterator,
    T::Item: Into<Robj>,
{
    /// Make an expression object from an iterator of Robjs.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let list_of_ints = r!(Expr(&[1, 2]));
    ///     assert_eq!(list_of_ints.len(), 2);
    /// }
    /// ```
    fn from(val: Expr<T>) -> Self {
        make_vector(EXPRSXP, val.0)
    }
}
