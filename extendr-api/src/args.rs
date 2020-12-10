//! Argument parsing and checking.
//!

use libR_sys::*;
//use crate::robj::*;
use crate::robj::Robj;

/// Convert a list of tokens to an array of tuples.
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

pub unsafe fn append_with_name(tail: SEXP, obj: Robj, name: &str) -> SEXP {
    let mut name = Vec::from(name.as_bytes());
    name.push(0);
    let cons = Rf_cons(obj.get(), R_NilValue);
    SET_TAG(
        cons,
        Rf_install(name.as_ptr() as *const std::os::raw::c_char),
    );
    SETCDR(tail, cons);
    cons
}

pub unsafe fn append(tail: SEXP, obj: Robj) -> SEXP {
    let cons = Rf_cons(obj.get(), R_NilValue);
    SETCDR(tail, cons);
    cons
}

pub unsafe fn make_lang(sym: &str) -> Robj {
    let mut name = Vec::from(sym.as_bytes());
    name.push(0);
    let sexp = Rf_lang1(Rf_install(name.as_ptr() as *const std::os::raw::c_char));
    Robj::from(sexp)
}

/// Convert a list of tokens to an array of tuples.
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
#[macro_export]
macro_rules! lang {
    ($sym : expr) => {
        unsafe {
            make_lang($sym)
        }
    };
    ($sym : expr, $($rest: tt)*) => {
        unsafe {
            let res = make_lang($sym);
            let mut tail = res.get();
            append_lang!(tail, $($rest)*);
            let _ = tail;
            res
        }
    };
}

#[cfg(test)]
mod tests {
    //use crate::args;
    use super::*;
    use extendr_engine::start_r;

    #[test]
    fn test_args() {
        start_r();
        assert_eq!(Robj::from(1).eval().unwrap(), Robj::from(1));
        //assert_eq!(Robj::from(Lang("ls")), Robj::from(1));
        assert_eq!(lang!("+", 1, 1).eval().unwrap(), Robj::from(2));
        assert_eq!(lang!("+", x = 1, y = 1).eval().unwrap(), Robj::from(2));
        //assert_eq!(Robj::from(Lang("ls")).and(baseenv()).eval().unwrap(), Robj::from(1));
        //let plus = Robj::from(Lang("+"));
        /*assert_eq!(args!(), vec![]);
        assert_eq!(args!(1), vec![("", 1.into())]);
        assert_eq!(args!(a=1), vec![("a", 1.into())]);
        assert_eq!(args!(2, a=1), vec![("", 2.into()), ("a", 1.into())]);
        assert_eq!(args!(1+1), vec![("", Robj::from(2))]);
        assert_eq!(args!(1+1, 2), [("", Robj::from(2)), ("", Robj::from(2))]);
        assert_eq!(args!(a=1+1, b=2), [("a", Robj::from(2)), ("b", Robj::from(2))]);*/
        //end_r();
    }
}
