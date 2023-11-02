//! Argument parsing and checking.
//!

use libR_sys::*;
//use crate::robj::*;
use crate::robj::GetSexp;
use crate::robj::Robj;
use crate::single_threaded;

/// Convert a list of tokens to an array of tuples.
#[doc(hidden)]
#[macro_export]
macro_rules! push_args {
    ($args: expr, $name: ident = $val : expr) => {
        $args.push((stringify!($name), Robj::from($val)));
    };
    ($args: expr, $name: ident = $val : expr, $($rest: tt)*) => {
        $args.push((stringify!($name), Robj::from($val)));
        push_args!($args, $($rest)*);
    };
    ($args: expr, $val : expr) => {
        $args.push(("", Robj::from($val)));
    };
    ($args: expr, $val : expr, $($rest: tt)*) => {
        $args.push(("", Robj::from($val)));
        push_args!($args, $($rest)*);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! args {
    () => {
        Vec::<(&str, Robj)>::new()
    };
    ($($rest: tt)*) => {
        {
            let mut args = Vec::<(&str, Robj)>::new();
            push_args!(args, $($rest)*);
            args
        }
    };
}

#[doc(hidden)]
pub unsafe fn append_with_name(tail: SEXP, obj: Robj, name: &str) -> SEXP {
    single_threaded(|| {
        let cons = Rf_cons(obj.get(), R_NilValue);
        SET_TAG(cons, crate::make_symbol(name));
        SETCDR(tail, cons);
        cons
    })
}

#[doc(hidden)]
pub unsafe fn append(tail: SEXP, obj: Robj) -> SEXP {
    single_threaded(|| {
        let cons = Rf_cons(obj.get(), R_NilValue);
        SETCDR(tail, cons);
        cons
    })
}

#[doc(hidden)]
pub unsafe fn make_lang(sym: &str) -> Robj {
    Robj::from_sexp(single_threaded(|| Rf_lang1(crate::make_symbol(sym))))
}

/// Convert a list of tokens to an array of tuples.
#[doc(hidden)]
#[macro_export]
macro_rules! append_lang {
    ($tail: ident, $name: ident = $val : expr) => {
        $tail = append_with_name($tail, Robj::from($val), stringify!($name));
    };
    ($tail: ident, $name: ident = $val : expr, $($rest: tt)*) => {
        $tail = append_with_name($tail, Robj::from($val), stringify!($name));
        append_lang!($tail, $($rest)*);
    };
    ($tail: ident, $val : expr) => {
        $tail = append($tail, Robj::from($val));
    };
    ($tail: ident, $val : expr, $($rest: tt)*) => {
        $tail = append($tail, Robj::from($val));
        append_lang!($tail, $($rest)*);
    };
}

/// A macro for constructing R langage objects.
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
        unsafe {
            make_lang($sym)
        }
    };
    ($sym : expr, $($rest: tt)*) => {
        unsafe {
            use extendr_api::robj::GetSexp;
            let res = make_lang($sym);
            let mut tail = res.get();
            append_lang!(tail, $($rest)*);
            let _ = tail;
            res
        }
    };
}
