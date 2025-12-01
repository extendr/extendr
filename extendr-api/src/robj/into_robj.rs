use super::*;
use crate::single_threaded;
use extendr_ffi::{
    cetype_t, R_BlankString, R_NaInt, R_NaReal, R_NaString, R_NilValue, Rcomplex, Rf_mkCharLenCE,
    COMPLEX, INTEGER, LOGICAL, RAW, REAL, SET_STRING_ELT, SEXPTYPE,
};

/// Returns an `CHARSXP` based on the provided `&str`.
///
/// Note that R does string interning, thus repeated application of this
/// function on the same string, will incur little computational cost.
///
/// Note, that you must protect the return value somehow.
pub(crate) fn str_to_character(s: &str) -> SEXP {
    unsafe {
        if s.is_na() {
            R_NaString
        } else if s.is_empty() {
            R_BlankString
        } else {
            single_threaded(|| {
                // this function embeds a terminating \nul
                Rf_mkCharLenCE(s.as_ptr().cast(), s.len() as i32, cetype_t::CE_UTF8)
            })
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

/// Convert a [`Result`] to an [`Robj`].
///
/// To use the `?`-operator, an extendr-function must return either [`extendr_api::error::Result`] or [`std::result::Result`].
/// Use of `panic!` in extendr is discouraged due to memory leakage.
///
/// Alternative behaviors enabled by feature toggles:
/// extendr-api supports different conversions from [`Result<T,E>`] into `Robj`.
/// Below, `x_ok` represents an R variable on R side which was returned from rust via `T::into_robj()` or similar.
/// Likewise, `x_err` was returned to R side from rust via `E::into_robj()` or similar.
/// extendr-api
/// * `result_list`: `Ok(T)` is encoded as `list(ok = x_ok, err = NULL)` and `Err` as `list(ok = NULL, err = e_err)`.
/// * `result_condition'`: `Ok(T)` is encoded as `x_ok` and `Err(E)` as `condition(msg="extendr_error", value = x_err, class=c("extendr_error", "error", "condition"))`
/// * More than one enabled feature: Only one feature gate will take effect, the current order of precedence is [`result_list`, `result_condition`, ... ].
/// * Neither of the above (default): `Ok(T)` is encoded as `x_ok` and `Err(E)` will trigger `throw_r_error()` with the error message.
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
///
/// [`extendr_api::error::Result`]: crate::error::Result
#[cfg(not(any(feature = "result_list", feature = "result_condition")))]
impl<T, E> From<std::result::Result<T, E>> for Robj
where
    T: Into<Robj>,
    E: std::fmt::Debug + std::fmt::Display,
{
    fn from(res: std::result::Result<T, E>) -> Self {
        res.unwrap().into()
    }
}

/// Convert a [`Result`] to an [`Robj`]. Return either `Ok` value or `Err` value wrapped in an
/// error condition. This allows using `?` operator in functions
/// and returning [`Result<T>`] without panicking on `Err`. `T` must implement [`IntoRobj`].
///
/// Returns `Ok` value as is. Returns `Err` wrapped in an R error condition. The `Err` is placed in
/// $value field of the condition, and its message is set to 'extendr_err'
#[cfg(all(feature = "result_condition", not(feature = "result_list")))]
impl<T, E> From<std::result::Result<T, E>> for Robj
where
    T: Into<Robj>,
    E: Into<Robj>,
{
    fn from(res: std::result::Result<T, E>) -> Self {
        use crate as extendr_api;
        match res {
            Ok(x) => x.into(),
            Err(x) => {
                let mut err = list!(message = "extendr_err", value = x.into());
                err.set_class(["extendr_error", "error", "condition"])
                    .expect("internal error: failed to set class");
                err.into()
            }
        }
    }
}

/// Convert a `Result` to an R `List` with an `ok` and `err` elements.
/// This allows using `?` operator in functions
/// and returning [`std::result::Result`] or [`extendr_api::error::Result`]
/// without panicking on `Err`.
///
/// [`extendr_api::error::Result`]: crate::error::Result
#[cfg(feature = "result_list")]
impl<T, E> From<std::result::Result<T, E>> for Robj
where
    T: Into<Robj>,
    E: Into<Robj>,
{
    fn from(res: std::result::Result<T, E>) -> Self {
        use crate as extendr_api;
        let mut result = match res {
            Ok(x) => list!(ok = x.into(), err = NULL),
            Err(x) => {
                let err_robj = x.into();
                if err_robj.is_null() {
                    panic!("Internal error: result_list not allowed to return NULL as err-value")
                }
                list!(ok = NULL, err = err_robj)
            }
        };
        result
            .set_class(&["extendr_result"])
            .expect("Internal error: failed to set class");
        result.into()
    }
}

// string conversions from Error trait to Robj and String
impl From<Error> for Robj {
    fn from(res: Error) -> Self {
        res.to_string().into()
    }
}
impl From<Error> for String {
    fn from(res: Error) -> Self {
        res.to_string()
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

/// This is an extension trait to provide a convenience method `into_robj()`.
///
/// Defer to `From<T> for Robj`-impls if you have custom types.
///
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

// ===========================================================
// RNativeType: Marker trait for R's native vector element types
// ===========================================================

/// Marker trait for R's native vector element types.
///
/// This trait is implemented directly on the primitive types that R vectors store:
/// - `f64` for REALSXP (real/numeric vectors)
/// - `i32` for INTSXP (integer vectors)
/// - `Rbool` for LGLSXP (logical vectors)
/// - `u8` for RAWSXP (raw vectors)
/// - `Rcomplex` for CPLXSXP (complex vectors)
///
/// Note: Strings (STRSXP) are handled separately since they store SEXP elements.
pub trait RNativeType: Copy {
    const SEXPTYPE: SEXPTYPE;

    /// Write this value to an R vector at the given index.
    ///
    /// # Safety
    /// - `sexp` must be a vector of `Self::SEXPTYPE`
    /// - `idx` must be less than the vector's length
    unsafe fn write_to_sexp(self, sexp: SEXP, idx: usize);
}

impl RNativeType for f64 {
    const SEXPTYPE: SEXPTYPE = SEXPTYPE::REALSXP;

    #[inline]
    unsafe fn write_to_sexp(self, sexp: SEXP, idx: usize) {
        *REAL(sexp).add(idx) = self;
    }
}

impl RNativeType for i32 {
    const SEXPTYPE: SEXPTYPE = SEXPTYPE::INTSXP;

    #[inline]
    unsafe fn write_to_sexp(self, sexp: SEXP, idx: usize) {
        *INTEGER(sexp).add(idx) = self;
    }
}

impl RNativeType for Rbool {
    const SEXPTYPE: SEXPTYPE = SEXPTYPE::LGLSXP;

    #[inline]
    unsafe fn write_to_sexp(self, sexp: SEXP, idx: usize) {
        *LOGICAL(sexp).add(idx) = self.inner();
    }
}

impl RNativeType for u8 {
    const SEXPTYPE: SEXPTYPE = SEXPTYPE::RAWSXP;

    #[inline]
    unsafe fn write_to_sexp(self, sexp: SEXP, idx: usize) {
        *RAW(sexp).add(idx) = self;
    }
}

impl RNativeType for Rcomplex {
    const SEXPTYPE: SEXPTYPE = SEXPTYPE::CPLXSXP;

    #[inline]
    unsafe fn write_to_sexp(self, sexp: SEXP, idx: usize) {
        *COMPLEX(sexp).add(idx) = self;
    }
}

// &Rcomplex (Rcomplex itself is handled by blanket impl)
impl ToRNative for &Rcomplex {
    type Native = Rcomplex;
    #[inline]
    fn to_r_native(self) -> Rcomplex {
        *self
    }
}

// ===========================================================
// ToRNative: Convert Rust types to R native element types
// ===========================================================

/// Trait for types that can be converted to an R native element type.
///
/// This consolidates the mapping from Rust types to R types and the
/// conversion logic into a single trait.
pub trait ToRNative {
    /// The R native element type this converts to.
    type Native: RNativeType;

    /// Convert to the native R element type.
    fn to_r_native(self) -> Self::Native;
}

/// Backwards-compatible alias for [`ToRNative`].
#[deprecated(since = "0.8.0", note = "Use ToRNative instead")]
pub trait ToVectorValue: ToRNative {}

#[allow(deprecated)]
impl<T: ToRNative> ToVectorValue for T {}

// Blanket impl: any RNativeType converts to itself
impl<T: RNativeType> ToRNative for T {
    type Native = T;

    #[inline]
    fn to_r_native(self) -> Self::Native {
        self
    }
}

// ===========================================================
// fixed_size_collect using ToRNative
// ===========================================================

fn fixed_size_collect<I>(iter: I, len: usize) -> Robj
where
    I: Iterator,
    I::Item: ToRNative,
{
    single_threaded(|| unsafe {
        let sexptype = <I::Item as ToRNative>::Native::SEXPTYPE;
        let res = Robj::alloc_vector(sexptype, len);
        let sexp = res.get();

        for (i, v) in iter.enumerate() {
            v.to_r_native().write_to_sexp(sexp, i);
        }

        res
    })
}

// ===========================================================
// Real-like types -> f64
// ===========================================================

macro_rules! impl_to_r_native_real {
    ($($t:ty => $convert:expr),* $(,)?) => {
        $(
            impl ToRNative for $t {
                type Native = f64;
                #[inline]
                fn to_r_native(self) -> f64 {
                    $convert(self)
                }
            }

            impl ToRNative for &$t {
                type Native = f64;
                #[inline]
                fn to_r_native(self) -> f64 {
                    $convert(*self)
                }
            }

            impl ToRNative for Option<$t> {
                type Native = f64;
                #[inline]
                fn to_r_native(self) -> f64 {
                    match self {
                        Some(x) => $convert(x),
                        None => unsafe { R_NaReal },
                    }
                }
            }
        )*
    }
}

impl_to_r_native_real!(
    f32   => |x: f32| x as f64,
    i64   => |x: i64| x as f64,
    u32   => |x: u32| x as f64,
    u64   => |x: u64| x as f64,
    usize => |x: usize| x as f64,
);

// &f64 (f64 itself is handled by blanket impl)
impl ToRNative for &f64 {
    type Native = f64;
    #[inline]
    fn to_r_native(self) -> f64 {
        *self
    }
}

// Option<f64> (f64 itself is handled by blanket impl)
impl ToRNative for Option<f64> {
    type Native = f64;
    #[inline]
    fn to_r_native(self) -> f64 {
        self.unwrap_or(unsafe { R_NaReal })
    }
}

// Rfloat -> f64
impl ToRNative for Rfloat {
    type Native = f64;
    #[inline]
    fn to_r_native(self) -> f64 {
        self.inner()
    }
}

impl ToRNative for &Rfloat {
    type Native = f64;
    #[inline]
    fn to_r_native(self) -> f64 {
        self.inner()
    }
}

// ===========================================================
// Complex-like types -> Rcomplex
// ===========================================================

impl ToRNative for c64 {
    type Native = Rcomplex;
    #[inline]
    fn to_r_native(self) -> Rcomplex {
        unsafe { std::mem::transmute(self) }
    }
}

impl ToRNative for &c64 {
    type Native = Rcomplex;
    #[inline]
    fn to_r_native(self) -> Rcomplex {
        unsafe { std::mem::transmute(*self) }
    }
}

impl ToRNative for Rcplx {
    type Native = Rcomplex;
    #[inline]
    fn to_r_native(self) -> Rcomplex {
        unsafe { std::mem::transmute(self) }
    }
}

impl ToRNative for &Rcplx {
    type Native = Rcomplex;
    #[inline]
    fn to_r_native(self) -> Rcomplex {
        unsafe { std::mem::transmute(*self) }
    }
}

impl ToRNative for (f64, f64) {
    type Native = Rcomplex;
    #[inline]
    fn to_r_native(self) -> Rcomplex {
        unsafe { std::mem::transmute(self) }
    }
}

impl ToRNative for &(f64, f64) {
    type Native = Rcomplex;
    #[inline]
    fn to_r_native(self) -> Rcomplex {
        unsafe { std::mem::transmute(*self) }
    }
}

// ===========================================================
// Int-like types -> i32
// ===========================================================

macro_rules! impl_to_r_native_int {
    ($($t:ty => $convert:expr),* $(,)?) => {
        $(
            impl ToRNative for $t {
                type Native = i32;
                #[inline]
                fn to_r_native(self) -> i32 {
                    $convert(self)
                }
            }

            impl ToRNative for &$t {
                type Native = i32;
                #[inline]
                fn to_r_native(self) -> i32 {
                    $convert(*self)
                }
            }

            impl ToRNative for Option<$t> {
                type Native = i32;
                #[inline]
                fn to_r_native(self) -> i32 {
                    match self {
                        Some(x) => $convert(x),
                        None => unsafe { R_NaInt },
                    }
                }
            }
        )*
    }
}

impl_to_r_native_int!(
    i8  => |x: i8| x as i32,
    i16 => |x: i16| x as i32,
    u16 => |x: u16| x as i32,
);

// &i32 (i32 itself is handled by blanket impl)
impl ToRNative for &i32 {
    type Native = i32;
    #[inline]
    fn to_r_native(self) -> i32 {
        *self
    }
}

// Option<i32> (i32 itself is handled by blanket impl)
impl ToRNative for Option<i32> {
    type Native = i32;
    #[inline]
    fn to_r_native(self) -> i32 {
        self.unwrap_or(unsafe { R_NaInt })
    }
}

// Rint -> i32
impl ToRNative for Rint {
    type Native = i32;
    #[inline]
    fn to_r_native(self) -> i32 {
        self.inner()
    }
}

impl ToRNative for &Rint {
    type Native = i32;
    #[inline]
    fn to_r_native(self) -> i32 {
        self.inner()
    }
}

// ===========================================================
// Logical-like types -> Rbool
// ===========================================================

impl ToRNative for bool {
    type Native = Rbool;
    #[inline]
    fn to_r_native(self) -> Rbool {
        Rbool::from(self)
    }
}

impl ToRNative for &bool {
    type Native = Rbool;
    #[inline]
    fn to_r_native(self) -> Rbool {
        Rbool::from(*self)
    }
}

impl ToRNative for Option<bool> {
    type Native = Rbool;
    #[inline]
    fn to_r_native(self) -> Rbool {
        match self {
            Some(b) => Rbool::from(b),
            None => Rbool::na(),
        }
    }
}

// &Rbool (Rbool itself is handled by blanket impl)
impl ToRNative for &Rbool {
    type Native = Rbool;
    #[inline]
    fn to_r_native(self) -> Rbool {
        *self
    }
}

// ===========================================================
// Raw-like types -> u8
// ===========================================================

// u8 is handled by blanket impl since u8: RNativeType
// &u8 needs explicit impl
impl ToRNative for &u8 {
    type Native = u8;
    #[inline]
    fn to_r_native(self) -> u8 {
        *self
    }
}

// ===========================================================
// String-like types (special handling for STRSXP)
// ===========================================================

/// Wrapper type for string elements in STRSXP vectors.
/// This is needed because STRSXP stores SEXP (CHARSXP) elements.
#[derive(Copy, Clone)]
pub struct RString(pub SEXP);

impl RNativeType for RString {
    const SEXPTYPE: SEXPTYPE = SEXPTYPE::STRSXP;

    #[inline]
    unsafe fn write_to_sexp(self, sexp: SEXP, idx: usize) {
        SET_STRING_ELT(sexp, idx as isize, self.0);
    }
}

impl ToRNative for &str {
    type Native = RString;
    #[inline]
    fn to_r_native(self) -> RString {
        RString(str_to_character(self))
    }
}

impl ToRNative for &&str {
    type Native = RString;
    #[inline]
    fn to_r_native(self) -> RString {
        RString(str_to_character(self))
    }
}

impl ToRNative for Option<&str> {
    type Native = RString;
    #[inline]
    fn to_r_native(self) -> RString {
        RString(match self {
            Some(s) => str_to_character(s),
            None => unsafe { R_NaString },
        })
    }
}

impl ToRNative for String {
    type Native = RString;
    #[inline]
    fn to_r_native(self) -> RString {
        RString(str_to_character(self.as_str()))
    }
}

impl ToRNative for &String {
    type Native = RString;
    #[inline]
    fn to_r_native(self) -> RString {
        RString(str_to_character(self.as_str()))
    }
}

impl ToRNative for Option<String> {
    type Native = RString;
    #[inline]
    fn to_r_native(self) -> RString {
        RString(match self {
            Some(s) => str_to_character(s.as_str()),
            None => unsafe { R_NaString },
        })
    }
}

impl ToRNative for Rstr {
    type Native = RString;
    #[inline]
    fn to_r_native(self) -> RString {
        RString(unsafe { self.get() })
    }
}

impl ToRNative for &Rstr {
    type Native = RString;
    #[inline]
    fn to_r_native(self) -> RString {
        RString(unsafe { self.get() })
    }
}

impl ToRNative for Option<Rstr> {
    type Native = RString;
    #[inline]
    fn to_r_native(self) -> RString {
        RString(match self {
            Some(s) => unsafe { s.get() },
            None => unsafe { R_NaString },
        })
    }
}

impl TryFrom<&Robj> for Rstr {
    type Error = crate::Error;

    fn try_from(robj: &Robj) -> Result<Self> {
        let sexptype = robj.sexptype();
        if let SEXPTYPE::STRSXP = sexptype {
            if robj.len() == 1 {
                let strs = Strings::try_from(robj)?;
                Ok(strs.elt(0))
            } else {
                Err(Error::ExpectedRstr(robj.clone()))
            }
        } else if let SEXPTYPE::CHARSXP = sexptype {
            Ok(Rstr { robj: robj.clone() })
        } else {
            Err(Error::ExpectedRstr(robj.clone()))
        }
    }
}

impl TryFrom<Robj> for Rstr {
    type Error = crate::Error;

    fn try_from(value: Robj) -> std::result::Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl GetSexp for Rstr {
    unsafe fn get(&self) -> SEXP {
        self.robj.get()
    }

    unsafe fn get_mut(&mut self) -> SEXP {
        self.robj.get_mut()
    }

    fn as_robj(&self) -> &Robj {
        &self.robj
    }

    fn as_robj_mut(&mut self) -> &mut Robj {
        &mut self.robj
    }
}

// These traits all derive from GetSexp with default implementations
impl Length for Rstr {}
impl Types for Rstr {}
impl Conversions for Rstr {}
impl Rinternals for Rstr {}
impl Slices for Rstr {}
impl Operators for Rstr {}

impl ToVectorValue for bool {
    fn sexptype() -> SEXPTYPE {
        SEXPTYPE::LGLSXP
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
        SEXPTYPE::LGLSXP
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
        SEXPTYPE::LGLSXP
    }

    fn to_logical(&self) -> i32
    where
        Self: Sized,
    {
        self.0
    }
}

impl ToVectorValue for &Rbool {
    fn sexptype() -> SEXPTYPE {
        SEXPTYPE::LGLSXP
    }

    fn to_logical(&self) -> i32
    where
        Self: Sized,
    {
        self.0
    }
}

impl ToVectorValue for Option<bool> {
    fn sexptype() -> SEXPTYPE {
        SEXPTYPE::LGLSXP
    }

    fn to_logical(&self) -> i32 {
        if self.is_some() {
            self.unwrap() as i32
        } else {
            unsafe { R_NaInt }
        }
    }
}

impl<T> From<&Option<T>> for Robj
where
    Option<T>: ToRNative + Clone,
{
    fn from(value: &Option<T>) -> Self {
        value.clone().into()
    }
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
        Self::Item: ToRNative,
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

    /// Collects an iterable into an [`RArray`].
    /// The iterable must yield items column by column (aka Fortan order)
    ///
    /// # Arguments
    ///
    /// * `dims` - an array containing the length of each dimension
    fn collect_rarray<const LEN: usize>(self, dims: [usize; LEN]) -> Result<RArray<Self::Item, LEN>>
    where
        Self: Iterator,
        Self: Sized,
        Self::Item: ToRNative,
        Robj: for<'a> AsTypedSlice<'a, Self::Item>,
    {
        let mut vector = self.collect_robj();
        let prod = dims.iter().product::<usize>();
        if prod != vector.len() {
            return Err(Error::Other(format!(
                "The vector length ({}) does not match the length implied by the dimensions ({})",
                vector.len(),
                prod
            )));
        }
        vector.set_attrib(wrapper::symbol::dim_symbol(), dims.iter().collect_robj())?;
        let _data = vector.as_typed_slice().ok_or(Error::Other(
            "Unknown error in converting to slice".to_string(),
        ))?;
        Ok(RArray::from_parts(vector))
    }
}

// Thanks to *pretzelhammer* on stackoverflow for this.
impl<T> RobjItertools for T where T: Iterator {}

// Scalars which are ToRNative
impl<T> From<T> for Robj
where
    T: ToRNative,
{
    fn from(scalar: T) -> Self {
        fixed_size_collect(std::iter::once(scalar), 1)
    }
}

macro_rules! impl_from_as_iterator {
    ($t: ty) => {
        impl<T> From<$t> for Robj
        where
            $t: RobjItertools,
            <$t as Iterator>::Item: ToRNative,
            T: ToRNative,
        {
            fn from(val: $t) -> Self {
                val.collect_robj()
            }
        }
    };
}

impl<T, const N: usize> From<[T; N]> for Robj
where
    T: ToRNative,
{
    fn from(val: [T; N]) -> Self {
        fixed_size_collect(val.into_iter(), N)
    }
}

impl<'a, T, const N: usize> From<&'a [T; N]> for Robj
where
    Self: 'a,
    &'a T: ToRNative + 'a,
{
    fn from(val: &'a [T; N]) -> Self {
        fixed_size_collect(val.iter(), N)
    }
}

impl<'a, T, const N: usize> From<&'a mut [T; N]> for Robj
where
    Self: 'a,
    &'a mut T: ToRNative + 'a,
{
    fn from(val: &'a mut [T; N]) -> Self {
        fixed_size_collect(val.iter_mut(), N)
    }
}

impl<T: ToRNative + Clone> From<&Vec<T>> for Robj {
    fn from(value: &Vec<T>) -> Self {
        let len = value.len();
        fixed_size_collect(value.iter().cloned(), len)
    }
}

impl<T: ToRNative> From<Vec<T>> for Robj {
    fn from(value: Vec<T>) -> Self {
        let len = value.len();
        fixed_size_collect(value.into_iter(), len)
    }
}

impl<'a, T> From<&'a [T]> for Robj
where
    Self: 'a,
    T: 'a,
    &'a T: ToRNative,
{
    fn from(val: &'a [T]) -> Self {
        val.iter().collect_robj()
    }
}

impl_from_as_iterator! {Range<T>}
impl_from_as_iterator! {RangeInclusive<T>}

impl From<Vec<Robj>> for Robj {
    /// Convert a vector of Robj into a list.
    fn from(val: Vec<Robj>) -> Self {
        Self::from(&val)
    }
}

impl From<&Vec<Robj>> for Robj {
    fn from(val: &Vec<Robj>) -> Self {
        List::from_values(val.iter()).into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate as extendr_api;

    #[test]
    fn test_vec_rint_to_robj() {
        test! {
            let int_vec = vec![3,4,0,-2];
            let int_vec_robj: Robj = int_vec.clone().into();
            // unsafe { extendr_ffi::Rf_PrintValue(int_vec_robj.get())}
            assert_eq!(int_vec_robj.as_integer_slice().unwrap(), &int_vec);

            let rint_vec = vec![Rint::from(3), Rint::from(4), Rint::from(0), Rint::from(-2)];
            let rint_vec_robj: Robj = rint_vec.into();
            // unsafe { extendr_ffi::Rf_PrintValue(rint_vec_robj.get())}
            assert_eq!(rint_vec_robj.as_integer_slice().unwrap(), &int_vec);
        }
    }

    #[test]
    fn test_collect_rarray_matrix() {
        test! {
            // Check that collect_rarray works the same as R's matrix() function
            let rmat = (1i32..=16).collect_rarray([4, 4]);
            assert!(rmat.is_ok());
            assert_eq!(Robj::from(rmat), R!("matrix(1:16, nrow=4)").unwrap());
        }
    }

    #[test]
    fn test_collect_rarray_tensor() {
        test! {
            // Check that collect_rarray works the same as R's array() function
            let rmat = (1i32..=16).collect_rarray([2, 4, 2]);
            assert!(rmat.is_ok());
            assert_eq!(Robj::from(rmat), R!("array(1:16, dim=c(2, 4, 2))").unwrap());
        }
    }

    #[test]
    fn test_collect_rarray_matrix_failure() {
        test! {
            // Check that collect_rarray fails when given an invalid shape
            let rmat = (1i32..=16).collect_rarray([3, 3]);
            assert!(rmat.is_err());
            let msg = rmat.unwrap_err().to_string();
            assert!(msg.contains('9'));
            assert!(msg.contains("dimension"));
        }
    }

    #[test]
    fn test_collect_tensor_failure() {
        test! {
            // Check that collect_rarray fails when given an invalid shape
            let rmat = (1i32..=16).collect_rarray([3, 3, 3]);
            assert!(rmat.is_err());
            let msg = rmat.unwrap_err().to_string();
            assert!(msg.contains("27"));
            assert!(msg.contains("dimension"));
        }
    }

    #[test]
    #[cfg(all(feature = "result_condition", not(feature = "result_list")))]
    fn test_result_condition() {
        use crate::prelude::*;
        fn my_err_f() -> std::result::Result<f64, f64> {
            Err(42.0) // return err float
        }

        test! {
                  assert_eq!(
                    r!(my_err_f()),
                    R!(
        "structure(list(message = 'extendr_err',
        value = 42.0), class = c('extendr_error', 'error', 'condition'))"
                    ).unwrap()
                );
            }
    }

    #[test]
    #[cfg(feature = "result_list")]
    fn test_result_list() {
        use crate::prelude::*;
        fn my_err_f() -> std::result::Result<f64, String> {
            Err("We have water in the engine room!".to_string())
        }

        fn my_ok_f() -> std::result::Result<f64, String> {
            Ok(123.123)
        }

        test! {
            assert_eq!(
                r!(my_err_f()),
                R!("x=list(ok=NULL, err='We have water in the engine room!')
                    class(x)='extendr_result'
                    x"
                ).unwrap()
            );
            assert_eq!(
                r!(my_ok_f()),
                R!("x = list(ok=123.123, err=NULL)
                    class(x)='extendr_result'
                    x"
                ).unwrap()
            );
        }
    }
}
