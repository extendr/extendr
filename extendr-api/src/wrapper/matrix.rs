//! Wrappers for matrices with deferred arithmetic.
use self::robj::{AsTypedSlice, Robj};
use super::*;
use crate::scalar::Scalar;
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

    /// Dimensions of the array.
    dim: D,

    _data: std::marker::PhantomData<T>,
}

impl<T, D> RArray<T, D> {
    pub fn get_dimnames(&self) -> List {
        List::try_from(Robj::from_sexp(unsafe { Rf_GetArrayDimnames(self.get()) })).unwrap()
    }

    /// Set the names of the elements of an array.
    ///
    ///
    /// Equivalent to `names<-` in R
    pub fn set_names(&mut self, names: Strings) {
        // TODO: check what `names` are and validate the input...
        let _ = unsafe { Rf_namesgets(self.get_mut(), names.get()) };
    }

    /// Set the dimension names of an array.
    ///
    /// For [`RMatrix`] a list of length 2 is required, as that would entail
    /// column-names and row-names. If you only wish to set one, but not the other,
    /// then the unset element must be R `NULL`
    ///
    /// Equivalent to `dimnames<-` in R
    pub fn set_dimnames(&mut self, dimnames: List) {
        let _ = unsafe { Rf_dimnamesgets(self.get_mut(), dimnames.get()) };
    }

    /// Set the dimensions of an array.
    ///
    /// Equivalent to `dim<-`
    pub fn set_dim(&mut self, dim: Robj) {
        // TODO: ensure that Robj is LGLSXP, INTSXP, REALSXP, CPLXSXP, STRSXP, RAWSXP
        // or NilValue
        let _ = unsafe { Rf_dimgets(self.get_mut(), dim.get()) };
    }
}

pub type RColumn<T> = RArray<T, [usize; 1]>;
pub type RMatrix<T> = RArray<T, [usize; 2]>;
pub type RMatrix3D<T> = RArray<T, [usize; 3]>;

impl<T> RMatrix<T>
where
    T: ToVectorValue,
    Robj: for<'a> AsTypedSlice<'a, T>,
{
    /// Returns an [`RMatrix`] with dimensions according to `nrow` and `ncol`,
    /// with arbitrary entries. To initialize a matrix containing only `NA`
    /// values, use [`RMatrix::new_with_na`].
    pub fn new(nrow: usize, ncol: usize) -> Self {
        let sexptype = T::sexptype();
        let matrix = Robj::alloc_matrix(sexptype, nrow as _, ncol as _);
        RArray::from_parts(matrix, [nrow, ncol])
    }
}

impl<T> RMatrix<T>
where
    T: ToVectorValue + CanBeNA,
    Robj: for<'a> AsTypedSlice<'a, T>,
{
    /// Returns an [`RMatrix`] with dimensions according to `nrow` and `ncol`,
    /// with all entries set to `NA`.
    ///
    /// Note that since [`Raw`] does not have an NA representation in R,
    /// this method is not implemented for [`Rbyte`].
    pub fn new_with_na(nrow: usize, ncol: usize) -> Self {
        let mut matrix = Self::new(nrow, ncol);
        if nrow != 0 || ncol != 0 {
            matrix
                .as_typed_slice_mut()
                .unwrap()
                .iter_mut()
                .for_each(|x| {
                    *x = T::na();
                });
        }
        matrix
    }
}

