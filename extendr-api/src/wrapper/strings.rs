use std::convert::From;
use std::iter::FromIterator;

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Strings {
    pub(crate) robj: Robj,
}

impl Default for Strings {
    fn default() -> Self {
        Strings::new(0)
    }
}

impl Strings {
    /// Create a new, empty list.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let strings = Strings::new(10);
    ///     assert_eq!(strings.is_string(), true);
    ///     assert_eq!(strings.len(), 10);
    /// }
    /// ```
    pub fn new(size: usize) -> Strings {
        let robj = Robj::alloc_vector(STRSXP, size);
        Self { robj }
    }

    /// Wrapper for creating string vector (STRSXP) objects.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let list = r!(Strings::from_values(&["x", "y", "z"]));
    ///     assert_eq!(list.is_string(), true);
    ///     assert_eq!(list.len(), 3);
    /// }
    /// ```
    pub fn from_values<V>(values: V) -> Self
    where
        V: IntoIterator,
        V::IntoIter: ExactSizeIterator,
        V::Item: AsRef<str>,
    {
        single_threaded(|| unsafe {
            let values = values.into_iter();
            let maxlen = values.len();
            let robj = Robj::alloc_vector(STRSXP, maxlen);
            let sexp = robj.get();
            for (i, v) in values.into_iter().take(maxlen).enumerate() {
                let v = v.as_ref();
                let ch = str_to_character(v);
                // let ch = Rf_mkCharLen(v.as_ptr() as *mut c_char, v.len() as c_int);
                SET_STRING_ELT(sexp, i as R_xlen_t, ch);
            }
            Self { robj }
        })
    }

    /// This is a relatively expensive operation, so use a variable if using this in a loop.
    pub fn as_slice(&self) -> &[Rstr] {
        unsafe {
            let data = STRING_PTR_RO(self.robj.get()) as *const Rstr;
            let len = self.robj.len();
            std::slice::from_raw_parts(data, len)
        }
    }

    /// This is a relatively expensive operation, so use a variable if using this in a loop.
    pub fn elt(&self, i: usize) -> &str {
        if i >= self.len() {
            <&str>::na()
        } else {
            use crate::wrapper::rstr::sexp_to_str;
            unsafe { sexp_to_str(STRING_ELT(self.get(), i as R_xlen_t)) }
        }
    }

    /// Set a single element of this string vector.
    pub fn set_elt<T: AsRef<str>>(&mut self, i: usize, e: T) {
        single_threaded(|| unsafe {
            if i < self.len() {
                SET_STRING_ELT(self.robj.get(), i as isize, str_to_character(e.as_ref()));
            }
        });
    }

    /// Get an iterator for this string vector.
    pub fn iter(&self) -> impl Iterator<Item = &Rstr> {
        self.as_slice().iter()
    }

    /// Return `TRUE` if the vector is sorted, `FALSE` if not, or `NA_BOOL` if unknown.
    pub fn is_sorted(&self) -> Bool {
        unsafe { STRING_IS_SORTED(self.get()).into() }
    }

    /// Return `TRUE` if the vector has no `NA`s, `FALSE` if any, or `NA_BOOL` if unknown.
    pub fn no_na(&self) -> Bool {
        unsafe { STRING_NO_NA(self.get()).into() }
    }
}

impl<T: AsRef<str>> FromIterator<T> for Strings {
    /// Convert an iterator to a Strings object.
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        crate::single_threaded(|| unsafe {
            let values: Vec<SEXP> = iter
                .into_iter()
                .map(|s| Rf_protect(str_to_character(s.as_ref())))
                .collect();

            let len = values.len();
            let robj = Robj::alloc_vector(STRSXP, len);
            for (i, v) in values.into_iter().enumerate() {
                SET_STRING_ELT(robj.get(), i as isize, v);
            }
            Rf_unprotect(len as i32);

            Strings { robj }
        })
    }
}

impl<T> From<T> for Strings
where
    T: AsRef<str>,
{
    /// convert string-like objects into a Strings object.
    fn from(value: T) -> Self {
        Strings::from_values([value.as_ref()])
    }
}
