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

#[allow(non_snake_case)]
extern "C" fn LOGICAL_AS_RBOOL(sexp: SEXP) -> *mut Rbool {
    unsafe { LOGICAL(sexp).cast() }
}

#[allow(non_snake_case)]
extern "C" fn REAL_AS_RFLOAT(sexp: SEXP) -> *mut Rfloat {
    unsafe { REAL(sexp).cast() }
}

#[allow(non_snake_case)]
extern "C" fn COMPLEX_AS_RCPLX(sexp: SEXP) -> *mut Rcplx {
    unsafe { COMPLEX(sexp).cast() }
}

#[allow(non_snake_case)]
extern "C" fn COMPLEX_AS_C64(sexp: SEXP) -> *mut c64 {
    unsafe { COMPLEX(sexp).cast() }
}

#[allow(non_snake_case)]
extern "C" fn COMPLEX_AS_FLOAT_TUPLE(sexp: SEXP) -> *mut (f64, f64) {
    unsafe { COMPLEX(sexp).cast() }
}

#[allow(non_snake_case)]
extern "C" fn INTEGER_AS_RINT(sexp: SEXP) -> *mut Rint {
    unsafe { INTEGER(sexp).cast() }
}

pub trait RNativeType: Copy {
    const SEXPTYPE: SEXPTYPE;
    const PTR: unsafe extern "C" fn(SEXP) -> *mut Self;
}

pub trait RSliceNative: RNativeType {}

pub trait CoerceNative {
    type Target: RSliceNative;
    fn coerce(&self) -> Self::Target;
}

impl RNativeType for f64 {
    const SEXPTYPE: SEXPTYPE = SEXPTYPE::REALSXP;
    const PTR: unsafe extern "C" fn(SEXP) -> *mut Self = REAL;
}
impl RSliceNative for f64 {}

impl RNativeType for Rfloat {
    const SEXPTYPE: SEXPTYPE = SEXPTYPE::REALSXP;
    const PTR: unsafe extern "C" fn(SEXP) -> *mut Self = REAL_AS_RFLOAT;
}
impl RSliceNative for Rfloat {}

impl RNativeType for Rcomplex {
    const SEXPTYPE: SEXPTYPE = SEXPTYPE::CPLXSXP;
    const PTR: unsafe extern "C" fn(SEXP) -> *mut Self = COMPLEX;
}
impl RSliceNative for Rcomplex {}

impl RNativeType for Rcplx {
    const SEXPTYPE: SEXPTYPE = SEXPTYPE::CPLXSXP;
    const PTR: unsafe extern "C" fn(SEXP) -> *mut Self = COMPLEX_AS_RCPLX;
}
impl RSliceNative for Rcplx {}

impl RNativeType for c64 {
    const SEXPTYPE: SEXPTYPE = SEXPTYPE::CPLXSXP;
    const PTR: unsafe extern "C" fn(SEXP) -> *mut Self = COMPLEX_AS_C64;
}
impl RSliceNative for c64 {}

impl RNativeType for (f64, f64) {
    const SEXPTYPE: SEXPTYPE = SEXPTYPE::CPLXSXP;
    const PTR: unsafe extern "C" fn(SEXP) -> *mut Self = COMPLEX_AS_FLOAT_TUPLE;
}
impl RSliceNative for (f64, f64) {}

impl RNativeType for i32 {
    const SEXPTYPE: SEXPTYPE = SEXPTYPE::INTSXP;
    const PTR: unsafe extern "C" fn(SEXP) -> *mut Self = INTEGER;
}
impl RSliceNative for i32 {}

impl RNativeType for Rint {
    const SEXPTYPE: SEXPTYPE = SEXPTYPE::INTSXP;
    const PTR: unsafe extern "C" fn(SEXP) -> *mut Self = INTEGER_AS_RINT;
}
impl RSliceNative for Rint {}

impl RNativeType for Rbool {
    const SEXPTYPE: SEXPTYPE = SEXPTYPE::LGLSXP;
    const PTR: unsafe extern "C" fn(SEXP) -> *mut Self = LOGICAL_AS_RBOOL;
}
impl RSliceNative for Rbool {}

impl RNativeType for u8 {
    const SEXPTYPE: SEXPTYPE = SEXPTYPE::RAWSXP;
    const PTR: unsafe extern "C" fn(SEXP) -> *mut Self = RAW;
}
impl RSliceNative for u8 {}

impl CoerceNative for bool {
    type Target = Rbool;

