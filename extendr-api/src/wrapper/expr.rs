use super::*;

#[derive(PartialEq, Clone)]
pub struct Expressions {
    pub(crate) robj: Robj,
}

impl Expressions {
    /// Wrapper for creating Expressions (EXPRSXP) objects.
    pub fn new() -> Self {
        Expressions::from_values([Robj::from(()); 0])
    }

    /// Wrapper for creating Expressions (EXPRSXP) objects.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let expr = r!(Expressions::from_values(&[r!(0), r!(1), r!(2)]));
    ///     assert_eq!(expr.is_expressions(), true);
    ///     assert_eq!(expr.len(), 3);
    /// }
    /// ```
    pub fn from_values<V>(values: V) -> Self
    where
        V: IntoIterator,
        V::IntoIter: ExactSizeIterator,
        V::Item: Into<Robj>,
    {
        Self {
            robj: make_vector(SEXPTYPE::EXPRSXP, values),
        }
    }

    /// Return an iterator over the values of this expression list.
    pub fn values(&self) -> ListIter {
        ListIter::from_parts(self.robj.clone(), 0, self.robj.len())
    }
}

impl std::default::Default for Expressions {
    fn default() -> Self {
        Expressions::new()
    }
}

impl std::fmt::Debug for Expressions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Expressions")
            .field("values", &self.values())
            .finish()
    }
}
