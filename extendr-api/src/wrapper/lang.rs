use super::*;

/// Wrapper for creating language objects.
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let call_to_xyz = r!(Lang(&[r!(Symbol("xyz")), r!(1), r!(2)]));
///     assert_eq!(call_to_xyz.is_language(), true);
///     assert_eq!(call_to_xyz.len(), 3);
/// }
/// ```
///
/// Note: You can use the [lang!] macro for this.
#[derive(Debug, PartialEq, Clone)]
pub struct Lang<T>(pub T);

impl<T> From<Lang<T>> for Robj
where
    T: IntoIterator,
    T::IntoIter: DoubleEndedIterator,
    T::Item: Into<Robj>,
{
    /// Convert a wrapper to an R language object.
    fn from(val: Lang<T>) -> Self {
        single_threaded(|| unsafe {
            let mut res = R_NilValue;
            let mut num_protected = 0;
            for val in val.0.into_iter().rev() {
                let val = Rf_protect(val.into().get());
                res = Rf_protect(Rf_lcons(val, res));
                num_protected += 2;
            }
            let res = new_owned(res);
            Rf_unprotect(num_protected);
            res
        })
    }
}
