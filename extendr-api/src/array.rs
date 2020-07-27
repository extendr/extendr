//! Owning array types. These hold owndership of an R
//! object. Primary use case is for writing R arrays.


use std::iter::FromIterator;
use std::ops::{Deref, DerefMut};
use crate::robj::*;
use crate::logical::*;
use libR_sys::*;

/// A base class for vectors.
pub struct VectorBase<T> {
    robj: Robj,
    ptr: *mut T,
    length: usize,
}

/// An owning floating point vector class.
struct NumericVector {
    base: VectorBase<f64>,
}

/// An owning integer vector class.
struct IntegerVector {
    base: VectorBase<i32>,
}

/// An owning boolean vector class.
struct LogicalVector {
    base: VectorBase<Bool>,
}

/// An owning byte vector class.
struct RawVector {
    base: VectorBase<u8>,
}

impl std::fmt::Debug for NumericVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NumericVector({:?})", self.base.robj)
    }
}

impl std::fmt::Debug for IntegerVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IntegerVector({:?})", self.base.robj)
    }
}

impl std::fmt::Debug for LogicalVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LogicalVector({:?})", self.base.robj)
    }
}

impl std::fmt::Debug for RawVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RawVector({:?})", self.base.robj)
    }
}

impl<T: Copy> VectorBase<T> {
    // Convert a vector to a slice.
    fn deref(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.ptr as *const T, self.length) }
    }

    // Convert a vector to a slice.
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr as *mut T, self.length) }
    }

    fn fill(&mut self, val: T) {
        let slice = self.deref_mut();
        slice.iter_mut().for_each(|d| *d = val);
    }
}

macro_rules! impl_vector {
    ($name: ident, $type: ty, $sexptype: ident) => {
        impl $name {
            /// Create a new uninitialised R object for the vector.
            pub unsafe fn allocate(length: usize) -> Self {
                let mut robj = Robj::allocVector($sexptype, length);
                let slice = robj.as_typed_slice_mut().unwrap();
                let ptr = slice.as_mut_ptr();
                Self {
                    base: VectorBase { robj, ptr, length },
                }
            }
        
            /// Create a new vector filled with zeros.
            pub fn zeros(length: usize) -> Self {
                unsafe {
                    let mut res = Self::allocate(length);
                    res.base.fill(0.into());
                    res
                }
            }
        
            /// Read only access to the underlying R object.
            pub fn _robj(&self) -> &Robj {
                &self.base.robj
            }
        
            /// Read-write access to the underlying R object.
            pub fn _robj_mut(&mut self) -> &mut Robj {
                &mut self.base.robj
            }
        }
        
        impl Default for $name {
            fn default() -> Self {
                Self::zeros(0)
            }
        }
        
        /// Make the vector behave like Vec (ie. deref to a slice).
        /// Because we have already unpacked the pointer and length, this
        /// should be a nop in many cases.
        impl Deref for $name {
            type Target = [$type];

            fn deref(&self) -> &Self::Target {
                self.base.deref()
            }
        }

        /// Make the vector behave like Vec (ie. deref to a slice).
        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.base.deref_mut()
            }
        }
        impl From<Vec<$type>> for $name {
            fn from(v: Vec<$type>) -> Self {
                v.into_iter().collect::<$name>()
            }
        }
        
        impl From<&[$type]> for $name {
            fn from(v: &[$type]) -> Self {
                v.into_iter().cloned().collect::<$name>()
            }
        }
        
        /// Convert incoming parameters.
        impl<'a> FromRobj<'a> for $name {
            fn from_robj(robj: &'a Robj) -> Result<Self, &'static str> {
                let mut robj = unsafe { new_owned(robj.get()) };
                if let Some(slice) = robj.as_typed_slice_mut() {
                    let ptr = slice.as_mut_ptr();
                    let length = slice.len();
                    Ok(Self {
                        base: VectorBase::<$type> { robj, ptr, length },
                    })
                } else {
                    Err("not a numeric vector")
                }
            }
        }
        
        /// Comparison for assert_eq.
        impl PartialEq<[$type]> for $name {
            fn eq(&self, rhs: &[$type]) -> bool {
                self.base.robj.as_typed_slice() == Some(rhs)
            }
        }
        
        /// Implement collect.
        impl FromIterator<$type> for $name {
            fn from_iter<I: IntoIterator<Item = $type>>(iter: I) -> Self {
                let iter = iter.into_iter();
                let res = match iter.size_hint() {
                    // fast path: direct to vector
                    (min, Some(max)) if min == max => {
                        let mut res = unsafe { $name::allocate(min) };
                        res
                            .iter_mut()
                            .zip(iter)
                            .for_each(|(d, s)| *d = s);
                        res
                    }
                    // slow path: build a vec first.
                    _ => {
                        let values: Vec<_> = iter.collect();
                        let res = values.into_iter().collect::<$name>();
                        res
                    }
                };
                res
            }
        }
    }
}


impl_vector!(NumericVector, f64, REALSXP);
impl_vector!(IntegerVector, i32, INTSXP);
impl_vector!(LogicalVector, Bool, LGLSXP);
impl_vector!(RawVector, u8, RAWSXP);

/// A base class for matrices.
pub struct MatrixBase<T> {
    robj: Robj,
    ptr: *mut T,
    rows: usize,
    cols: usize,
}

impl<T: Copy> MatrixBase<T> {
    // Convert a vector to a slice.
    fn deref(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.ptr as *const T, self.rows*self.cols) }
    }

    // Convert a vector to a slice.
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr as *mut T, self.rows*self.cols) }
    }

    fn fill(&mut self, val: T) {
        let slice = self.deref_mut();
        slice.iter_mut().for_each(|d| *d = val);
    }
}

