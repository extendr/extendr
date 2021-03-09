use super::*;

/// Wrapper for creating pair list (LISTSXP) objects.
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let names_and_values = (0..100).map(|i| (format!("n{}", i), i));
///     let expr = r!(Pairlist{names_and_values});
///     assert_eq!(expr.len(), 100);
/// }
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct Pairlist<NV> {
    pub names_and_values: NV,
}

impl<'a, NV> From<Pairlist<NV>> for Robj
where
    NV: IntoIterator + 'a,
    NV::Item: SymPair,
{
    /// Convert a wrapper to a LISTSXP object.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let names_and_values = (0..100).map(|i| (format!("n{}", i), i));
    ///     let expr = r!(Pairlist{names_and_values});
    ///     assert_eq!(expr.len(), 100);
    /// }
    /// ```
    fn from(val: Pairlist<NV>) -> Self {
        single_threaded(|| unsafe {
            let names_and_values = val.names_and_values;
            let mut num_protects = 0;
            let mut res = R_NilValue;
            let names_and_values: Vec<_> = names_and_values.into_iter().collect();
            for nv in names_and_values.into_iter().rev() {
                let (name, val) = nv.sym_pair();
                let val = Rf_protect(val.get());
                res = Rf_protect(Rf_cons(val, res));
                num_protects += 2;
                if !name.is_na() {
                    SET_TAG(res, name.get());
                }
            }
            let res = new_owned(res);
            Rf_unprotect(num_protects as i32);
            res
        })
    }
}
