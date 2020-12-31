//! Wrappers are lightweight proxies for references to R datatypes.
//! They do not contain an Robj (see array.rs for an example of this).

use crate::*;
#[doc(hidden)]
use libR_sys::*;
#[doc(hidden)]

/// Wrapper for creating symbols.
///
/// ```
/// use extendr_api::*;
/// extendr_engine::start_r();
/// let symbol = r!(Symbol("xyz"));
/// assert_eq!(symbol.as_symbol(), Some(Symbol("xyz")));
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
/// ```
/// use extendr_api::*;
/// extendr_engine::start_r();
/// let chr = r!(Character("xyz"));
/// assert_eq!(chr.as_character(), Some(Character("xyz")));
/// ```
///
#[derive(Debug, PartialEq)]
pub struct Character<'a>(pub &'a str);

/// Wrapper for creating raw (byte) objects.
///
/// ```
/// use extendr_api::*;
/// extendr_engine::start_r();
/// let bytes = r!(Raw(&[1, 2, 3]));
/// assert_eq!(bytes.len(), 3);
/// assert_eq!(bytes.as_raw(), Some(Raw(&[1, 2, 3])));
/// ```
///
#[derive(Debug, PartialEq)]
pub struct Raw<'a>(pub &'a [u8]);

/// Wrapper for creating language objects.
/// ```
/// use extendr_api::*;
/// extendr_engine::start_r();
/// let call_to_xyz = r!(Lang(&[r!(Symbol("xyz")), r!(1), r!(2)]));
/// assert_eq!(call_to_xyz.is_language(), true);
/// assert_eq!(call_to_xyz.len(), 3);
/// assert_eq!(call_to_xyz.as_lang(), Some(Lang(vec![r!(Symbol("xyz")), r!(1), r!(2)])));
/// assert_eq!(format!("{:?}", call_to_xyz), r#"r!(Lang([r!(Symbol("xyz")), r!(1), r!(2)]))"#);
/// ```
///
/// Note: You can use the [lang!] macro for this.
#[derive(Debug, PartialEq)]
pub struct Lang<T>(pub T);

/// Wrapper for creating pair list (LISTSXP) objects.
/// ```
/// use extendr_api::*;
/// extendr_engine::start_r();
/// let pairlist = r!(Pairlist(&[r!(0), r!(1), r!(2)]));
/// assert_eq!(pairlist.is_pairlist(), true);
/// assert_eq!(pairlist.as_pairlist(), Some(Pairlist(vec![r!(0), r!(1), r!(2)])));
/// assert_eq!(format!("{:?}", pairlist), r#"r!(Pairlist([r!(0), r!(1), r!(2)]))"#);
/// ```
#[derive(Debug, PartialEq)]
pub struct Pairlist<T>(pub T);

/// Wrapper for creating list (VECSXP) objects.
/// ```
/// use extendr_api::*;
/// extendr_engine::start_r();
/// let list = r!(List(&[r!(0), r!(1), r!(2)]));
/// assert_eq!(list.is_list(), true);
/// assert_eq!(list.as_list(), Some(List(vec![r!(0), r!(1), r!(2)])));
/// assert_eq!(format!("{:?}", list), r#"r!(List([r!(0), r!(1), r!(2)]))"#);
/// ```
///
/// Note: you can use the [list!] macro for named lists.
#[derive(Debug, PartialEq)]
pub struct List<T>(pub T);

/// Wrapper for creating expression objects.
/// ```
/// use extendr_api::*;
/// extendr_engine::start_r();
/// let expr = r!(Expr(&[r!(1.), r!("xyz")]));
/// assert_eq!(expr.len(), 2);
/// ```
#[derive(Debug, PartialEq)]
pub struct Expr<T>(pub T);

/// Wrapper for creating environments.
/// ```
/// use extendr_api::*;
/// extendr_engine::start_r();
/// let expr = r!(Env{parent: global_env(), names: &["a", "b"], values: &[1, 2]});
/// assert_eq!(expr.as_env(), Some((Env{parent: global_env(), names: vec!["a", "b"], values: vec![r!(1), r!(2)]})));
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

