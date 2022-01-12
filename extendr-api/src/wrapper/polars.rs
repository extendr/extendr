use std::convert::TryFrom;

use arrow2::ffi::*;
use polars::prelude::{df, ArrowField, DataFrame, NamedFrom, Series};

use extendr_macros::{call, R};

use crate as extendr_api;
use crate::prelude::*;
use crate::wrapper::List;

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

#[cfg(test)]
mod test {
    use super::*;
    use crate::test;

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
}
