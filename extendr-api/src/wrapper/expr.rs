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
    ///     let expr = r!(Expression::from_values(&[r!(0), r!(1), r!(2)]));
    ///     assert_eq!(expr.is_expression(), true);
    ///     assert_eq!(expr.len(), 3);
    ///     assert_eq!(format!("{:?}", expr), r#"r!(Expression::from_values([r!(0), r!(1), r!(2)]))"#);
    /// }
    /// ```
    pub fn from_values<V>(values: V) -> Self
    where
        V: IntoIterator,
        V::IntoIter: ExactSizeIterator,
        V::Item: Into<Robj>,
    {
        Self {
            robj: make_vector(EXPRSXP, values),
        }
    }

    /// Return an iterator over the values of this expression list.
    pub fn values(&self) -> ListIter {
        ListIter::from_parts(self.robj.clone(), 0, self.robj.len())
    }
}