    fn coerce(&self) -> Self::Target {
        Rbool::from(*self)
    }
}

impl CoerceNative for Option<bool> {
    type Target = Rbool;

    fn coerce(&self) -> Self::Target {
        match self {
            Some(v) => Rbool::from(*v),
            None => Rbool::na(),
        }
    }
}

impl<T> CoerceNative for Option<T>
where
    T: RSliceNative + CanBeNA,
{
    type Target = T;

    fn coerce(&self) -> Self::Target {
        match self {
            Some(v) => *v,
            None => T::na(),
        }
    }
}

fn collect_native<T, I>(iterable: I) -> Robj
where
    T: RNativeType,
    I: IntoIterator<Item = T>,
{
    let mut iter = iterable.into_iter();
    if let (len, Some(max)) = iter.size_hint() {
        if len == max {
            return single_threaded(|| unsafe {
                let robj = Robj::alloc_vector(T::SEXPTYPE, len);
                if len != 0 {
                    let dest = std::slice::from_raw_parts_mut((T::PTR)(robj.get()), len);
                    for (idx, value) in (&mut iter).enumerate() {
                        *dest.get_unchecked_mut(idx) = value;
                    }
                }
                robj
            });
        }
    }

    let values: Vec<T> = iter.collect();
    single_threaded(|| unsafe {
        let len = values.len();
        let robj = Robj::alloc_vector(T::SEXPTYPE, len);
        if len != 0 {
            let dest = std::slice::from_raw_parts_mut((T::PTR)(robj.get()), len);
            dest.copy_from_slice(&values);
        }
        robj
    })
}

fn copy_slice_to_robj<T>(slice: &[T]) -> Robj
where
    T: RSliceNative,
{
    single_threaded(|| unsafe {
        let robj = Robj::alloc_vector(T::SEXPTYPE, slice.len());
        if !slice.is_empty() {
            let dest = std::slice::from_raw_parts_mut((T::PTR)(robj.get()), slice.len());
            dest.copy_from_slice(slice);
        }
        robj
    })
}

fn coerce_slice_to_robj<T>(slice: &[T]) -> Robj
where
    T: CoerceNative,
{
    let mut values = Vec::with_capacity(slice.len());
    for value in slice {
        values.push(value.coerce());
    }
    copy_slice_to_robj(values.as_slice())
}

fn collect_strings<I, S, F>(iterable: I, to_sexp: F) -> Robj
where
    I: IntoIterator<Item = S>,
    F: Fn(S) -> SEXP + Copy,
{
    let mut iter = iterable.into_iter();
    if let (len, Some(max)) = iter.size_hint() {
        if len == max {
            let mut robj = Robj::alloc_vector(SEXPTYPE::STRSXP, len);
            let sexp = unsafe { robj.get_mut() };
            return single_threaded(|| unsafe {
                for (idx, value) in (&mut iter).enumerate() {
                    SET_STRING_ELT(sexp, idx as isize, to_sexp(value));
                }
                robj
            });
        }
    }

    let values: Vec<S> = iter.collect();
    single_threaded(|| unsafe {
        let robj = Robj::alloc_vector(SEXPTYPE::STRSXP, values.len());
        let sexp = robj.get();
        for (idx, value) in values.into_iter().enumerate() {
            SET_STRING_ELT(sexp, idx as isize, to_sexp(value));
        }
        robj
    })
}

trait StrLike: Clone {
    fn to_sexp(&self) -> SEXP;
}

impl StrLike for &str {
    fn to_sexp(&self) -> SEXP {
        str_to_character(self)
    }
}

impl StrLike for String {
    fn to_sexp(&self) -> SEXP {
        str_to_character(self.as_str())
    }
}

impl StrLike for Rstr {
    fn to_sexp(&self) -> SEXP {
        unsafe { self.get() }
    }
}

impl FromIterator<f64> for Robj {
    fn from_iter<I: IntoIterator<Item = f64>>(iter: I) -> Self {
        collect_native(iter)
    }
}

impl<'a> FromIterator<&'a f64> for Robj {
    fn from_iter<I: IntoIterator<Item = &'a f64>>(iter: I) -> Self {
        collect_native(iter.into_iter().copied())
    }
}

impl FromIterator<f32> for Robj {
    fn from_iter<I: IntoIterator<Item = f32>>(iter: I) -> Self {
        collect_native(iter.into_iter().map(|v| v as f64))
    }
}

