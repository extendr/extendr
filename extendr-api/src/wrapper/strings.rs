use std::convert::From;
use std::iter::FromIterator;

use super::*;

#[derive(PartialEq, Clone)]
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
        let robj = Robj::alloc_vector(SEXPTYPE::STRSXP, size);
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
            let mut robj = Robj::alloc_vector(SEXPTYPE::STRSXP, maxlen);
            let sexp = robj.get_mut();
            for (i, v) in values.into_iter().take(maxlen).enumerate() {
                let v = v.as_ref();
                let ch = str_to_character(v);
                SET_STRING_ELT(sexp, i as R_xlen_t, ch);
            }
            Self { robj }
        })
    }

    /// This is a relatively expensive operation, so use a variable if using this in a loop.
    pub fn as_slice<'a>(&self) -> &'a [Rstr] {
        unsafe {
            let data = STRING_PTR_RO(self.robj.get()) as *const Rstr;
            let len = self.robj.len();
            std::slice::from_raw_parts(data, len)
        }
    }

    /// Get an element in a string vector.
    pub fn elt(&self, i: usize) -> Rstr {
        if i >= self.len() {
            Rstr::na()
        } else {
            Robj::from_sexp(unsafe { STRING_ELT(self.get(), i as R_xlen_t) })
                .try_into()
                .unwrap()
        }
    }

    /// Set a single element of this string vector.
    pub fn set_elt(&mut self, i: usize, e: Rstr) {
        single_threaded(|| unsafe {
            if i < self.len() {
                SET_STRING_ELT(self.robj.get_mut(), i as isize, e.get());
            }
        });
    }

    /// Get an iterator for this string vector.
    pub fn iter(&self) -> impl Iterator<Item = &Rstr> {
        self.as_slice().iter()
    }

    /// Return `TRUE` if the vector is sorted, `FALSE` if not, or `NA_BOOL` if unknown.
    pub fn is_sorted(&self) -> Rbool {
        unsafe { STRING_IS_SORTED(self.get()).into() }
    }

    /// Return `TRUE` if the vector has no `NA`s, `FALSE` if any, or `NA_BOOL` if unknown.
    pub fn no_na(&self) -> Rbool {
        unsafe { STRING_NO_NA(self.get()).into() }
    }
}

impl Attributes for Strings {}

impl<T: AsRef<str>> FromIterator<T> for Strings {
    /// Convert an iterator to a Strings object.
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let iter_collect: Vec<_> = iter.into_iter().collect();
        let len = iter_collect.len();

        let mut robj = Strings::alloc_vector(SEXPTYPE::STRSXP, len);
        crate::single_threaded(|| unsafe {
            for (i, v) in iter_collect.into_iter().enumerate() {
                SET_STRING_ELT(robj.get_mut(), i as isize, str_to_character(v.as_ref()));
            }
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

impl Deref for Strings {
    type Target = [Rstr];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl std::fmt::Debug for Strings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.len() == 1 {
            write!(f, "{:?}", self.elt(0))
        } else {
            f.debug_list().entries(self.iter()).finish()
        }
    }
}

impl From<Option<Strings>> for Robj {
    fn from(value: Option<Strings>) -> Self {
        match value {
            Some(value_strings) => value_strings.into(),
            None => nil_value(),
        }
    }
}
