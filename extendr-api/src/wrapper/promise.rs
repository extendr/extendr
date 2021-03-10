use super::*;

/// Wrapper for creating promises (PROMSXP).
#[derive(Debug, PartialEq, Clone)]
pub struct Promise {
    pub(crate) robj: Robj,
}

impl Promise {
    /// Make a Promise from parts.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let promise = Promise::from_parts(r!(1), global_env())?;
    ///     assert_eq!(promise.value(), unbound_value());
    ///     assert_eq!(promise.eval_promise()?, r!(1));
    ///     assert_eq!(promise.value(), r!(1));
    /// }
    /// ```
    pub fn from_parts(code: Robj, environment: Robj) -> Result<Self> {
        if !environment.is_environment() {
            return Err(Error::ExpectedEnviroment(environment));
        }

        unsafe {
            let sexp = Rf_allocSExp(PROMSXP);
            let robj = new_owned(sexp);
            SET_PRCODE(sexp, code.get());
            SET_PRENV(sexp, environment.get());
            SET_PRVALUE(sexp, R_UnboundValue);
            SET_PRSEEN(sexp, 0);
            Ok(Promise { robj })
        }
    }

    /// Get the code to be executed from the promise.
    pub fn code(&self) -> Robj {
        unsafe {
            let sexp = self.robj.get();
            new_owned(PRCODE(sexp))
        }
    }

    /// Get the environment for the execution from the promise.
    pub fn environment(&self) -> Robj {
        unsafe {
            let sexp = self.robj.get();
            new_owned(PRENV(sexp))
        }
    }

    /// Get the value of the promise, once executed.
    pub fn value(&self) -> Robj {
        unsafe {
            let sexp = self.robj.get();
            new_owned(PRVALUE(sexp))
        }
    }

    /// Get the seen flag (avoids recursion).
    pub fn seen(&self) -> i32 {
        unsafe {
            let sexp = self.robj.get();
            PRSEEN(sexp)
        }
    }
}
