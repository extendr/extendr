use crate::*;
use std::marker::PhantomData;

// Iterator over the objects in a VECSXP, EXPRSXP or WEAKREFSXP.
#[derive(Clone)]
pub struct ListIter {
    vector: SEXP,
    i: usize,
    len: usize,
}

impl Iterator for ListIter {
    type Item = Robj;

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.i;
        self.i += 1;
        if i >= self.len {
            return None;
        } else {
            Some(unsafe { new_owned(VECTOR_ELT(self.vector, i as isize)) })
        }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.i += n;
        self.next()
    }
}

impl std::fmt::Debug for ListIter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for s in self.clone() {
            write!(f, "{:?}", s)?;
        }
        write!(f, "]")
    }
}

/// Iterator over name-value pairs in lists.
pub type NamedListIter = std::iter::Zip<StrIter, ListIter>;

/// Iterator over primitives in integer objects.
pub type IntegerIter<'a> = std::iter::Cloned<std::slice::Iter<'a, i32>>;

/// Iterator over primitives in real objects.
pub type RealIter<'a> = std::iter::Cloned<std::slice::Iter<'a, f64>>;

/// Iterator over primitives in logical objects.
pub type LogicalIter<'a> = std::iter::Cloned<std::slice::Iter<'a, Bool>>;

/// Iterator over the objects in a vector or string.
///
/// ```
/// use extendr_api::*;        // Put API in scope.
/// extendr_engine::start_r(); // Start test environment.
///
/// let my_list = list!(a = 1, b = 2);
/// let mut total = 0;
/// for robj in my_list.as_list_iter().unwrap() {
///   if let Some(val) = robj.as_integer() {
///     total += val;
///   }
/// }
/// assert_eq!(total, 3);
///
/// for name in my_list.names().unwrap() {
///    assert!(name == "a" || name == "b")
/// }
/// ```
#[derive(Clone)]
pub struct PairlistIter {
    list_elem: SEXP,
}

impl PairlistIter {
    /// Make an empty list iterator.
    pub fn new() -> Self {
        unsafe {
            Self {
                list_elem: R_NilValue,
            }
        }
    }
}

impl Iterator for PairlistIter {
    type Item = Robj;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let sexp = self.list_elem;
            if sexp == R_NilValue {
                None
            } else {
                self.list_elem = CDR(sexp);
                Some(new_borrowed(CAR(sexp)))
            }
        }
    }
}

impl std::fmt::Debug for PairlistIter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for s in self.clone() {
            write!(f, "{:?}", s)?;
        }
        write!(f, "]")
    }
}

#[derive(Clone)]
/// Iterator over pairlist tag names.
/// ```
/// use extendr_api::*;        // Put API in scope.
/// extendr_engine::start_r(); // Start test environment.
///
/// let mut robj = R!(pairlist(a = 1, b = 2, 3)).unwrap();
/// // let mut robj = pairlist!(a = 1, b = 2, 3);
/// let tags : Vec<_> = robj.as_pairlist_tag_iter().unwrap().collect();
/// assert_eq!(tags, vec![Some("a"), Some("b"), None]);
/// ```
pub struct PairlistTagIter<'a> {
    list_elem: SEXP,
    phantom: PhantomData<&'a ()>,
}

impl<'a> PairlistTagIter<'a> {
    /// Make an empty list iterator.
    pub fn new() -> Self {
        unsafe {
            Self {
                list_elem: R_NilValue,
                phantom: PhantomData,
            }
        }
    }
}

// 'a is the lifetime of the pairlist.
impl<'a> Iterator for PairlistTagIter<'a> {
    type Item = Option<&'a str>;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let sexp = self.list_elem;
            if sexp == R_NilValue {
                None
            } else {
                self.list_elem = CDR(sexp);
                if let Some(symbol) = new_borrowed::<'a>(TAG(sexp)).as_symbol() {
                    Some(Some(std::mem::transmute(symbol.0)))
                } else {
                    Some(None)
                }
            }
        }
    }
}

