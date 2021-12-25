use super::*;

/// Wrapper for creating language objects.
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let call_to_xyz = r!(Language::from_values(&[sym!(xyz), r!(1), r!(2)]));
///     assert_eq!(call_to_xyz.is_language(), true);
///     assert_eq!(call_to_xyz.len(), 3);
/// }
/// ```
///
/// Note: You can use the [lang!] macro for this.
#[derive(PartialEq, Clone)]
pub struct Language {
    pub(crate) robj: Robj,
}

impl Language {
    pub fn from_values<T>(values: T) -> Self
    where
        T: IntoIterator,
        T::IntoIter: DoubleEndedIterator,
        T::Item: Into<Robj>,
    {
        single_threaded(|| unsafe {
            let mut res = R_NilValue;
            let mut num_protected = 0;
            for val in values.into_iter().rev() {
                let val = Rf_protect(val.into().get());
                res = Rf_protect(Rf_lcons(val, res));
                num_protected += 2;
            }
            let robj = Robj::from_sexp(res);
            Rf_unprotect(num_protected);
            Language { robj }
        })
    }

    pub fn iter(&self) -> PairlistIter {
        unsafe {
            PairlistIter {
                robj: self.robj.clone(),
                list_elem: self.robj.get(),
            }
        }
    }

    pub fn names(&self) -> impl Iterator<Item = &'static str> {
        self.iter().map(|(tag, _)| tag)
    }

    pub fn values(&self) -> impl Iterator<Item = Robj> {
        self.iter().map(|(_, robj)| robj)
    }
}

impl std::fmt::Debug for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "lang!({})",
            self.iter()
                .map(|(k, v)| if k.is_empty() {
                    format!("{:?}", v)
                } else {
                    format!("{}={:?}", k, v)
                })
                .collect::<Vec<_>>()
                .join(", ")
        )?;
        Ok(())
    }
}
