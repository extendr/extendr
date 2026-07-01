use super::*;

#[derive(PartialEq, Clone)]
pub struct Expressions {
    pub(crate) robj: RObj,
}

impl Expressions {
    /// Wrapper for creating Expressions (EXPRSXP) objects.
    pub fn new() -> Self {
        Expressions::from_values([()])
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
        V::Item: Into<RObj>,
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

impl std::str::FromStr for Expressions {
    type Err = Error;

    fn from_str(code: &str) -> Result<Expressions> {
        single_threaded(|| unsafe {
            use extendr_ffi::{ParseStatus, R_NilValue, R_ParseVector};
            let mut status = ParseStatus::PARSE_NULL;
            let status_ptr = (&mut status) as *mut _;
            let codeobj: RObj = code.into();
            let parsed = RObj::from_sexp(R_ParseVector(codeobj.get(), -1, status_ptr, R_NilValue));
            match status {
                ParseStatus::PARSE_OK => parsed.try_into(),
                _ => Err(Error::ParseError {
                    status,
                    code: code.into(),
                }),
            }
        })
    }
}
