use crate::robj::Attributes;
use std::iter::FromIterator;

use super::*;

#[derive(PartialEq, Clone)]
pub struct List {
    pub(crate) robj: Robj,
}

impl Default for List {
    fn default() -> Self {
        List::new(0)
    }
}

impl List {
    /// Create a new list.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let list = List::new(10);
    ///     assert_eq!(list.is_list(), true);
    ///     assert_eq!(list.len(), 10);
    /// }
    /// ```
    pub fn new(size: usize) -> Self {
        let robj = Robj::alloc_vector(SEXPTYPE::VECSXP, size);
        Self { robj }
    }

    /// Wrapper for creating a list (VECSXP) object.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let list = r!(List::from_values(&[r!(0), r!(1), r!(2)]));
    ///     assert_eq!(list.is_list(), true);
    ///     assert_eq!(list.len(), 3);
    /// }
    /// ```
    pub fn from_values<V>(values: V) -> Self
    where
        V: IntoIterator,
        V::IntoIter: ExactSizeIterator,
        V::Item: Into<Robj>,
    {
        Self {
            robj: make_vector(SEXPTYPE::VECSXP, values),
        }
    }

    pub fn from_pairs<V>(pairs: V) -> Self
    where
        V: IntoIterator,
        V::IntoIter: ExactSizeIterator + Clone,
        V::Item: KeyValue,
    {
        let iter = pairs.into_iter();
        let mut names = Vec::with_capacity(iter.len());
        let mut values = Vec::with_capacity(iter.len());
        for pair in iter {
            names.push(pair.key());
            values.push(pair.value());
        }
        let mut res = List::from_values(values);
        res.as_robj_mut()
            .set_names(names)
            .unwrap()
            .as_list()
            .unwrap()
    }

    /// Wrapper for creating a list (VECSXP) object from an existing `HashMap`.
    /// The `HashMap` is consumed.
    /// ```
    /// use extendr_api::prelude::*;
    /// use std::collections::HashMap;
    /// test! {
    ///     let mut map: HashMap<&str, Robj> = HashMap::new();
    ///     map.insert("a", r!(1));
    ///     map.insert("b", r!(2));
    ///
    ///     let list = List::from_hashmap(map).unwrap();
    ///     assert_eq!(list.is_list(), true);
    ///
    ///     let mut names : Vec<_> = list.names().unwrap().collect();
    ///     names.sort();
    ///     assert_eq!(names, vec!["a", "b"]);
    /// }
    /// ```
    pub fn from_hashmap<K>(val: HashMap<K, Robj>) -> Result<Self>
    where
        K: Into<String>,
    {
        let mut res: Self = Self::from_values(val.values());
        res.set_names(val.into_keys().map(|k| k.into()))?;
        Ok(res)
    }

    /// Build a list using separate names and values iterators.
    /// Used internally by the `list!` macro.
    pub fn from_names_and_values<N, V>(names: N, values: V) -> Result<Self>
    where
        N: IntoIterator,
        N::IntoIter: ExactSizeIterator,
        N::Item: ToVectorValue + AsRef<str>,
        V: IntoIterator,
        V::IntoIter: ExactSizeIterator,
        V::Item: Into<Robj>,
    {
        let mut list = List::from_values(values);
        list.set_names(names)?;
        Ok(list)
    }

    /// Return an iterator over the values of this list.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let mut robj = list!(1, 2, 3);
    ///     let objects : Vec<_> = robj.as_list().unwrap().values().collect();
    ///     assert_eq!(objects, vec![r!(1), r!(2), r!(3)]);
    /// }
    /// ```
    pub fn values(&self) -> ListIter {
        ListIter::from_parts(self.robj.clone(), 0, self.robj.len())
    }

    /// Return an iterator over the names and values of this list.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let mut list = list!(a=1, 2);
    ///     let names_and_values : Vec<_> = list.iter().collect();
    ///     assert_eq!(names_and_values, vec![("a", r!(1)), ("", r!(2))]);
    /// }
    /// ```
    pub fn iter(&self) -> NamedListIter {
        // TODO: Make a proper NamedListIter.
        self.names()
            .map(|n| n.zip(self.values()))
            .unwrap_or_else(|| StrIter::new(self.len()).zip(self.values()))
    }

    /// Get the list a slice of `Robj`s.
    pub fn as_slice(&self) -> &[Robj] {
        unsafe {
            let data = DATAPTR(self.robj.get()) as *const Robj;
            let len = self.robj.len();
            std::slice::from_raw_parts(data, len)
        }
    }

    /// Get a reference to an element in the list.
    pub fn elt(&self, i: usize) -> Result<Robj> {
        if i >= self.robj.len() {
            Err(Error::OutOfRange(self.robj.clone()))
        } else {
            unsafe {
                let sexp = VECTOR_ELT(self.robj.get(), i as R_xlen_t);
                Ok(Robj::from_sexp(sexp))
            }
        }
    }

    /// Set an element in the list.
    pub fn set_elt(&mut self, i: usize, value: Robj) -> Result<()> {
        single_threaded(|| unsafe {
            if i >= self.robj.len() {
                Err(Error::OutOfRange(self.robj.clone()))
            } else {
                SET_VECTOR_ELT(self.robj.get_mut(), i as R_xlen_t, value.get());
                Ok(())
            }
        })
    }

