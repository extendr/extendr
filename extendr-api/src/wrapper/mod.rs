//! Wrappers are lightweight proxies for references to R datatypes.
//! They do not contain an Robj (see array.rs for an example of this).

use crate::robj::{GetSexp, Rinternals};
use crate::*;
use libR_sys::*;

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

pub use self::rstr::Rstr;
#[cfg(use_r_altlist)]
pub use altrep::AltListImpl;
pub use altrep::{
    AltComplexImpl, AltIntegerImpl, AltLogicalImpl, AltRawImpl, AltRealImpl, AltStringImpl, Altrep,
    AltrepImpl,
};
pub use complexes::Complexes;
pub use dataframe::{Dataframe, IntoDataFrameRow};
pub use doubles::Doubles;
pub use environment::{EnvIter, Environment};
pub use expr::Expressions;
pub use externalptr::ExternalPtr;
pub use function::Function;
pub use integers::Integers;
pub use lang::Language;
pub use list::{FromList, List, ListIter};
pub use logicals::Logicals;
pub use matrix::{MatrixConversions, RArray, RColumn, RMatrix, RMatrix3D};
pub use nullable::Nullable;
pub use pairlist::{Pairlist, PairlistIter};
pub use primitive::Primitive;
pub use promise::Promise;
pub use raw::Raw;
pub use s4::S4;
pub use strings::Strings;
pub use symbol::Symbol;

pub trait RTypeAssoc {
    // vector_type: Integers, // Implements for
    type VectorType;
    // scalar_type: Rint,     // Element type
    type ScalarType;
    // primitive_type: i32,   // Raw element type
    type PrimitiveType;
    // r_prefix: INTEGER,     // `R` functions prefix
    // SEXP: INTSXP,          // `SEXP`
    // doc_name: integer,     // Singular type name used in docs
    // altrep_constructor: make_altinteger_from_iterator,
}

pub(crate) fn make_symbol(name: &str) -> SEXP {
    let name = CString::new(name).unwrap();
    unsafe { libR_sys::Rf_install(name.as_ptr()) }
}