impl<'a> std::fmt::Debug for PairlistTagIter<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for s in self.clone() {
            write!(f, "{:?}", s)?;
        }
        write!(f, "]")
    }
}

/// Iterator over strings or string factors.
///
/// ```
/// use extendr_api::*;        // Put API in scope.
/// extendr_engine::start_r(); // Start test environment.
///
/// let robj = r!(["a", "b", "c"]);
/// assert_eq!(robj.as_str_iter().unwrap().collect::<Vec<_>>(), vec!["a", "b", "c"]);
///
/// let factor = factor!(["abcd", "def", "fg", "fg"]);
/// assert_eq!(factor.levels().unwrap().collect::<Vec<_>>(), vec!["abcd", "def", "fg"]);
/// assert_eq!(factor.as_integer_vector().unwrap(), vec![1, 2, 3, 3]);
/// assert_eq!(factor.as_str_iter().unwrap().collect::<Vec<_>>(), vec!["abcd", "def", "fg", "fg"]);
/// assert_eq!(factor.as_str_iter().unwrap().collect::<Vec<_>>(), vec!["abcd", "def", "fg", "fg"]);
/// ```
#[derive(Clone)]
pub struct StrIter {
    vector: SEXP,
    i: usize,
    len: usize,
    levels: SEXP,
}

impl StrIter {
    /// Make an empty str iterator.
    pub fn new() -> Self {
        unsafe {
            Self {
                vector: R_NilValue,
                i: 0,
                len: 0,
                levels: R_NilValue,
            }
        }
    }
}

// Get a string reference from a CHARSXP
fn str_from_strsxp<'a>(sexp: SEXP, index: isize) -> &'a str {
    unsafe {
        if index < 0 || index >= Rf_xlength(sexp) {
            na_str()
        } else {
            let charsxp = STRING_ELT(sexp, index);
            if charsxp == R_NaString {
                na_str()
            } else if TYPEOF(charsxp) == CHARSXP as i32 {
                let ptr = R_CHAR(charsxp) as *const u8;
                let slice = std::slice::from_raw_parts(ptr, Rf_xlength(charsxp) as usize);
                std::str::from_utf8_unchecked(slice)
            } else {
                na_str()
            }
        }
    }
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
            if i >= self.len {
                return None;
            } else if TYPEOF(self.vector) as u32 == STRSXP {
                Some(str_from_strsxp(self.vector, i as isize))
            } else if TYPEOF(self.vector) as u32 == INTSXP && TYPEOF(self.levels) as u32 == STRSXP {
                let j = *(INTEGER(self.vector).offset(i as isize));
                Some(str_from_strsxp(self.levels, j as isize - 1))
            } else {
                return None;
            }
        }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.i += n;
        self.next()
    }
}

impl std::fmt::Debug for StrIter {
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

impl Robj {
    /// Get an iterator over a pairlist objects.
    /// ```
    /// use extendr_api::*;        // Put API in scope.
    /// extendr_engine::start_r(); // Start test environment.
    ///
    /// let mut robj = R!(pairlist(a = 1, b = 2, 3)).unwrap();
    /// let objects : Vec<_> = robj.as_pairlist_iter().unwrap().collect();
    /// assert_eq!(objects, vec![r!(1.0), r!(2.0), r!(3.0)])
    /// ```
    pub fn as_pairlist_iter(&self) -> Option<PairlistIter> {
        match self.sexptype() {
            LISTSXP | LANGSXP | DOTSXP => unsafe {
                Some(PairlistIter {
                    list_elem: self.get(),
                })
            },
            _ => None,
        }
    }