impl<'a> FromIterator<&'a f32> for Robj {
    fn from_iter<I: IntoIterator<Item = &'a f32>>(iter: I) -> Self {
        collect_native(iter.into_iter().map(|v| *v as f64))
    }
}

impl FromIterator<i64> for Robj {
    fn from_iter<I: IntoIterator<Item = i64>>(iter: I) -> Self {
        collect_native(iter.into_iter().map(|v| v as f64))
    }
}

impl<'a> FromIterator<&'a i64> for Robj {
    fn from_iter<I: IntoIterator<Item = &'a i64>>(iter: I) -> Self {
        collect_native(iter.into_iter().map(|v| *v as f64))
    }
}

impl FromIterator<u32> for Robj {
    fn from_iter<I: IntoIterator<Item = u32>>(iter: I) -> Self {
        collect_native(iter.into_iter().map(|v| v as f64))
    }
}

impl<'a> FromIterator<&'a u32> for Robj {
    fn from_iter<I: IntoIterator<Item = &'a u32>>(iter: I) -> Self {
        collect_native(iter.into_iter().map(|v| *v as f64))
    }
}

impl FromIterator<u64> for Robj {
    fn from_iter<I: IntoIterator<Item = u64>>(iter: I) -> Self {
        collect_native(iter.into_iter().map(|v| v as f64))
    }
}

impl<'a> FromIterator<&'a u64> for Robj {
    fn from_iter<I: IntoIterator<Item = &'a u64>>(iter: I) -> Self {
        collect_native(iter.into_iter().map(|v| *v as f64))
    }
}

impl FromIterator<usize> for Robj {
    fn from_iter<I: IntoIterator<Item = usize>>(iter: I) -> Self {
        collect_native(iter.into_iter().map(|v| v as f64))
    }
}

impl<'a> FromIterator<&'a usize> for Robj {
    fn from_iter<I: IntoIterator<Item = &'a usize>>(iter: I) -> Self {
        collect_native(iter.into_iter().map(|v| *v as f64))
    }
}

impl FromIterator<Option<f64>> for Robj {
    fn from_iter<I: IntoIterator<Item = Option<f64>>>(iter: I) -> Self {
        collect_native(iter.into_iter().map(|v| v.unwrap_or(unsafe { R_NaReal })))
    }
}

impl FromIterator<Option<f32>> for Robj {
    fn from_iter<I: IntoIterator<Item = Option<f32>>>(iter: I) -> Self {
        collect_native(
            iter.into_iter()
                .map(|v| v.map(|v| v as f64).unwrap_or(unsafe { R_NaReal })),
        )
    }
}

impl FromIterator<Option<i64>> for Robj {
    fn from_iter<I: IntoIterator<Item = Option<i64>>>(iter: I) -> Self {
        collect_native(
            iter.into_iter()
                .map(|v| v.map(|v| v as f64).unwrap_or(unsafe { R_NaReal })),
        )
    }
}

impl FromIterator<Option<u32>> for Robj {
    fn from_iter<I: IntoIterator<Item = Option<u32>>>(iter: I) -> Self {
        collect_native(
            iter.into_iter()
                .map(|v| v.map(|v| v as f64).unwrap_or(unsafe { R_NaReal })),
        )
    }
}

impl FromIterator<Option<u64>> for Robj {
    fn from_iter<I: IntoIterator<Item = Option<u64>>>(iter: I) -> Self {
        collect_native(
            iter.into_iter()
                .map(|v| v.map(|v| v as f64).unwrap_or(unsafe { R_NaReal })),
        )
    }
}

impl FromIterator<Option<usize>> for Robj {
    fn from_iter<I: IntoIterator<Item = Option<usize>>>(iter: I) -> Self {
        collect_native(
            iter.into_iter()
                .map(|v| v.map(|v| v as f64).unwrap_or(unsafe { R_NaReal })),
        )
    }
}

impl FromIterator<c64> for Robj {
    fn from_iter<I: IntoIterator<Item = c64>>(iter: I) -> Self {
        collect_native(iter.into_iter().map(|v| {
            let rc = Rcplx::from(v);
            Rcomplex {
                r: rc.re().0,
                i: rc.im().0,
            }
        }))
    }
}

impl<'a> FromIterator<&'a c64> for Robj {
    fn from_iter<I: IntoIterator<Item = &'a c64>>(iter: I) -> Self {
        collect_native(iter.into_iter().map(|v| {
            let rc = Rcplx::from(*v);
            Rcomplex {
                r: rc.re().0,
                i: rc.im().0,
            }
        }))
    }
}

