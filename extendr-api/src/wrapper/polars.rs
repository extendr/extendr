use std::convert::TryFrom;
use polars::prelude::*;
use crate::{List, Robj, RobjItertools, ToVectorValue};

impl<'a, T> From<&'a ChunkedArray<T>> for Robj
where
    T: PolarsNumericType,
      &'a<T as PolarsNumericType>::Native: ToVectorValue
{
    fn from(ca: &'a ChunkedArray<T>) -> Self {
        ca.cont_slice().unwrap().iter().collect_robj()
    }
}

impl TryFrom<DataFrame> for Robj {
    type Error = crate::Error;

    fn try_from(df: DataFrame) -> crate::Result<Self> {
        Ok(List::from_pairs(df.get_columns().into_iter().map(|col| {
            (col.name(), col.unpack().unwrap().into())
        })).into())
    }
}

impl From<Robj> for DataFrame {
    fn from(_: Robj) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;
    use polars::prelude::*;
    use crate::test;
    use crate::prelude::{Robj};

    #[test]
    fn test_df_to_robj(){
        test! {
        let df = df!(
            "a" => [1, 2, 3],
            "b" => [1, 2, 3]
        );
        let robj: Robj = df.try_into().unwrap();
        }
    }
}