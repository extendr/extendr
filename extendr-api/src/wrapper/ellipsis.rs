use crate::prelude::*;
use crate::{Error, R_NilValue, Robj, Rtype, Types, SEXP};

#[derive(PartialEq, Clone)]
pub struct Ellipsis {
    pub(crate) robj: Robj,
}

#[derive(PartialEq, Clone, Debug)]
pub struct EllipsisValue {
    pub name: Option<String>,
    pub value: Robj,
}

impl Ellipsis {
    pub(crate) fn new() -> Ellipsis {
        Self { robj: ().into() }
    }

    pub fn iter(&self) -> EllipsisIter {
        unsafe {
            EllipsisIter {
                robj: self.robj.clone(),
                list_elem: self.robj.get(),
            }
        }
    }

    pub fn collect_values(&self) -> Result<Vec<Robj>> {
        self.iter()
            .filter_map(|x| x.value.to_promise())
            .map(|p| p.eval())
            .collect()
    }

    pub fn values(&self) -> Result<Vec<EllipsisValue>> {
        let values = self
            .iter()
            .map(|x| (x.name, x.value.to_promise()))
            .collect::<Vec<_>>();

        let n = values.len() - 1;

        values
            .iter()
            .enumerate()
            .filter_map(|(i, (name, value))| {
                if let Some(prom) = value {
                    Some(prom.eval().map(|value| EllipsisValue {
                        name: name.clone().map(|nm| nm.as_str().to_string()),
                        value,
                    }))
                } else {
                    if i == n {
                        None
                    } else {
                        let name = name
                            .clone()
                            .map(|nm| format!("`{}`", nm.as_str()))
                            .unwrap_or_else(|| format!("[{}] element", i));
                        Some(Err(Error::Other(format!("Missing value for {}", name))))
                    }
                }
            })
            .collect::<Result<Vec<_>>>()
    }
}

impl<'a> TryFrom<&'a Robj> for Ellipsis {
    type Error = Error;

    fn try_from(robj: &'a Robj) -> std::result::Result<Self, Self::Error> {
        match robj.rtype() {
            Rtype::Dot => Ok(Self { robj: robj.clone() }),
            Rtype::Environment => try_from_env(robj),
            Rtype::Symbol if robj.is_missing_arg() => Ok(Ellipsis::new()),
            tp => Err(Error::Other(format!("Got {:?}: {:?}", tp, robj))),
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
pub struct EllipsisIter {
    robj: Robj,
    list_elem: SEXP,
}

impl Default for EllipsisIter {
    fn default() -> Self {
        EllipsisIter::new()
    }
}

impl EllipsisIter {
    pub fn new() -> Self {
        unsafe {
            Self {
                robj: ().into(),
                list_elem: R_NilValue,
            }
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum EllipsisItemValue {
    Promise(Promise),
    MissingArg,
}

impl EllipsisItemValue {
    pub fn to_promise(self) -> Option<Promise> {
        match self {
            EllipsisItemValue::Promise(p) => Some(p),
            _ => None,
        }
    }
}

impl TryFrom<&Robj> for EllipsisItemValue {
    type Error = Error;

    fn try_from(value: &Robj) -> std::result::Result<Self, Self::Error> {
        if value.is_missing_arg() {
            Ok(EllipsisItemValue::MissingArg)
        } else {
            <Promise as TryFrom<&Robj>>::try_from(value).map(EllipsisItemValue::Promise)
        }
    }
}

impl TryFrom<Robj> for EllipsisItemValue {
    type Error = Error;

    fn try_from(value: Robj) -> std::result::Result<Self, Self::Error> {
        <EllipsisItemValue as TryFrom<&Robj>>::try_from(&value)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct EllipsisIterItem {
    pub name: Option<Symbol>,
    pub value: EllipsisItemValue,
}

impl Iterator for EllipsisIter {
    type Item = EllipsisIterItem;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.robj.is_null() || self.list_elem == R_NilValue {
                return None;
            }

            let tag = libR_sys::TAG(self.list_elem);
            let elem = libR_sys::CAR(self.list_elem);
            self.list_elem = libR_sys::CDR(self.list_elem);
            Some(EllipsisIterItem {
                name: <Symbol as TryFrom<&Robj>>::try_from(&Robj::from_sexp(tag)).ok(),
                value: (&Robj::from_sexp(elem))
                    .try_into()
                    .expect("Should not happen"),
            })
        }
    }
}