impl FromIterator<Rcplx> for Robj {
    fn from_iter<I: IntoIterator<Item = Rcplx>>(iter: I) -> Self {
        collect_native(iter.into_iter().map(|v| Rcomplex {
            r: v.re().0,
            i: v.im().0,
        }))
    }
}

impl<'a> FromIterator<&'a Rcplx> for Robj {
    fn from_iter<I: IntoIterator<Item = &'a Rcplx>>(iter: I) -> Self {
        collect_native(iter.into_iter().map(|v| Rcomplex {
            r: v.re().0,
            i: v.im().0,
        }))
    }
}

impl FromIterator<(f64, f64)> for Robj {
    fn from_iter<I: IntoIterator<Item = (f64, f64)>>(iter: I) -> Self {
        collect_native(iter.into_iter().map(|(r, i)| Rcomplex { r, i }))
    }
}

impl<'a> FromIterator<&'a (f64, f64)> for Robj {
    fn from_iter<I: IntoIterator<Item = &'a (f64, f64)>>(iter: I) -> Self {
        collect_native(iter.into_iter().map(|(r, i)| Rcomplex { r: *r, i: *i }))
    }
}

impl FromIterator<i8> for Robj {
    fn from_iter<I: IntoIterator<Item = i8>>(iter: I) -> Self {
        collect_native(iter.into_iter().map(|v| v as i32))
    }
}

impl<'a> FromIterator<&'a i8> for Robj {
    fn from_iter<I: IntoIterator<Item = &'a i8>>(iter: I) -> Self {
        collect_native(iter.into_iter().map(|v| *v as i32))
    }
}

impl FromIterator<i16> for Robj {
    fn from_iter<I: IntoIterator<Item = i16>>(iter: I) -> Self {
        collect_native(iter.into_iter().map(|v| v as i32))
    }
}

impl<'a> FromIterator<&'a i16> for Robj {
    fn from_iter<I: IntoIterator<Item = &'a i16>>(iter: I) -> Self {
        collect_native(iter.into_iter().map(|v| *v as i32))
    }
}

impl FromIterator<i32> for Robj {
    fn from_iter<I: IntoIterator<Item = i32>>(iter: I) -> Self {
        collect_native(iter)
    }
}

impl<'a> FromIterator<&'a i32> for Robj {
    fn from_iter<I: IntoIterator<Item = &'a i32>>(iter: I) -> Self {
        collect_native(iter.into_iter().copied())
    }
}

impl FromIterator<u16> for Robj {
    fn from_iter<I: IntoIterator<Item = u16>>(iter: I) -> Self {
        collect_native(iter.into_iter().map(|v| v as i32))
    }
}

impl<'a> FromIterator<&'a u16> for Robj {
    fn from_iter<I: IntoIterator<Item = &'a u16>>(iter: I) -> Self {
        collect_native(iter.into_iter().map(|v| *v as i32))
    }
}

impl FromIterator<Option<i8>> for Robj {
    fn from_iter<I: IntoIterator<Item = Option<i8>>>(iter: I) -> Self {
        collect_native(
            iter.into_iter()
                .map(|v| v.map(|v| v as i32).unwrap_or(unsafe { R_NaInt })),
        )
    }
}

impl FromIterator<Option<i16>> for Robj {
    fn from_iter<I: IntoIterator<Item = Option<i16>>>(iter: I) -> Self {
        collect_native(
            iter.into_iter()
                .map(|v| v.map(|v| v as i32).unwrap_or(unsafe { R_NaInt })),
        )
    }
}

impl FromIterator<Option<i32>> for Robj {
    fn from_iter<I: IntoIterator<Item = Option<i32>>>(iter: I) -> Self {
        collect_native(iter.into_iter().map(|v| v.unwrap_or(unsafe { R_NaInt })))
    }
}

impl FromIterator<Option<u16>> for Robj {
    fn from_iter<I: IntoIterator<Item = Option<u16>>>(iter: I) -> Self {
        collect_native(
            iter.into_iter()
                .map(|v| v.map(|v| v as i32).unwrap_or(unsafe { R_NaInt })),
        )
    }
}

impl FromIterator<u8> for Robj {
    fn from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Self {
        collect_native(iter)
    }
}

impl<'a> FromIterator<&'a u8> for Robj {
    fn from_iter<I: IntoIterator<Item = &'a u8>>(iter: I) -> Self {
        collect_native(iter.into_iter().copied())
    }
}

