use super::*;

/// Wrapper for creating list (VECSXP) objects.
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let list = r!(List(&[r!(0), r!(1), r!(2)]));
///     assert_eq!(list.is_list(), true);
///     assert_eq!(list.len(), 3);
///     assert_eq!(format!("{:?}", list), r#"r!(List([r!(0), r!(1), r!(2)]))"#);
/// }
/// ```
///
/// Note: you can use the [list!] macro for named lists.
#[derive(Debug, PartialEq, Clone)]
pub struct List<T>(pub T);

impl<T> From<List<T>> for Robj
where
    T: IntoIterator,
    T::IntoIter: ExactSizeIterator,
    T::Item: Into<Robj>,
{
    /// Make a list object from an iterator of Robjs.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let list_of_ints = r!(List(&[1, 2]));
    ///     assert_eq!(list_of_ints.len(), 2);
    /// }
    /// ``````
    fn from(val: List<T>) -> Self {
        make_vector(VECSXP, val.0)
    }
}
