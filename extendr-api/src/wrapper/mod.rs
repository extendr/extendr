//! Wrappers are lightweight proxies for references to R datatypes.
//! They do not contain an Robj (see array.rs for an example of this).

use crate::*;
use libR_sys::*;

pub mod altrep;
pub mod char;
pub mod environment;
pub mod expr;
pub mod function;
pub mod integers;
pub mod lang;
pub mod list;
pub mod matrix;
pub mod nullable;
pub mod pairlist;
pub mod primitive;
pub mod promise;
pub mod raw;
pub mod s4;
pub mod symbol;

pub use self::char::Rstr;
pub use altrep::{
    AltComplexImpl, AltIntegerImpl, AltLogicalImpl, AltRawImpl, AltRealImpl, AltStringImpl, Altrep,
    AltrepImpl,
};
pub use environment::{EnvIter, Environment};
pub use expr::Expression;
pub use function::Function;
pub use integers::Integers;
pub use lang::Language;
pub use list::{FromList, List, ListIter};
pub use matrix::{RArray, RColumn, RMatrix, RMatrix3D};
pub use nullable::Nullable;
pub use pairlist::{Pairlist, PairlistIter};
pub use primitive::Primitive;
pub use promise::Promise;
pub use raw::Raw;
pub use s4::S4;
pub use symbol::Symbol;

pub(crate) fn make_symbol(name: &str) -> SEXP {
    let mut bytes = Vec::with_capacity(name.len() + 1);
    bytes.extend(name.bytes());
    bytes.push(0);
    unsafe { Rf_install(bytes.as_ptr() as *const ::std::os::raw::c_char) }
}

pub(crate) fn make_vector<T>(sexptype: u32, values: T) -> Robj
where
    T: IntoIterator,
    T::IntoIter: ExactSizeIterator,
    T::Item: Into<Robj>,
{
    single_threaded(|| unsafe {
        let values = values.into_iter();
        let sexp = Rf_allocVector(sexptype, values.len() as R_xlen_t);
        ownership::protect(sexp);
        for (i, val) in values.enumerate() {
            SET_VECTOR_ELT(sexp, i as R_xlen_t, val.into().get());
        }
        Robj::Owned(sexp)
    })
}

macro_rules! make_conversions {
    ($typename: ident, $errname: ident, $isfunc: ident, $errstr: expr) => {
        impl<'a> FromRobj<'a> for $typename {
            fn from_robj(robj: &'a Robj) -> std::result::Result<Self, &'static str> {
                if let Ok(f) = $typename::try_from(robj.clone()) {
                    Ok(f)
                } else {
                    Err($errstr)
                }
            }
        }

        impl From<$typename> for Robj {
            /// Make an robj from a wrapper.
            fn from(val: $typename) -> Self {
                val.robj
            }
        }

        impl TryFrom<Robj> for $typename {
            type Error = crate::Error;

            /// Make a wrapper from a robj if it matches.
            fn try_from(robj: Robj) -> Result<Self> {
                if robj.$isfunc() {
                    Ok($typename { robj })
                } else {
                    Err(Error::$errname(robj))
                }
            }
        }

        impl Deref for $typename {
            type Target = Robj;

            /// Make a wrapper behave like an Robj.
            fn deref(&self) -> &Self::Target {
                &self.robj
            }
        }

        impl DerefMut for $typename {
            /// Make a wrapper behave like a writable Robj.
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.robj
            }
        }
    };
}

make_conversions!(Pairlist, ExpectedPairlist, is_pairlist, "Not a pairlist");

make_conversions!(
    Function,
    ExpectedFunction,
    is_function,
    "Not a function or primitive."
);

make_conversions!(Raw, ExpectedRaw, is_raw, "Not a raw object");

make_conversions!(Rstr, ExpectedRstr, is_char, "Not a character object");

make_conversions!(
    Environment,
    ExpectedEnvironment,
    is_environment,
    "Not an Environment"
);

make_conversions!(List, ExpectedList, is_list, "Not a List");

make_conversions!(
    Expression,
    ExpectedExpression,
    is_expression,
    "Not an Expression"
);

make_conversions!(
    Language,
    ExpectedLanguage,
    is_language,
    "Not a Language object"
);

make_conversions!(Symbol, ExpectedSymbol, is_symbol, "Not a Symbol object");

make_conversions!(
    Primitive,
    ExpectedPrimitive,
    is_primitive,
    "Not a Primitive object"
);

make_conversions!(Promise, ExpectedPromise, is_promise, "Not a Promise object");

make_conversions!(Altrep, ExpectedAltrep, is_altrep, "Not an Altrep type");

make_conversions!(S4, ExpectedS4, is_s4, "Not a S4 type");

make_conversions!(Integers, ExpectedInteger, is_integer, "Not an integer type");

impl Robj {
    /// Convert a symbol object to a Symbol wrapper.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let fred = sym!(fred);
    ///     assert_eq!(fred.as_symbol(), Some(Symbol::from_string("fred")));
    /// }
    /// ```
    pub fn as_symbol(&self) -> Option<Symbol> {
        Symbol::try_from(self.clone()).ok()
    }