impl<T> From<Expr<T>> for Robj
where
    T: IntoIterator,
    Robj: From<T::Item>,
{
    /// Make an expression object from an array of Robjs.
    /// ```
    /// use extendr_api::*;
    /// extendr_engine::start_r();
    /// let list_of_ints = r!(Expr(&[1, 2]));
    /// assert_eq!(list_of_ints.len(), 2);
    /// ```
    fn from(val: Expr<T>) -> Self {
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
    /// assert_eq!(expr.as_env(), Some((Env{parent: global_env(), names: vec!["a", "b"], values: vec![r!(1), r!(2)]})));
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

impl Robj {
    /// Convert a symbol object to a Symbol wrapper.
    /// ```
    /// use extendr_api::*;
    /// extendr_engine::start_r();
    /// let fred = sym!(fred);
    /// assert_eq!(fred.as_symbol(), Some(Symbol("fred")));
    /// ```
    pub fn as_symbol(&self) -> Option<Symbol> {
        if self.is_symbol() {
            Some(Symbol(unsafe {
                to_str(R_CHAR(PRINTNAME(self.get())) as *const u8)
            }))
        } else {
            None
        }
    }

    /// Convert a character object to a Character wrapper.
    /// ```
    /// use extendr_api::*;
    /// extendr_engine::start_r();
    /// let fred = r!(Character("fred"));
    /// assert_eq!(fred.as_character(), Some(Character("fred")));
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
    /// use extendr_api::*;
    /// extendr_engine::start_r();
    /// let bytes = r!(Raw(&[1, 2, 3]));
    /// assert_eq!(bytes.len(), 3);
    /// assert_eq!(bytes.as_raw(), Some(Raw(&[1, 2, 3])));
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
    /// use extendr_api::*;
    /// extendr_engine::start_r();
    /// let call_to_xyz = r!(Lang(&[r!(Symbol("xyz")), r!(1), r!(2)]));
    /// assert_eq!(call_to_xyz.is_language(), true);
    /// assert_eq!(call_to_xyz.len(), 3);
    /// assert_eq!(call_to_xyz.as_lang(), Some(Lang(vec![r!(Symbol("xyz")), r!(1), r!(2)])));
    /// assert_eq!(format!("{:?}", call_to_xyz), r#"r!(Lang([r!(Symbol("xyz")), r!(1), r!(2)]))"#);
    /// ```
    pub fn as_lang(&self) -> Option<Lang<Vec<Robj>>> {
        if self.sexptype() == LANGSXP {
            let res: Vec<_> = self
                .as_pairlist_iter()
                .unwrap()
                .map(|robj| robj.to_owned())
                .collect();
            Some(Lang(res))
        } else {
            None
        }
    }

    /// Convert a pair list object (LISTSXP) to a Pairlist wrapper.
    /// ```
    /// use extendr_api::*;
    /// extendr_engine::start_r();
    /// let pairlist = r!(Pairlist(&[r!(0), r!(1), r!(2)]));
    /// assert_eq!(pairlist.is_pairlist(), true);
    /// assert_eq!(pairlist.as_pairlist(), Some(Pairlist(vec![r!(0), r!(1), r!(2)])));
    /// assert_eq!(format!("{:?}", pairlist), r#"r!(Pairlist([r!(0), r!(1), r!(2)]))"#);
    /// ```
    pub fn as_pairlist(&self) -> Option<Pairlist<Vec<Robj>>> {
        if self.sexptype() == LISTSXP {
            let res: Vec<_> = self
                .as_pairlist_iter()
                .unwrap()
                .map(|robj| robj.to_owned())
                .collect();
            Some(Pairlist(res))
        } else {
            None
        }
    }

    /// Convert a list object (VECSXP) to a List wrapper.
    /// ```
    /// use extendr_api::*;
    /// extendr_engine::start_r();
    /// let list = r!(List(&[r!(0), r!(1), r!(2)]));
    /// assert_eq!(list.is_list(), true);
    /// assert_eq!(list.as_list(), Some(List(vec![r!(0), r!(1), r!(2)])));
    /// assert_eq!(format!("{:?}", list), r#"r!(List([r!(0), r!(1), r!(2)]))"#);
    /// ```
    pub fn as_list(&self) -> Option<List<Vec<Robj>>> {
        if self.sexptype() == VECSXP {
            let res: Vec<_> = self
                .as_list_iter()
                .unwrap()
                .map(|robj| robj.to_owned())
                .collect();
            Some(List(res))
        } else {
            None
        }
    }

    /// Convert an expression object (EXPRSXP) to a Expr wrapper.
    /// ```
    /// use extendr_api::*;
    /// extendr_engine::start_r();
    /// let expr = r!(Expr(&[r!(0), r!(1), r!(2)]));
    /// assert_eq!(expr.is_expr(), true);
    /// assert_eq!(expr.as_expr(), Some(Expr(vec![r!(0), r!(1), r!(2)])));
    /// assert_eq!(format!("{:?}", expr), r#"r!(Expr([r!(0), r!(1), r!(2)]))"#);
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
    /// use extendr_api::*;
    /// extendr_engine::start_r();
    /// let expr = r!(Env{parent: global_env(), names: &["a", "b"], values: &[1, 2]});
    /// assert_eq!(expr.as_env(), Some(Env{parent: global_env(), names: vec!["a", "b"], values: vec![r!(1), r!(2)]}))
    /// ```
    pub fn as_env(&self) -> Option<Env<Robj, Vec<&str>, Vec<Robj>>> {
        if self.sexptype() == ENVSXP {
            unsafe {
                let parent = new_owned(ENCLOS(self.get()));
                let hashtab = new_owned(HASHTAB(self.get()));
                let frame = new_owned(FRAME(self.get()));
                let mut names = Vec::new();
                let mut values = Vec::new();
                if let Some(as_list_iter) = hashtab.as_list_iter() {
                    for frame in as_list_iter {
                        if let (Some(obj_iter), Some(tag_iter)) =
                            (frame.as_pairlist_iter(), frame.as_pairlist_tag_iter())
                        {
                            for (obj, tag) in obj_iter.zip(tag_iter) {
                                if !obj.is_null() && tag.is_some() {
                                    values.push(obj);
                                    names.push(tag.unwrap());
                                }
                            }
                        }
                    }
                } else if let (Some(obj_iter), Some(tag_iter)) =
                    (frame.as_pairlist_iter(), frame.as_pairlist_tag_iter())
                {
                    for (obj, tag) in obj_iter.zip(tag_iter) {
                        if !obj.is_null() && tag.is_some() {
                            values.push(obj);
                            names.push(tag.unwrap());
                        }
                    }
                }
                Some(Env {
                    parent,
                    names,
                    values,
                })
            }
        } else {
            None
        }
    }
}
