//!
//! rmacros - a set of macros to call actual R functions in a rusty way.
//! 


/// Concatenation operator.
/// 
/// Example:
/// ```
/// use rapi::*;
/// start_r();
/// let fred = c!(1, 2, 3).unwrap();
/// assert_eq!(fred, Robj::from(&[1, 2, 3][..]));
/// ```
#[macro_export]
macro_rules! c {
    () => {
        lang!("c").eval()
    };
    ($($rest: tt)*) => {
        lang!("c", $($rest)*).eval()
    };
}

/// Read a CSV file.
/// 
/// Example:
/// ```no-run
/// use rapi::*;
/// start_r();
/// let mydata = read_table!("mydata.csv").unwrap();
/// assert_eq!(mydata, NULL);
/// ```
#[macro_export]
macro_rules! read_table {
    ($($rest: tt)*) => {
        lang!("read.table", $($rest)*).eval()
    };
}

/// Create a list.
/// 
/// Example:
/// ```
/// use rapi::*;
/// start_r();
/// let mylist = list!(x=1, y=2).unwrap();
/// assert_eq!(mylist, List(&[1.into(), 2.into()]));
/// ```
#[macro_export]
macro_rules! list {
    () => {
        lang!("list").eval()
    };
    ($($rest: tt)*) => {
        lang!("list", $($rest)*).eval()
    };
}

/// Create a dataframe.
/// 
/// Example:
/// ```
/// use rapi::*;
/// start_r();
/// let mydata = data_frame!(x=1, y=2).unwrap();
/// assert_eq!(mydata, List(&[1.into(), 2.into()]));
/// ```
#[macro_export]
macro_rules! data_frame {
    () => {
        lang!("data.frame").eval()
    };
    ($($rest: tt)*) => {
        lang!("data.frame", $($rest)*).eval()
    };
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_macros() {
        start_r();
        assert_eq!(c!(1, 2, 3).unwrap(), Robj::from(&[1, 2, 3][..]));
    }
}
