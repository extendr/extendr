use super::*;
use crate as extendr_api;
use extendr_ffi::{R_xlen_t, SET_VECTOR_ELT};

pub(crate) fn make_symbol(name: &str) -> SEXP {
    let name = std::ffi::CString::new(name).unwrap();
    unsafe { extendr_ffi::Rf_install(name.as_ptr()) }
}

pub(crate) fn make_vector<T>(sexptype: SEXPTYPE, values: T) -> RObj
where
    T: IntoIterator,
    T::IntoIter: ExactSizeIterator,
    T::Item: Into<RObj>,
{
    single_threaded(|| unsafe {
        let values = values.into_iter();
        let mut res = RObj::alloc_vector(sexptype, values.len());
        let sexp = res.get_mut();
        for (i, val) in values.enumerate() {
            SET_VECTOR_ELT(sexp, i as R_xlen_t, val.into().get());
        }
        res
    })
}

macro_rules! make_conversions {
    ($typename: ident, $errname: ident, $isfunc: ident, $errstr: expr) => {
        impl From<$typename> for RObj {
            /// Make an robj from a wrapper.
            fn from(val: $typename) -> Self {
                val.robj
            }
        }

        // We can convert a reference to any wrapper to a RObj by cloning the robj pointer
        impl From<&$typename> for RObj {
            /// Make an robj from a wrapper.
            fn from(val: &$typename) -> Self {
                val.robj.to_owned()
            }
        }

        impl TryFrom<&RObj> for $typename {
            type Error = crate::Error;

            /// Make a wrapper from a robj if it matches.
            fn try_from(robj: &RObj) -> Result<Self> {
                if robj.$isfunc() {
                    Ok($typename { robj: robj.clone() })
                } else {
                    Err(Error::$errname(robj.clone()))
                }
            }
        }

        impl TryFrom<RObj> for $typename {
            type Error = crate::Error;

            /// Make a wrapper from a robj if it matches.
            fn try_from(robj: RObj) -> Result<Self> {
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

            fn as_robj(&self) -> &RObj {
                &self.robj
            }

            fn as_robj_mut(&mut self) -> &mut RObj {
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
        $($impl)* RInternals for $typename {}

        /// as_typed_slice_raw() etc.
        $($impl)* Slices for $typename {}

        /// dollar() etc.
        $($impl)* Operators for $typename {}
    };
}

make_conversions!(PairList, ExpectedPairList, is_pairlist, "Not a pairlist");

make_conversions!(
    Function,
    ExpectedFunction,
    is_function,
    "Not a function or primitive."
);

make_conversions!(Raw, ExpectedRaw, is_raw, "Not a raw object");

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

make_getsexp!(DataFrame<T>, impl<T>);

// impl Deref for Integers {
//     type Target = [RInt];

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
        Symbol::try_from(self.as_robj()).ok()
    }

    /// Convert a `CHARSXP` object to a `RStr` wrapper.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let fred = RStr::from_string("fred");
    ///     assert_eq!(fred.as_char(), Some(RStr::from_string("fred")));
    /// }
    /// ```
    fn as_char(&self) -> Option<RStr> {
        RStr::try_from(self.as_robj()).ok()
    }

    /// Convert a raw object to a RStr wrapper.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let bytes = r!(Raw::from_bytes(&[1, 2, 3]));
    ///     assert_eq!(bytes.len(), 3);
    ///     assert_eq!(bytes.as_raw(), Some(Raw::from_bytes(&[1, 2, 3])));
    /// }
    /// ```
    fn as_raw(&self) -> Option<Raw> {
        Raw::try_from(self.as_robj()).ok()
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
        Language::try_from(self.as_robj()).ok()
    }

    /// Convert a pair list object (LISTSXP) to a PairList wrapper.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let names_and_values = vec![("a", r!(1)), ("b", r!(2)), ("", r!(3))];
    ///     let pairlist = PairList::from_pairs(names_and_values);
    ///     let robj = r!(pairlist.clone());
    ///     assert_eq!(robj.as_pairlist().unwrap(), pairlist);
    /// }
    /// ```
    fn as_pairlist(&self) -> Option<PairList> {
        PairList::try_from(self.as_robj()).ok()
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
        List::try_from(self.as_robj()).ok()
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
        Expressions::try_from(self.as_robj()).ok()
    }

    /// Convert an environment object (ENVSXP) to a Env wrapper.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let names_and_values = (0..100).map(|i| (format!("n{}", i), i));
    ///     let env = Environment::from_pairs(Environment::global(), names_and_values);
    ///     let expr = env.clone();
    ///     assert_eq!(expr.len(), 100);
    ///     let env2 = expr.as_environment().unwrap();
    ///     assert_eq!(env2.len(), 100);
    /// }
    /// ```
    fn as_environment(&self) -> Option<Environment> {
        Environment::try_from(self.as_robj()).ok()
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
        Function::try_from(self.as_robj()).ok()
    }

    /// Get a wrapper for a promise.
    fn as_promise(&self) -> Option<Promise> {
        Promise::try_from(self.as_robj()).ok()
    }
}

impl Conversions for RObj {}

pub trait SymPair {
    fn sym_pair(self) -> (Option<RObj>, RObj);
}

impl<S, R> SymPair for (S, R)
where
    S: AsRef<str>,
    R: Into<RObj>,
{
    fn sym_pair(self) -> (Option<RObj>, RObj) {
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
    R: Into<RObj>,
    R: Clone,
{
    fn sym_pair(self) -> (Option<RObj>, RObj) {
        use crate as extendr_api;
        let val = self.0.as_ref();
        let nm = if val.is_empty() {
            None
        } else {
            Some(r!(Symbol::from_string(val)))
        };
        (nm, self.1.clone().into())
    }
}
