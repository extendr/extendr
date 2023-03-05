//! Wrappers for matrices with deferred arithmetic.

use super::*;
use crate::robj::GetSexp;
use std::ops::{Index, IndexMut};

/// Wrapper for creating and using matrices and arrays.
///
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let matrix = RMatrix::new_matrix(3, 2, |r, c| [
///         [1., 2., 3.],
///          [4., 5., 6.]][c][r]);
///     let robj = r!(matrix);
///     assert_eq!(robj.is_matrix(), true);
///     assert_eq!(robj.nrows(), 3);
///     assert_eq!(robj.ncols(), 2);
///
///     let matrix2 : RMatrix<f64> = robj.as_matrix().ok_or("error")?;
///     assert_eq!(matrix2.data().len(), 6);
///     assert_eq!(matrix2.nrows(), 3);
///     assert_eq!(matrix2.ncols(), 2);
/// }
/// ```
#[derive(Debug, PartialEq)]
pub struct RArray<T, D> {
    /// Owning Robj (probably should be a Pin).
    robj: Robj,

    /// Slice of the data references the Robj.
    data: *mut T,

    /// Dimensions of the array.
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
    pub fn from_parts(robj: Robj, data: *mut T, dim: D) -> Self {
        Self { robj, data, dim }
    }

    /// Get the underlying data fro this array.
    pub fn data(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.data, self.robj.len()) }
    }

    /// Get the dimensions for this array.
    pub fn dim(&self) -> &D {
        &self.dim
    }
}

impl<'a, T: ToVectorValue + 'a> RColumn<T>
where
    Robj: AsTypedSlice<'a, T>,
{
    /// Make a new column type.
    pub fn new_column<F: FnMut(usize) -> T>(nrows: usize, f: F) -> Self {
        let robj = (0..nrows).map(f).collect_robj();
        let dim = [nrows];
        let mut robj = robj.set_attrib(wrapper::symbol::dim_symbol(), dim).unwrap();
        let slice = robj.as_typed_slice_mut().unwrap();
        let data = slice.as_mut_ptr();
        RArray::from_parts(robj, data, dim)
    }

    /// Get the number of rows.
    pub fn nrows(&self) -> usize {
        self.dim[0]
    }
}

