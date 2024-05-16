use crate::*;

use wrapper::symbol::levels_symbol;

/// Iterator over name-value pairs in lists.
pub type NamedListIter = std::iter::Zip<StrIter, ListIter>;

/// Iterator over strings or string factors.
///
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let robj = r!(["a", "b", "c"]);
///     assert_eq!(robj.as_str_iter().unwrap().collect::<Vec<_>>(), vec!["a", "b", "c"]);
///
///     let factor = factor!(["abcd", "def", "fg", "fg"]);
///     assert_eq!(factor.levels().unwrap().collect::<Vec<_>>(), vec!["abcd", "def", "fg"]);
///     assert_eq!(factor.as_integer_vector().unwrap(), vec![1, 2, 3, 3]);
///     assert_eq!(factor.as_str_iter().unwrap().collect::<Vec<_>>(), vec!["abcd", "def", "fg", "fg"]);
///     assert_eq!(factor.as_str_iter().unwrap().collect::<Vec<_>>(), vec!["abcd", "def", "fg", "fg"]);
/// }
/// ```
#[derive(Clone)]
pub struct StrIter {
    vector: Robj,
    i: usize,
    len: usize,
    levels: SEXP,
}

impl Default for StrIter {
    fn default() -> Self {
        StrIter::new(0)
    }
}

impl StrIter {
    /// Make an empty str iterator.
    pub fn new(len: usize) -> Self {
        unsafe {
            Self {
                vector: ().into(),
                i: 0,
                len,
                levels: R_NilValue,
            }
        }
    }

    pub fn na_iter(len: usize) -> StrIter {
        Self {
            len,
            ..Default::default()
        }
    }
}

// Get a string reference from a `CHARSXP`
pub(crate) fn str_from_strsxp<'a>(sexp: SEXP, index: usize) -> &'a str {
    single_threaded(|| unsafe {
        let charsxp = STRING_ELT(sexp, index as _);
        if charsxp == R_NilValue {
            panic!("out of bound error")
        } else if charsxp == R_NaString {
            <&str>::na()
        } else if charsxp == R_BlankString {
            ""
        } else {
            rstr::charsxp_to_str(charsxp)
        }
    })
}

impl Iterator for StrIter {
    type Item = &'static str;

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let i = self.i;
            self.i += 1;
            let vector = self.vector.get();
            if i >= self.len {
                None
            } else if TYPEOF(vector) == SEXPTYPE::NILSXP {
                Some(<&str>::na())
            } else if TYPEOF(vector) == SEXPTYPE::STRSXP {
                Some(str_from_strsxp(vector, i))
            } else if Rf_isFactor(vector).into() {
                // factor support: factor is an integer, and we need
                // the value of it, to retrieve the assigned label
                let j = *(INTEGER(vector).add(i));
                let j = j.saturating_sub_unsigned(1) as _;
                // assert_eq!(TYPEOF(self.levels), STRSXP, "levels of a factor must always be a character-vector");
                // assert_ne!(j, 0, "invalid factor, where level/label i 0-indexed");
                Some(str_from_strsxp(self.levels, j))
            } else {
                None
            }
        }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.i += n;
        self.next()
    }
}

impl ExactSizeIterator for StrIter {
    fn len(&self) -> usize {
        self.len - self.i
    }
}

macro_rules! impl_iter_debug {
    ($name: ty) => {
        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "[")?;
                let mut comma = "";
                for s in self.clone() {
                    write!(f, "{}{:?}", comma, s)?;
                    comma = ", ";
                }
                write!(f, "]")
            }
        }
    };
}

impl_iter_debug!(ListIter);
impl_iter_debug!(PairlistIter);
impl_iter_debug!(StrIter);
impl_iter_debug!(EnvIter);

// Lets us create a StrIter from an Robj, e.g. Strings or a factor
impl TryFrom<&Robj> for StrIter {
    type Error = Error;

    fn try_from(value: &Robj) -> Result<Self> {
        value
            .as_str_iter()
            .ok_or_else(|| Error::ExpectedString(value.clone()))
    }
}

pub trait AsStrIter: GetSexp + Types + Length + Attributes + Rinternals {
    /// Get an iterator over a string vector.
    /// Returns None if the object is not a string vector
    /// but works for factors.
    ///
    /// ```
    /// use extendr_api::prelude::*;
    ///
    /// test! {
    ///     let obj = Robj::from(vec!["a", "b", "c"]);
    ///     assert_eq!(obj.as_str_iter().unwrap().collect::<Vec<_>>(), vec!["a", "b", "c"]);
    ///
    ///     let factor = factor!(vec!["abcd", "def", "fg", "fg"]);
    ///     assert_eq!(factor.levels().unwrap().collect::<Vec<_>>(), vec!["abcd", "def", "fg"]);
    ///     assert_eq!(factor.as_integer_vector().unwrap(), vec![1, 2, 3, 3]);
    ///     assert_eq!(factor.as_str_iter().unwrap().collect::<Vec<_>>(), vec!["abcd", "def", "fg", "fg"]);
    ///     assert_eq!(factor.as_str_iter().unwrap().collect::<Vec<_>>(), vec!["abcd", "def", "fg", "fg"]);
    ///
    ///     let obj = Robj::from(vec![Some("a"), Some("b"), None]);
    ///     assert_eq!(obj.as_str_iter().unwrap().map(|s| s.is_na()).collect::<Vec<_>>(), vec![false, false, true]);
    ///
    ///     let obj = Robj::from(vec!["a", "b", <&str>::na()]);
    ///     assert_eq!(obj.as_str_iter().unwrap().map(|s| s.is_na()).collect::<Vec<_>>(), vec![false, false, true]);
    ///
    ///     let obj = Robj::from(vec!["a", "b", "NA"]);
    ///     assert_eq!(obj.as_str_iter().unwrap().map(|s| s.is_na()).collect::<Vec<_>>(), vec![false, false, false]);
    /// }
    /// ```
    fn as_str_iter(&self) -> Option<StrIter> {
        let i = 0;
        let len = self.len();
        if self.sexptype() == SEXPTYPE::STRSXP {
            unsafe {
                Some(StrIter {
                    vector: self.as_robj().clone(),
                    i,
                    len,
                    levels: R_NilValue,
                })
            }
        } else if self.sexptype() == SEXPTYPE::CHARSXP {
            let len = 1;
            unsafe {
                Some(StrIter {
                    vector: self.as_robj().clone(),
                    i,
                    len,
                    levels: R_NilValue,
                })
            }
        } else if self.is_factor() {
            let levels = self.get_attrib(levels_symbol()).unwrap();
            unsafe {
                Some(StrIter {
                    vector: self.as_robj().clone(),
                    i,
                    len,
                    levels: levels.get(),
                })
            }
        } else {
            None
        }
    }
}

impl AsStrIter for Robj {}