    /// Convert a List into a HashMap, consuming the list.
    ///
    /// - If an element doesn't have a name, an empty string (i.e. `""`) will be used as the key.
    /// - If there are some duplicated names (including no name, which will be translated as `""`) of elements, only one of those will be preserved.
    /// ```
    /// use extendr_api::prelude::*;
    /// use std::collections::HashMap;
    /// test! {
    ///     let mut robj = list!(a=1, 2);
    ///     let names_and_values = robj.as_list().unwrap().into_hashmap();
    ///     assert_eq!(names_and_values, vec![("a", r!(1)), ("", r!(2))].into_iter().collect::<HashMap<_, _>>());
    /// }
    /// ```
    pub fn into_hashmap(self) -> HashMap<&'static str, Robj> {
        self.iter().collect::<HashMap<&str, Robj>>()
    }
}

impl IntoIterator for List {
    type IntoIter = NamedListIter;
    type Item = (&'static str, Robj);

    /// Convert a List into an interator, consuming the list.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let list = list!(a=1, 2).as_list().unwrap();
    ///     let vec : Vec<_> = list.into_iter().collect();
    ///     assert_eq!(vec, vec![("a", r!(1)), ("", r!(2))]);
    /// }
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Iterator over the objects in a VECSXP, EXPRSXP or WEAKREFSXP.
///
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let my_list = list!(a = 1, b = 2);
///     let mut total = 0;
///     for robj in my_list.as_list().unwrap().values() {
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
    robj: Robj,
    i: usize,
    len: usize,
}

impl Default for ListIter {
    fn default() -> Self {
        ListIter::new()
    }
}

impl ListIter {
    // A new, empty list iterator.
    pub fn new() -> Self {
        ListIter::from_parts(().into(), 0, 0)
    }

    pub(crate) fn from_parts(robj: Robj, i: usize, len: usize) -> Self {
        Self { robj, i, len }
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
            None
        } else {
            Some(unsafe { Robj::from_sexp(VECTOR_ELT(self.robj.get(), i as isize)) })
        }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.i += n;
        self.next()
    }
}

impl ExactSizeIterator for ListIter {
    /// Length of a list iterator.
    fn len(&self) -> usize {
        self.len - self.i
    }
}

/// You can use the FromList wrapper to coerce a Robj into a list.
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let list = Robj::from(list!(1, 2));
///     let vec : FromList<Vec<i32>> = list.try_into()?;
///     assert_eq!(vec.0, vec![1, 2]);
/// }
/// ```
pub struct FromList<T>(pub T);

impl<T> TryFrom<&Robj> for FromList<Vec<T>>
where
    T: TryFrom<Robj>,
    <T as TryFrom<Robj>>::Error: Into<Error>,
{
    type Error = Error;

    fn try_from(robj: &Robj) -> Result<Self> {
        let listiter: ListIter = robj.try_into()?;
        let res: Result<Vec<_>> = listiter
            .map(|robj| T::try_from(robj).map_err(|e| e.into()))
            .collect();
        res.map(FromList)
    }
}

impl<T> TryFrom<Robj> for FromList<Vec<T>>
where
    T: TryFrom<Robj>,
    <T as TryFrom<Robj>>::Error: Into<Error>,
{
    type Error = Error;

    fn try_from(robj: Robj) -> Result<Self> {
        <FromList<Vec<T>>>::try_from(&robj)
    }
}

impl TryFrom<&Robj> for ListIter {
    type Error = Error;

    /// Convert a general R object into a List iterator if possible.
    fn try_from(robj: &Robj) -> Result<Self> {
        let list: List = robj.try_into()?;
        Ok(list.values())
    }
}

impl TryFrom<Robj> for ListIter {
    type Error = Error;

    /// Convert a general R object into a List iterator if possible.
    fn try_from(robj: Robj) -> Result<Self> {
        <ListIter>::try_from(&robj)
    }
}

impl From<ListIter> for Robj {
    /// You can return a ListIter from a function.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let listiter = list!(1, 2).values();
    ///     assert_eq!(Robj::from(listiter), Robj::from(list!(1, 2)));
    /// }
    /// ```
    fn from(iter: ListIter) -> Self {
        iter.robj
    }
}

// TODO: use Rstr or Sym instead of String.
pub trait KeyValue {
    fn key(&self) -> String;
    fn value(self) -> Robj;
}

impl<T: AsRef<str>> KeyValue for (T, Robj) {
    fn key(&self) -> String {
        self.0.as_ref().to_owned()
    }
    fn value(self) -> Robj {
        self.1
    }
}

impl<T: Into<Robj>> FromIterator<T> for List {
    /// Convert an iterator to a `List` object.
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let iter_collect: Vec<_> = iter.into_iter().collect();
        let len = iter_collect.len();

        crate::single_threaded(|| unsafe {
            let mut robj = Robj::alloc_vector(SEXPTYPE::VECSXP, len);
            for (i, v) in iter_collect.into_iter().enumerate() {
                // We don't PROTECT each element here, as they will be immediately
                // placed into a list which will protect them:
                // https://cran.r-project.org/doc/manuals/R-exts.html#Garbage-Collection
                // note: Currently, `Robj` automatically registers `v` by the
                // `ownership`-module, making it protected, even though it isn't necessary to do so.
                let item: Robj = v.into();
                SET_VECTOR_ELT(robj.get_mut(), i as isize, item.get());
            }

            List { robj }
        })
    }
}

impl Attributes for List {}

impl Deref for List {
    type Target = [Robj];

    /// Lists behave like slices of Robj.
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl std::fmt::Debug for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.names().is_none() {
            write!(
                f,
                "list!({})",
                self.values()
                    .map(|v| format!("{:?}", v))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        } else {
            write!(
                f,
                "list!({})",
                self.iter()
                    .map(|(k, v)| if !k.is_empty() {
                        format!("{}={:?}", k, v)
                    } else {
                        format!("{:?}", v)
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
    }
}
