use crate::*;

/// Iterator over the objects in a VECSXP, EXPRSXP or WEAKREFSXP.
///
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let my_list = list!(a = 1, b = 2);
///     let mut total = 0;
///     for robj in my_list.as_list_iter().unwrap() {
///       if let Some(val) = robj.as_integer() {
///         total += val;
///       }
///     }
///     assert_eq!(total, 3);
///    
///     for name in my_list.names().unwrap() {
///        assert!(name == "a" || name == "b")
///     }
/// }
/// ```
#[derive(Clone)]
pub struct ListIter {
    vector: Robj,
    i: usize,
    len: usize,
}

impl ListIter {
    // A new, empty list iterator.
    pub fn new() -> Self {
        ListIter {
            vector: ().into(),
            i: 0,
            len: 0,
        }
    }
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
            Some(unsafe { new_owned(VECTOR_ELT(self.vector.get(), i as isize)) })
        }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.i += n;
        self.next()
    }
}

/// Generalised iterator of numbers and logical. See Int, Real and Logical.
pub struct SliceIter<T> {
    // Control lifetime of vector to make sure the memory is not freed.
    #[allow(dead_code)]
    vector: Robj,
    i: usize,
    len: usize,
    ptr: *const T,
}

impl<T> SliceIter<T> {
    // A new, empty list iterator.
    pub fn new() -> Self {
        SliceIter {
            vector: ().into(),
            i: 0,
            len: 0,
            ptr: std::ptr::null(),
        }
    }

    pub fn from_slice(vector: Robj, slice: &[T]) -> Self {
        SliceIter {
            vector,
            i: 0,
            len: slice.len(),
            ptr: slice.as_ptr(),
        }
    }
}

/// Basis of Int, Real and Logical.
impl<T: Copy> Iterator for SliceIter<T> {
    type Item = T;

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.i;
        self.i += 1;
        if i >= self.len {
            self.i = self.len;
            None
        } else {
            unsafe { Some(*self.ptr.offset(i as isize)) }
        }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.i += n;
        self.next()
    }
}

/// Iterator over name-value pairs in lists.
pub type NamedListIter = std::iter::Zip<StrIter, ListIter>;

/// Iterator over primitives in integer objects.
/// ```
/// use extendr_api::prelude::*;
///
/// fn add(a: Int, b: Int) -> Robj {
///     a.zip(b).map(|(a, b)| a+b).collect_robj()
/// }
/// ```
pub type Int = SliceIter<i32>;

/// Iterator over primitives in real objects.
/// ```
/// use extendr_api::prelude::*;
///
/// fn add1(a: Real) -> Robj {
///     a.map(|a| a + 1.0).collect_robj()
/// }
/// ```
pub type Real = SliceIter<f64>;

/// Iterator over primitives in logical objects.
/// ```
/// use extendr_api::prelude::*;
///
/// fn all_true(mut a: Logical) -> bool {
///     a.all(|a| a.is_true())
/// }
/// ```
pub type Logical = SliceIter<Bool>;

#[derive(Clone)]
pub struct PairlistIter {
    root_obj: Robj,
    list_elem: SEXP,
}

