//! Wrappers are lightweight proxies for references to R datatypes.
//! They do not contain an Robj (see array.rs for an example of this).

use crate::*;
#[doc(hidden)]
use libR_sys::*;
#[doc(hidden)]

/// Wrapper for creating symbols.
///
/// Example:
/// ```
/// use extendr_api::*;
/// extendr_engine::start_r();
/// let symbol = r!(Symbol("xyz"));
/// assert_eq!(symbol.as_str(), Some("xyz"));
/// assert!(symbol.is_symbol());
/// ```
/// Note that creating a symbol from a string is expensive
/// and so you may want to cache them.
///
#[derive(Debug, PartialEq)]
pub struct Symbol<'a>(pub &'a str);

/// Wrapper for creating character objects.
/// These are used only as the contents of a character
/// vector.
///
/// Example:
/// ```
/// use extendr_api::*;
/// extendr_engine::start_r();
/// let chr = r!(Character("xyz"));
/// assert_eq!(chr.as_str(), Some("xyz"));
/// ```
///
#[derive(Debug, PartialEq)]
pub struct Character<'a>(pub &'a str);

/// Wrapper for creating raw (byte) objects.
///
/// Example:
/// ```
/// use extendr_api::*;
/// extendr_engine::start_r();
/// let bytes = r!(Raw(&[1, 2, 3]));
/// assert_eq!(bytes.len(), 3);
/// assert_eq!(bytes, r!(Raw(&[1, 2, 3])));
/// ```
///
#[derive(Debug, PartialEq)]
pub struct Raw<'a>(pub &'a [u8]);

/// Wrapper for creating language objects.
/// Example:
/// ```
/// use extendr_api::*;
/// extendr_engine::start_r();
/// let call_to_xyz = r!(Lang(&[r!(Symbol("xyz")), r!(1), r!(2)]));
/// assert_eq!(call_to_xyz.is_language(), true);
/// assert_eq!(call_to_xyz.len(), 3);
/// assert_eq!(format!("{:?}", call_to_xyz), r#"r!(Lang(&[r!(Symbol("xyz")), r!(1), r!(2)]))"#);
/// ```
///
/// Note: You can use the [lang!] macro for this.
#[derive(Debug, PartialEq)]
pub struct Lang<T>(pub T);

/// Wrapper for creating pair list (LISTSXP) objects.
/// Example:
/// ```
/// use extendr_api::*;
/// extendr_engine::start_r();
/// let call_to_xyz = r!(Pairlist(&[r!(NULL), r!(1), r!(2)]));
/// assert_eq!(call_to_xyz.is_pair_list(), true);
/// assert_eq!(call_to_xyz.len(), 3);
/// assert_eq!(format!("{:?}", call_to_xyz), r#"r!(Pairlist(&[r!(NULL), r!(1), r!(2)]))"#);
/// ```
///
/// Note: You can use the [lang!] macro for this.
#[derive(Debug, PartialEq)]
pub struct Pairlist<T>(pub T);

/// Wrapper for creating list objects.
/// ```
/// use extendr_api::*;
/// extendr_engine::start_r();
/// let mixed_list = r!(List(&[r!(1.), r!("xyz")]));
/// assert_eq!(mixed_list.len(), 2);
/// ```
///
/// Note: prefer to use the [list!] macro for named lists.
#[derive(Debug, PartialEq)]
pub struct List<T>(pub T);

/// Wrapper for creating expression objects.
/// Example:
/// ```
/// use extendr_api::*;
/// extendr_engine::start_r();
/// let expr = r!(Expr(&[r!(1.), r!("xyz")]));
/// assert_eq!(expr.len(), 2);
/// ```
#[derive(Debug, PartialEq)]
pub struct Expr<'a>(pub &'a [Robj]);

/// Wrapper for creating environments.
/// Example:
/// ```
/// use extendr_api::*;
/// extendr_engine::start_r();
/// let expr = r!(Env{parent: global_env(), names: &["a", "b"], values: &[1, 2]});
/// // assert_eq!(expr, r!(Env{parent: global_env(), names: vec!["a".to_string(), "b".to_string()], values: &[1, 2]}));
/// assert_eq!(expr.len(), 2);
/// ```
#[derive(Debug, PartialEq)]
pub struct Env<P, N, V> {
    pub parent: P,
    pub names: N,
    pub values: V,
}

impl<T> From<List<T>> for Robj
where
    T: IntoIterator,
    Robj: From<T::Item>,
{
    /// Make a list object from an array of Robjs.
    /// ```
    /// use extendr_api::*;
    /// extendr_engine::start_r();
    /// let list_of_ints = r!(List(&[1, 2]));
    /// assert_eq!(list_of_ints.len(), 2);
    /// ```
    fn from(val: List<T>) -> Self {
        make_vector(VECSXP, val.0)
    }
}

