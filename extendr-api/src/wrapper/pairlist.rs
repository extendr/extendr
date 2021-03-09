use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Pairlist {
    robj: Robj,
}

impl<'a> FromRobj<'a> for Pairlist {
    /// Convert an object that may be null to a rust type.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let robj = pairlist!(a=1, b=2);
    ///     let pairlist = <Pairlist>::from_robj(&robj);
    /// }
    /// ```
    fn from_robj(robj: &'a Robj) -> std::result::Result<Self, &'static str> {
        if let Some(f) = robj.as_pairlist() {
            Ok(f)
        } else {
            Err("Not a pairlist")
        }
    }
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

impl From<Pairlist> for Robj {
    /// Make an robj from a pairlist wrapper.
    fn from(val: Pairlist) -> Self {
        val.robj
    }
}

impl TryFrom<Robj> for Pairlist {
    type Error = crate::Error;

    /// Make an pairlist from a robj if it matches.
    fn try_from(robj: Robj) -> Result<Self> {
        if robj.is_pairlist() {
            Ok(Pairlist { robj })
        } else {
            Err(Error::ExpectedPairlist(robj))
        }
    }
}

impl Deref for Pairlist {
    type Target = Robj;

    /// Make a Pairlist behave like an Robj.
    fn deref(&self) -> &Self::Target {
        &self.robj
    }
}
