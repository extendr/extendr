//! Argument parsing and checking.

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

#[cfg(test)]
mod tests {
    use crate::args;
    use crate::robj::Robj;
    use crate::{start_r, end_r};

    #[test]
    fn test_args() {
        start_r();
        assert_eq!(args!(), vec![]);
        assert_eq!(args!(1), vec![("", 1.into())]);
        assert_eq!(args!(a=1), vec![("a", 1.into())]);
        assert_eq!(args!(2, a=1), vec![("", 2.into()), ("a", 1.into())]);
        assert_eq!(args!(1+1), vec![("", Robj::from(2))]);
        assert_eq!(args!(1+1, 2), [("", Robj::from(2)), ("", Robj::from(2))]);
        assert_eq!(args!(a=1+1, b=2), [("a", Robj::from(2)), ("b", Robj::from(2))]);
        end_r();
    }
}
