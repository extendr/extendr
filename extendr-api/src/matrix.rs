//! Wrappers for matrices with deferred arithmetic.

use crate::*;
use std::ops::{Index, IndexMut};

/// Wrapper for creating and using matrices and arrays.
///
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let matrix = RMatrix::new([
///          1., 2., 3.,
///          4., 5., 6.], 3, 2);
///     let robj = r!(matrix);
///     assert_eq!(robj.is_matrix(), true);
///     assert_eq!(robj.nrows(), 3);
///     assert_eq!(robj.ncols(), 2);
///
///     let matrix2 : RMatrix<&[f64]> = robj.as_matrix().ok_or("error")?;
///     assert_eq!(matrix2.data().len(), 6);
///     assert_eq!(matrix2.nrows(), 3);
///     assert_eq!(matrix2.ncols(), 2);
/// }
/// ```
#[derive(Debug, PartialEq)]
pub struct RArray<T, D> {
    data: T,
    dim: D,
}

pub type RColumn<T> = RArray<T, [usize; 1]>;
pub type RMatrix<T> = RArray<T, [usize; 2]>;
pub type RMatrix3D<T> = RArray<T, [usize; 3]>;

const BASE: usize = 0;

trait Offset<D> {
    /// Get the offset into the array for a given index.
    fn offset(&self, idx: D) -> usize;
}

impl<T> Offset<[usize; 1]> for RArray<T, [usize; 1]> {
    /// Get the offset into the array for a given index.
    fn offset(&self, index: [usize; 1]) -> usize {
        if index[0] - BASE > self.dim[0] {
            panic!("array index: row overflow");
        }
        index[0] - BASE
    }
}

impl<T> Offset<[usize; 2]> for RArray<T, [usize; 2]> {
    /// Get the offset into the array for a given index.
    fn offset(&self, index: [usize; 2]) -> usize {
        if index[0] - BASE > self.dim[0] {
            panic!("matrix index: row overflow");
        }
        if index[1] - BASE > self.dim[1] {
            panic!("matrix index: column overflow");
        }
        (index[0] - BASE) + self.dim[0] * (index[1] - BASE)
    }
}

impl<T> Offset<[usize; 3]> for RArray<T, [usize; 3]> {
    /// Get the offset into the array for a given index.
    fn offset(&self, index: [usize; 3]) -> usize {
        if index[0] - BASE > self.dim[0] {
            panic!("RMatrix3D index: row overflow");
        }
        if index[1] - BASE > self.dim[1] {
            panic!("RMatrix3D index: column overflow");
        }
        if index[2] - BASE > self.dim[2] {
            panic!("RMatrix3D index: submatrix overflow");
        }
        (index[0] - BASE) + self.dim[0] * (index[1] - BASE + self.dim[1] * (index[2] - BASE))
    }
}

impl<T, D> RArray<T, D> {
    /// Get the underlying data fro this array.
    pub fn data(&self) -> &T {
        &self.data
    }

    /// Get the dimensions for this array.
    pub fn dim(&self) -> &D {
        &self.dim
    }
}

impl<T> RColumn<T> {
    /// Make a new vector type.
    pub fn new(data: T, nrows: usize) -> Self {
        let dim = [nrows];
        Self { data, dim }
    }

    /// Get the number of rows.
    pub fn nrows(&self) -> usize {
        self.dim[0]
    }
}

impl<T> RMatrix<T> {
    /// Create a new matrix wrapper.
    pub fn new(data: T, nrows: usize, ncols: usize) -> Self {
        let dim = [nrows, ncols];
        Self { data, dim }
    }

    /// Get the number of rows.
    pub fn nrows(&self) -> usize {
        self.dim[0]
    }

    /// Get the number of columns.
    pub fn ncols(&self) -> usize {
        self.dim[1]
    }
}

impl<T> RMatrix3D<T> {
    /// Create a new matrix wrapper.
    pub fn new(data: T, nrows: usize, ncols: usize, nsub: usize) -> Self {
        let dim = [nrows, ncols, nsub];
        Self { data, dim }
    }

    /// Get the number of rows.
    pub fn nrows(&self) -> usize {
        self.dim[0]
    }

    /// Get the number of columns.
    pub fn ncols(&self) -> usize {
        self.dim[1]
    }

    /// Get the number of submatrices.
    pub fn nsub(&self) -> usize {
        self.dim[2]
    }
}

impl<T> From<RColumn<T>> for Robj
where
    T: Into<Robj>,
{
    fn from(array: RColumn<T>) -> Self {
        let res = array.data.into();
        res
    }
}

impl<T> From<RArray<T, [usize; 2]>> for Robj
where
    T: Into<Robj>,
{
    fn from(array: RArray<T, [usize; 2]>) -> Self {
        let res = array.data.into();
        let dim = [array.dim[0] as i32, array.dim[1] as i32];
        res.set_attrib(dim_symbol(), dim)
            .unwrap()
            .set_attrib(class_symbol(), "matrix")
            .unwrap()
    }
}

impl<T> From<RArray<T, [usize; 3]>> for Robj
where
    T: Into<Robj>,
{
    fn from(array: RArray<T, [usize; 3]>) -> Self {
        let res = array.data.into();
        let dim = [
            array.dim[0] as i32,
            array.dim[1] as i32,
            array.dim[2] as i32,
        ];
        res.set_attrib(dim_symbol(), dim)
            .unwrap()
            .set_attrib(class_symbol(), "array")
            .unwrap()
    }
}

impl Robj {
    pub fn as_vector<'a, E>(&self) -> Option<RColumn<&'a [E]>>
    where
        Self: AsTypedSlice<'a, E>,
    {
        if let Some(data) = self.as_typed_slice() {
            let dim = [self.nrows() as usize];
            return Some(RArray { data, dim });
        }
        None
    }

    pub fn as_matrix<'a, E>(&self) -> Option<RMatrix<&'a [E]>>
    where
        Self: AsTypedSlice<'a, E>,
    {
        if self.is_matrix() {
            if let Some(data) = self.as_typed_slice() {
                let dim = [self.nrows() as usize, self.ncols() as usize];
                return Some(RArray { data, dim });
            }
        }
        None
    }

    pub fn as_matrix3d<'a, E>(&self) -> Option<RMatrix3D<&'a [E]>>
    where
        Self: AsTypedSlice<'a, E>,
    {
        if self.is_array() {
            if let Some(data) = self.as_typed_slice() {
                if let Some(dim) = self.dim() {
                    let dim: Vec<_> = dim.collect();
                    let dim = [dim[0] as usize, dim[1] as usize, dim[2] as usize];
                    return Some(RArray { data, dim });
                }
            }
        }
        None
    }
}

impl<T> Index<[usize; 2]> for RArray<T, [usize; 2]>
where
    T: Index<usize>,
{
    type Output = <T as Index<usize>>::Output;

    /// Zero-based indexing in row, column order.
    ///
    /// Panics if out of bounds.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let matrix = RMatrix::new(vec![
    ///          1., 2., 3.,
    ///          4., 5., 6.], 3, 2);
    ///     assert_eq!(matrix[[0, 0]], 1.);
    ///     assert_eq!(matrix[[1, 0]], 2.);
    ///     assert_eq!(matrix[[2, 1]], 6.);
    /// }
    /// ```
    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.data[self.offset(index)]
    }
}

impl<T> IndexMut<[usize; 2]> for RArray<T, [usize; 2]>
where
    T: IndexMut<usize>,
{
    /// Zero-based mutable indexing in row, column order.
    ///
    /// Panics if out of bounds.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let mut matrix = RMatrix::new(vec![0.; 6], 3, 2);
    ///     matrix[[0, 0]] = 1.;
    ///     matrix[[1, 0]] = 2.;
    ///     matrix[[2, 0]] = 3.;
    ///     assert_eq!(matrix, RMatrix::new(vec![1., 2., 3., 0., 0., 0.], 3, 2));
    /// }
    /// ```
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        let offset = self.offset(index);
        &mut self.data[offset]
    }
}