    /// Get an iterator over pairlist tags.
    /// ```
    /// use extendr_api::*;        // Put API in scope.
    /// extendr_engine::start_r(); // Start test environment.
    ///
    /// let mut robj = R!(pairlist(a = 1, b = 2, 3)).unwrap();
    /// // let mut robj = pairlist!(a = 1, b = 2, 3);
    /// let tags : Vec<_> = robj.as_pairlist_tag_iter().unwrap().collect();
    /// assert_eq!(tags, vec![Some("a"), Some("b"), None]);
    /// ```
    pub fn as_pairlist_tag_iter<'a>(&self) -> Option<PairlistTagIter<'a>> {
        match self.sexptype() {
            LISTSXP | LANGSXP | DOTSXP => unsafe {
                Some(PairlistTagIter {
                    list_elem: self.get(),
                    phantom: PhantomData,
                })
            },
            _ => None,
        }
    }

    /// Get an iterator over a list (VECSXP).
    /// ```
    /// use extendr_api::*;        // Put API in scope.
    /// extendr_engine::start_r(); // Start test environment.
    ///
    /// let mut robj = list!(1, 2, 3);
    /// let objects : Vec<_> = robj.as_list_iter().unwrap().collect();
    /// assert_eq!(objects, vec![r!(1), r!(2), r!(3)])
    /// ```
    pub fn as_list_iter(&self) -> Option<ListIter> {
        match self.sexptype() {
            VECSXP | EXPRSXP | WEAKREFSXP => unsafe {
                Some(ListIter {
                    vector: self.get(),
                    i: 0,
                    len: self.len(),
                })
            },
            _ => None,
        }
    }

    /// Get an iterator over a string vector.
    /// Returns None if the object is not a string vector
    /// but works for factors.
    ///
    /// ```
    /// use extendr_api::*;
    ///
    /// extendr_engine::start_r();
    ///
    /// let obj = Robj::from(vec!["a", "b", "c"]);
    /// assert_eq!(obj.as_str_iter().unwrap().collect::<Vec<_>>(), vec!["a", "b", "c"]);
    ///
    /// let factor = factor!(vec!["abcd", "def", "fg", "fg"]);
    /// assert_eq!(factor.levels().unwrap().collect::<Vec<_>>(), vec!["abcd", "def", "fg"]);
    /// assert_eq!(factor.as_integer_vector().unwrap(), vec![1, 2, 3, 3]);
    /// assert_eq!(factor.as_str_iter().unwrap().collect::<Vec<_>>(), vec!["abcd", "def", "fg", "fg"]);
    /// assert_eq!(factor.as_str_iter().unwrap().collect::<Vec<_>>(), vec!["abcd", "def", "fg", "fg"]);
    ///
    /// let obj = Robj::from(vec![Some("a"), Some("b"), None]);
    /// assert_eq!(obj.as_str_iter().unwrap().map(|s| s.is_na()).collect::<Vec<_>>(), vec![false, false, true]);
    ///
    /// let obj = Robj::from(vec!["a", "b", na_str()]);
    /// assert_eq!(obj.as_str_iter().unwrap().map(|s| s.is_na()).collect::<Vec<_>>(), vec![false, false, true]);
    ///
    /// let obj = Robj::from(vec!["a", "b", "NA"]);
    /// assert_eq!(obj.as_str_iter().unwrap().map(|s| s.is_na()).collect::<Vec<_>>(), vec![false, false, false]);
    /// ```
    pub fn as_str_iter(&self) -> Option<StrIter> {
        let i = 0;
        let len = self.len();
        match self.sexptype() {
            STRSXP => unsafe {
                let vector = self.get();
                Some(StrIter {
                    vector,
                    i,
                    len,
                    levels: R_NilValue,
                })
            },
            INTSXP => unsafe {
                let vector = self.get();
                if let Some(levels) = self.get_attrib(levels_symbol()) {
                    if self.is_factor() && levels.sexptype() == STRSXP {
                        Some(StrIter {
                            vector,
                            i,
                            len,
                            levels: levels.get(),
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
            _ => None,
        }
    }
}