impl<T> RMatrix<T> {
    pub fn get_colnames(&self) -> Option<Strings> {
        unsafe {
            let maybe_colnames = Rf_GetColNames(Rf_GetArrayDimnames(self.get()));
            match TYPEOF(maybe_colnames) {
                SEXPTYPE::NILSXP => None,
                SEXPTYPE::STRSXP => {
                    let colnames = Robj::from_sexp(maybe_colnames);
                    Some(std::mem::transmute(colnames))
                }
                _ => unreachable!("This should not have occurred. Please report an error at https://github.com/extendr/extendr/issues"),
            }
        }
    }
    pub fn get_rownames(&self) -> Option<Strings> {
        unsafe {
            let maybe_rownames = Rf_GetRowNames(Rf_GetArrayDimnames(self.get()));
            match TYPEOF(maybe_rownames) {
                SEXPTYPE::NILSXP => None,
                SEXPTYPE::STRSXP => {
                    let rownames = Robj::from_sexp(maybe_rownames);
                    Some(std::mem::transmute(rownames))
                }
                _ => unreachable!("This should not have occurred. Please report an error at https://github.com/extendr/extendr/issues"),
            }
        }
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

impl<T, D> RArray<T, D>
where
    Robj: for<'a> AsTypedSlice<'a, T>,
{
    pub fn from_parts(robj: Robj, dim: D) -> Self {
        Self {
            robj,
            dim,
            _data: std::marker::PhantomData,
        }
    }

    /// Returns a flat representation of the array in col-major.
    pub fn data(&self) -> &[T] {
        self.as_typed_slice().unwrap()
    }

    /// Returns a flat, mutable representation of the array in col-major.
    pub fn data_mut(&mut self) -> &mut [T] {
        self.as_typed_slice_mut().unwrap()
    }

    /// Get the dimensions for this array.
    pub fn dim(&self) -> &D {
        &self.dim
    }
}

impl<T> RColumn<T>
where
    T: ToVectorValue,
    Robj: for<'a> AsTypedSlice<'a, T>,
{
    /// Make a new column type.
    pub fn new_column<F: FnMut(usize) -> T>(nrows: usize, f: F) -> Self {
        let mut robj = (0..nrows).map(f).collect_robj();
        let dim = [nrows];
        robj.set_attrib(wrapper::symbol::dim_symbol(), dim).unwrap();
        RArray::from_parts(robj, dim)
    }

    /// Get the number of rows.
    pub fn nrows(&self) -> usize {
        self.dim[0]
    }
}

impl<T> RMatrix<T>
where
    T: ToVectorValue,
    Robj: for<'a> AsTypedSlice<'a, T>,
{
    /// Create a new matrix wrapper.
    ///
    /// # Arguments
    ///
    /// * `nrows` - the number of rows the returned matrix will have
    /// * `ncols` - the number of columns the returned matrix will have
    /// * `f` - a function that will be called for each entry of the matrix in order to populate it with values.
    ///     It must return a scalar value that can be converted to an R scalar, such as `i32`, `u32`, or `f64`, i.e. see [ToVectorValue].
    ///     It accepts two arguments:
    ///     * `r` - the current row of the entry we are creating
    ///     * `c` - the current column of the entry we are creating
    pub fn new_matrix<F: Clone + FnMut(usize, usize) -> T>(
        nrows: usize,
        ncols: usize,
        f: F,
    ) -> Self {
        let mut robj = (0..ncols)
            .flat_map(|c| {
                let mut g = f.clone();
                (0..nrows).map(move |r| g(r, c))
            })
            .collect_robj();
        let dim = [nrows, ncols];
        robj.set_attrib(wrapper::symbol::dim_symbol(), dim).unwrap();
        RArray::from_parts(robj, dim)
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

impl<T> RMatrix3D<T>
where
    T: ToVectorValue,
    Robj: for<'a> AsTypedSlice<'a, T>,
{
    pub fn new_matrix3d<F: Clone + FnMut(usize, usize, usize) -> T>(
        nrows: usize,
        ncols: usize,
        nmatrix: usize,
        f: F,
    ) -> Self {
        let mut robj = (0..nmatrix)
            .flat_map(|m| {
                let h = f.clone();
                (0..ncols).flat_map(move |c| {
                    let mut g = h.clone();
                    (0..nrows).map(move |r| g(r, c, m))
                })
            })
            .collect_robj();
        let dim = [nrows, ncols, nmatrix];
        robj.set_attrib(wrapper::symbol::dim_symbol(), dim).unwrap();
        RArray::from_parts(robj, dim)
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

impl<T> TryFrom<&Robj> for RColumn<T>
where
    Robj: for<'a> AsTypedSlice<'a, T>,
{
    type Error = Error;

    fn try_from(robj: &Robj) -> Result<Self> {
        if let Some(_slice) = robj.as_typed_slice() {
            let len = robj.len();
            Ok(RArray::from_parts(robj.clone(), [len]))
        } else {
            Err(Error::ExpectedVector(robj.clone()))
        }
    }
}

impl<T> TryFrom<&Robj> for RMatrix<T>
where
    Robj: for<'a> AsTypedSlice<'a, T>,
{
    type Error = Error;

    fn try_from(robj: &Robj) -> Result<Self> {
        if !robj.is_matrix() {
            Err(Error::ExpectedMatrix(robj.clone()))
        } else if let Some(_slice) = robj.as_typed_slice() {
            if let Some(dim) = robj.dim() {
                let dim: Vec<_> = dim.iter().map(|d| d.inner() as usize).collect();
                if dim.len() != 2 {
                    Err(Error::ExpectedMatrix(robj.clone()))
                } else {
                    Ok(RArray::from_parts(robj.clone(), [dim[0], dim[1]]))
                }
            } else {
                Err(Error::ExpectedMatrix(robj.clone()))
            }
        } else {
            Err(Error::TypeMismatch(robj.clone()))
        }
    }
}

impl<T> TryFrom<&Robj> for RMatrix3D<T>
where
    Robj: for<'a> AsTypedSlice<'a, T>,
{
    type Error = Error;

    fn try_from(robj: &Robj) -> Result<Self> {
        if let Some(_slice) = robj.as_typed_slice() {
            if let Some(dim) = robj.dim() {
                if dim.len() != 3 {
                    Err(Error::ExpectedMatrix3D(robj.clone()))
                } else {
                    let dim: Vec<_> = dim.iter().map(|d| d.inner() as usize).collect();
                    Ok(RArray::from_parts(robj.clone(), [dim[0], dim[1], dim[2]]))
                }
            } else {
                Err(Error::ExpectedMatrix3D(robj.clone()))
            }
        } else {
            Err(Error::TypeMismatch(robj.clone()))
        }
    }
}

macro_rules! impl_try_from_robj_ref {
    ($($type : tt)*) => {
        $(
            impl<T> TryFrom<Robj> for $type<T>
            where
                Robj: for<'a> AsTypedSlice<'a, T>,
            {
                type Error = Error;

                fn try_from(robj: Robj) -> Result<Self> {
                    <$type<T>>::try_from(&robj)
                }
            }

            impl<T> TryFrom<&Robj> for Option<$type<T>>
            where
                Robj: for<'a> AsTypedSlice<'a, T>,
            {
                type Error = Error;

                fn try_from(robj: &Robj) -> Result<Self> {
                    if robj.is_null() || robj.is_na() {
                        Ok(None)
                    } else {
                        Ok(Some(<$type<T>>::try_from(robj)?))
                    }
                }
            }

            impl<T> TryFrom<Robj> for Option<$type<T>>
            where
                Robj: for<'a> AsTypedSlice<'a, T>,
            {
                type Error = Error;

                fn try_from(robj: Robj) -> Result<Self> {
                    <Option::<$type<T>>>::try_from(&robj)
                }
            }
        )*
    }
}

impl_try_from_robj_ref!(
    RMatrix
    RColumn
    RMatrix3D
);

impl<T, D> From<RArray<T, D>> for Robj {
    /// Convert a column, matrix or matrix3d to an Robj.
    fn from(array: RArray<T, D>) -> Self {
        array.robj
    }
}

pub trait MatrixConversions: GetSexp {
    fn as_column<E>(&self) -> Option<RColumn<E>>
    where
        Robj: for<'a> AsTypedSlice<'a, E>,
    {
        <RColumn<E>>::try_from(self.as_robj()).ok()
    }

    fn as_matrix<E>(&self) -> Option<RMatrix<E>>
    where
        Robj: for<'a> AsTypedSlice<'a, E>,
    {
        <RMatrix<E>>::try_from(self.as_robj()).ok()
    }

    fn as_matrix3d<E>(&self) -> Option<RMatrix3D<E>>
    where
        Robj: for<'a> AsTypedSlice<'a, E>,
    {
        <RMatrix3D<E>>::try_from(self.as_robj()).ok()
    }
}

impl MatrixConversions for Robj {}

impl<T> Index<[usize; 2]> for RArray<T, [usize; 2]>
where
    Robj: for<'a> AsTypedSlice<'a, T>,
{
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
        unsafe {
            self.data()
                .as_ptr()
                .add(self.offset(index))
                .as_ref()
                .unwrap()
        }
    }
}

impl<T> IndexMut<[usize; 2]> for RArray<T, [usize; 2]>
where
    Robj: for<'a> AsTypedSlice<'a, T>,
{
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
        unsafe {
            self.data_mut()
                .as_mut_ptr()
                .add(self.offset(index))
                .as_mut()
                .unwrap()
        }
    }
}

impl<T, D> Deref for RArray<T, D> {
    type Target = Robj;

    fn deref(&self) -> &Self::Target {
        &self.robj
    }
}

impl<T, D> DerefMut for RArray<T, D> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.robj
    }
}

impl<T, D> From<Option<RArray<T, D>>> for Robj {
    fn from(value: Option<RArray<T, D>>) -> Self {
        match value {
            None => nil_value(),
            Some(value) => value.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate as extendr_api;
    use extendr_engine::with_r;
    use prelude::{Rcplx, Rfloat, Rint};

    #[test]
    fn test_empty_matrix_new() {
        with_r(|| {
            // These are arbitrarily filled. We cannot create assertions for them.
            // let m: RMatrix<Rbyte> = RMatrix::new(5, 2); //   Error: Error: unimplemented type 'char' in 'eval'
            // unsafe { Rf_PrintValue(m.get()) };
            let m: RMatrix<Rbool> = RMatrix::new(5, 2);
            unsafe { Rf_PrintValue(m.get()) };
            let m: RMatrix<Rint> = RMatrix::new(5, 2);
            unsafe { Rf_PrintValue(m.get()) };
            let m: RMatrix<Rfloat> = RMatrix::new(5, 2);
            unsafe { Rf_PrintValue(m.get()) };
            let m: RMatrix<Rcplx> = RMatrix::new(5, 2);
            unsafe { Rf_PrintValue(m.get()) };
            rprintln!();

            // let m: RMatrix<Rbyte> = RMatrix::new_with_na(10, 2); // not possible!
            // unsafe { Rf_PrintValue(m.get()) };
            let m: RMatrix<Rbool> = RMatrix::new_with_na(10, 2);
            assert_eq!(R!("matrix(NA, 10, 2)").unwrap(), m.into_robj());

            let m: RMatrix<Rint> = RMatrix::new_with_na(10, 2);
            assert_eq!(R!("matrix(NA_integer_, 10, 2)").unwrap(), m.into_robj());

            let m: RMatrix<Rfloat> = RMatrix::new_with_na(10, 2);
            assert_eq!(R!("matrix(NA_real_, 10, 2)").unwrap(), m.into_robj());

            let m: RMatrix<Rcplx> = RMatrix::new_with_na(10, 2);
            assert_eq!(R!("matrix(NA_complex_, 10, 2)").unwrap(), m.into_robj());
        });
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

    #[test]
    fn test_from_vec_doubles_to_matrix() {
        test! {
        // R: pracma::magic(5) -> x
        //    x[1:5**2]
        // Thus `res` is a list of col-vectors.
        let res: Vec<Doubles> = vec![
            vec![17.0, 23.0, 4.0, 10.0, 11.0].try_into().unwrap(),
            vec![24.0, 5.0, 6.0, 12.0, 18.0].try_into().unwrap(),
            vec![1.0, 7.0, 13.0, 19.0, 25.0].try_into().unwrap(),
            vec![8.0, 14.0, 20.0, 21.0, 2.0].try_into().unwrap(),
            vec![15.0, 16.0, 22.0, 3.0, 9.0].try_into().unwrap(),
        ];
        let (n_x, n_y) = (5, 5);
        let _matrix = RMatrix::new_matrix(n_x, n_y, |r, c| res[c][r]);

        }
    }
}
