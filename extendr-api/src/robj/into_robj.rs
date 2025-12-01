use super::*;
use crate::single_threaded;
use extendr_ffi::{
    cetype_t, R_BlankString, R_NaInt, R_NaReal, R_NaString, R_NilValue, Rcomplex, Rf_mkCharLenCE,
    INTEGER, LOGICAL, RAW, REAL, SET_STRING_ELT, SEXPTYPE,
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

/// This is an extension trait to provide a convenience method `into_robj()`.
///
/// Defer to `From<T> for Robj`-impls if you have custom types.
///
pub trait IntoRobj {
    fn into_robj(self) -> Robj;
}

impl<T: Into<Robj>> IntoRobj for T {
    fn into_robj(self) -> Robj {
        self.into_robj()
    }
}

/// Marker trait for R's native vector element types.
///
/// This trait is implemented directly on the primitive types that R vectors store:
/// - `f64` for `REALSXP` (real/numeric vectors)
/// - `i32` for `INTSXP` (integer vectors)
/// - `Rbool` for `LGLSXP` (logical vectors)
/// - `u8` for `RAWSXP` (raw vectors)
/// - `Rcomplex` for `CPLXSXP` (complex vectors)
///
/// Note: Strings (`STRSXP`) are handled separately since they store `SEXP` elements.
pub trait RNativeType: Sized {
    const SEXPTYPE: SEXPTYPE;
    const TYPE_ERROR: &'static str = concat!("expected ", stringify!(Self::SEXPTYPE), "; ");
    const PTR_ACCESS: unsafe extern "C" fn(SEXP) -> *mut Self;
    const LEN: usize;
}

fn alloc_vector<T: RNativeType>() -> Robj {
    Robj::alloc_vector(T::SEXPTYPE, T::LEN)
}

unsafe fn as_slice_unchecked<'a, T: RNativeType>(value: SEXP) -> &'a [T] {
    let length = (unsafe { Rf_xlength(value) }) as _;
    if length == 0 || std::ptr::addr_eq(value, unsafe { R_NilValue }) || value.is_null() {
        return unsafe { std::ptr::NonNull::<[T; 0]>::dangling().as_ref() };
    }
    unsafe {
        std::ptr::NonNull::slice_from_raw_parts(
            std::ptr::NonNull::new_unchecked(T::PTR_ACCESS(value)),
            length,
        )
        .as_ref()
    }
}

unsafe fn as_slice_mut_unchecked<'a, T: RNativeType>(value: SEXP) -> &'a mut [T] {
    let length: usize = (unsafe { Rf_xlength(value) }) as _;
    if length == 0 || std::ptr::addr_eq(value, unsafe { R_NilValue }) || value.is_null() {
        return unsafe { std::ptr::NonNull::<[T; 0]>::dangling().as_mut() };
    }
    unsafe {
        std::ptr::NonNull::slice_from_raw_parts(
            std::ptr::NonNull::new_unchecked(T::PTR_ACCESS(value)),
            length,
        )
        .as_mut()
    }
}

impl RNativeType for f64 {
    const SEXPTYPE: SEXPTYPE = SEXPTYPE::REALSXP;
    const PTR_ACCESS: unsafe extern "C" fn(SEXP) -> *mut Self = REAL;
    const LEN: usize = 1;
}

impl RNativeType for Rcomplex {
    const SEXPTYPE: SEXPTYPE = SEXPTYPE::CPLXSXP;
    const PTR_ACCESS: unsafe extern "C" fn(SEXP) -> *mut Self = COMPLEX;
    const LEN: usize = 1;
}

impl RNativeType for i32 {
    const SEXPTYPE: SEXPTYPE = SEXPTYPE::INTSXP;
    const PTR_ACCESS: unsafe extern "C" fn(SEXP) -> *mut Self = INTEGER;
    const LEN: usize = 1;
}

impl RNativeType for u8 {
    const SEXPTYPE: SEXPTYPE = SEXPTYPE::RAWSXP;
    const PTR_ACCESS: unsafe extern "C" fn(SEXP) -> *mut Self = RAW;
    const LEN: usize = 1;
}

// region: synonyms

#[allow(non_snake_case)]
extern "C" fn LOGICAL_AS_RBOOL(sexp: SEXP) -> *mut Rbool {
    unsafe { LOGICAL(sexp) }.cast()
}

impl RNativeType for Rbool {
    const SEXPTYPE: SEXPTYPE = SEXPTYPE::LGLSXP;
    const PTR_ACCESS: unsafe extern "C" fn(SEXP) -> *mut Self = LOGICAL_AS_RBOOL;
    const LEN: usize = 1;
}

#[allow(non_snake_case)]
extern "C" fn REAL_AS_RFLOAT(sexp: SEXP) -> *mut Rfloat {
    unsafe { REAL(sexp) }.cast()
}

impl RNativeType for Rfloat {
    const SEXPTYPE: SEXPTYPE = SEXPTYPE::REALSXP;
    const PTR_ACCESS: unsafe extern "C" fn(SEXP) -> *mut Self = REAL_AS_RFLOAT;
    const LEN: usize = 1;
}

#[allow(non_snake_case)]
extern "C" fn COMPLEX_AS_RCPLX(sexp: SEXP) -> *mut Rcplx {
    unsafe { COMPLEX(sexp) }.cast()
}

impl RNativeType for Rcplx {
    const SEXPTYPE: SEXPTYPE = SEXPTYPE::CPLXSXP;
    const PTR_ACCESS: unsafe extern "C" fn(SEXP) -> *mut Self = COMPLEX_AS_RCPLX;
    const LEN: usize = 1;
}

#[allow(non_snake_case)]
extern "C" fn COMPLEX_AS_C64(sexp: SEXP) -> *mut c64 {
    unsafe { COMPLEX(sexp) }.cast()
}

impl RNativeType for c64 {
    const SEXPTYPE: SEXPTYPE = SEXPTYPE::CPLXSXP;
    const PTR_ACCESS: unsafe extern "C" fn(SEXP) -> *mut Self = COMPLEX_AS_C64;
    const LEN: usize = 1;
}

#[allow(non_snake_case)]
extern "C" fn COMPLEX_AS_FLOAT_TUPLE(sexp: SEXP) -> *mut (f64, f64) {
    unsafe { COMPLEX(sexp) }.cast()
}

impl RNativeType for (f64, f64) {
    const SEXPTYPE: SEXPTYPE = SEXPTYPE::CPLXSXP;
    const PTR_ACCESS: unsafe extern "C" fn(SEXP) -> *mut Self = COMPLEX_AS_FLOAT_TUPLE;
    const LEN: usize = 1;
}

#[allow(non_snake_case)]
extern "C" fn INTEGER_AS_RINT(sexp: SEXP) -> *mut Rint {
    unsafe { INTEGER(sexp) }.cast()
}

impl RNativeType for Rint {
    const SEXPTYPE: SEXPTYPE = SEXPTYPE::INTSXP;
    const TYPE_ERROR: &str = "expected `INTSXP`";
    const PTR_ACCESS: unsafe extern "C" fn(SEXP) -> *mut Self = INTEGER_AS_RINT;
    const LEN: usize = 1;
}

// endregion

// CONFLICT
// impl<T> IntoRobj for T {
//     fn into_robj(self) -> Robj {
//         let mut result_robj = alloc_vector::<T>();
//         *unsafe {
//             as_slice_mut_unchecked::<T>(result_robj.get_mut())
//                 .first_mut()
//                 .unwrap()
//         } = self;
//         result_robj
//     }
// }

// CONFLICT
// impl<'a, T: Clone> IntoRobj for &'a T {
//     fn into_robj(self) -> Robj {
//         let mut result_robj = alloc_vector::<T>();
//         *unsafe {
//             as_slice_mut_unchecked::<T>(result_robj.get_mut())
//                 .first_mut()
//                 .unwrap()
//         } = self.clone();
//         result_robj
//     }
// }

impl IntoRobj for &i32 {
    fn into_robj(self) -> Robj {
        self.into()
    }
}

impl IntoRobj for &f64 {
    fn into_robj(self) -> Robj {
        self.into()
    }
}

impl IntoRobj for &u8 {
    fn into_robj(self) -> Robj {
        self.into()
    }
}

impl IntoRobj for &Rint {
    fn into_robj(self) -> Robj {
        self.0.into()
    }
}

impl IntoRobj for &Rfloat {
    fn into_robj(self) -> Robj {
        self.0.into()
    }
}

// impl IntoRobj for &Rbool {
//     fn into_robj(self) -> Robj {
//         self.0.into()
//     }
// }

impl IntoRobj for &Rcomplex {
    fn into_robj(self) -> Robj {
        self.into()
    }
}

impl IntoRobj for &Rcplx {
    fn into_robj(self) -> Robj {
        self.0.into()
    }
}

impl IntoRobj for &c64 {
    fn into_robj(self) -> Robj {
        self.into()
    }
}

impl IntoRobj for &(f64, f64) {
    fn into_robj(self) -> Robj {
        self.into()
    }
}

// impl<'a, T: RNativeType + Clone> IntoRobj for &'a mut T {
//     fn into_robj(self) -> Robj {
//         let mut result_robj = alloc_vector::<T>();
//         *unsafe {
//             as_slice_mut_unchecked::<T>(result_robj.get_mut())
//                 .first_mut()
//                 .unwrap()
//         } = self.clone();
//         result_robj
//     }
// }

impl<'a, T: RNativeType + Clone> IntoRobj for &'a [T] {
    fn into_robj(self) -> Robj {
        let mut result_robj = alloc_vector::<T>();
        *unsafe { as_slice_mut_unchecked::<T>(result_robj.get_mut()) }.clone_from_slice(self);
        result_robj
    }
}

impl<'a, T: RNativeType + Clone> IntoRobj for &'a mut [T] {
    fn into_robj(self) -> Robj {
        let mut result_robj = alloc_vector::<T>();
        *unsafe { as_slice_mut_unchecked::<T>(result_robj.get_mut()) }.clone_from_slice(self);
        result_robj
    }
}

impl<T: RNativeType + Clone, const N: usize> IntoRobj for [T; N] {
    fn into_robj(self) -> Robj {
        self.as_slice().into_robj()
    }
}

impl<T: RNativeType + Clone, const N: usize> IntoRobj for &[T; N] {
    fn into_robj(self) -> Robj {
        self.as_slice().into_robj()
    }
}

impl<T: RNativeType + Clone, const N: usize> IntoRobj for &mut [T; N] {
    fn into_robj(self) -> Robj {
        self.as_slice().into_robj()
    }
}

// region: Option<_>

impl<'a, T: RNativeType + Clone> IntoRobj for Option<&'a mut T> {
    fn into_robj(self) -> Robj {
        if let Some(value) = self {
            value.into()
        } else {
            ().into()
        }
    }
}

impl<'a, T: RNativeType + Clone> IntoRobj for Option<&'a [T]> {
    fn into_robj(self) -> Robj {
        if let Some(value) = self {
            value.into()
        } else {
            ().into()
        }
    }
}

impl<'a, T: RNativeType + Clone> IntoRobj for Option<&'a mut [T]> {
    fn into_robj(self) -> Robj {
        if let Some(value) = self {
            value.into()
        } else {
            ().into()
        }
    }
}

impl<T: RNativeType + Clone, const N: usize> IntoRobj for Option<[T; N]> {
    fn into_robj(self) -> Robj {
        if let Some(value) = self {
            value.into()
        } else {
            ().into()
        }
    }
}

impl<T: RNativeType + Clone, const N: usize> IntoRobj for Option<&[T; N]> {
    fn into_robj(self) -> Robj {
        if let Some(value) = self {
            value.into()
        } else {
            ().into()
        }
    }
}

impl<T: RNativeType + Clone, const N: usize> IntoRobj for Option<&mut [T; N]> {
    fn into_robj(self) -> Robj {
        if let Some(value) = self {
            value.into()
        } else {
            ().into()
        }
    }
}

impl<'a, T: RNativeType + Clone> IntoRobj for &'a mut Option<T> {
    fn into_robj(self) -> Robj {
        if let Some(value) = self {
            value.into()
        } else {
            ().into()
        }
    }
}

impl<'a, T: CanBeNA + RNativeType + Clone> IntoRobj for &'a [Option<T>] {
    fn into_robj(self) -> Robj {
        let values: Vec<_> = self
            .into_iter()
            .map(|x| if let Some(value) = x { value } else { &T::na() })
            .collect();
        let mut result = alloc_vector::<T>();
        unsafe { as_slice_mut_unchecked(result.get_mut()).clone_from_slice(values.as_slice()) };
        result
    }
}

impl<'a, T: CanBeNA + RNativeType + Clone> IntoRobj for &'a mut [Option<T>] {
    fn into_robj(self) -> Robj {
        let values: Vec<_> = self
            .into_iter()
            .map(|x| if let Some(value) = x { value } else { &T::na() })
            .collect();
        let mut result = alloc_vector::<T>();
        unsafe { as_slice_mut_unchecked(result.get_mut()).clone_from_slice(values.as_slice()) };
        result
    }
}

impl<T: RNativeType + Clone, const N: usize> IntoRobj for [Option<T>; N] {
    fn into_robj(self) -> Robj {
        self.as_slice().into()
    }
}

impl<T: RNativeType + Clone, const N: usize> IntoRobj for &[Option<T>; N] {
    fn into_robj(self) -> Robj {
        self.into()
    }
}

impl<T: RNativeType + Clone, const N: usize> IntoRobj for &mut [Option<T>; N] {
    fn into_robj(self) -> Robj {
        self.into()
    }
}

impl IntoRobj for Option<f64> {
    fn into_robj(self) -> Robj {
        if let Some(value) = self {
            value.into()
        } else {
            ().into()
        }
    }
}

impl IntoRobj for Option<i32> {
    fn into_robj(self) -> Robj {
        if let Some(value) = self {
            value.into()
        } else {
            ().into()
        }
    }
}

impl IntoRobj for Option<u8> {
    fn into_robj(self) -> Robj {
        if let Some(value) = self {
            value.into()
        } else {
            ().into()
        }
    }
}

impl IntoRobj for Option<Rbool> {
    fn into_robj(self) -> Robj {
        if let Some(value) = self {
            value.into()
        } else {
            ().into()
        }
    }
}

impl IntoRobj for Option<Rcomplex> {
    fn into_robj(self) -> Robj {
        if let Some(value) = self {
            value.into()
        } else {
            ().into()
        }
    }
}

impl IntoRobj for Option<Rcplx> {
    fn into_robj(self) -> Robj {
        if let Some(value) = self {
            value.into()
        } else {
            ().into()
        }
    }
}

impl IntoRobj for Option<c64> {
    fn into_robj(self) -> Robj {
        if let Some(value) = self {
            value.into()
        } else {
            ().into()
        }
    }
}

impl IntoRobj for Option<(f64, f64)> {
    fn into_robj(self) -> Robj {
        if let Some(value) = self {
            value.into()
        } else {
            ().into()
        }
    }
}

impl IntoRobj for Option<&f64> {
    fn into_robj(self) -> Robj {
        if let Some(value) = self {
            value.into()
        } else {
            ().into()
        }
    }
}

impl IntoRobj for Option<&i32> {
    fn into_robj(self) -> Robj {
        if let Some(value) = self {
            value.into()
        } else {
            ().into()
        }
    }
}

impl IntoRobj for Option<&u8> {
    fn into_robj(self) -> Robj {
        if let Some(value) = self {
            value.into()
        } else {
            ().into()
        }
    }
}

impl IntoRobj for Option<&Rbool> {
    fn into_robj(self) -> Robj {
        if let Some(value) = self {
            value.into()
        } else {
            ().into()
        }
    }
}

impl IntoRobj for Option<&Rcomplex> {
    fn into_robj(self) -> Robj {
        if let Some(value) = self {
            value.into()
        } else {
            ().into()
        }
    }
}

impl IntoRobj for Option<&Rcplx> {
    fn into_robj(self) -> Robj {
        if let Some(value) = self {
            value.into()
        } else {
            ().into()
        }
    }
}

impl IntoRobj for Option<&c64> {
    fn into_robj(self) -> Robj {
        if let Some(value) = self {
            value.into()
        } else {
            ().into()
        }
    }
}

impl IntoRobj for Option<&(f64, f64)> {
    fn into_robj(self) -> Robj {
        if let Some(value) = self {
            value.into()
        } else {
            ().into()
        }
    }
}

// endregion

pub trait IntoRInteger: num_traits::AsPrimitive<i32> {}

impl IntoRInteger for i8 {}
impl IntoRInteger for i16 {}
impl IntoRInteger for u16 {}
impl IntoRInteger for bool {}

impl<T: IntoRInteger> From<T> for Robj {
    fn from(value: T) -> Robj {
        value.as_().into()
    }
}

impl From<&bool> for Robj {
    fn from(value: &bool) -> Self {
        value.clone().into()
    }
}

impl From<Option<bool>> for Robj {
    fn from(value: Option<bool>) -> Self {
        if let Some(value) = value {
            value.into()
        } else {
            ().into()
        }
    }
}

impl From<&Rbool> for Robj {
    fn from(value: &Rbool) -> Self {
        value.0.into()
    }
}

// impl ToRNative for &str {
//     type Native = RString;
//     #[inline]
//     fn to_r_native(self) -> RString {
//         RString(str_to_character(self))
//     }
// }

// impl ToRNative for &&str {
//     type Native = RString;
//     #[inline]
//     fn to_r_native(self) -> RString {
//         RString(str_to_character(self))
//     }
// }

// impl ToRNative for Option<&str> {
//     type Native = RString;
//     #[inline]
//     fn to_r_native(self) -> RString {
//         RString(match self {
//             Some(s) => str_to_character(s),
//             None => unsafe { R_NaString },
//         })
//     }
// }

// impl ToRNative for String {
//     type Native = RString;
//     #[inline]
//     fn to_r_native(self) -> RString {
//         RString(str_to_character(self.as_str()))
//     }
// }

// impl ToRNative for &String {
//     type Native = RString;
//     #[inline]
//     fn to_r_native(self) -> RString {
//         RString(str_to_character(self.as_str()))
//     }
// }

// impl ToRNative for Option<String> {
//     type Native = RString;
//     #[inline]
//     fn to_r_native(self) -> RString {
//         RString(match self {
//             Some(s) => str_to_character(s.as_str()),
//             None => unsafe { R_NaString },
//         })
//     }
// }

// impl ToRNative for Rstr {
//     type Native = RString;
//     #[inline]
//     fn to_r_native(self) -> RString {
//         RString(unsafe { self.get() })
//     }
// }

// impl ToRNative for &Rstr {
//     type Native = RString;
//     #[inline]
//     fn to_r_native(self) -> RString {
//         RString(unsafe { self.get() })
//     }
// }

// impl ToRNative for Option<Rstr> {
//     type Native = RString;
//     #[inline]
//     fn to_r_native(self) -> RString {
//         RString(match self {
//             Some(s) => unsafe { s.get() },
//             None => unsafe { R_NaString },
//         })
//     }
// }

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
    Option<T>: RNativeType + Clone,
{
    fn from(value: &Option<T>) -> Self {
        value.into()
    }
}
impl<T: Clone> From<&Vec<T>> for Robj {
    fn from(value: &Vec<T>) -> Self {
        value.as_slice().into()
    }
}

impl<T> From<Vec<T>> for Robj {
    fn from(value: Vec<T>) -> Self {
        value.as_slice().into()
    }
}

// TODO:
// impl_from_as_iterator! {Range<T>}
// impl_from_as_iterator! {RangeInclusive<T>}

// impl From<Vec<Robj>> for Robj {
//     /// Convert a vector of Robj into a list.
//     fn from(val: Vec<Robj>) -> Self {
//         Self::from(&val)
//     }
// }

// impl From<&Vec<Robj>> for Robj {
//     fn from(val: &Vec<Robj>) -> Self {
//         |
// }

#[cfg(test)]
mod test;