impl<'a> FromIterator<&'a str> for Robj {
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        collect_strings(iter, str_to_character)
    }
}

impl<'a, 'b> FromIterator<&'a &'b str> for Robj {
    fn from_iter<I: IntoIterator<Item = &'a &'b str>>(iter: I) -> Self {
        collect_strings(iter.into_iter().copied(), str_to_character)
    }
}

impl FromIterator<String> for Robj {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        collect_strings(iter, |s| str_to_character(&s))
    }
}

impl<'a> FromIterator<&'a String> for Robj {
    fn from_iter<I: IntoIterator<Item = &'a String>>(iter: I) -> Self {
        collect_strings(iter.into_iter().map(|s| s.as_str()), str_to_character)
    }
}

impl<'a> FromIterator<Option<&'a str>> for Robj {
    fn from_iter<I: IntoIterator<Item = Option<&'a str>>>(iter: I) -> Self {
        collect_strings(
            iter.into_iter().map(|s| {
                s.map(str_to_character)
                    .unwrap_or_else(|| unsafe { R_NaString })
            }),
            |sexp| sexp,
        )
    }
}

impl FromIterator<Option<String>> for Robj {
    fn from_iter<I: IntoIterator<Item = Option<String>>>(iter: I) -> Self {
        collect_strings(
            iter.into_iter().map(|s| {
                s.as_ref()
                    .map(|s| str_to_character(s.as_str()))
                    .unwrap_or_else(|| unsafe { R_NaString })
            }),
            |sexp| sexp,
        )
    }
}

impl FromIterator<Rstr> for Robj {
    fn from_iter<I: IntoIterator<Item = Rstr>>(iter: I) -> Self {
        collect_strings(iter, |s| unsafe { s.get() })
    }
}

impl<'a> FromIterator<&'a Rstr> for Robj {
    fn from_iter<I: IntoIterator<Item = &'a Rstr>>(iter: I) -> Self {
        collect_strings(iter, |s| unsafe { s.get() })
    }
}

impl FromIterator<Option<Rstr>> for Robj {
    fn from_iter<I: IntoIterator<Item = Option<Rstr>>>(iter: I) -> Self {
        collect_strings(
            iter.into_iter().map(|s| {
                s.map(|s| unsafe { s.get() })
                    .unwrap_or_else(|| unsafe { R_NaString })
            }),
            |sexp| sexp,
        )
    }
}

impl FromIterator<bool> for Robj {
    fn from_iter<I: IntoIterator<Item = bool>>(iter: I) -> Self {
        collect_native(iter.into_iter().map(Rbool::from))
    }
}

impl<'a> FromIterator<&'a bool> for Robj {
    fn from_iter<I: IntoIterator<Item = &'a bool>>(iter: I) -> Self {
        collect_native(iter.into_iter().map(|v| Rbool::from(*v)))
    }
}

impl FromIterator<Rbool> for Robj {
    fn from_iter<I: IntoIterator<Item = Rbool>>(iter: I) -> Self {
        collect_native(iter)
    }
}

impl<'a> FromIterator<&'a Rbool> for Robj {
    fn from_iter<I: IntoIterator<Item = &'a Rbool>>(iter: I) -> Self {
        collect_native(iter.into_iter().copied())
    }
}

impl<const N: usize> From<[bool; N]> for Robj {
    fn from(val: [bool; N]) -> Self {
        collect_native(val.into_iter().map(Rbool::from))
    }
}

impl<'a, const N: usize> From<&'a [bool; N]> for Robj {
    fn from(val: &'a [bool; N]) -> Self {
        collect_native(val.iter().copied().map(Rbool::from))
    }
}

impl<'a, const N: usize> From<&'a mut [bool; N]> for Robj {
    fn from(val: &'a mut [bool; N]) -> Self {
        collect_native(val.iter().copied().map(Rbool::from))
    }
}

impl<'a> From<&'a [bool]> for Robj {
    fn from(val: &'a [bool]) -> Self {
        collect_native(val.iter().copied().map(Rbool::from))
    }
}

impl<const N: usize> From<[Option<bool>; N]> for Robj {
    fn from(val: [Option<bool>; N]) -> Self {
        collect_native(val.into_iter().map(Rbool::from))
    }
}

impl<'a, const N: usize> From<&'a [Option<bool>; N]> for Robj {
    fn from(val: &'a [Option<bool>; N]) -> Self {
        collect_native(val.iter().cloned().map(Rbool::from))
    }
}

