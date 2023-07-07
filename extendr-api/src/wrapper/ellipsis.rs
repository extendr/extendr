use crate::prelude::*;
use crate::{Error, R_NilValue, Robj, Rtype, Types, SEXP};

#[derive(PartialEq, Clone)]
pub struct Ellipsis {
    pub(crate) robj: Robj,
}

impl Ellipsis {
    pub fn iter(&self) -> EllipsisIter {
        unsafe {
            EllipsisIter {
                robj: Some(&self.robj),
                list_elem: self.robj.get(),
            }
        }
    }

    pub fn collect_values(&self) -> Result<Vec<Robj>> {
        self.iter()
            .filter_map(|x| <Promise as TryFrom<&Robj>>::try_from(&x).ok())
            .map(|p| p.eval())
            .collect()
    }
}

impl<'a> TryFrom<&'a Robj> for Ellipsis {
    type Error = Error;

    fn try_from(robj: &'a Robj) -> std::result::Result<Self, Self::Error> {
        match robj.rtype() {
            Rtype::Dot => Ok(Self { robj: robj.clone() }),
            Rtype::Environment => try_from_env(robj),
            tp => Err(Error::Other(format!("Got {:?}", tp))),
        }
    }
}

fn try_from_env(env: &Robj) -> Result<Ellipsis> {
    <Environment as TryFrom<&Robj>>::try_from(env)
        .and_then(|e| {
            e.iter()
                .find(|(k, _)| *k == "...")
                .ok_or(Error::Other("Ellipsis missing".into()))
        })
        .and_then(|(_, v)| <Ellipsis as TryFrom<&Robj>>::try_from(&v))
}

#[derive(Clone)]
pub struct EllipsisIter<'a> {
    pub(crate) robj: Option<&'a Robj>,
    pub(crate) list_elem: SEXP,
}

impl<'a> Default for EllipsisIter<'a> {
    fn default() -> Self {
        EllipsisIter::new()
    }
}

impl<'a> EllipsisIter<'a> {
    pub fn new() -> Self {
        unsafe {
            Self {
                robj: None,
                list_elem: R_NilValue,
            }
        }
    }
}

impl<'a> Iterator for EllipsisIter<'a> {
    type Item = Robj;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.robj.is_none() || self.list_elem == R_NilValue {
                return None;
            }

            let elem = libR_sys::CAR(self.list_elem);
            self.list_elem = libR_sys::CDR(self.list_elem);
            Some(Robj::from_sexp(elem))
        }
    }
}
