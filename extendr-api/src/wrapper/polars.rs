use arrow2::error::ArrowError;
use std::convert::TryFrom;

use arrow2::ffi::*;
use polars::error::PolarsError;
use polars::prelude::{df, ArrowField, DataFrame, NamedFrom, Series};

use extendr_macros::{call, R};

use crate as extendr_api;
use crate::prelude::*;
use crate::wrapper::List;
use ListIter;

impl From<polars::error::PolarsError> for crate::Error {
    fn from(polars_err: PolarsError) -> Self {
        Error::Other(polars_err.to_string())
    }
}

impl From<polars::error::ArrowError> for crate::Error {
    fn from(polars_err: ArrowError) -> Self {
        Error::Other(polars_err.to_string())
    }
}

impl TryFrom<DataFrame> for Robj {
    type Error = crate::Error;

    fn try_from(value: DataFrame) -> crate::Result<Self> {
        // Check if the `arrow` package is installed
        let pkg: &str = R!("system.file(package='arrow')")?.try_into()?;
        if pkg.is_empty() {
            return Err(Error::Other(
                "The arrow package needs to be installed".into(),
            ));
        }

        let (_nrow, ncol) = value.shape();
        let mut pairs = Vec::<(String, Robj)>::with_capacity(ncol);

        // Make a list of name/arrow array pairs
        for series in value.get_columns() {
            // We push the arrays onto the heap and get a pointer to them
            let array_ptr = Box::into_raw(Box::new(Ffi_ArrowArray::empty()));
            let schema_ptr = Box::into_raw(Box::new(Ffi_ArrowSchema::empty()));

            // Rechunk the array so it's all contiguous
            let array_ref = series.rechunk().to_arrow(0);
            let field = &ArrowField::new(series.name(), array_ref.data_type().clone(), true);
            unsafe {
                export_array_to_c(array_ref, array_ptr);
                export_field_to_c(field, schema_ptr);
            }
            let imported_arr = call!(
                "arrow::Array$import_from_c",
                array_ptr as usize,
                schema_ptr as usize
            )?;
            pairs.push((series.name().into(), imported_arr));
        }

        // Pass this list into the record_batch constructor
        let list = List::from_pairs(pairs);
        let table = call!("do.call", R!("arrow::record_batch"), list)?;
        Ok(table)
    }
}

impl TryFrom<Robj> for DataFrame {
    type Error = crate::Error;

    fn try_from(value: Robj) -> Result<Self> {
        // Check the input is the right class
        if !value.class().unwrap().any(|it| it == "RecordBatch") {
            return Err(Error::TypeMismatch(value));
        }

        // Prepare a vector of output Series
        let ncol: usize = u16::try_from(value.dollar("num_columns")?)? as usize;
        let mut series = Vec::<Series>::with_capacity(ncol);

        // Get iterators over the columns and column names
        let names = value
            .dollar("names")?
            .call(pairlist!())?
            .as_str_iter()
            .ok_or(Error::Other("Could not iterate column names list".into()))?;
        let cols = ListIter::try_from(value.dollar("columns")?)?;

        // Iterate name/column pairs
        for (name, column) in names.zip(cols) {
            // Use the C API to move the data from Rust to R
            let array = Box::new(Ffi_ArrowArray::empty());
            let array_ptr = (&*array as *const Ffi_ArrowArray) as usize;

            let schema = Box::new(Ffi_ArrowSchema::empty());
            let schema_ptr = (&*schema as *const Ffi_ArrowSchema) as usize;

            column
                .dollar("export_to_c")?
                .call(pairlist!(array_ptr, schema_ptr))?;

            let field;
            unsafe {
                field = import_field_from_c(&schema).map_err(|err| Error::from(err))?;
            }

            let arr;
            unsafe {
                arr = import_array_from_c(array, &field).map_err(|err| Error::from(err))?;
            }

            series.push(Series::try_from((name, arr)).map_err(|err| Error::from(err))?);
        }

        Ok(DataFrame::new(series).map_err(|e| Error::from(e))?)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test;
    use polars::prelude::DataType;

    fn assert_true_r(expr: Result<Robj>) {
        assert!(bool::try_from(expr.unwrap()).unwrap())
    }

    #[test]
    fn test_df_to_robj() {
        test! {
            let df = df!(
                "a" => &[1, 2, 3, 4],
                "b" => &[1., 2., 3., 4.],
                "c" => &["1", "2", "3", "4"]
            ).unwrap();
            let df_robj: &Robj = &df.try_into().unwrap();

            // Check for the right subclass
            assert_true_r(R!("'RecordBatch' %in% class({{df_robj}})"));

            // These are all R6 methods that can only work if the data frame was properly
            // converted: https://arrow.apache.org/docs/r/reference/RecordBatch.html#r-methods
            assert_true_r(R!("{{df_robj}}$num_rows == 4"));
            assert_true_r(R!("{{df_robj}}$num_columns == 3"));
            assert_true_r(R!("all({{df_robj}}$names() == c('a', 'b', 'c'))"));
        }
    }

    #[test]
    fn test_robj_to_df() {
        test! {
            let r_df = R!(r#"
            arrow::record_batch(
                a = 1:3,
                b = c(1., 2., 3.),
                c = c("a", "b", "c"),
                d = c(TRUE, FALSE, TRUE)
            )
            "#).unwrap();
            let df = DataFrame::try_from(r_df).unwrap();

            // 3 rows and 4 cols
            assert_eq!(df.shape(), (3, 4));

            // Check types
            assert_eq!(df.column("a").unwrap().dtype(), &DataType::Int32);
            assert_eq!(df.column("b").unwrap().dtype(), &DataType::Float64);
            assert_eq!(df.column("c").unwrap().dtype(), &DataType::Utf8);
            assert_eq!(df.column("d").unwrap().dtype(), &DataType::Boolean);

            // Check indexing
            assert_eq!(df.slice(2, 1), df!(
                "a" => [3],
                "b" => [3.],
                "c" => ["c"],
                "d" => [true],
            ).unwrap());
        }
    }
}