impl<'a, const N: usize> From<&'a mut [Option<bool>; N]> for Robj {
    fn from(val: &'a mut [Option<bool>; N]) -> Self {
        collect_native(val.iter().cloned().map(Rbool::from))
    }
}

impl<'a> From<&'a [Option<bool>]> for Robj {
    fn from(val: &'a [Option<bool>]) -> Self {
        collect_native(val.iter().cloned().map(Rbool::from))
    }
}

impl FromIterator<Option<bool>> for Robj {
    fn from_iter<I: IntoIterator<Item = Option<bool>>>(iter: I) -> Self {
        collect_native(iter.into_iter().map(Rbool::from))
    }
}

impl FromIterator<Rfloat> for Robj {
    fn from_iter<I: IntoIterator<Item = Rfloat>>(iter: I) -> Self {
        collect_native(iter)
    }
}

impl<'a> FromIterator<&'a Rfloat> for Robj {
    fn from_iter<I: IntoIterator<Item = &'a Rfloat>>(iter: I) -> Self {
        collect_native(iter.into_iter().copied())
    }
}

impl FromIterator<Rint> for Robj {
    fn from_iter<I: IntoIterator<Item = Rint>>(iter: I) -> Self {
        collect_native(iter)
    }
}

impl<'a> FromIterator<&'a Rint> for Robj {
    fn from_iter<I: IntoIterator<Item = &'a Rint>>(iter: I) -> Self {
        collect_native(iter.into_iter().copied())
    }
}

macro_rules! impl_scalar_from_iter {
    ($t:ty) => {
        impl From<$t> for Robj {
            fn from(value: $t) -> Self {
                std::iter::once(value).collect()
            }
        }
    };
}

impl_scalar_from_iter!(f64);
impl_scalar_from_iter!(f32);
impl_scalar_from_iter!(i64);
impl_scalar_from_iter!(u32);
impl_scalar_from_iter!(u64);
impl_scalar_from_iter!(usize);
impl_scalar_from_iter!(i32);
impl_scalar_from_iter!(i16);
impl_scalar_from_iter!(i8);
impl_scalar_from_iter!(u16);
impl_scalar_from_iter!(u8);
impl_scalar_from_iter!(c64);
impl_scalar_from_iter!(Rcplx);
impl_scalar_from_iter!((f64, f64));
impl_scalar_from_iter!(Rfloat);
impl_scalar_from_iter!(Rint);
impl_scalar_from_iter!(Rbool);
impl_scalar_from_iter!(bool);
impl_scalar_from_iter!(String);
impl_scalar_from_iter!(&str);
impl_scalar_from_iter!(Rstr);
impl_scalar_from_iter!(Option<f64>);
impl_scalar_from_iter!(Option<f32>);
impl_scalar_from_iter!(Option<i64>);
impl_scalar_from_iter!(Option<u32>);
impl_scalar_from_iter!(Option<u64>);
impl_scalar_from_iter!(Option<usize>);
impl_scalar_from_iter!(Option<i32>);
impl_scalar_from_iter!(Option<i16>);
impl_scalar_from_iter!(Option<i8>);
impl_scalar_from_iter!(Option<u16>);
impl_scalar_from_iter!(Option<bool>);
impl_scalar_from_iter!(Option<String>);
impl_scalar_from_iter!(Option<&str>);
impl_scalar_from_iter!(Option<Rstr>);

macro_rules! impl_ref_from_scalar {
    ($($t:ty),+ $(,)?) => {
        $(impl<'a> From<&'a $t> for Robj {
            fn from(value: &'a $t) -> Self {
                (*value).into()
            }
        })+
    };
}

impl_ref_from_scalar!(
    f64, f32, i64, u32, u64, usize, i32, i16, i8, u16, u8, bool, c64, Rcplx, Rfloat, Rint, Rbool
);

impl<'a, 'b> From<&'a &'b str> for Robj {
    fn from(value: &'a &'b str) -> Self {
        (*value).into()
    }
}

impl<'a> From<&'a String> for Robj {
    fn from(value: &'a String) -> Self {
        value.as_str().into()
    }
}

