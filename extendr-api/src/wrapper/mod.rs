//! Wrappers are lightweight proxies for references to R datatypes.
//! They do not contain an Robj (see array.rs for an example of this).

use crate::*;
use libR_sys::*;

pub mod character;
pub mod environment;
pub mod expr;
pub mod function;
pub mod lang;
pub mod list;
pub mod matrix;
pub mod nullable;
pub mod pairlist;
pub mod primitive;
pub mod promise;
pub mod raw;
pub mod symbol;

pub use character::Character;
pub use environment::Env;
pub use expr::Expr;
pub use function::Function;
pub use lang::Lang;
pub use list::List;
pub use matrix::{RArray, RColumn, RMatrix, RMatrix3D};
pub use nullable::Nullable;
pub use pairlist::Pairlist;
pub use primitive::Primitive;
pub use promise::Promise;
pub use raw::Raw;
pub use symbol::Symbol;

pub(crate) fn make_symbol(name: &str) -> SEXP {
    let mut bytes = Vec::with_capacity(name.len() + 1);
    bytes.extend(name.bytes());
    bytes.push(0);
    unsafe { Rf_install(bytes.as_ptr() as *const i8) }
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

impl Robj {
    /// Convert a symbol object to a Symbol wrapper.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let fred = sym!(fred);
    ///     assert_eq!(fred.as_symbol(), Some(Symbol("fred")));
    /// }
    /// ```
    pub fn as_symbol(&self) -> Option<Symbol> {
        if self.is_symbol() {
            unsafe {
                let printname = PRINTNAME(self.get());
                if TYPEOF(printname) as u32 == CHARSXP {
                    Some(Symbol(to_str(R_CHAR(printname) as *const u8)))
                } else {
                    // This should never occur.
                    None
                }
            }
        } else {
            None
        }
    }

    /// Convert a character object to a Character wrapper.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let fred = r!(Character("fred"));
    ///     assert_eq!(fred.as_character(), Some(Character("fred")));
    /// }
    /// ```
    pub fn as_character(&self) -> Option<Character> {
        if self.sexptype() == CHARSXP {
            Some(Character(unsafe {
                to_str(R_CHAR(self.get()) as *const u8)
            }))
        } else {
            None
        }
    }

    /// Convert a raw object to a Character wrapper.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let bytes = r!(Raw(&[1, 2, 3]));
    ///     assert_eq!(bytes.len(), 3);
    ///     assert_eq!(bytes.as_raw(), Some(Raw(&[1, 2, 3])));
    /// }
    /// ```
    pub fn as_raw(&self) -> Option<Raw> {
        if self.sexptype() == RAWSXP {
            Some(Raw(self.as_raw_slice().unwrap()))
        } else {
            None
        }
    }
    /// Convert a language object to a Lang wrapper.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let call_to_xyz = r!(Lang(&[r!(Symbol("xyz")), r!(1), r!(2)]));
    ///     assert_eq!(call_to_xyz.is_language(), true);
    ///     assert_eq!(call_to_xyz.len(), 3);
    ///     assert_eq!(format!("{:?}", call_to_xyz), r#"r!(Lang([sym!(xyz), r!(1), r!(2)]))"#);
    /// }
    /// ```
    pub fn as_lang(&self) -> Option<Lang<PairlistIter>> {
        if self.sexptype() == LANGSXP {
            Some(Lang(self.as_pairlist_iter().unwrap()))
        } else {
            None
        }
    }

    /// Convert a pair list object (LISTSXP) to a Pairlist wrapper.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let names_and_values = vec![("a", r!(1)), ("b", r!(2)), (na_str(), r!(3))];
    ///     let pairlist = Pairlist{ names_and_values };
    ///     let robj = r!(pairlist.clone());
    ///     assert_eq!(robj.as_pairlist().unwrap(), pairlist);
    /// }
    /// ```
    pub fn as_pairlist(&self) -> Option<Pairlist<Vec<(&str, Robj)>>> {
        if self.sexptype() == LISTSXP {
            let names = self.as_pairlist_tag_iter().unwrap();
            let values = self.as_pairlist_iter().unwrap();
            let names_and_values: Vec<_> = names.zip(values).collect();
            Some(Pairlist { names_and_values })
        } else {
            None
        }
    }

    /// Convert a list object (VECSXP) to a List wrapper.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let list = r!(List(&[r!(0), r!(1), r!(2)]));
    ///     assert_eq!(list.is_list(), true);
    ///     assert_eq!(format!("{:?}", list), r#"r!(List([r!(0), r!(1), r!(2)]))"#);
    /// }
    /// ```
    pub fn as_list(&self) -> Option<List<ListIter>> {
        self.as_list_iter().map(|l| List(l))
    }

    /// Convert an expression object (EXPRSXP) to a Expr wrapper.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let expr = r!(Expr(&[r!(0), r!(1), r!(2)]));
    ///     assert_eq!(expr.is_expr(), true);
    ///     assert_eq!(expr.as_expr(), Some(Expr(vec![r!(0), r!(1), r!(2)])));
    ///     assert_eq!(format!("{:?}", expr), r#"r!(Expr([r!(0), r!(1), r!(2)]))"#);
    /// }
    /// ```
    pub fn as_expr(&self) -> Option<Expr<Vec<Robj>>> {
        if self.sexptype() == EXPRSXP {
            let res: Vec<_> = self
                .as_list_iter()
                .unwrap()
                .map(|robj| robj.to_owned())
                .collect();
            Some(Expr(res))
        } else {
            None
        }
    }

    /// Convert an environment object (ENVSXP) to a Env wrapper.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let names_and_values = (0..100).map(|i| (format!("n{}", i), r!(i)));
    ///     let env = Env{parent: global_env(), names_and_values};
    ///     let expr = r!(env.clone());
    ///     assert_eq!(expr.len(), 100);
    ///     let env2 = expr.as_environment().unwrap();
    ///     assert_eq!(env2.names_and_values.count(), 100);
    /// }
    /// ```
    pub fn as_environment(&self) -> Option<Env<Robj, EnvIter>> {
        if self.is_environment() {
            Some(Env {
                parent: self.parent().unwrap(),
                names_and_values: self.as_env_iter().unwrap(),
            })
        } else {
            None
        }
    }

    /// Convert a function object (CLOSXP) to a Function wrapper.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let func = R!(function(a,b) a + b).unwrap();
    ///     println!("{:?}", func.as_func());
    /// }
    /// ```
    pub fn as_func(&self) -> Option<Function> {
        if let Ok(f) = Function::new(self.clone()) {
            Some(f)
        } else {
            None
        }
    }

    // /// Convert a primitive object (BUILTINSXP or SPECIALSXP) to a wrapper.
    // /// ```
    // /// use extendr_api::prelude::*;
    // /// test! {
    // ///  let builtin = r!(Primitive("+"));
    // ///  let special = r!(Primitive("if"));
    // ///  assert_eq!(builtin.sexptype(), libR_sys::BUILTINSXP);
    // ///  assert_eq!(special.sexptype(), libR_sys::SPECIALSXP);
    // /// }
    // /// ```
    // pub fn as_primitive(&self) -> Option<Primitive> {
    //     match self.sexptype() {
    //         BUILTINSXP | SPECIALSXP => {
    //             // Unfortunately, for now PRIMNAME is out of bounds.
    //             //Some(Primitive(unsafe {to_str(PRIMNAME(self.get()) as * const u8)}))
    //             None
    //         }
    //         _ => None,
    //     }
    // }

    /// Get a wrapper for a promise.
    pub fn as_promise(&self) -> Option<Promise<Robj, Robj, Robj>> {
        if self.is_promise() {
            unsafe {
                let sexp = self.get();
                Some(Promise {
                    code: new_owned(PRCODE(sexp)),
                    env: new_owned(PRENV(sexp)),
                    value: new_owned(PRVALUE(sexp)),
                    seen: PRSEEN(sexp) != 0,
                })
            }
        } else {
            None
        }
    }
}

pub trait SymPair {
    fn sym_pair(self) -> (Robj, Robj);
}

impl<S, R> SymPair for (S, R)
where
    S: AsRef<str>,
    R: Into<Robj>,
{
    fn sym_pair(self) -> (Robj, Robj) {
        (r!(Symbol(self.0.as_ref())), self.1.into())
    }
}

impl<R> SymPair for R
where
    R: Into<Robj>,
{
    fn sym_pair(self) -> (Robj, Robj) {
        (r!(NULL), self.into())
    }
}
