//! Wrappers are lightweight proxies for references to R datatypes.
//! They do not contain an RObj (see array.rs for an example of this).

use crate::robj::{GetSexp, RInternals};
use crate::*;

pub mod altrep;
pub mod complexes;
pub mod dataframe;
pub mod doubles;
pub mod environment;
pub mod expr;
pub mod externalptr;
pub mod function;
pub mod integers;
pub mod lang;
pub mod list;
pub mod logicals;
mod macros;
pub mod matrix;
pub mod nullable;
pub mod pairlist;
pub mod primitive;
pub mod promise;
pub mod raw;
pub mod rstr;
pub mod s4;
pub mod strings;
pub mod symbol;
pub mod wrapper_macros;

pub use self::rstr::RStr;

#[deprecated(note = "Use RStr instead", since = "0.9.0")]
pub use self::rstr::Rstr;
#[cfg(use_r_altlist)]
pub use altrep::AltListImpl;
pub use altrep::{
    AltComplexImpl, AltIntegerImpl, AltLogicalImpl, AltRawImpl, AltRealImpl, AltStringImpl, Altrep,
    AltrepImpl,
};
pub use complexes::Complexes;
pub use dataframe::{DataFrame, IntoDataFrameRow};

#[deprecated(note = "Use DataFrame instead", since = "0.9.0")]
pub use dataframe::Dataframe;
pub use doubles::Doubles;
pub use environment::{EnvIter, Environment};
pub use expr::Expressions;
pub use externalptr::ExternalPtr;
pub use function::Function;
pub use integers::Integers;
pub use lang::Language;
pub use list::{FromList, List, ListIter};
pub use logicals::Logicals;
pub use matrix::{MatrixConversions, RArray, RColumn, RMatrix, RMatrix3D, RMatrix4D, RMatrix5D};
pub use nullable::Nullable;
pub use pairlist::{PairList, PairListIter};

#[deprecated(note = "Use PairList instead", since = "0.9.0")]
pub use pairlist::Pairlist;
#[deprecated(note = "Use PairListIter instead", since = "0.9.0")]
pub use pairlist::PairlistIter;
pub use primitive::Primitive;
pub use promise::Promise;
pub use raw::Raw;
pub use s4::S4;
pub use strings::Strings;
pub use symbol::Symbol;
pub use wrapper_macros::*;