    /// Convert a CHARSXP object to a Rstr wrapper.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let fred = r!(Rstr::from_string("fred"));
    ///     assert_eq!(fred.as_char(), Some(Rstr::from_string("fred")));
    /// }
    /// ```
    pub fn as_char(&self) -> Option<Rstr> {
        Rstr::try_from(self.clone()).ok()
    }

    /// Convert a raw object to a Rstr wrapper.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let bytes = r!(Raw::from_bytes(&[1, 2, 3]));
    ///     assert_eq!(bytes.len(), 3);
    ///     assert_eq!(bytes.as_raw(), Some(Raw::from_bytes(&[1, 2, 3])));
    /// }
    /// ```
    pub fn as_raw(&self) -> Option<Raw> {
        Raw::try_from(self.clone()).ok()
    }

    /// Convert a language object to a Language wrapper.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let call_to_xyz = r!(Language::from_values(&[r!(Symbol::from_string("xyz")), r!(1), r!(2)]));
    ///     assert_eq!(call_to_xyz.is_language(), true);
    ///     assert_eq!(call_to_xyz.len(), 3);
    ///     assert_eq!(format!("{:?}", call_to_xyz), r#"r!(Language::from_values([sym!(xyz), r!(1), r!(2)]))"#);
    /// }
    /// ```
    pub fn as_language(&self) -> Option<Language> {
        Language::try_from(self.clone()).ok()
    }

    /// Convert a pair list object (LISTSXP) to a Pairlist wrapper.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let names_and_values = vec![("a", r!(1)), ("b", r!(2)), ("", r!(3))];
    ///     let pairlist = Pairlist::from_pairs(names_and_values);
    ///     let robj = r!(pairlist.clone());
    ///     assert_eq!(robj.as_pairlist().unwrap(), pairlist);
    /// }
    /// ```
    pub fn as_pairlist(&self) -> Option<Pairlist> {
        Pairlist::try_from(self.clone()).ok()
    }

    /// Convert a list object (VECSXP) to a List wrapper.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let list = r!(List::from_values(&[r!(0), r!(1), r!(2)]));
    ///     assert_eq!(list.is_list(), true);
    ///     assert_eq!(format!("{:?}", list), r#"r!(List::from_values([r!(0), r!(1), r!(2)]))"#);
    /// }
    /// ```
    pub fn as_list(&self) -> Option<List> {
        List::try_from(self.clone()).ok()
    }

    /// Convert an expression object (EXPRSXP) to a Expr wrapper.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let expr = r!(Expression::from_values(&[r!(0), r!(1), r!(2)]));
    ///     assert_eq!(expr.is_expression(), true);
    ///     assert_eq!(expr.as_expression(), Some(Expression::from_values(vec![r!(0), r!(1), r!(2)])));
    ///     assert_eq!(format!("{:?}", expr), r#"r!(Expression::from_values([r!(0), r!(1), r!(2)]))"#);
    /// }
    /// ```
    pub fn as_expression(&self) -> Option<Expression> {
        Expression::try_from(self.clone()).ok()
    }

    /// Convert an environment object (ENVSXP) to a Env wrapper.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let names_and_values = (0..100).map(|i| (format!("n{}", i), i));
    ///     let env = Environment::from_pairs(global_env(), names_and_values);
    ///     let expr = env.clone();
    ///     assert_eq!(expr.len(), 100);
    ///     let env2 = expr.as_environment().unwrap();
    ///     assert_eq!(env2.len(), 100);
    /// }
    /// ```
    pub fn as_environment(&self) -> Option<Environment> {
        Environment::try_from(self.clone()).ok()
    }

    /// Convert a function object (CLOSXP) to a Function wrapper.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let func = R!("function(a,b) a + b").unwrap();
    ///     println!("{:?}", func.as_function());
    /// }
    /// ```
    pub fn as_function(&self) -> Option<Function> {
        Function::try_from(self.clone()).ok()
    }

    /// Get a wrapper for a promise.
    pub fn as_promise(&self) -> Option<Promise> {
        Promise::try_from(self.clone()).ok()
    }
}

pub trait SymPair {
    fn sym_pair(self) -> (Option<Robj>, Robj);
}

impl<S, R> SymPair for (S, R)
where
    S: AsRef<str>,
    R: Into<Robj>,
{
    fn sym_pair(self) -> (Option<Robj>, Robj) {
        let val = self.0.as_ref();
        // "" represents the absense of the name
        let nm = if val.is_empty() {
            None
        } else {
            Some(r!(Symbol::from_string(val)))
        };
        (nm, self.1.into())
    }
}

impl<S, R> SymPair for &(S, R)
where
    S: AsRef<str>,
    R: Into<Robj>,
    R: Clone,
{
    fn sym_pair(self) -> (Option<Robj>, Robj) {
        let val = self.0.as_ref();
        let nm = if val.is_empty() {
            None
        } else {
            Some(r!(Symbol::from_string(val)))
        };
        (nm, self.1.clone().into())
    }
}
