//! Use `...` from Rust
//!
//! This module implements the `...` for function signatures in R using
//! the delayed expression API introduced in R 4.6.
//! See [Working with variable bindings](https://cran.r-project.org/doc/manuals/r-devel/R-exts.html#Working-with-variable-bindings-1).
//!
//! [`Dots`] is built upon [`Environment`].
//! `Dots` should always be fetched from inside
//! the body of a function via `Dots::find()`.
//! This calls [`extendr_ffi::dots::R_findDotsEnv`] to walk the
//! environments and fine where `...`, if anywhere, is defined.
//!
//! `Dots` can be iterated over using `.iter()` or extracted
//! manually by index or name via [`Dots::get()`].
//!
//! [`Dot`] enum represents a single element of [`Dots`].
//! It can either be a value, missing, a promise, or an already-evaluated expression.
//!
//! To extract the underlying value of a `Dot` call [`Dot::value()`].
//!
//! Common operations (note you should always handle `None` safely!):
//!
//! - Dots are missing: `Dots::find().is_none()`
//! - Dots are empty: `Dots::find().unwrap().is_empty()`
//! - Dots are unnamed: `Dots::find().unwrap().names().is_none()`
//!
//! ## Example:
//!
//! ```rust,ignore
//! use extendr_api::dots::Dots;
//! use extendr_api::prelude::*;
//! use extendr_api::Result;
//!
//! #[derive(Debug, Clone)]
//! pub struct BulletList(Strings);
//!
//! impl TryFrom<&Robj> for BulletList {
//!     type Error = Error;
//!
//!     fn try_from(robj: &Robj) -> Result<Self> {
//!         if !robj.inherits("bullet_list") {
//!             return Err(Error::TypeMismatch(robj.clone()));
//!         }
//!         Ok(BulletList(Strings::try_from(robj)?))
//!     }
//! }
//!
//! impl TryFrom<Robj> for BulletList {
//!     type Error = Error;
//!
//!     fn try_from(robj: Robj) -> Result<Self> {
//!         Self::try_from(&robj)
//!     }
//! }
//!
//! impl From<BulletList> for Robj {
//!     fn from(x: BulletList) -> Self {
//!         x.0.into()
//!     }
//! }
//!
//! /// @export
//! #[extendr]
//! fn bullet_list(items: Dots) -> Result<BulletList> {
//!     let items = items
//!         .into_iter()
//!         .map(|dot| match dot.value()? {
//!             Some(value) => Rstr::try_from(value),
//!             None => Ok(Rstr::na()),
//!         })
//!         .collect::<Result<Vec<Rstr>>>()?;
//!     let mut items = Strings::from_values(items);
//!     items.set_class(["bullet_list"])?;
//!     Ok(BulletList(items))
//! }
//!
//! /// @export
//! #[extendr(invisible, r_name = "print.bullet_list")]
//! fn print_bullet_list(x: BulletList, dots: Dots) -> Result<BulletList> {
//!     let arg = |name: &str| match dots.get(name) {
//!         Some(dot) => dot.value(),
//!         None => Ok(None),
//!     };
//!     let width = arg("width")?.map(|w| usize::try_from(&w)).transpose()?;
//!     let symbol = arg("symbol")?
//!         .map(String::try_from)
//!         .transpose()?
//!         .unwrap_or_else(|| "•".to_string());
//!
//!     for item in x.0.iter() {
//!         let text: String = if item.is_na() {
//!             "NA".to_string()
//!         } else {
//!             match width {
//!                 Some(width) => item.as_str().chars().take(width).collect(),
//!                 None => item.as_str().to_string(),
//!             }
//!         };
//!         rprintln!("{symbol} {text}");
//!     }
//!     Ok(x)
//! }
//! ```
use crate::wrapper::symbol::dots_symbol;
use crate::{robj::Eval, Environment, Error, GetSexp, Result, Robj, Strings, Symbol};
use extendr_ffi::dots::R_DotType_t;
use std::os::raw::c_int;

/// Represents `...` as a struct.
#[derive(Debug, Clone, PartialEq)]
pub struct Dots(Environment);

#[derive(Debug, Clone, PartialEq)]
pub enum Dot {
    Missing,
    Value(Robj),
    DelayedExpression { i: usize, env: Environment },
    ForcedExpression { i: usize, env: Environment },
}

impl Dot {
    pub fn value(&self) -> Result<Option<Robj>> {
        match self {
            Self::Missing => Ok(None),
            Self::Value(v) => Ok(Some(v.clone())),
            Self::DelayedExpression { i, env } | Self::ForcedExpression { i, env } => {
                let symbol = Symbol::from_string(format!("..{}", i + 1));
                Robj::from(symbol).eval_with_env(env).map(Some)
            }
        }
    }
}

