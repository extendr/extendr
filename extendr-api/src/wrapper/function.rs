use super::*;

/// Wrapper for creating functions (CLOSSXP).
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let expr = R!(function(a = 1, b) {c <- a + b}).unwrap();
///     let func = expr.as_func().unwrap();
///
///     let expected_formals = Pairlist {
///         names_and_values: vec![("a", r!(1.0)), ("b", missing_arg())] };
///     let expected_body = lang!(
///         "{", lang!("<-", sym!(c), lang!("+", sym!(a), sym!(b))));
///     assert_eq!(func.formals().as_pairlist().unwrap(), expected_formals);
///     assert_eq!(func.body(), expected_body);
///     assert_eq!(func.env(), global_env());
/// }
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    robj: Robj,
}

impl<'a> FromRobj<'a> for Function {
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
        if let Some(f) = robj.as_func() {
            Ok(f)
        } else {
            Err("Not a function")
        }
    }
}

impl From<Function> for Robj {
    /// Make an robj from a function wrapper.
    /// The function wrapper is guaranteed to contain a function object.
    fn from(val: Function) -> Self {
        val.robj
    }
}

impl Function {
    /// Make a function from an Robj or return an error.
    pub fn new(robj: Robj) -> Result<Self> {
        if robj.is_function() {
            Ok(Function { robj })
        } else {
            Err(Error::ExpectedFunction(robj))
        }
    }

    /// Make a function from parts.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let formals = pairlist!(a=NULL);
    ///     let body = lang!("+", sym!(a), r!(1));
    ///     let env = global_env();
    ///     let f = r!(Function::from_parts(formals, body, env )?);
    ///     assert_eq!(f.call(pairlist!(a=1))?, r!(2));
    /// }
    /// ```
    pub fn from_parts(formals: Robj, body: Robj, env: Robj) -> Result<Self> {
        if !formals.is_pairlist() {
            return Err(Error::ExpectedPairlist(formals));
        }
        if !env.is_environment() {
            return Err(Error::ExpectedEnviroment(env));
        }
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
    ///     let function = R!(function(a, b) a + b).unwrap().as_func().unwrap();
    ///     assert_eq!(function.call(pairlist!(a=1, b=2)).unwrap(), r!(3));
    /// }
    /// ```
    pub fn call(&self, args: Robj) -> Result<Robj> {
        self.robj.call(args)
    }

    /// Get the formal arguments of the function.
    pub fn formals(&self) -> Robj {
        unsafe {
            let sexp = self.robj.get();
            new_owned(FORMALS(sexp))
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
    pub fn env(&self) -> Robj {
        unsafe {
            let sexp = self.robj.get();
            new_owned(CLOENV(sexp))
        }
    }
}
