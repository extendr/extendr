use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Expression {
    pub(crate) robj: Robj,
}

impl Expression {
    /// Wrapper for creating Expression (EXPRSXP) objects.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let expr = r!(Expression::from_objects(&[r!(0), r!(1), r!(2)]));
    ///     assert_eq!(expr.is_expression(), true);
    ///     assert_eq!(expr.len(), 3);
    ///     assert_eq!(format!("{:?}", expr), r#"r!(Expression::from_objects([r!(0), r!(1), r!(2)]))"#);
    /// }
    /// ```
    pub fn from_objects<V>(values: V) -> Self
    where
        V: IntoIterator,
        V::IntoIter: ExactSizeIterator,
        V::Item: Into<Robj>,
    {
        Self {
            robj: make_vector(EXPRSXP, values),
        }
    }
}
