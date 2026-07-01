//! Argument parsing and checking.
//!

use crate::robj::GetSexp;
use crate::robj::RObj;
use crate::single_threaded;
use extendr_ffi::{R_NilValue, Rf_cons, Rf_lang1, SETCDR, SET_TAG, SEXP};
/// Convert a list of tokens to an array of tuples.
#[doc(hidden)]
#[macro_export]
macro_rules! push_args {
    ($args: expr, $name: ident = $val : expr) => {
        $args.push((stringify!($name), RObj::from($val)));
    };
    ($args: expr, $name: ident = $val : expr, $($rest: tt)*) => {
        $args.push((stringify!($name), RObj::from($val)));
        push_args!($args, $($rest)*);
    };
    ($args: expr, $val : expr) => {
        $args.push(("", RObj::from($val)));
    };
    ($args: expr, $val : expr, $($rest: tt)*) => {
        $args.push(("", RObj::from($val)));
        push_args!($args, $($rest)*);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! args {
    () => {
        Vec::<(&str, RObj)>::new()
    };
    ($($rest: tt)*) => {
        {
            let mut args = Vec::<(&str, RObj)>::new();
            push_args!(args, $($rest)*);
            args
        }
    };
}

#[doc(hidden)]
pub unsafe fn append_with_name(tail: SEXP, obj: RObj, name: &str) -> SEXP {
    single_threaded(|| {
        let cons = Rf_cons(obj.get(), R_NilValue);
        SET_TAG(cons, crate::make_symbol(name));
        SETCDR(tail, cons);
        cons
    })
}

#[doc(hidden)]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn append(tail: SEXP, obj: RObj) -> SEXP {
    single_threaded(|| unsafe {
        let cons = Rf_cons(obj.get(), R_NilValue);
        SETCDR(tail, cons);
        cons
    })
}

#[doc(hidden)]
pub fn make_lang(sym: &str) -> RObj {
    unsafe { RObj::from_sexp(single_threaded(|| Rf_lang1(crate::make_symbol(sym)))) }
}

/// Convert a list of tokens to an array of tuples.
#[doc(hidden)]
#[macro_export]
macro_rules! append_lang {
    ($tail: ident, $name: ident = $val : expr) => {
        $tail = append_with_name($tail, RObj::from($val), stringify!($name));
    };
    ($tail: ident, $name: ident = $val : expr, $($rest: tt)*) => {
        $tail = append_with_name($tail, RObj::from($val), stringify!($name));
        append_lang!($tail, $($rest)*);
    };
    ($tail: ident, $val : expr) => {
        $tail = append($tail, RObj::from($val));
    };
    ($tail: ident, $val : expr, $($rest: tt)*) => {
        $tail = append($tail, RObj::from($val));
        append_lang!($tail, $($rest)*);
    };
}

/// A macro for constructing R language objects.
///
/// Example:
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let call_to_c = lang!("c", 1., 2., 3.);
///     let vec = call_to_c.eval().unwrap();
///     assert_eq!(vec, r!([1., 2., 3.]));
///
///     let list = lang!("list", a=1, b=2).eval().unwrap();
///     assert_eq!(list.len(), 2);
/// }
/// ```
#[macro_export]
macro_rules! lang {
    ($sym : expr) => {
        #[allow(unused_unsafe)]
        unsafe {
            make_lang($sym)
        }
    };
    ($sym : expr, $($rest: tt)*) => {
        unsafe {
            use $crate::robj::GetSexp;
            let res = make_lang($sym);
            let mut tail = res.get();
            append_lang!(tail, $($rest)*);
            let _ = tail;
            res
        }
    };
}
