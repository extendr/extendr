use super::*;
use crate::single_threaded;

pub(crate) fn str_to_character(s: &str) -> SEXP {
    unsafe {
        if s.is_na() {
            R_NaString
        } else {
            single_threaded(|| Rf_mkCharLen(s.as_ptr() as *const raw::c_char, s.len() as i32))
        }
    }
}

/// Convert a null to an Robj.
impl From<()> for Robj {
    fn from(_: ()) -> Self {
        // Note: we do not need to protect this.
        unsafe { Robj::from_sexp(R_NilValue) }
    }
}

/// Convert a Result to an Robj. This is used to allow
/// functions to use the ? operator and return [Result<T>].
///
/// Panics if there is an error.
/// ```
/// use extendr_api::prelude::*;
/// fn my_func() -> Result<f64> {
///     Ok(1.0)
/// }
///
/// test! {
///     assert_eq!(r!(my_func()), r!(1.0));
/// }
/// ```
impl<T> From<Result<T>> for Robj
where
    T: Into<Robj>,
{
    fn from(res: Result<T>) -> Self {
        // Force a panic on error.
        res.unwrap().into()
    }
}

/// Convert an Robj reference into a borrowed Robj.
impl From<&Robj> for Robj {
    // Note: we should probably have a much better reference
    // mechanism as double-free or underprotection is a distinct possibility.
    fn from(val: &Robj) -> Self {
        unsafe { Robj::from_sexp(val.get()) }
    }
}

pub trait IntoRobj {
    fn into_robj(self) -> Robj;
}

impl<T> IntoRobj for T
where
    Robj: From<T>,
{
    fn into_robj(self) -> Robj {
        self.into()
    }
}

/// `ToVectorValue` is a trait that allows many different types
/// to be converted to vectors. It is used as a type parameter
/// to `collect_robj()`.
pub trait ToVectorValue {
    fn sexptype() -> SEXPTYPE {
        0
    }

    fn to_real(&self) -> f64
    where
        Self: Sized,
    {
        0.
    }

    fn to_complex(&self) -> Rcomplex
    where
        Self: Sized,
    {
        Rcomplex { r: 0., i: 0. }
    }

    fn to_integer(&self) -> i32
    where
        Self: Sized,
    {
        std::i32::MIN
    }

    fn to_logical(&self) -> i32
    where
        Self: Sized,
    {
        std::i32::MIN
    }

    fn to_raw(&self) -> u8
    where
        Self: Sized,
    {
        0
    }

    fn to_sexp(&self) -> SEXP
    where
        Self: Sized,
    {
        unsafe { R_NilValue }
    }
}

macro_rules! impl_real_tvv {
    ($t: ty) => {
        impl ToVectorValue for $t {
            fn sexptype() -> SEXPTYPE {
                REALSXP
            }

            fn to_real(&self) -> f64 {
                *self as f64
            }
        }

        impl ToVectorValue for &$t {
            fn sexptype() -> SEXPTYPE {
                REALSXP
            }

            fn to_real(&self) -> f64 {
                **self as f64
            }
        }

        impl ToVectorValue for Option<$t> {
            fn sexptype() -> SEXPTYPE {
                REALSXP
            }

            fn to_real(&self) -> f64 {
                if self.is_some() {
                    self.unwrap() as f64
                } else {
                    unsafe { R_NaReal }
                }
            }
        }
    };
}

impl_real_tvv!(f64);
impl_real_tvv!(f32);

// Since these types might exceeds the max or min of R's 32bit integer, we need
// to return as REALSXP
impl_real_tvv!(i64);
impl_real_tvv!(u32);
impl_real_tvv!(u64);
impl_real_tvv!(usize);

macro_rules! impl_complex_tvv {
    ($t: ty) => {
        impl ToVectorValue for $t {
            fn sexptype() -> SEXPTYPE {
                CPLXSXP
            }

            fn to_complex(&self) -> Rcomplex {
                unsafe { std::mem::transmute(*self) }
            }
        }

        impl ToVectorValue for &$t {
            fn sexptype() -> SEXPTYPE {
                CPLXSXP
            }

            fn to_complex(&self) -> Rcomplex {
                unsafe { std::mem::transmute(**self) }
            }
        }
    };
}

impl_complex_tvv!(c64);
impl_complex_tvv!(Rcplx);
impl_complex_tvv!((f64, f64));

macro_rules! impl_integer_tvv {
    ($t: ty) => {
        impl ToVectorValue for $t {
            fn sexptype() -> SEXPTYPE {
                INTSXP
            }

            fn to_integer(&self) -> i32 {
                *self as i32
            }
        }

        impl ToVectorValue for &$t {
            fn sexptype() -> SEXPTYPE {
                INTSXP
            }

            fn to_integer(&self) -> i32 {
                **self as i32
            }
        }

        impl ToVectorValue for Option<$t> {
            fn sexptype() -> SEXPTYPE {
                INTSXP
            }

            fn to_integer(&self) -> i32 {
                if self.is_some() {
                    self.unwrap() as i32
                } else {
                    unsafe { R_NaInt }
                }
            }
        }
    };
}