impl<'a> From<Expr<'a>> for Robj {
    /// Make an expression object from a collection Robjs.
    fn from(val: Expr<'a>) -> Self {
        make_vector(EXPRSXP, val.0)
    }
}

impl<'a> From<Raw<'a>> for Robj {
    /// Make a raw object from bytes.
    fn from(val: Raw<'a>) -> Self {
        single_threaded(|| unsafe {
            let val = val.0;
            let sexp = Rf_allocVector(RAWSXP, val.len() as R_xlen_t);
            R_PreserveObject(sexp);
            let ptr = RAW(sexp);
            for (i, &v) in val.iter().enumerate() {
                *ptr.offset(i as isize) = v;
            }
            Robj::Owned(sexp)
        })
    }
}

impl<'a> From<Symbol<'a>> for Robj {
    /// Make a symbol object.
    fn from(name: Symbol) -> Self {
        single_threaded(|| unsafe { new_owned(make_symbol(name.0)) })
    }
}

impl<T> From<Lang<T>> for Robj
where
    T: IntoIterator,
    Robj: From<T::Item>,
{
    /// Convert a wrapper to an R language object.
    fn from(val: Lang<T>) -> Self {
        single_threaded(|| unsafe {
            let values = get_protected_values(val.0);
            let mut res = R_NilValue;
            let len = values.len();
            for val in values.into_iter().rev() {
                res = Rf_lcons(val, res);
            }
            Rf_unprotect(len as i32);
            new_owned(res)
        })
    }
}

impl<P, N, V, NI, VI> From<Env<P, N, V>> for Robj
where
    N: IntoIterator<Item = NI>,
    V: IntoIterator<Item = VI>,
    Robj: From<P>,
    NI: AsRef<str>,
    Robj: From<VI>,
{
    /// Convert a wrapper to an R environment object.
    /// ```
    /// use extendr_api::*;
    /// extendr_engine::start_r();
    /// let expr = r!(Env{parent: global_env(), names: &["a", "b"], values: &[1, 2]});
    /// assert_eq!(expr.len(), 2);
    /// ```
    fn from(val: Env<P, N, V>) -> Self {
        single_threaded(|| {
            let (parent, names, values) = (val.parent, val.names, val.values);
            let values = get_protected_values(values);
            let names = get_protected_names(names);
            let len = values.len().min(names.len());
            let dict_len = (len * 2 + 1) as i32;
            // This call is only available in later R libs.
            // let mut res = R_NewEnv(parent.get(), 1, dict_len);
            let res = call!("new.env", TRUE, parent, dict_len).unwrap();
            assert!(res.is_owned());

            let res_sexp = unsafe { res.get() };
            for (name, value) in names.into_iter().zip(values.into_iter()) {
                unsafe { Rf_defineVar(name, value, res_sexp) };
            }

            unsafe { Rf_unprotect(len as i32 * 2) };
            res
        })
    }
}

impl<T> From<Pairlist<T>> for Robj
where
    T: IntoIterator,
    Robj: From<T::Item>,
{
    /// Convert a wrapper to a LISTSXP object.
    fn from(val: Pairlist<T>) -> Self {
        single_threaded(|| unsafe {
            let values = get_protected_values(val.0);
            let mut res = R_NilValue;
            let len = values.len();
            for val in values.into_iter().rev() {
                res = Rf_cons(val, res);
            }
            Rf_unprotect(len as i32);
            new_owned(res)
        })
    }
}

fn get_protected_values<T>(values: T) -> Vec<SEXP>
where
    T: IntoIterator,
    Robj: From<T::Item>,
{
    unsafe {
        values
            .into_iter()
            .map(|item| Rf_protect(Robj::from(item).get()))
            .collect()
    }
}

fn make_symbol(name: &str) -> SEXP {
    let mut bytes = Vec::with_capacity(name.len() + 1);
    bytes.extend(name.bytes());
    bytes.push(0);
    unsafe { Rf_install(bytes.as_ptr() as *const i8) }
}

fn get_protected_names<T>(values: T) -> Vec<SEXP>
where
    T: IntoIterator,
    T::Item: AsRef<str>,
{
    unsafe {
        values
            .into_iter()
            .map(|item| Rf_protect(make_symbol(item.as_ref())))
            .collect()
    }
}

fn make_vector<T>(sexptype: u32, val: T) -> Robj
where
    T: IntoIterator,
    Robj: From<T::Item>,
{
    single_threaded(|| unsafe {
        let values = get_protected_values(val);
        let sexp = Rf_allocVector(sexptype, values.len() as R_xlen_t);
        R_PreserveObject(sexp);
        for i in 0..values.len() {
            SET_VECTOR_ELT(sexp, i as R_xlen_t, values[i]);
        }
        Rf_unprotect(values.len() as i32);
        Robj::Owned(sexp)
    })
}

/// Allow you to skip the Symbol() in some cases.
impl<'a> From<&'a str> for Symbol<'a> {
    fn from(val: &'a str) -> Self {
        Self(val)
    }
}