impl PairlistIter {
    /// Make an empty list iterator.
    pub fn new() -> Self {
        unsafe {
            Self {
                root_obj: ().into(),
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

#[derive(Clone)]
/// Iterator over pairlist tag names.
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let mut robj = R!(pairlist(a = 1, b = 2, 3)).unwrap();
///     let tags : Vec<_> = robj.as_pairlist_tag_iter().unwrap().collect();
///     assert_eq!(tags, vec!["a", "b", na_str()]);
/// }
/// ```
pub struct PairlistTagIter {
    root_obj: Robj,
    list_elem: SEXP,
}

impl PairlistTagIter {
    /// Make an empty list iterator.
    pub fn new() -> Self {
        unsafe {
            Self {
                root_obj: ().into(),
                list_elem: R_NilValue,
            }
        }
    }
}

impl Iterator for PairlistTagIter {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let sexp = self.list_elem;
            if sexp == R_NilValue {
                None
            } else {
                self.list_elem = CDR(sexp);
                if let Some(symbol) = new_borrowed(TAG(sexp)).as_symbol() {
                    Some(std::mem::transmute(symbol.0))
                } else {
                    Some(na_str())
                }
            }
        }
    }
}

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

impl StrIter {
    /// Make an empty str iterator.
    pub fn new() -> Self {
        unsafe {
            Self {
                vector: ().into(),
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
            let vector = self.vector.get();
            if i >= self.len {
                return None;
            } else if TYPEOF(vector) as u32 == STRSXP {
                Some(str_from_strsxp(vector, i as isize))
            } else if TYPEOF(vector) as u32 == INTSXP && TYPEOF(self.levels) as u32 == STRSXP {
                let j = *(INTEGER(vector).offset(i as isize));
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

/// Iterator over the names and values of an environment
///
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let names_and_values = (0..100).map(|i| (format!("n{}", i), i));
///     let env = Env{parent: global_env(), names_and_values};
///     let robj = r!(env);
///     let names_and_values = robj.as_env_iter().unwrap().collect::<Vec<_>>();
///     assert_eq!(names_and_values.len(), 100);
///
///     let small_env = new_env_with_capacity(1);
///     small_env.set_local(sym!(x), 1);
///     let names_and_values = small_env.as_env_iter().unwrap().collect::<Vec<_>>();
///     assert_eq!(names_and_values, vec![("x", r!(1))]);
///
///     let large_env = new_env_with_capacity(1000);
///     large_env.set_local(sym!(x), 1);
///     let names_and_values = large_env.as_env_iter().unwrap().collect::<Vec<_>>();
///     assert_eq!(names_and_values, vec![("x", r!(1))]);
/// }
///
/// ```
#[derive(Clone)]
pub struct EnvIter {
    hash_table: ListIter,
    pairlist: PairlistIter,
    pairlisttags: PairlistTagIter,
}

impl Iterator for EnvIter {
    type Item = (&'static str, Robj);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Environments are a hash table (list) or pair lists (pairlist)
            // Get the first available value from the pair list.
            loop {
                match (self.pairlisttags.next(), self.pairlist.next()) {
                    (Some(key), Some(value)) => {
                        // if the key and value are valid, return a pair.
                        if !key.is_na() && !value.is_unbound_value() {
                            println!("value: {:?}", (&key, &value));
                            return Some((key, value));
                        }
                    }
                    // if the key and value are invalid, move on to the hash table.
                    _ => break,
                }
                // continue pair list loop.
            }

            // Get the first pairlist from the hash table.
            loop {
                if let Some(obj) = self.hash_table.next() {
                    if !obj.is_null() && obj.is_pairlist() {
                        self.pairlisttags = obj.as_pairlist_tag_iter().unwrap();
                        self.pairlist = obj.as_pairlist_iter().unwrap();
                        break;
                    }
                // continue hash table loop.
                } else {
                    // The hash table is empty, end of iteration.
                    return None;
                }
            }
        }
    }
}

impl_iter_debug!(ListIter);
impl_iter_debug!(PairlistIter);
impl_iter_debug!(PairlistTagIter);
impl_iter_debug!(StrIter);
impl_iter_debug!(EnvIter);

impl Robj {
    /// Get an iterator over a pairlist objects.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let mut robj = R!(pairlist(a = 1, b = 2, 3)).unwrap();
    ///     let objects : Vec<_> = robj.as_pairlist_iter().unwrap().collect();
    ///     assert_eq!(objects, vec![r!(1.0), r!(2.0), r!(3.0)]);
    /// }
    /// ```
    pub fn as_pairlist_iter(&self) -> Option<PairlistIter> {
        match self.sexptype() {
            LISTSXP | LANGSXP | DOTSXP => unsafe {
                Some(PairlistIter {
                    root_obj: self.into(),
                    list_elem: self.get(),
                })
            },
            _ => None,
        }
    }

    /// Get an iterator over pairlist tags.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let mut robj = R!(pairlist(a = 1, b = 2, 3)).unwrap();
    ///     // let mut robj = pairlist!(a = 1, b = 2, 3);
    ///     let tags : Vec<_> = robj.as_pairlist_tag_iter().unwrap().collect();
    ///     assert_eq!(tags, vec!["a", "b", na_str()]);
    /// }
    /// ```
    pub fn as_pairlist_tag_iter(&self) -> Option<PairlistTagIter> {
        match self.sexptype() {
            LISTSXP | LANGSXP | DOTSXP => unsafe {
                Some(PairlistTagIter {
                    root_obj: self.into(),
                    list_elem: self.get(),
                })
            },
            _ => None,
        }
    }

    /// Get an iterator over a list (VECSXP).
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let mut robj = list!(1, 2, 3);
    ///     let objects : Vec<_> = robj.as_list_iter().unwrap().collect();
    ///     assert_eq!(objects, vec![r!(1), r!(2), r!(3)]);
    /// }
    /// ```
    pub fn as_list_iter(&self) -> Option<ListIter> {
        match self.sexptype() {
            VECSXP | EXPRSXP | WEAKREFSXP => Some(ListIter {
                vector: self.into(),
                i: 0,
                len: self.len(),
            }),
            _ => None,
        }
    }

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
    ///     let obj = Robj::from(vec!["a", "b", na_str()]);
    ///     assert_eq!(obj.as_str_iter().unwrap().map(|s| s.is_na()).collect::<Vec<_>>(), vec![false, false, true]);
    ///
    ///     let obj = Robj::from(vec!["a", "b", "NA"]);
    ///     assert_eq!(obj.as_str_iter().unwrap().map(|s| s.is_na()).collect::<Vec<_>>(), vec![false, false, false]);
    /// }
    /// ```
    pub fn as_str_iter(&self) -> Option<StrIter> {
        let i = 0;
        let len = self.len();
        match self.sexptype() {
            STRSXP => unsafe {
                Some(StrIter {
                    vector: self.into(),
                    i,
                    len,
                    levels: R_NilValue,
                })
            },
            INTSXP => unsafe {
                if let Some(levels) = self.get_attrib(levels_symbol()) {
                    if self.is_factor() && levels.sexptype() == STRSXP {
                        Some(StrIter {
                            vector: self.into(),
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

    /// Iterate over an environment.
    pub fn as_env_iter(&self) -> Option<EnvIter> {
        if self.is_environment() {
            unsafe {
                let hashtab = new_owned(HASHTAB(self.get()));
                let frame = new_owned(FRAME(self.get()));
                if hashtab.is_null() && frame.is_pairlist() {
                    Some(EnvIter {
                        hash_table: ListIter::new(),
                        pairlisttags: frame.as_pairlist_tag_iter().unwrap(),
                        pairlist: frame.as_pairlist_iter().unwrap(),
                    })
                } else if hashtab.is_list() {
                    Some(EnvIter {
                        hash_table: hashtab.as_list_iter().unwrap(),
                        pairlist: PairlistIter::new(),
                        pairlisttags: PairlistTagIter::new(),
                    })
                } else {
                    None
                }
            }
        } else {
            None
        }
    }
}