impl_integer_tvv!(i8);
impl_integer_tvv!(i16);
impl_integer_tvv!(i32);
impl_integer_tvv!(u16);

impl ToVectorValue for u8 {
    fn sexptype() -> SEXPTYPE {
        RAWSXP
    }

    fn to_raw(&self) -> u8 {
        *self as u8
    }
}

impl ToVectorValue for &u8 {
    fn sexptype() -> SEXPTYPE {
        RAWSXP
    }

    fn to_raw(&self) -> u8 {
        **self as u8
    }
}

macro_rules! impl_str_tvv {
    ($t: ty) => {
        impl ToVectorValue for $t {
            fn sexptype() -> SEXPTYPE {
                STRSXP
            }

            fn to_sexp(&self) -> SEXP
            where
                Self: Sized,
            {
                str_to_character(self.as_ref())
            }
        }

        impl ToVectorValue for &$t {
            fn sexptype() -> SEXPTYPE {
                STRSXP
            }

            fn to_sexp(&self) -> SEXP
            where
                Self: Sized,
            {
                str_to_character(self.as_ref())
            }
        }

        impl ToVectorValue for Option<$t> {
            fn sexptype() -> SEXPTYPE {
                STRSXP
            }

            fn to_sexp(&self) -> SEXP
            where
                Self: Sized,
            {
                if let Some(s) = self {
                    str_to_character(s.as_ref())
                } else {
                    unsafe { R_NaString }
                }
            }
        }
    };
}

impl_str_tvv! {&str}
impl_str_tvv! {String}

impl ToVectorValue for bool {
    fn sexptype() -> SEXPTYPE {
        LGLSXP
    }

    fn to_logical(&self) -> i32
    where
        Self: Sized,
    {
        *self as i32
    }
}

impl ToVectorValue for &bool {
    fn sexptype() -> SEXPTYPE {
        LGLSXP
    }

    fn to_logical(&self) -> i32
    where
        Self: Sized,
    {
        **self as i32
    }
}

impl ToVectorValue for Rbool {
    fn sexptype() -> SEXPTYPE {
        LGLSXP
    }

    fn to_logical(&self) -> i32
    where
        Self: Sized,
    {
        self.inner()
    }
}

impl ToVectorValue for &Rbool {
    fn sexptype() -> SEXPTYPE {
        LGLSXP
    }

    fn to_logical(&self) -> i32
    where
        Self: Sized,
    {
        self.inner()
    }
}

impl ToVectorValue for Option<bool> {
    fn sexptype() -> SEXPTYPE {
        LGLSXP
    }

    fn to_logical(&self) -> i32 {
        if self.is_some() {
            self.unwrap() as i32
        } else {
            unsafe { R_NaInt }
        }
    }
}

// Not thread safe.
fn fixed_size_collect<I>(iter: I, len: usize) -> Robj
where
    I: Iterator,
    I: Sized,
    I::Item: ToVectorValue,
{
    single_threaded(|| unsafe {
        // Length of the vector is known in advance.
        let sexptype = I::Item::sexptype();
        if sexptype != 0 {
            let res = Robj::alloc_vector(sexptype, len);
            let sexp = res.get();
            match sexptype {
                REALSXP => {
                    let ptr = REAL(sexp);
                    for (i, v) in iter.enumerate() {
                        *ptr.add(i) = v.to_real();
                    }
                }
                CPLXSXP => {
                    let ptr = COMPLEX(sexp);
                    for (i, v) in iter.enumerate() {
                        *ptr.add(i) = v.to_complex();
                    }
                }
                INTSXP => {
                    let ptr = INTEGER(sexp);
                    for (i, v) in iter.enumerate() {
                        *ptr.add(i) = v.to_integer();
                    }
                }
                LGLSXP => {
                    let ptr = LOGICAL(sexp);
                    for (i, v) in iter.enumerate() {
                        *ptr.add(i) = v.to_logical();
                    }
                }
                STRSXP => {
                    for (i, v) in iter.enumerate() {
                        SET_STRING_ELT(sexp, i as isize, v.to_sexp());
                    }
                }
                RAWSXP => {
                    let ptr = RAW(sexp);
                    for (i, v) in iter.enumerate() {
                        *ptr.add(i) = v.to_raw();
                    }
                }
                _ => {
                    panic!("unexpected SEXPTYPE in collect_robj");
                }
            }
            res
        } else {
            Robj::from(())
        }
    })
}