impl<'a> From<&'a Rstr> for Robj {
    fn from(value: &'a Rstr) -> Self {
        value.clone().into()
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

impl<T> From<&Option<T>> for Robj
where
    T: Clone,
    Robj: From<Option<T>>,
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
        Self: Sized,
        Robj: FromIterator<Self::Item>,
    {
        Robj::from_iter(self)
    }

    /// Collects an iterable into an [`RArray`].
    /// The iterable must yield items column by column (aka Fortan order)
    ///
    /// # Arguments
    ///
    /// * `dims` - an array containing the length of each dimension
    fn collect_rarray<const LEN: usize>(self, dims: [usize; LEN]) -> Result<RArray<Self::Item, LEN>>
    where
        Self: Sized,
        Robj: FromIterator<Self::Item>,
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

macro_rules! impl_native_slice_conversions {
    ($t:ty) => {
        impl<const N: usize> From<[$t; N]> for Robj {
            fn from(val: [$t; N]) -> Self {
                copy_slice_to_robj(&val)
            }
        }

        impl<'a, const N: usize> From<&'a [$t; N]> for Robj {
            fn from(val: &'a [$t; N]) -> Self {
                copy_slice_to_robj(val)
            }
        }

        impl<'a, const N: usize> From<&'a mut [$t; N]> for Robj {
            fn from(val: &'a mut [$t; N]) -> Self {
                copy_slice_to_robj(val)
            }
        }

        impl<'a> From<&'a [$t]> for Robj {
            fn from(val: &'a [$t]) -> Self {
                copy_slice_to_robj(val)
            }
        }
    };
}

impl_native_slice_conversions!(f64);
impl_native_slice_conversions!(Rfloat);
impl_native_slice_conversions!(Rcomplex);
impl_native_slice_conversions!(Rcplx);
impl_native_slice_conversions!(c64);
impl_native_slice_conversions!((f64, f64));
impl_native_slice_conversions!(i32);
impl_native_slice_conversions!(Rint);
impl_native_slice_conversions!(Rbool);
impl_native_slice_conversions!(u8);

impl<T, const N: usize> From<[T; N]> for Robj
where
    T: StrLike,
{
    fn from(val: [T; N]) -> Self {
        collect_strings(val, |s| s.to_sexp())
    }
}

impl<'a, T, const N: usize> From<&'a [T; N]> for Robj
where
    T: StrLike,
{
    fn from(val: &'a [T; N]) -> Self {
        collect_strings(val.iter().cloned(), |s| s.to_sexp())
    }
}

impl<'a, T, const N: usize> From<&'a mut [T; N]> for Robj
where
    T: StrLike,
{
    fn from(val: &'a mut [T; N]) -> Self {
        collect_strings(val.iter().cloned(), |s| s.to_sexp())
    }
}

impl<T> From<&[T]> for Robj
where
    T: StrLike,
{
    fn from(val: &[T]) -> Self {
        collect_strings(val.iter().cloned(), |s| s.to_sexp())
    }
}

impl<const N: usize> From<[Option<&str>; N]> for Robj {
    fn from(val: [Option<&str>; N]) -> Self {
        collect_strings(
            val.into_iter()
                .map(|s| s.map(str_to_character).unwrap_or(unsafe { R_NaString })),
            |sexp| sexp,
        )
    }
}

impl<'a, const N: usize> From<&'a [Option<&str>; N]> for Robj {
    fn from(val: &'a [Option<&str>; N]) -> Self {
        collect_strings(
            val.iter()
                .cloned()
                .map(|s| s.map(str_to_character).unwrap_or(unsafe { R_NaString })),
            |sexp| sexp,
        )
    }
}

impl<'a, const N: usize> From<&'a mut [Option<&str>; N]> for Robj {
    fn from(val: &'a mut [Option<&str>; N]) -> Self {
        collect_strings(
            val.iter()
                .cloned()
                .map(|s| s.map(str_to_character).unwrap_or(unsafe { R_NaString })),
            |sexp| sexp,
        )
    }
}

impl<'a> From<&'a [Option<&str>]> for Robj {
    fn from(val: &'a [Option<&str>]) -> Self {
        collect_strings(
            val.iter()
                .cloned()
                .map(|s| s.map(str_to_character).unwrap_or(unsafe { R_NaString })),
            |sexp| sexp,
        )
    }
}

impl<const N: usize> From<[Option<String>; N]> for Robj {
    fn from(val: [Option<String>; N]) -> Self {
        collect_strings(
            val.into_iter().map(|s| {
                s.as_ref()
                    .map(|s| str_to_character(s.as_str()))
                    .unwrap_or(unsafe { R_NaString })
            }),
            |sexp| sexp,
        )
    }
}

impl<'a, const N: usize> From<&'a [Option<String>; N]> for Robj {
    fn from(val: &'a [Option<String>; N]) -> Self {
        collect_strings(
            val.iter().cloned().map(|s| {
                s.as_ref()
                    .map(|s| str_to_character(s.as_str()))
                    .unwrap_or(unsafe { R_NaString })
            }),
            |sexp| sexp,
        )
    }
}

impl<'a, const N: usize> From<&'a mut [Option<String>; N]> for Robj {
    fn from(val: &'a mut [Option<String>; N]) -> Self {
        collect_strings(
            val.iter().cloned().map(|s| {
                s.as_ref()
                    .map(|s| str_to_character(s.as_str()))
                    .unwrap_or(unsafe { R_NaString })
            }),
            |sexp| sexp,
        )
    }
}

impl<'a> From<&'a [Option<String>]> for Robj {
    fn from(val: &'a [Option<String>]) -> Self {
        collect_strings(
            val.iter().cloned().map(|s| {
                s.as_ref()
                    .map(|s| str_to_character(s.as_str()))
                    .unwrap_or(unsafe { R_NaString })
            }),
            |sexp| sexp,
        )
    }
}

impl<const N: usize> From<[Option<Rstr>; N]> for Robj {
    fn from(val: [Option<Rstr>; N]) -> Self {
        collect_strings(
            val.into_iter().map(|s| {
                s.map(|s| unsafe { s.get() })
                    .unwrap_or(unsafe { R_NaString })
            }),
            |sexp| sexp,
        )
    }
}

impl<'a, const N: usize> From<&'a [Option<Rstr>; N]> for Robj {
    fn from(val: &'a [Option<Rstr>; N]) -> Self {
        collect_strings(
            val.iter().cloned().map(|s| {
                s.map(|s| unsafe { s.get() })
                    .unwrap_or(unsafe { R_NaString })
            }),
            |sexp| sexp,
        )
    }
}

impl<'a, const N: usize> From<&'a mut [Option<Rstr>; N]> for Robj {
    fn from(val: &'a mut [Option<Rstr>; N]) -> Self {
        collect_strings(
            val.iter().cloned().map(|s| {
                s.map(|s| unsafe { s.get() })
                    .unwrap_or(unsafe { R_NaString })
            }),
            |sexp| sexp,
        )
    }
}

impl<'a> From<&'a [Option<Rstr>]> for Robj {
    fn from(val: &'a [Option<Rstr>]) -> Self {
        collect_strings(
            val.iter().cloned().map(|s| {
                s.map(|s| unsafe { s.get() })
                    .unwrap_or(unsafe { R_NaString })
            }),
            |sexp| sexp,
        )
    }
}

impl<T, const N: usize> From<[Option<T>; N]> for Robj
where
    T: RSliceNative + CanBeNA,
{
    fn from(val: [Option<T>; N]) -> Self {
        coerce_slice_to_robj(&val)
    }
}

impl<'a, T, const N: usize> From<&'a [Option<T>; N]> for Robj
where
    T: RSliceNative + CanBeNA,
{
    fn from(val: &'a [Option<T>; N]) -> Self {
        coerce_slice_to_robj(val)
    }
}

impl<'a, T, const N: usize> From<&'a mut [Option<T>; N]> for Robj
where
    T: RSliceNative + CanBeNA,
{
    fn from(val: &'a mut [Option<T>; N]) -> Self {
        coerce_slice_to_robj(val)
    }
}

impl<'a, T> From<&'a [Option<T>]> for Robj
where
    T: RSliceNative + CanBeNA,
{
    fn from(val: &'a [Option<T>]) -> Self {
        coerce_slice_to_robj(val)
    }
}

impl<T> From<Vec<T>> for Robj
where
    Robj: FromIterator<T>,
{
    fn from(value: Vec<T>) -> Self {
        value.into_iter().collect()
    }
}

impl<T: Clone> From<&Vec<T>> for Robj
where
    Robj: FromIterator<T>,
{
    fn from(value: &Vec<T>) -> Self {
        value.iter().cloned().collect()
    }
}

macro_rules! impl_range_from {
    ($t:ty) => {
        impl From<std::ops::Range<$t>> for Robj {
            fn from(val: std::ops::Range<$t>) -> Self {
                val.collect()
            }
        }

        impl From<std::ops::RangeInclusive<$t>> for Robj {
            fn from(val: std::ops::RangeInclusive<$t>) -> Self {
                val.collect()
            }
        }
    };
}

impl_range_from!(i32);
impl_range_from!(i64);
impl_range_from!(u32);
impl_range_from!(u64);
impl_range_from!(usize);

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