pub(crate) fn make_vector<T>(sexptype: u32, values: T) -> Robj
where
    T: IntoIterator,
    T::IntoIter: ExactSizeIterator,
    T::Item: Into<Robj>,
{
    single_threaded(|| unsafe {
        let values = values.into_iter();
        let mut res = Robj::alloc_vector(sexptype, values.len());
        let sexp = res.get_mut();
        for (i, val) in values.enumerate() {
            SET_VECTOR_ELT(sexp, i as R_xlen_t, val.into().get());
        }
        res
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

        // We can convert a reference to any wrapper to a Robj by cloning the robj pointer
        impl From<&$typename> for Robj {
            /// Make an robj from a wrapper.
            fn from(val: &$typename) -> Self {
                val.robj.to_owned()
            }
        }

        impl TryFrom<&Robj> for $typename {
            type Error = crate::Error;

            /// Make a wrapper from a robj if it matches.
            fn try_from(robj: &Robj) -> Result<Self> {
                if robj.$isfunc() {
                    Ok($typename { robj: robj.clone() })
                } else {
                    Err(Error::$errname(robj.clone()))
                }
            }
        }

        impl TryFrom<Robj> for $typename {
            type Error = crate::Error;

            /// Make a wrapper from a robj if it matches.
            fn try_from(robj: Robj) -> Result<Self> {
                <$typename>::try_from(&robj)
            }
        }

        make_getsexp!($typename, impl);
    };
}

macro_rules! make_getsexp {
    ($typename: ty, $($impl : tt)*) => {
        $($impl)* GetSexp for $typename {
            unsafe fn get(&self) -> SEXP {
                self.robj.get()
            }

            unsafe fn get_mut(&mut self) -> SEXP {
                self.robj.get_mut()
            }

            fn as_robj(&self) -> &Robj {
                &self.robj
            }

            fn as_robj_mut(&mut self) -> &mut Robj {
                &mut self.robj
            }
        }

        // These traits all derive from GetSexp

        /// len() and is_empty()
        $($impl)* Length for $typename {}

        /// rtype() and rany()
        $($impl)* Types for $typename {}

        /// as_*()
        $($impl)* Conversions for $typename {}

        /// find_var() etc.
        $($impl)* Rinternals for $typename {}

        /// as_typed_slice_raw() etc.
        $($impl)* Slices for $typename {}

        /// dollar() etc.
        $($impl)* Operators for $typename {}
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
    Expressions,
    ExpectedExpression,
    is_expressions,
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
make_conversions!(Logicals, ExpectedLogical, is_logical, "Not a logical type");
make_conversions!(Doubles, ExpectedReal, is_real, "Not a floating point type");
make_conversions!(
    Complexes,
    ExpectedComplex,
    is_complex,
    "Not a complex number or vector"
);
// make_conversions!(Function, ExpectedFunction, is_function, "Not a function");

make_conversions!(Strings, ExpectedString, is_string, "Not a string vector");

make_getsexp!(Dataframe<T>, impl<T>);

// impl Deref for Integers {
//     type Target = [Rint];

//     fn deref(&self) -> &Self::Target {
//         unsafe { self.as_typed_slice_raw() }
//     }
// }

pub trait Conversions: GetSexp {
    /// Convert a symbol object to a Symbol wrapper.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let fred = sym!(fred);
    ///     assert_eq!(fred.as_symbol(), Some(Symbol::from_string("fred")));
    /// }
    /// ```
    fn as_symbol(&self) -> Option<Symbol> {
        Symbol::try_from(self.as_robj().clone()).ok()
    }

    /// Convert a CHARSXP object to a Rstr wrapper.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let fred = r!(Rstr::from_string("fred"));
    ///     assert_eq!(fred.as_char(), Some(Rstr::from_string("fred")));
    /// }
    /// ```
    fn as_char(&self) -> Option<Rstr> {
        Rstr::try_from(self.as_robj().clone()).ok()
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
    fn as_raw(&self) -> Option<Raw> {
        Raw::try_from(self.as_robj().clone()).ok()
    }

    /// Convert a language object to a Language wrapper.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let call_to_xyz = r!(Language::from_values(&[r!(Symbol::from_string("xyz")), r!(1), r!(2)]));
    ///     assert_eq!(call_to_xyz.is_language(), true);
    ///     assert_eq!(call_to_xyz.len(), 3);
    /// }
    /// ```
    fn as_language(&self) -> Option<Language> {
        Language::try_from(self.as_robj().clone()).ok()
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
    fn as_pairlist(&self) -> Option<Pairlist> {
        Pairlist::try_from(self.as_robj().clone()).ok()
    }

    /// Convert a list object (VECSXP) to a List wrapper.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let list = r!(List::from_values(&[r!(0), r!(1), r!(2)]));
    ///     assert_eq!(list.is_list(), true);
    /// }
    /// ```
    fn as_list(&self) -> Option<List> {
        List::try_from(self.as_robj().clone()).ok()
    }

    /// Convert an expression object (EXPRSXP) to a Expr wrapper.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let expr = r!(Expressions::from_values(&[r!(0), r!(1), r!(2)]));
    ///     assert_eq!(expr.is_expressions(), true);
    ///     assert_eq!(expr.as_expressions(), Some(Expressions::from_values(vec![r!(0), r!(1), r!(2)])));
    /// }
    /// ```
    fn as_expressions(&self) -> Option<Expressions> {
        Expressions::try_from(self.as_robj().clone()).ok()
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
    fn as_environment(&self) -> Option<Environment> {
        Environment::try_from(self.as_robj().clone()).ok()
    }

    /// Convert a function object (CLOSXP) to a Function wrapper.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let func = R!("function(a,b) a + b").unwrap();
    ///     println!("{:?}", func.as_function());
    /// }
    /// ```
    fn as_function(&self) -> Option<Function> {
        Function::try_from(self.as_robj().clone()).ok()
    }

    /// Get a wrapper for a promise.
    fn as_promise(&self) -> Option<Promise> {
        Promise::try_from(self.as_robj().clone()).ok()
    }
}

impl Conversions for Robj {}

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
