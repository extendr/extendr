use super::*;

/// Wrapper for creating functions (CLOSSXP).
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     // Closures are functions.
///     let expr = R!("function(a = 1, b) {c <- a + b}")?;
///     let func = expr.as_function().unwrap();
///
///     let expected_formals = Pairlist::from_pairs(vec![("a", r!(1.0)), ("b", missing_arg().into())]);
///     let expected_body = lang!(
///         "{", lang!("<-", sym!(c), lang!("+", sym!(a), sym!(b))));
///     assert_eq!(func.formals().unwrap(), expected_formals);
///     assert_eq!(func.body().unwrap(), expected_body);
///     assert_eq!(func.environment().unwrap(), global_env());
///
///     // Primitives can also be functions.
///     let expr = R!("`~`")?;
///     let func = expr.as_function().unwrap();
///     assert_eq!(func.formals(), None);
///     assert_eq!(func.body(), None);
///     assert_eq!(func.environment(), None);
/// }
/// ```
#[derive(PartialEq, Clone)]
pub struct Function {
    pub(crate) robj: Robj,
}

impl Function {
    #[cfg(feature = "non-api")]
    /// Make a function from parts.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let formals = pairlist!(a=NULL);
    ///     let body = lang!("+", sym!(a), r!(1)).try_into()?;
    ///     let env = global_env();
    ///     let f = r!(Function::from_parts(formals, body, env )?);
    ///     assert_eq!(f.call(pairlist!(a=1))?, r!(2));
    /// }
    /// ```
    pub fn from_parts(formals: Pairlist, body: Language, env: Environment) -> Result<Self> {
        single_threaded(|| unsafe {
            let sexp = Rf_allocSExp(SEXPTYPE::CLOSXP);
            let robj = Robj::from_sexp(sexp);
            SET_FORMALS(sexp, formals.get());
            SET_BODY(sexp, body.get());
            SET_CLOENV(sexp, env.get());
            Ok(Function { robj })
        })
    }

    /// Do the equivalent of x(a, b, c)
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let function = R!("function(a, b) a + b").unwrap().as_function().unwrap();
    ///     assert_eq!(function.call(pairlist!(a=1, b=2)).unwrap(), r!(3));
    /// }
    /// ```
    pub fn call(&self, args: Pairlist) -> Result<Robj> {
        single_threaded(|| unsafe {
            let call = Robj::from_sexp(Rf_lcons(self.get(), args.get()));
            call.eval()
        })
    }

    /// Get the formal arguments of the function or None if it is a primitive.
    pub fn formals(&self) -> Option<Pairlist> {
        unsafe {
            if self.rtype() == Rtype::Function {
                let sexp = self.robj.get();
                Some(Robj::from_sexp(FORMALS(sexp)).try_into().unwrap())
            } else {
                None
            }
        }
    }

    /// Get the body of the function or None if it is a primitive.
    pub fn body(&self) -> Option<Robj> {
        unsafe {
            if self.rtype() == Rtype::Function {
                let sexp = self.robj.get();
                Some(Robj::from_sexp(BODY(sexp)))
            } else {
                None
            }
        }
    }

    /// Get the environment of the function or None if it is a primitive.
    pub fn environment(&self) -> Option<Environment> {
        unsafe {
            if self.rtype() == Rtype::Function {
                let sexp = self.robj.get();
                Some(
                    Robj::from_sexp(CLOENV(sexp))
                        .try_into()
                        .expect("Should be an environment"),
                )
            } else {
                None
            }
        }
    }
}

impl std::fmt::Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.deparse().unwrap())
    }
}
