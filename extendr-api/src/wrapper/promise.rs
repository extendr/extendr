use super::*;

/// Wrapper for creating promises (PROMSXP).
#[derive(PartialEq, Clone)]
pub struct Promise {
    pub(crate) robj: Robj,
}

impl Promise {
    /// Make a Promise from parts.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let promise = Promise::from_parts(r!(1), global_env())?;
    ///     assert!(promise.value().is_unbound_value());
    ///     assert_eq!(promise.eval_promise()?, r!(1));
    ///     assert_eq!(promise.value(), r!(1));
    /// }
    /// ```
    pub fn from_parts(code: Robj, environment: Environment) -> Result<Self> {
        unsafe {
            let sexp = Rf_allocSExp(PROMSXP);
            let robj = Robj::from_sexp(sexp);
            SET_PRCODE(sexp, code.get());
            SET_PRENV(sexp, environment.robj.get());
            SET_PRVALUE(sexp, R_UnboundValue);
            Ok(Promise { robj })
        }
    }

    /// Get the code to be executed from the promise.
    pub fn code(&self) -> Robj {
        unsafe {
            let sexp = self.robj.get();
            Robj::from_sexp(PRCODE(sexp))
        }
    }

    /// Get the environment for the execution from the promise.
    pub fn environment(&self) -> Environment {
        unsafe {
            let sexp = self.robj.get();
            Robj::from_sexp(PRENV(sexp)).try_into().unwrap()
        }
    }

    /// Get the value of the promise, once executed.
    pub fn value(&self) -> Robj {
        unsafe {
            let sexp = self.robj.get();
            Robj::from_sexp(PRVALUE(sexp))
        }
    }

    /// Get the seen flag (avoids recursion).
    pub fn seen(&self) -> i32 {
        unsafe {
            let sexp = self.robj.get();
            PRSEEN(sexp)
        }
    }

    /// If this promise has not been evaluated, evaluate it, otherwise return the value.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///    let iris_promise = global_env().find_var(sym!(iris)).unwrap();
    ///    let iris_dataframe = iris_promise.as_promise().unwrap().eval().unwrap();
    ///    assert_eq!(iris_dataframe.is_frame(), true);
    /// }
    /// ```
    pub fn eval(&self) -> Result<Robj> {
        assert!(self.is_promise());
        if !self.value().is_unbound_value() {
            Ok(self.value())
        } else {
            self.robj.eval()
        }
    }
}

impl std::fmt::Debug for Promise {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Promise")
            .field("code", &self.code())
            .field("environment", &self.environment())
            .finish()
    }
}