impl<'a, T: ToVectorValue + 'a> RMatrix<T>
where
    Robj: AsTypedSlice<'a, T>,
{
    /// Create a new matrix wrapper.
    ///
    /// # Arguments
    ///
    /// * `nrows` - the number of rows the returned matrix will have
    /// * `ncols` - the number of columns the returned matrix will have
    /// * `f` - a function that will be called for each entry of the matrix in order to populate it with values.
    ///     It must return a scalar value that can be converted to an R scalar, such as `u32`, `f64`, i.e. see [ToVectorValue].
    ///     It accepts two arguments:
    ///     * `r` - the current row of the entry we are creating
    ///     * `c` - the current column of the entry we are creating
    pub fn new_matrix<F: Clone + FnMut(usize, usize) -> T>(
        nrows: usize,
        ncols: usize,
        f: F,
    ) -> Self {
        let robj = (0..ncols)
            .flat_map(|c| {
                let mut g = f.clone();
                (0..nrows).map(move |r| g(r, c))
            })
            .collect_robj();
        let dim = [nrows, ncols];
        let mut robj = robj.set_attrib(wrapper::symbol::dim_symbol(), dim).unwrap();
        let data = robj.as_typed_slice_mut().unwrap().as_mut_ptr();
        RArray::from_parts(robj, data, dim)
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

impl<'a, T: ToVectorValue + 'a> RMatrix3D<T>
where
    Robj: AsTypedSlice<'a, T>,
{
    pub fn new_matrix3d<F: Clone + FnMut(usize, usize, usize) -> T>(
        nrows: usize,
        ncols: usize,
        nmatrix: usize,
        f: F,
    ) -> Self {
        let robj = (0..nmatrix)
            .flat_map(|m| {
                let h = f.clone();
                (0..ncols).flat_map(move |c| {
                    let mut g = h.clone();
                    (0..nrows).map(move |r| g(r, c, m))
                })
            })
            .collect_robj();
        let dim = [nrows, ncols, nmatrix];
        let mut robj = robj.set_attrib(wrapper::symbol::dim_symbol(), dim).unwrap();
        let data = robj.as_typed_slice_mut().unwrap().as_mut_ptr();
        RArray::from_parts(robj, data, dim)
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

impl<'a, T: 'a> TryFrom<Robj> for RColumn<T>
where
    Robj: AsTypedSlice<'a, T>,
{
    type Error = Error;

    fn try_from(mut robj: Robj) -> Result<Self> {
        if let Some(slice) = robj.as_typed_slice_mut() {
            Ok(RArray::from_parts(robj, slice.as_mut_ptr(), [slice.len()]))
        } else {
            Err(Error::ExpectedVector(robj))
        }
    }
}

impl<'a, T: 'a> TryFrom<Robj> for RMatrix<T>
where
    Robj: AsTypedSlice<'a, T>,
{
    type Error = Error;

    fn try_from(mut robj: Robj) -> Result<Self> {
        if !robj.is_matrix() {
            Err(Error::ExpectedMatrix(robj))
        } else if let Some(slice) = robj.as_typed_slice_mut() {
            if let Some(dim) = robj.dim() {
                let dim: Vec<_> = dim.iter().map(|d| d.inner() as usize).collect();
                if dim.len() != 2 {
                    Err(Error::ExpectedMatrix(robj))
                } else {
                    Ok(RArray::from_parts(
                        robj,
                        slice.as_mut_ptr(),
                        [dim[0], dim[1]],
                    ))
                }
            } else {
                Err(Error::ExpectedMatrix(robj))
            }
        } else {
            Err(Error::TypeMismatch(robj))
        }
    }
}

impl<'a, T: 'a> TryFrom<Robj> for RMatrix3D<T>
where
    Robj: AsTypedSlice<'a, T>,
{
    type Error = Error;

    fn try_from(mut robj: Robj) -> Result<Self> {
        if let Some(slice) = robj.as_typed_slice_mut() {
            if let Some(dim) = robj.dim() {
                if dim.len() != 3 {
                    Err(Error::ExpectedMatrix3D(robj))
                } else {
                    let dim: Vec<_> = dim.iter().map(|d| d.inner() as usize).collect();
                    Ok(RArray::from_parts(
                        robj,
                        slice.as_mut_ptr(),
                        [dim[0], dim[1], dim[2]],
                    ))
                }
            } else {
                Err(Error::ExpectedMatrix3D(robj))
            }
        } else {
            Err(Error::TypeMismatch(robj))
        }
    }
}

impl<T, D> From<RArray<T, D>> for Robj {
    /// Convert a column, matrix or matrix3d to an Robj.
    fn from(array: RArray<T, D>) -> Self {
        array.robj
    }
}

pub trait MatrixConversions: GetSexp {
    fn as_column<'a, E: 'a>(&self) -> Option<RColumn<E>>
    where
        Robj: AsTypedSlice<'a, E>,
    {
        <RColumn<E>>::try_from(self.as_robj().clone()).ok()
    }

    fn as_matrix<'a, E: 'a>(&self) -> Option<RMatrix<E>>
    where
        Robj: AsTypedSlice<'a, E>,
    {
        <RMatrix<E>>::try_from(self.as_robj().clone()).ok()
    }

    fn as_matrix3d<'a, E: 'a>(&self) -> Option<RMatrix3D<E>>
    where
        Robj: AsTypedSlice<'a, E>,
    {
        <RMatrix3D<E>>::try_from(self.as_robj().clone()).ok()
    }
}

impl MatrixConversions for Robj {}

impl<T> Index<[usize; 2]> for RArray<T, [usize; 2]> {
    type Output = T;

    /// Zero-based indexing in row, column order.
    ///
    /// Panics if out of bounds.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///    let matrix = RArray::new_matrix(3, 2, |r, c| [
    ///        [1., 2., 3.],
    ///        [4., 5., 6.]][c][r]);
    ///     assert_eq!(matrix[[0, 0]], 1.);
    ///     assert_eq!(matrix[[1, 0]], 2.);
    ///     assert_eq!(matrix[[2, 1]], 6.);
    /// }
    /// ```
    fn index(&self, index: [usize; 2]) -> &Self::Output {
        unsafe { self.data.add(self.offset(index)).as_ref().unwrap() }
    }
}

impl<T> IndexMut<[usize; 2]> for RArray<T, [usize; 2]> {
    /// Zero-based mutable indexing in row, column order.
    ///
    /// Panics if out of bounds.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let mut matrix = RMatrix::new_matrix(3, 2, |_, _| 0.);
    ///     matrix[[0, 0]] = 1.;
    ///     matrix[[1, 0]] = 2.;
    ///     matrix[[2, 0]] = 3.;
    ///     matrix[[0, 1]] = 4.;
    ///     assert_eq!(matrix.as_real_slice().unwrap(), &[1., 2., 3., 4., 0., 0.]);
    /// }
    /// ```
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        unsafe { self.data.add(self.offset(index)).as_mut().unwrap() }
    }
}

impl<T, D> Deref for RArray<T, D> {
    type Target = Robj;

    fn deref(&self) -> &Self::Target {
        &self.robj
    }
}

#[test]
fn matrix_ops() {
    test! {
        let vector = RColumn::new_column(3, |r| [1., 2., 3.][r]);
        let robj = r!(vector);
        assert_eq!(robj.is_vector(), true);
        assert_eq!(robj.nrows(), 3);

        let vector2 : RColumn<f64> = robj.as_column().ok_or("expected array")?;
        assert_eq!(vector2.data().len(), 3);
        assert_eq!(vector2.nrows(), 3);

        let matrix = RMatrix::new_matrix(3, 2, |r, c| [
            [1., 2., 3.],
            [4., 5., 6.]][c][r]);
        let robj = r!(matrix);
        assert_eq!(robj.is_matrix(), true);
        assert_eq!(robj.nrows(), 3);
        assert_eq!(robj.ncols(), 2);
        let matrix2 : RMatrix<f64> = robj.as_matrix().ok_or("expected matrix")?;
        assert_eq!(matrix2.data().len(), 6);
        assert_eq!(matrix2.nrows(), 3);
        assert_eq!(matrix2.ncols(), 2);

        let array = RMatrix3D::new_matrix3d(2, 2, 2, |r, c, m| [
            [[1., 2.],  [3., 4.]],
            [[5.,  6.], [7., 8.]]][m][c][r]);
        let robj = r!(array);
        assert_eq!(robj.is_array(), true);
        assert_eq!(robj.nrows(), 2);
        assert_eq!(robj.ncols(), 2);
        let array2 : RMatrix3D<f64> = robj.as_matrix3d().ok_or("expected matrix3d")?;
        assert_eq!(array2.data().len(), 8);
        assert_eq!(array2.nrows(), 2);
        assert_eq!(array2.ncols(), 2);
        assert_eq!(array2.nsub(), 2);
    }
}
