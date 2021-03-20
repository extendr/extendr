use crate::*;

use wrapper::symbol::levels_symbol;

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

impl<T: Copy> ExactSizeIterator for SliceIter<T> {
    fn len(&self) -> usize {
        self.len - self.i
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

    pub fn na_iter(len: usize) -> StrIter {
        unsafe {
            Self {
                vector: ().into(),
                i: 0,
                len,
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
                None
            } else if TYPEOF(vector) as u32 == STRSXP {
                Some(str_from_strsxp(vector, i as isize))
            } else if TYPEOF(vector) as u32 == INTSXP && TYPEOF(self.levels) as u32 == STRSXP {
                let j = *(INTEGER(vector).offset(i as isize));
                Some(str_from_strsxp(self.levels, j as isize - 1))
            } else if TYPEOF(vector) as u32 == NILSXP {
                Some(na_str())
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

impl Robj {
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
}
