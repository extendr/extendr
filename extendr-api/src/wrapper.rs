use crate::robj::*;
use libR_sys::*;
use std::ffi::CString;
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut};

/// Wrapper for creating symbols.
#[derive(Debug, PartialEq)]
pub struct Symbol<'a>(pub &'a str);

/// Wrapper for creating logical vectors.
#[derive(Debug, PartialEq)]
pub struct Logical<'a>(pub &'a [i32]);

/// Wrapper for creating character objects.
#[derive(Debug, PartialEq)]
pub struct Character<'a>(pub &'a str);

/// Wrapper for creating language objects.
#[derive(Debug, PartialEq)]
pub struct Lang<'a>(pub &'a str);

/// Wrapper for creating list objects.
#[derive(Debug, PartialEq)]
pub struct List<'a>(pub &'a [Robj]);

pub struct VectorBase<T> {
    robj: Robj,
    ptr: *mut T,
    length: usize,
}
struct NumericVector {
    inner: VectorBase<f64>,
}

impl std::fmt::Debug for NumericVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NumericVector({:?})", self.inner.robj)
    }
}

impl<'a> PartialEq<List<'a>> for Robj {
    fn eq(&self, rhs: &List) -> bool {
        match self.sexptype() {
            VECSXP if self.len() == rhs.0.len() => {
                for (l, r) in self.list_iter().unwrap().zip(rhs.0.iter()) {
                    if !l.eq(r) {
                        return false;
                    }
                }
                true
            }
            _ => false,
        }
    }
}

/// Make a list object from an array of Robjs.
impl<'a> From<List<'a>> for Robj {
    fn from(val: List<'a>) -> Self {
        unsafe {
            let sexp = Rf_allocVector(VECSXP, val.0.len() as R_xlen_t);
            R_PreserveObject(sexp);
            for i in 0..val.0.len() {
                SET_VECTOR_ELT(sexp, i as R_xlen_t, val.0[i].get());
            }
            Robj::Owned(sexp)
        }
    }
}

/// Convert an integer slice to a logical object.
impl<'a> From<Logical<'a>> for Robj {
    fn from(vals: Logical) -> Self {
        unsafe {
            let len = vals.0.len();
            let sexp = Rf_allocVector(LGLSXP, len as R_xlen_t);
            R_PreserveObject(sexp);
            let ptr = LOGICAL(sexp);
            let slice = std::slice::from_raw_parts_mut(ptr, len);
            for (i, &v) in vals.0.iter().enumerate() {
                slice[i] = v;
            }
            Robj::Owned(sexp)
        }
    }
}

/// Convert a string to a symbol.
impl<'a> From<Symbol<'a>> for Robj {
    fn from(name: Symbol) -> Self {
        unsafe {
            if let Ok(name) = CString::new(name.0) {
                new_owned(Rf_install(name.as_ptr()))
            } else {
                Robj::from(())
            }
        }
    }
}

impl<T> VectorBase<T> {
    // Convert a vector to a slice.
    fn deref(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.ptr as *const T, self.length) }
    }

    // Convert a vector to a slice.
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr as *mut T, self.length) }
    }

    // Get a reference to the inner Robj.
    fn robj(&self) -> &Robj {
        &self.robj
    }

    // Get a reference to the inner Robj.
    fn robj_mut(&mut self) -> &mut Robj {
        &mut self.robj
    }
}

impl NumericVector {
    /// Create a new uninitialised R object for the vector.
    pub unsafe fn allocate(length: usize) -> Self {
        let robj = Robj::allocVector(REALSXP, length);
        let slice = robj.as_f64_slice_mut().unwrap();
        let ptr = slice.as_mut_ptr();
        Self {
            inner: VectorBase::<f64> { robj, ptr, length },
        }
    }

    /// Create a new R object for the vector.
    pub fn zeros(length: usize) -> Self {
        unsafe {
            let mut res = Self::allocate(length);
            res
                .iter_mut()
                .for_each(|d| *d = 0.);
            res
        }
    }

    /// Read only access to the underlying R object.
    pub fn robj(&self) -> &Robj {
        self.inner.robj()
    }

    /// Read-write access to the underlying R object.
    pub fn robj_mut(&mut self) -> &mut Robj {
        self.inner.robj_mut()
    }
}

impl Default for NumericVector {
    fn default() -> Self {
        Self::zeros(1)
    }
}

/// Make NumericVector behave like Vec (ie. deref to a slice).
/// Because we have already unpacked the pointer and length, this
/// should be a nop in many cases.
impl Deref for NumericVector {
    type Target = [f64];

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl DerefMut for NumericVector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.deref_mut()
    }
}

impl From<Vec<f64>> for NumericVector {
    fn from(v: Vec<f64>) -> Self {
        v.into_iter().collect::<NumericVector>()
    }
}

impl From<&[f64]> for NumericVector {
    fn from(v: &[f64]) -> Self {
        v.into_iter().cloned().collect::<NumericVector>()
    }
}

/// Convert incoming parameters.
impl<'a> FromRobj<'a> for NumericVector {
    fn from_robj(robj: &'a Robj) -> Result<Self, &'static str> {
        if let Some(slice) = robj.as_f64_slice_mut() {
            let robj = unsafe { new_owned(robj.get()) };
            let ptr = slice.as_mut_ptr();
            let length = slice.len();
            Ok(Self {
                inner: VectorBase::<f64> { robj, ptr, length },
            })
        } else {
            Err("not a numeric vector")
        }
    }
}

/// Comparison for assqrt_eq.
impl PartialEq<[f64]> for NumericVector {
    fn eq(&self, rhs: &[f64]) -> bool {
        self.inner.robj.as_f64_slice() == Some(rhs)
    }
}

/// Implement collect.
impl FromIterator<f64> for NumericVector {
    fn from_iter<I: IntoIterator<Item = f64>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let res = match iter.size_hint() {
            // fast path: direct to vector
            (min, Some(max)) if min == max => {
                let mut res = unsafe { NumericVector::allocate(min) };
                res
                    .iter_mut()
                    .zip(iter)
                    .for_each(|(d, s)| *d = s);
                res
            }
            // slow path: build a vec first.
            _ => {
                let values: Vec<_> = iter.collect();
                let res = values.into_iter().collect::<NumericVector>();
                res
            }
        };
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector() {
        let mut nv = NumericVector::zeros(100);
        assert_eq!(nv[0], 0.);
        assert_eq!(nv[99], 0.);

        for v in nv.iter() {
            assert_eq!(v, &0.);
        }

        for v in nv.iter_mut() {
            *v = 1.;
        }

        for v in nv.iter() {
            assert_eq!(v, &1.);
        }

        // convert a generic Robj into a numeric vector.
        assert_eq!(
            &from_robj::<NumericVector>(&Robj::from(1.)).unwrap(),
            &[1.][..]
        );
        assert!(from_robj::<NumericVector>(&Robj::from("x")).is_err());
    }
}
