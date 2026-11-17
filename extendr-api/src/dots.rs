//! Use `...` from Rust
//!
//! This module implements the `...` for function signatures in R using
//! the delayed expression API introduced in R 4.6.
//! See [Working with variable bindings](https://cran.r-project.org/doc/manuals/r-devel/R-exts.html#Working-with-variable-bindings-1).
//!
//! Example:
//!
//! ```rust
//! struct BulletList(Strings);
//!
//! TryFrom<&Robj> for BulletList {
//!   todo!()
//! }
//!
//! From<BulletList> for Robj {
//!   todo!()
//! }
//! #[extendr(dots, r_name = "print.bullet_list")]
//! fn print_bulletlist(x: Robj) -> Strings {
//!   x.into_iter().map(|xi| {
//!     if xi.is_na() {
//!       Rstr::na()
//!     } else {
//!       Rstr::from(format("• {}", xi.as_ref())
//!     }
//!   }).collect()
//! }
use crate::{catch_r_error, Environment, Error, GetSexp, Result, Robj};
use extendr_ffi::dots::R_DotType_t;
use std::os::raw::c_int;

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
                let r_i = c_int::try_from(i + 1)
                    .map_err(|_| Error::OutOfRange(Robj::from(env.clone())))?;
                let env = unsafe { env.get() };
                let sexp =
                    catch_r_error(move || unsafe { extendr_ffi::dots::R_DotsElt(r_i, env) })?;
                Ok(Some(unsafe { Robj::from_sexp(sexp) }))
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

    pub fn get(&self, i: usize) -> Option<Dot> {
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
