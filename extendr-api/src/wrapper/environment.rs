use super::*;

/// Wrapper for creating environments.
#[derive(Debug, PartialEq, Clone)]
pub struct Env<P, NV> {
    pub parent: P,
    pub names_and_values: NV,
}

impl<'a, P, NV> From<Env<P, NV>> for Robj
where
    P: Into<Robj>,
    NV: IntoIterator + 'a,
    NV::Item: SymPair,
{
    /// Convert a wrapper to an R environment object.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let names_and_values = (0..100).map(|i| (format!("n{}", i), i));
    ///     let env = Env{parent: global_env(), names_and_values};
    ///     let expr = r!(env);
    ///     assert_eq!(expr.len(), 100);
    /// }
    /// ```
    fn from(val: Env<P, NV>) -> Self {
        single_threaded(|| {
            let (parent, names_and_values) = (val.parent, val.names_and_values);
            let dict_len = 29;
            let res = call!("new.env", TRUE, parent.into(), dict_len).unwrap();
            for nv in names_and_values {
                let (n, v) = nv.sym_pair();
                unsafe { Rf_defineVar(n.get(), v.get(), res.get()) }
            }
            res
        })
    }
}
