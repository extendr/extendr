//! A pairlist is a linked list of values with optional symbol tags.

use super::*;
use extendr_ffi::{
    R_NilValue, Rf_cons, Rf_protect, Rf_unprotect, CAR, CDR, PRINTNAME, SET_TAG, TAG, TYPEOF,
};

#[derive(PartialEq, Clone)]
pub struct PairList {
    pub(crate) robj: RObj,
}

impl PairList {
    pub fn new() -> Self {
        let robj = RObj::from(());
        Self { robj }
    }

    /// Convert an iterator of names and values to a pairlist object.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let pairs = (0..100).map(|i| (format!("n{}", i), i));
    ///     let pairlist = PairList::from_pairs(pairs);
    ///     assert_eq!(pairlist.len(), 100);
    ///
    ///     // Use "" to indicate the absense of the name
    ///     let unnamed_pairlist = PairList::from_pairs([("", "a"), ("", "b")]);
    ///     assert_eq!(call!("names", unnamed_pairlist)?, r!(NULL));
    ///     let unnamed_pairlist_r = R!(r#"pairlist("a", "b")"#)?.as_pairlist().unwrap();
    ///     assert_eq!(unnamed_pairlist_r.names().collect::<Vec<_>>(), vec!["", ""]);
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
                if let Some(name) = name {
                    SET_TAG(res, name.get());
                }
            }
            let res = PairList {
                robj: RObj::from_sexp(res),
            };
            Rf_unprotect(num_protects);
            res
        })
    }

    /// Generate paits of names and values.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let pairs = (0..100).map(|i| (format!("n{}", i), i));
    ///     let pairlist = PairList::from_pairs(pairs);
    ///     assert_eq!(pairlist.iter().count(), 100);
    ///     assert_eq!(pairlist.iter().nth(50), Some(("n50", r!(50))));
    /// }
    /// ```
    pub fn iter(&self) -> PairListIter {
        unsafe {
            PairListIter {
                robj: self.robj.clone(),
                list_elem: self.robj.get(),
            }
        }
    }

    pub fn names(&self) -> impl Iterator<Item = &'static str> {
        self.iter().map(|(tag, _)| tag)
    }

    pub fn values(&self) -> impl Iterator<Item = RObj> {
        self.iter().map(|(_, robj)| robj)
    }
}

impl Default for wrapper::pairlist::PairList {
    fn default() -> Self {
        Self::new()
    }
}

/// Generate paits of names and values.
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let pairs = (0..100).map(|i| (format!("n{}", i), i));
///     let pairlist = PairList::from_pairs(pairs);
///     assert_eq!(pairlist.iter().count(), 100);
///     assert_eq!(pairlist.iter().nth(50), Some(("n50", r!(50))));
/// }
/// ```
#[derive(Clone)]
pub struct PairListIter {
    pub(crate) robj: RObj,
    pub(crate) list_elem: SEXP,
}

impl Default for PairListIter {
    fn default() -> Self {
        PairListIter::new()
    }
}

impl PairListIter {
    /// Make an empty pairlist iterator.
    pub fn new() -> Self {
        unsafe {
            Self {
                robj: ().into(),
                list_elem: R_NilValue,
            }
        }
    }
}

impl Iterator for PairListIter {
    // Note: The static is bad here, but we await RFC 1598
    // to do this properly. Howevere, symbols live forever.
    // https://github.com/rust-lang/rfcs/blob/master/text/1598-generic_associated_types.md
    type Item = (&'static str, RObj);

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let sexp = self.list_elem;
            if sexp == R_NilValue {
                None
            } else {
                let tag = TAG(sexp);
                let value = RObj::from_sexp(CAR(sexp));
                self.list_elem = CDR(sexp);
                if TYPEOF(tag) == SEXPTYPE::SYMSXP {
                    // printname is always a CHARSXP
                    let printname = PRINTNAME(tag);
                    rstr::charsxp_to_str(printname).map(|x| (x, value))
                } else {
                    // empty string represents the absense of the name
                    Some(("", value))
                }
            }
        }
    }
}

impl IntoIterator for PairList {
    type IntoIter = PairListIter;
    type Item = (&'static str, RObj);

    /// Convert a PairList into an interator, consuming the pairlist.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let pairlist = pairlist!(a=1, 2).as_pairlist().unwrap();
    ///     let vec : Vec<_> = pairlist.into_iter().collect();
    ///     assert_eq!(vec, vec![("a", r!(1)), ("", r!(2))]);
    /// }
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        unsafe {
            let sexp = self.robj.get();
            PairListIter {
                robj: self.robj,
                list_elem: sexp,
            }
        }
    }
}

impl TryFrom<&RObj> for PairListIter {
    type Error = Error;

    /// You can pass a PairListIter to a function.
    fn try_from(robj: &RObj) -> Result<Self> {
        let pairlist: PairList = robj.try_into()?;
        Ok(pairlist.into_iter())
    }
}

impl From<PairListIter> for RObj {
    /// You can return a PairListIter from a function.
    fn from(iter: PairListIter) -> Self {
        iter.robj
    }
}

impl From<()> for PairList {
    /// Construct a NULL pairlist (which is a NULL).
    fn from(_: ()) -> Self {
        PairList {
            robj: RObj::from(()),
        }
    }
}

impl std::fmt::Debug for PairList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "pairlist!({})",
            self.iter()
                .map(|(k, v)| format!("{}={:?}", k, v))
                .collect::<Vec<_>>()
                .join(", ")
        )?;
        Ok(())
    }
}

#[deprecated(note = "Use PairList instead", since = "0.9.0")]
pub type Pairlist = PairList;

#[deprecated(note = "Use PairListIter instead", since = "0.9.0")]
pub type PairlistIter = PairListIter;