#[test]
fn matrix_ops() {
    test! {
        let vector = RColumn::new([1., 2., 3.], 3);
        let robj = r!(vector);
        assert_eq!(robj.is_vector(), true);
        assert_eq!(robj.nrows(), 3);

        let vector2 : RColumn<&[f64]> = robj.as_vector().ok_or("expected array")?;
        assert_eq!(vector2.data().len(), 3);
        assert_eq!(vector2.nrows(), 3);

        let matrix = RMatrix::new([
            1., 2., 3.,
            4., 5., 6.], 3, 2);
        let robj = r!(matrix);
        assert_eq!(robj.is_matrix(), true);
        assert_eq!(robj.nrows(), 3);
        assert_eq!(robj.ncols(), 2);
        let matrix2 : RMatrix<&[f64]> = robj.as_matrix().ok_or("expected matrix")?;
        assert_eq!(matrix2.data().len(), 6);
        assert_eq!(matrix2.nrows(), 3);
        assert_eq!(matrix2.ncols(), 2);

        let array = RMatrix3D::new([
            1., 2.,  3., 4.,
            5.,  6., 7., 8.], 2, 2, 2);
        let robj = r!(array);
        assert_eq!(robj.is_array(), true);
        assert_eq!(robj.nrows(), 2);
        assert_eq!(robj.ncols(), 2);
        let array2 : RMatrix3D<&[f64]> = robj.as_matrix3d().ok_or("expected matrix3d")?;
        assert_eq!(array2.data().len(), 8);
        assert_eq!(array2.nrows(), 2);
        assert_eq!(array2.ncols(), 2);
        assert_eq!(array2.nsub(), 2);
    }
}