/// An owning floating point matrix class.
struct NumericMatrix {
    base: MatrixBase<f64>,
}

/// An owning integer matrix class.
struct IntegerMatrix {
    base: MatrixBase<i32>,
}

/// An owning boolean matrix class.
struct LogicalMatrix {
    base: MatrixBase<Bool>,
}

/// An owning byte matrix class.
struct RawMatrix {
    base: MatrixBase<u8>,
}

macro_rules! impl_matrix {
    ($name: ident, $type: ty, $sexptype: ident) => {
        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}({:?} {}x{})", stringify!($name), self.base.robj, self.base.rows, self.base.cols)
            }
        }
        
        impl $name {
            /// Create a new uninitialised R object for the matrix.
            pub unsafe fn allocate(rows: usize, cols: usize) -> Self {
                let mut robj = Robj::allocVector($sexptype, rows*cols);
                let slice = robj.as_typed_slice_mut().unwrap();
                let ptr = slice.as_mut_ptr();
                Self {
                    base: MatrixBase { robj, ptr, rows, cols },
                }
            }

            /// Create a new matrix filled with zeros.
            pub fn zeros(rows: usize, cols: usize) -> Self {
                unsafe {
                    let mut res = Self::allocate(rows, cols);
                    res.base.fill(0.into());
                    res
                }
            }

            /// Read only access to the underlying R object.
            pub fn _robj(&self) -> &Robj {
                &self.base.robj
            }

            /// Read-write access to the underlying R object.
            pub fn _robj_mut(&mut self) -> &mut Robj {
                &mut self.base.robj
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::zeros(0, 0)
            }
        }

        /// Make the vector behave like Vec (ie. deref to a slice).
        /// Because we have already unpacked the pointer and length, this
        /// should be a nop in many cases.
        impl Deref for $name {
            type Target = [$type];

            fn deref(&self) -> &Self::Target {
                self.base.deref()
            }
        }

        /// Make the vector behave like Vec (ie. deref to a slice).
        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.base.deref_mut()
            }
        }

        impl From<Vec<$type>> for $name {
            fn from(v: Vec<$type>) -> Self {
                v.into_iter().collect::<$name>()
            }
        }

        impl From<&[$type]> for $name {
            fn from(v: &[$type]) -> Self {
                v.into_iter().cloned().collect::<$name>()
            }
        }

        /// Convert incoming parameters.
        impl<'a> FromRobj<'a> for $name {
            fn from_robj(robj: &'a Robj) -> Result<Self, &'static str> {
                if !robj.isMatrix() {
                    Err("not a numeric vector")
                } else {
                    let rows = robj.nrows();
                    let cols = robj.ncols();
                    let mut robj = unsafe { new_owned(robj.get()) };
                    if let Some(slice) = robj.as_typed_slice_mut() {
                        let ptr = slice.as_mut_ptr();
                        Ok(Self {
                            base: MatrixBase::<$type> { robj, ptr, rows, cols },
                        })
                    } else {
                        Err("not a numeric vector")
                    }
                }
            }
        }

        /// Comparison for assert_eq.
        impl PartialEq<[$type]> for $name {
            fn eq(&self, rhs: &[$type]) -> bool {
                self.base.robj.as_typed_slice() == Some(rhs)
            }
        }
        /// Implement collect.
        impl FromIterator<$type> for $name {
            fn from_iter<I: IntoIterator<Item = $type>>(iter: I) -> Self {
                let iter = iter.into_iter();
                let res = match iter.size_hint() {
                    // fast path: direct to vector
                    (min, Some(max)) if min == max => {
                        let mut res = unsafe { $name::allocate(1, min) };
                        res
                            .iter_mut()
                            .zip(iter)
                            .for_each(|(d, s)| *d = s);
                        res
                    }
                    // slow path: build a vec first.
                    _ => {
                        let values: Vec<_> = iter.collect();
                        let res = values.into_iter().collect::<$name>();
                        res
                    }
                };
                res
            }
        }
    }
}

impl_matrix!(NumericMatrix, f64, REALSXP);
impl_matrix!(IntegerMatrix, i32, INTSXP);
impl_matrix!(LogicalMatrix, Bool, LGLSXP);
impl_matrix!(RawMatrix, u8, RAWSXP);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector() {
        // Necessary for memory allocation,
        crate::engine::start_r();

        let mut nv = NumericVector::zeros(100);
        let mut iv = IntegerVector::zeros(100);
        assert_eq!(nv[0], 0.);
        assert_eq!(nv[99], 0.);
        for v in nv.iter() {
            assert_eq!(v, &0.);
        }
        nv[1] = 5.;
        assert_eq!(nv[1], 5.);

        for v in nv.iter_mut() {
            *v = 1.;
        }

        for v in nv.iter() {
            assert_eq!(v, &1.);
        }

        assert_eq!(iv[0], 0);
        assert_eq!(iv[99], 0);
        for v in iv.iter() {
            assert_eq!(v, &0);
        }
        iv[1] = 5;
        assert_eq!(iv[1], 5);

        for v in iv.iter_mut() {
            *v = 1;
        }

        for v in iv.iter() {
            assert_eq!(v, &1);
        }


        // convert a generic Robj into a numeric vector.
        assert_eq!(
            &from_robj::<NumericVector>(&Robj::from(1.)).unwrap(),
            &[1.][..]
        );
        assert!(from_robj::<NumericVector>(&Robj::from("x")).is_err());
    }
}
