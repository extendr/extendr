/*!
Enables support for `...` parameters in R functions.
`Ellipsis` can be used as a regular extendr-function argument. At most one `Ellipsis` is allowed per function,
and it requires `#[ellipsis]` attribute in front of it in the signature.
When encountered, `#[ellipsis]` attribute transforms parameter name into `...` on R side and captures current `environment()`.

`Ellipsis` can be iterated over to obtain `EllipsisItemValue` objects, each representing a (potentially named) argument captured by `...`.
`EllipsisItemValue` contains either a `Promise` that can be evaluated for value, or MissingArg, marking a missing argument in `...`.

`Ellipsis` can quickly collect all promises and evaluate them, returning a vector of `EllipsisValue` objects.
This allows at most one trailing missing arg, in all other cases collecting values will fail.

The following Rust sample:
```rust
use extendr_api::prelude::*;

#[extendr(use_try_from = true)]
fn capture_dots(x : i32, y : i32, #[ellipsis]dots: Ellipsis) -> Result<List> {
    let dots = dots.values()?;
    let dots = List::from_pairs(dots.into_iter());

    Ok(list!(x = x, y = y, dots = dots))
}
```
producing this wrapper on R side:
```R
capture_dots <- function(x, y, ...) {
    .Call("wrap__capture_dots", x, y, environment())
}
```

*/
use crate::list::KeyValue;
use crate::prelude::*;
use crate::{Error, MissingArgId, R_NilValue, Robj, Rtype, Types, SEXP};

/// Ellipsis or dot-dot-dot, representing R's `...` parameter.
#[derive(PartialEq, Clone)]
pub struct Ellipsis {
    pub(crate) robj: Robj,
}

/// Materialized value of an argument captured by `...`.
#[derive(PartialEq, Clone, Debug)]
pub struct EllipsisValue {
    pub name: Option<String>,
    pub value: Robj,
}

impl KeyValue for EllipsisValue {
    fn key(&self) -> String {
        self.name.clone().unwrap_or("".to_string())
    }

    fn value(self) -> Robj {
        self.value
    }
}

impl<'a> KeyValue for &'a EllipsisValue {
    fn key(&self) -> String {
        self.name.clone().unwrap_or("".to_string())
    }

    fn value(self) -> Robj {
        self.value.clone()
    }
}

impl Ellipsis {
    /// Create new empty `Ellipsis`
    pub(crate) fn new() -> Ellipsis {
        Self { robj: ().into() }
    }

    /// Iterate over arguments captured by `...`, without evaluating them.
    pub fn iter(&self) -> EllipsisIter {
        unsafe {
            EllipsisIter {
                robj: self.robj.clone(),
                list_elem: self.robj.get(),
            }
        }
    }

    /// Collect all arguments captured by `...`, evaluating promises, allowing at most
    /// one trailing missing argument. Every other missing argument will result in an error.
    pub fn values(&self) -> Result<Vec<EllipsisValue>> {
        let values = self
            .iter()
            .map(|x| (x.name, x.value.to_promise()))
            .collect::<Vec<_>>();

        if values.len() == 0 {
            return Ok(vec![]);
        }

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
                        name.clone()
                            .map(|nm| MissingArgId::Name(nm.as_str().into()))
                            .or_else(|| Some(MissingArgId::Index(i + 1)))
                            .map(|x| Err(Error::NonTrailingMissingArg(x)))
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
            _ => Err(Error::ExpectedEllipsis(robj.clone(), None)),
        }
    }
}

impl TryFrom<Robj> for Ellipsis {
    type Error = Error;

    fn try_from(value: Robj) -> std::result::Result<Self, Self::Error> {
        Ellipsis::try_from(&value)
    }
}

impl<'a> FromRobj<'a> for Ellipsis {
    fn from_robj(robj: &'a Robj) -> std::result::Result<Self, &'static str> {
        if let Ok(f) = Ellipsis::try_from(robj) {
            Ok(f)
        } else {
            Err("Not an ellipsis (`...`)")
        }
    }
}

fn try_from_env(env: &Robj) -> Result<Ellipsis> {
    <Environment as TryFrom<&Robj>>::try_from(env)
        .and_then(|e| {
            e.iter()
                .find(|(k, _)| *k == "...")
                .ok_or(Error::ExpectedEllipsis(
                    e.into(),
                    Some("`...` is missing from the captured environment".into()),
                ))
        })
        .and_then(|(_, v)| <Ellipsis as TryFrom<&Robj>>::try_from(&v))
}

/// Iterator over arguments captured by `...`.
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

/// An argument captured by `...`.
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

/// An unevaluated (potentially) named argument captured by `...`.
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
