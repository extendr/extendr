use super::*;

/// Wrapper for creating functions (CLOSSXP).
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let expr = R!(function(a = 1, b) {c <- a + b}).unwrap();
///     let func = expr.as_function().unwrap();
///
///     let expected_formals = Pairlist::from_pairs(vec![("a", r!(1.0)), ("b", missing_arg().into())]);
///     let expected_body = lang!(
///         "{", lang!("<-", sym!(c), lang!("+", sym!(a), sym!(b))));
///     assert_eq!(func.formals().as_pairlist().unwrap(), expected_formals);
///     assert_eq!(func.body(), expected_body);
///     assert_eq!(func.env(), global_env());
/// }
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub(crate) robj: Robj,
}

impl Function {
    /// Make a function from parts.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let formals = pairlist!(a=NULL).try_into()?;
    ///     let body = lang!("+", sym!(a), r!(1)).try_into()?;
    ///     let env = global_env();
    ///     let f = r!(Function::from_parts(formals, body, env )?);
    ///     assert_eq!(f.call(pairlist!(a=1))?, r!(2));
    /// }
    /// ```
    pub fn from_parts(formals: Pairlist, body: Language, env: Environment) -> Result<Self> {
        unsafe {
            let sexp = Rf_allocSExp(CLOSXP);
            let robj = new_owned(sexp);
            SET_FORMALS(sexp, formals.get());
            SET_BODY(sexp, body.get());
            SET_CLOENV(sexp, env.get());
            Ok(Function { robj })
        }
    }

    /// Do the equivalent of x(a, b, c)
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let function = R!(function(a, b) a + b).unwrap().as_function().unwrap();
    ///     assert_eq!(function.call(pairlist!(a=1, b=2)).unwrap(), r!(3));
    /// }
    /// ```
    pub fn call(&self, args: Robj) -> Result<Robj> {
        self.robj.call(args)
    }

    /// Get the formal arguments of the function.
    pub fn formals(&self) -> Pairlist {
        unsafe {
            let sexp = self.robj.get();
            new_owned(FORMALS(sexp)).try_into().unwrap()
        }
    }

    /// Get the body of the function.
    pub fn body(&self) -> Robj {
        unsafe {
            let sexp = self.robj.get();
            new_owned(BODY(sexp))
        }
    }

    /// Get the environment of the function.
    pub fn env(&self) -> Environment {
        unsafe {
            let sexp = self.robj.get();
            new_owned(CLOENV(sexp))
                .try_into()
                .expect("Should be an environment")
        }
    }
}
