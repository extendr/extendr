//! A pairlist is a linked list of values with optional symbol tags.

use super::*;

#[derive(PartialEq, Clone)]
pub struct Pairlist {
    pub(crate) robj: Robj,
}

impl Pairlist {
    pub fn new() -> Self {
        let robj = Robj::from(());
        Self { robj }
    }

    /// Convert an iterator of names and values to a pairlist object.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let pairs = (0..100).map(|i| (format!("n{}", i), i));
    ///     let pairlist = Pairlist::from_pairs(pairs);
    ///     assert_eq!(pairlist.len(), 100);
    ///
    ///     // Use "" to indicate the absense of the name
    ///     let unnamed_pairlist = Pairlist::from_pairs([("", "a"), ("", "b")]);
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
            let res = Pairlist {
                robj: Robj::from_sexp(res),
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
    ///     let pairlist = Pairlist::from_pairs(pairs);
    ///     assert_eq!(pairlist.iter().count(), 100);
    ///     assert_eq!(pairlist.iter().nth(50), Some(("n50", r!(50))));
    /// }
    /// ```
    pub fn iter(&self) -> PairlistIter {
        unsafe {
            PairlistIter {
                robj: self.robj.clone(),
                list_elem: self.robj.get(),
            }
        }
    }

    pub fn names(&self) -> impl Iterator<Item = &'static str> {
        self.iter().map(|(tag, _)| tag)
    }

    pub fn values(&self) -> impl Iterator<Item = Robj> {
        self.iter().map(|(_, robj)| robj)
    }
}

impl Default for wrapper::pairlist::Pairlist {
    fn default() -> Self {
        Self::new()
    }
}

/// Generate paits of names and values.
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let pairs = (0..100).map(|i| (format!("n{}", i), i));
///     let pairlist = Pairlist::from_pairs(pairs);
///     assert_eq!(pairlist.iter().count(), 100);
///     assert_eq!(pairlist.iter().nth(50), Some(("n50", r!(50))));
/// }
/// ```
#[derive(Clone)]
pub struct PairlistIter {
    pub(crate) robj: Robj,
    pub(crate) list_elem: SEXP,
}

impl Default for PairlistIter {
    fn default() -> Self {
        PairlistIter::new()
    }
}

impl PairlistIter {
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

impl Iterator for PairlistIter {
    // Note: The static is bad here, but we await RFC 1598
    // to do this properly. Howevere, symbols live forever.
    // https://github.com/rust-lang/rfcs/blob/master/text/1598-generic_associated_types.md
    type Item = (&'static str, Robj);

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let sexp = self.list_elem;
            if sexp == R_NilValue {
                None
            } else {
                let tag = TAG(sexp);
                let value = Robj::from_sexp(CAR(sexp));
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

impl IntoIterator for Pairlist {
    type IntoIter = PairlistIter;
    type Item = (&'static str, Robj);

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
            PairlistIter {
                robj: self.robj,
                list_elem: sexp,
            }
        }
    }
}

impl TryFrom<&Robj> for PairlistIter {
    type Error = Error;

    /// You can pass a PairlistIter to a function.
    fn try_from(robj: &Robj) -> Result<Self> {
        let pairlist: Pairlist = robj.try_into()?;
        Ok(pairlist.into_iter())
    }
}

impl From<PairlistIter> for Robj {
    /// You can return a PairlistIter from a function.
    fn from(iter: PairlistIter) -> Self {
        iter.robj
    }
}

impl From<()> for Pairlist {
    /// Construct a NULL pairlist (which is a NULL).
    fn from(_: ()) -> Self {
        Pairlist {
            robj: Robj::from(()),
        }
    }
}

impl std::fmt::Debug for Pairlist {
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
