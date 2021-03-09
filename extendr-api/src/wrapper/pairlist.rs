use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Pairlist {
    pub (crate) robj: Robj,
}

impl Pairlist {
    /// Convert an iterator of names and values to a pairlist object.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let pairs = (0..100).map(|i| (format!("n{}", i), i));
    ///     let pairlist = Pairlist::from_pairs(pairs);
    ///     assert_eq!(pairlist.len(), 100);
    /// }
    /// ```
    pub fn from_pairs<NV>(pairs: NV) -> Self
    where
        NV: IntoIterator,
        NV::IntoIter: DoubleEndedIterator,
        NV::Item: SymPair,
    {
        crate::single_threaded(|| unsafe {
            let mut num_protects = 0;
            let mut res = R_NilValue;
            for nv in pairs.into_iter().rev() {
                let (name, val) = nv.sym_pair();
                let val = Rf_protect(val.get());
                res = Rf_protect(Rf_cons(val, res));
                num_protects += 2;
                if !name.is_na() {
                    SET_TAG(res, name.get());
                }
            }
            let res = Pairlist {
                robj: new_owned(res),
            };
            Rf_unprotect(num_protects as i32);
            res
        })
    }
}