/// Extensions to iterators for R objects including [RobjItertools::collect_robj()].
pub trait RobjItertools: Iterator {
    /// Convert a wide range of iterators to Robj.
    /// ```
    /// use extendr_api::prelude::*;
    ///
    /// test! {
    /// // Integer iterators.
    /// let robj = (0..3).collect_robj();
    /// assert_eq!(robj.as_integer_vector().unwrap(), vec![0, 1, 2]);
    ///
    /// // Logical iterators.
    /// let robj = (0..3).map(|x| x % 2 == 0).collect_robj();
    /// assert_eq!(robj.as_logical_vector().unwrap(), vec![TRUE, FALSE, TRUE]);
    ///
    /// // Numeric iterators.
    /// let robj = (0..3).map(|x| x as f64).collect_robj();
    /// assert_eq!(robj.as_real_vector().unwrap(), vec![0., 1., 2.]);
    ///
    /// // String iterators.
    /// let robj = (0..3).map(|x| format!("{}", x)).collect_robj();
    /// assert_eq!(robj.as_str_vector(), Some(vec!["0", "1", "2"]));
    /// }
    /// ```
    fn collect_robj(self) -> Robj
    where
        Self: Iterator,
        Self: Sized,
        Self::Item: ToVectorValue,
    {
        if let (len, Some(max)) = self.size_hint() {
            if len == max {
                return fixed_size_collect(self, len);
            }
        }
        // If the size is indeterminate, create a vector and call recursively.
        let vec: Vec<_> = self.collect();
        assert!(vec.iter().size_hint() == (vec.len(), Some(vec.len())));
        vec.into_iter().collect_robj()
    }
}

// Thanks to *pretzelhammer* on stackoverflow for this.
impl<T> RobjItertools for T where T: Iterator {}

// Scalars which are ToVectorValue
impl<T> From<T> for Robj
where
    T: ToVectorValue,
{
    fn from(scalar: T) -> Self {
        Some(scalar).into_iter().collect_robj()
    }
}

// We would love to do a blanket IntoIterator impl.
// But the matching rules would clash with the above.
macro_rules! impl_from_iter {
    ($t: ty) => {
        impl<'a, T> From<$t> for Robj
        where
            Self: 'a,
            T: Clone + 'a,
            T: ToVectorValue,
        {
            fn from(val: $t) -> Self {
                val.iter().cloned().collect_robj()
            }
        }
    };
}

macro_rules! impl_from_into_iter {
    ($t: ty) => {
        impl<'a, T> From<$t> for Robj
        where
            Self: 'a,
            T: 'a,
            &'a T: ToVectorValue,
        {
            fn from(val: $t) -> Self {
                val.into_iter().collect_robj()
            }
        }
    };
}

macro_rules! impl_from_as_iterator {
    ($t: ty) => {
        impl<T> From<$t> for Robj
        where
            $t: RobjItertools,
            <$t as Iterator>::Item: ToVectorValue,
            T: ToVectorValue,
        {
            fn from(val: $t) -> Self {
                val.collect_robj()
            }
        }
    };
}

// impl<T> From<Range<T>> for Robj
// where
//     Range<T> : RobjItertools,
//     <Range<T> as Iterator>::Item: ToVectorValue,
//     T : ToVectorValue
// {
//     fn from(val: Range<T>) -> Self {
//         val.collect_robj()
//     }
// }

// Template constants are still unstable in rust.
impl_from_iter! {[T; 1]}
impl_from_iter! {[T; 2]}
impl_from_iter! {[T; 3]}
impl_from_iter! {[T; 4]}
impl_from_iter! {[T; 5]}
impl_from_iter! {[T; 6]}
impl_from_iter! {[T; 7]}
impl_from_iter! {[T; 8]}
impl_from_iter! {[T; 9]}
impl_from_iter! {[T; 10]}
impl_from_iter! {[T; 11]}
impl_from_iter! {[T; 12]}
impl_from_iter! {[T; 13]}
impl_from_iter! {[T; 14]}
impl_from_iter! {[T; 15]}
impl_from_iter! {[T; 16]}
impl_from_iter! {[T; 17]}
impl_from_iter! {[T; 18]}
impl_from_iter! {[T; 19]}
impl_from_iter! {Vec<T>}
impl_from_iter! {&Vec<T>}

impl_from_into_iter! {&'a [T]}

impl_from_as_iterator! {Range<T>}
impl_from_as_iterator! {RangeInclusive<T>}

impl<'a> From<Vec<Robj>> for Robj {
    /// Convert a vector of Robj into a list.
    fn from(val: Vec<Robj>) -> Self {
        List::from_values(val.iter()).into()
    }
}

impl<'a> From<Vec<Rstr>> for Robj {
    /// Convert a vector of Rstr into strings.
    fn from(val: Vec<Rstr>) -> Self {
        Strings::from_values(val.into_iter()).into()
    }
}

impl<'a> From<Vec<Rint>> for Robj {
    /// Convert a vector of Rint into integers.
    fn from(val: Vec<Rint>) -> Self {
        Integers::from_values(val.into_iter()).into()
    }
}

impl<'a> From<Vec<Rfloat>> for Robj {
    /// Convert a vector of Rfloat into doubles.
    fn from(val: Vec<Rfloat>) -> Self {
        Doubles::from_values(val.into_iter()).into()
    }
}
