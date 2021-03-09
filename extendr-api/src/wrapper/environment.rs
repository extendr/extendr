use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Environment {
    pub(crate) robj: Robj,
}

impl Environment {
    /// Make an R environment object.
    /// ```
    /// use extendr_api::prelude::*;
    /// use std::convert::TryInto;
    /// test! {
    ///     let names_and_values = (0..100).map(|i| (format!("n{}", i), i));
    ///     let mut env = Environment::from_pairs(names_and_values);
    ///     env.set_enclos(global_env().try_into()?);
    ///     assert_eq!(env.len(), 100);
    /// }
    /// ```
    pub fn from_pairs<NV>(names_and_values: NV) -> Self
    where
        NV: IntoIterator,
        NV::Item: SymPair,
    {
        single_threaded(|| {
            let dict_len = 29;
            let robj = call!("new.env", TRUE, global_env(), dict_len).unwrap();
            for nv in names_and_values {
                let (n, v) = nv.sym_pair();
                unsafe { Rf_defineVar(n.get(), v.get(), robj.get()) }
            }
            Environment { robj }
        })
    }

    pub fn enclos(&self) -> Robj {
        unsafe {
            let sexp = self.robj.get();
            new_owned(ENCLOS(sexp))
        }
    }

    pub fn set_enclos(&mut self, parent: Environment) -> &mut Self {
        single_threaded(|| unsafe {
            let sexp = self.robj.get();
            SET_ENCLOS(sexp, parent.robj.get());
        });
        self
    }

    pub fn envflags(&self) -> i32 {
        unsafe {
            let sexp = self.robj.get();
            ENVFLAGS(sexp) as i32
        }
    }

    pub fn set_envflags(&mut self, flags: i32) -> &mut Self {
        unsafe {
            let sexp = self.robj.get();
            SET_ENVFLAGS(sexp, flags.into())
        }
        self
    }
}
