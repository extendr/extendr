use super::*;
use extendr_ffi::{R_xlen_t, SET_VECTOR_ELT};

pub(crate) fn make_symbol(name: &str) -> SEXP {
    let name = std::ffi::CString::new(name).unwrap();
    unsafe { extendr_ffi::Rf_install(name.as_ptr()) }
}

pub(crate) fn make_vector<T>(sexptype: SEXPTYPE, values: T) -> Robj
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

        impl TryFrom<&Robj> for Option<$typename> {
            type Error = crate::Error;

            /// Make an optional wrapper from a robj. `NULL` becomes `None`.
            fn try_from(robj: &Robj) -> Result<Self> {
                if robj.is_null() {
                    Ok(None)
                } else {
                    Ok(Some(<$typename>::try_from(robj)?))
                }
            }
        }

        impl TryFrom<Robj> for Option<$typename> {
            type Error = crate::Error;

            /// Make an optional wrapper from a robj. `NULL` becomes `None`.
            fn try_from(robj: Robj) -> Result<Self> {
                <Option<$typename>>::try_from(&robj)
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
