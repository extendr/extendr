use crate::{Error, List, Result, Robj};
use polars::prelude::*;
use std::convert::{TryFrom, TryInto};
impl TryFrom<&Series> for Robj {
    type Error = crate::Error;

    fn try_from(value: &Series) -> Result<Self> {
        match value.dtype() {
            DataType::UInt8 => Ok(value.u8().unwrap().cont_slice().unwrap().into()),
            DataType::UInt16 => Ok(value.u16().unwrap().cont_slice().unwrap().into()),
            DataType::UInt32 => Ok(value.u32().unwrap().cont_slice().unwrap().into()),
            DataType::UInt64 => Ok(value.u64().unwrap().cont_slice().unwrap().into()),
            DataType::Int8 => Ok(value.i8().unwrap().cont_slice().unwrap().into()),
            DataType::Int16 => Ok(value.i16().unwrap().cont_slice().unwrap().into()),
            DataType::Int32 => Ok(value.i32().unwrap().cont_slice().unwrap().into()),
            DataType::Int64 => Ok(value.i64().unwrap().cont_slice().unwrap().into()),
            DataType::Float32 => Ok(value.f32().unwrap().cont_slice().unwrap().into()),
            DataType::Float64 => Ok(value.f64().unwrap().cont_slice().unwrap().into()),
            DataType::Date => Ok(value.date().unwrap().cont_slice().unwrap().into()),
            DataType::Datetime => Ok(value.datetime().unwrap().cont_slice().unwrap().into()),
            DataType::Time => Ok(value.time().unwrap().cont_slice().unwrap().into()),
            _ => Err(Error::Other(
                "Can't convert arbitrary Rust types to Robj".into(),
            )),
        }
    }
}

impl TryFrom<DataFrame> for Robj {
    type Error = crate::Error;

    fn try_from(df: DataFrame) -> crate::Result<Self> {
        // Map each series into a tuple or an error
        let res: Result<Vec<(String, Robj)>> = df
            .get_columns()
            .into_iter()
            .map(|col| Ok((col.name().into(), col.try_into()?)))
            .collect();
        // If we didn't have any errors, convert each key/value tuple into a List entry
        res.map(|vec| List::from_pairs(vec).into())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Robj;
    use crate::test;
    use polars::prelude::*;
    use std::convert::TryInto;

    #[test]
    fn test_df_to_robj() {
        test! {
        let df = df!(
            "a" => [1, 2, 3],
            "b" => [1, 2, 3]
        ).unwrap();
        let _robj: Robj = df.try_into().unwrap();
        }
    }
}
