//! Wrappers for matrices with deferred arithmetic.

use super::scalar::Scalar;
use super::*;
use std::{
    marker::PhantomData,
    ops::{Index, IndexMut},
};

/// Wrapper for creating and using matrices and arrays.
///
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let matrix = RArray::from_slice([1,2,3,4,5,6], [3, 2]);
///     let robj = r!(matrix);
///     assert_eq!(robj.is_matrix(), true);
///     assert_eq!(robj.nrows(), 3);
///     assert_eq!(robj.ncols(), 2);
/// }
/// ```
#[derive(Debug, PartialEq)]
pub struct RArray<T, D> {
    /// Owning Robj (probably should be a Pin).
    robj: Robj,

    /// Dimensions of the array.
    dim: D,

    phantom: PhantomData<T>,
}

pub type RMatrix<T> = RArray<T, [usize; 2]>;

impl<'a, T, D> RArray<T, D>
where
    Robj: AsTypedSlice<'a, T>,
    Self: 'a,
{
    // Make a new RArray type.
    // This function doesn't insert a dim attribute to the Robj.
    pub(crate) fn new(robj: Robj, dim: D) -> Self {
        Self {
            robj,
            dim,
            phantom: PhantomData,
        }
    }

    /// Get the underlying data from this RArray.
    pub fn data(&'a self) -> &'a [T] {
        self.robj.as_typed_slice().unwrap()
    }

    /// Get the dimensions for this array.
    pub fn dim(&self) -> &D {
        &self.dim
    }

    /// Make a new RArray type.
    /// This function inserts a dim attribute to the Robj.
    pub fn from_slice<P>(slice: P, dims: D) -> Result<Self>
    where
        P: AsRef<[T]>,
        T: ToVectorValue + Clone + 'a,
        D: AsRef<[usize]> + Clone,
    {
        let dims_ref = dims.as_ref();
        let slice_ref = slice.as_ref();
        let robj = slice_ref.iter().cloned().collect_robj();

        let prod = dims_ref.iter().product::<usize>();
        if prod != robj.len() {
            return Err(Error::Other(format!(
                "The vector length ({}) does not match the length implied by the dimensions ({})",
                slice_ref.len(),
                prod
            )));
        }

        let robj = robj
            .set_attrib(
                wrapper::symbol::dim_symbol(),
                dims_ref.iter().collect_robj(),
            )
            .unwrap();

        Ok(Self {
            robj,
            dim: dims,
            phantom: PhantomData,
        })
    }

    /// Make a new RArray type.
    /// This function inserts a dim attribute to the Robj.
    /// # Safety
    /// The caller must ensure that the product of the dimensions is equal to the slice length.
    pub unsafe fn from_slice_unchecked<P>(slice: P, dims: D) -> Result<Self>
    where
        P: AsRef<[T]>,
        T: ToVectorValue + Clone + 'a,
        D: AsRef<[usize]> + Clone,
    {
        let dims_ref = dims.as_ref();
        let robj = slice.as_ref().iter().cloned().collect_robj();

        let robj = robj
            .set_attrib(
                wrapper::symbol::dim_symbol(),
                dims_ref.iter().collect_robj(),
            )
            .unwrap();

        Ok(Self {
            robj,
            dim: dims,
            phantom: PhantomData,
        })
    }
}

impl<T, D> From<RArray<T, D>> for Robj {
    fn from(array: RArray<T, D>) -> Self {
        array.robj
    }
}

impl<'a, T: 'a> TryFrom<Robj> for RArray<T, Integers>
where
    Robj: AsTypedSlice<'a, T>,
{
    type Error = Error;

    fn try_from(robj: Robj) -> Result<Self> {
        if !robj.is_array() {
            Err(Error::ExpectedArray(robj))
        } else {
            // Ok to unwrap since the robj is an array.
            let dim = robj.dim().unwrap();
            Ok(RArray::new(robj, dim))
        }
    }
}

impl<'a, T: 'a> TryFrom<&'a Robj> for RArray<T, Integers>
where
    Robj: AsTypedSlice<'a, T>,
{
    type Error = Error;

    fn try_from(robj: &'a Robj) -> Result<Self> {
        Self::try_from(robj.clone())
    }
}

impl<'a, T: 'a> TryFrom<Robj> for RMatrix<T>
where
    Robj: AsTypedSlice<'a, T>,
{
    type Error = Error;

    fn try_from(robj: Robj) -> Result<Self> {
        if !robj.is_array() {
            Err(Error::ExpectedArray(robj))
        } else {
            // Ok to unwrap since the robj is an array.
            let dim = robj.dim().unwrap();
            Ok(RArray::new(
                robj,
                [dim[0].inner() as usize, dim[1].inner() as usize],
            ))
        }
    }
}

impl<'a, T: 'a> TryFrom<&'a Robj> for RMatrix<T>
where
    Robj: AsTypedSlice<'a, T>,
{
    type Error = Error;

    fn try_from(robj: &'a Robj) -> Result<Self> {
        Self::try_from(robj.clone())
    }
}

impl<T, D> Deref for RArray<T, D> {
    type Target = Robj;

    fn deref(&self) -> &Self::Target {
        &self.robj
    }
}

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

impl<T> Offset<[usize; 2]> for RMatrix<T> {
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

impl<'a, T: 'a> Index<[usize; 2]> for RMatrix<T>
where
    Robj: AsTypedSlice<'a, T>,
{
    type Output = T;

    /// Zero-based indexing in row, column order.
    ///
    /// Panics if out of bounds.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let matrix = RArray::from_slice([1.,2.,3.,4.,5.,6.], [3, 2]).unwrap();
    ///     assert_eq!(matrix[[0, 0]], 1.);
    ///     assert_eq!(matrix[[1, 0]], 2.);
    ///     assert_eq!(matrix[[2, 1]], 6.);
    /// }
    /// ```
    fn index(&self, index: [usize; 2]) -> &Self::Output {
        let ptr = self.robj.as_typed_slice().unwrap().as_ptr();
        unsafe { ptr.add(self.offset(index)).as_ref().unwrap() }
    }
}

impl<'a, T: 'a> IndexMut<[usize; 2]> for RMatrix<T>
where
    Robj: AsTypedSlice<'a, T>,
{
    /// Zero-based mutable indexing in row, column order.
    ///
    /// Panics if out of bounds.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let mut matrix = RArray::from_slice([0.,0.,0.,0.,0.,0.], [3, 2]).unwrap();
    ///     matrix[[0, 0]] = 1.;
    ///     matrix[[1, 0]] = 2.;
    ///     matrix[[2, 0]] = 3.;
    ///     matrix[[0, 1]] = 4.;
    ///     assert_eq!(matrix.as_real_slice().unwrap(), &[1., 2., 3., 4., 0., 0.]);
    /// }
    /// ```
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        let ptr = self.robj.as_typed_slice_mut().unwrap().as_mut_ptr();
        unsafe { ptr.add(self.offset(index)).as_mut().unwrap() }
    }
}

mod tests {
    use super::*;

    #[test]
    fn from_slice() {
        test! {
            let slice = [1,2,3,4,5,6];
            let dim = [2,3];
            let array = RArray::from_slice(slice, dim).unwrap();
            let dim = array.dim().clone();
            let robj = r!(array);
            assert_eq!(dim, [2,3]);
            assert!(robj.is_array());
            assert!(robj.is_matrix());
            assert!(RArray::<i32, Integers>::try_from(robj).is_ok());
        }
    }

    #[test]
    fn try_from_rarray() {
        test! {
            let slice = [1,2,3,4,5,6];
            let dim = [2,3];
            let array = RArray::from_slice(slice, dim).unwrap();
            let robj = r!(array);
            let rarray = RArray::<i32, Integers>::try_from(robj).unwrap();
            let new_dim = rarray.dim();
            assert_eq!(new_dim[0].inner(), 2);
            assert_eq!(new_dim[1].inner(), 3);
            assert_eq!(new_dim.len(), 2);
        }
    }
}