impl Dots {
    pub fn find() -> Option<Self> {
        let dots_env = unsafe {
            let sexp = extendr_ffi::dots::R_findDotsEnv(Environment::current().get());
            if sexp == extendr_ffi::R_EmptyEnv || extendr_ffi::dots::R_DotsLength(sexp) == 0 {
                return None;
            }
            Robj::from_sexp(sexp)
        };
        Environment::try_from(dots_env).ok().map(Dots)
    }

    pub fn len(&self) -> usize {
        unsafe { extendr_ffi::dots::R_DotsLength(self.0.get()) as usize }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn names(&self) -> Option<Strings> {
        Strings::try_from(unsafe { Robj::from_sexp(extendr_ffi::dots::R_DotsNames(self.0.get())) })
            .ok()
    }

    pub fn get<I: DotsIndex>(&self, index: I) -> Option<Dot> {
        let i = index.position(self)?;
        if i >= self.len() {
            return None;
        }
        let r_i = c_int::try_from(i + 1).ok()?;
        let env = self.0.clone();
        unsafe {
            match extendr_ffi::dots::R_GetDotType(r_i, env.get()) {
                R_DotType_t::R_DotTypeValue => Some(Dot::Value(Robj::from_sexp(
                    extendr_ffi::dots::R_DotsElt(r_i, env.get()),
                ))),
                R_DotType_t::R_DotTypeMissing => Some(Dot::Missing),
                R_DotType_t::R_DotTypeDelayed => Some(Dot::DelayedExpression { i, env }),
                R_DotType_t::R_DotTypeForced => Some(Dot::ForcedExpression { i, env }),
            }
        }
    }
}

pub trait DotsIndex {
    fn position(&self, dots: &Dots) -> Option<usize>;
}

impl DotsIndex for usize {
    fn position(&self, _dots: &Dots) -> Option<usize> {
        Some(*self)
    }
}

impl DotsIndex for &str {
    fn position(&self, dots: &Dots) -> Option<usize> {
        if self.is_empty() {
            return None;
        }
        dots.names()?.iter().position(|n| n.as_ref() == *self)
    }
}

impl DotsIndex for String {
    fn position(&self, dots: &Dots) -> Option<usize> {
        self.as_str().position(dots)
    }
}

impl DotsIndex for &String {
    fn position(&self, dots: &Dots) -> Option<usize> {
        self.as_str().position(dots)
    }
}

impl AsRef<Environment> for Dots {
    fn as_ref(&self) -> &Environment {
        &self.0
    }
}

impl From<Dots> for Environment {
    fn from(dots: Dots) -> Self {
        dots.0
    }
}

impl TryFrom<&Robj> for Dots {
    type Error = Error;

    fn try_from(robj: &Robj) -> Result<Self> {
        let env = Environment::try_from(robj)?;
        let dots_env = unsafe {
            let sexp = extendr_ffi::dots::R_findDotsEnv(env.get());
            if sexp == extendr_ffi::R_EmptyEnv {
                return Err(Error::Other("no `...` found".to_string()));
            }
            Robj::from_sexp(sexp)
        };
        Ok(Dots(Environment::try_from(dots_env)?))
    }
}

impl TryFrom<Robj> for Dots {
    type Error = Error;

    fn try_from(robj: Robj) -> Result<Self> {
        Self::try_from(&robj)
    }
}

impl From<Dots> for Symbol {
    fn from(_: Dots) -> Self {
        dots_symbol()
    }
}

#[derive(Debug, Clone)]
pub struct DotsIter {
    dots: Dots,
    i: usize,
    len: usize,
}

impl Iterator for DotsIter {
    type Item = Dot;

    fn next(&mut self) -> Option<Dot> {
        let dot = self.dots.get(self.i)?;
        self.i += 1;
        Some(dot)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.len - self.i;
        (n, Some(n))
    }
}

impl ExactSizeIterator for DotsIter {}

impl IntoIterator for Dots {
    type Item = Dot;
    type IntoIter = DotsIter;

    fn into_iter(self) -> DotsIter {
        let len = self.len();
        DotsIter {
            dots: self,
            i: 0,
            len,
        }
    }
}

impl IntoIterator for &Dots {
    type Item = Dot;
    type IntoIter = DotsIter;

    fn into_iter(self) -> DotsIter {
        self.clone().into_iter()
    }
}
