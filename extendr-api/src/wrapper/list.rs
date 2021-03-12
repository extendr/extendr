use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct List {
    pub(crate) robj: Robj,
}

impl List {
    /// Wrapper for creating list (VECSXP) objects.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let list = r!(List::from_objects(&[r!(0), r!(1), r!(2)]));
    ///     assert_eq!(list.is_list(), true);
    ///     assert_eq!(list.len(), 3);
    ///     assert_eq!(format!("{:?}", list), r#"r!(List::from_objects([r!(0), r!(1), r!(2)]))"#);
    /// }
    /// ```
    pub fn from_objects<V>(values: V) -> Self
    where
        V: IntoIterator,
        V::IntoIter: ExactSizeIterator,
        V::Item: Into<Robj>,
    {
        Self {
            robj: make_vector(VECSXP, values),
        }
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
    ///     let mut robj = list!(a=1, 2);
    ///     let names_and_values : Vec<_> = robj.as_list().unwrap().iter().collect();
    ///     assert_eq!(names_and_values, vec![("a", r!(1)), ("", r!(2))]);
    /// }
    /// ```
    pub fn iter(&self) -> NamedListIter {
        self.names()
            .map(|n| n.zip(self.values()))
            .unwrap_or_else(|| StrIter::new().zip(ListIter::new()))
    }
}

impl IntoIterator for List {
    type IntoIter = NamedListIter;
    type Item = (&'static str, Robj);

    /// Convert a List into an interator, consuming th epairlist.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let list = list!(a=1, 2).as_list().unwrap();
    ///     let vec : Vec<_> = list.into_iter().collect();
    ///     assert_eq!(vec, vec![("a", r!(1)), ("", r!(2))]);
    /// }
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.iter().clone()
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

impl ListIter {
    // A new, empty list iterator.
    pub fn new() -> Self {
        ListIter::from_parts(().into(), 0, 0)
    }

    pub fn from_parts(robj: Robj, i: usize, len: usize) -> Self {
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
            return None;
        } else {
            Some(unsafe { new_owned(VECTOR_ELT(self.robj.get(), i as isize)) })
        }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.i += n;
        self.next()
    }
}

impl TryFrom<Robj> for ListIter {
    type Error = Error;

    /// You pass a ListIter to a function.
    fn try_from(robj: Robj) -> Result<Self> {
        let pairlist: ListIter = robj.try_into()?;
        Ok(pairlist.into_iter())
    }
}

impl From<ListIter> for Robj {
    /// You can return a ListIter from a function.
    fn from(iter: ListIter) -> Self {
        iter.robj.clone()
    }
}

impl<'a> FromRobj<'a> for ListIter {
    fn from_robj(robj: &'a Robj) -> std::result::Result<Self, &'static str> {
        robj.as_list().map(|l| l.values()).ok_or("Not a list.")
    }
}
