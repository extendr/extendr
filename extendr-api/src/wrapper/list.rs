use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct List {
    pub(crate) robj: Robj,
}

impl List {
    /// Wrapper for creating list (VECSXP) objects.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let list = r!(List::from_objects(&[r!(0), r!(1), r!(2)]));
    ///     assert_eq!(list.is_list(), true);
    ///     assert_eq!(list.len(), 3);
    ///     assert_eq!(format!("{:?}", list), r#"r!(List::from_objects([r!(0), r!(1), r!(2)]))"#);
    /// }
    /// ```
    pub fn from_objects<V>(values: V) -> Self
    where
        V: IntoIterator,
        V::IntoIter: ExactSizeIterator,
        V::Item: Into<Robj>,
    {
        Self {
            robj: make_vector(VECSXP, values),
        }
    }
}
