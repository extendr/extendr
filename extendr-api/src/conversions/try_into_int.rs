use std::fmt::{Debug, Display, Formatter};
use std::num::FpCategory;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ConversionError {
    Underflow,
    Overflow,
    NotIntegerish,
}

impl Display for ConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConversionError::Underflow => write!(f, "underflow"),
            ConversionError::Overflow => write!(f, "overflow"),
            ConversionError::NotIntegerish => write!(f, "not a whole number"),
        }
    }
}

pub(crate) trait FloatToInt<T: Sized> {
    fn try_into_int(&self) -> Result<T, ConversionError>
    where
        Self: Sized;
}

macro_rules! impl_into_integerish {
    ($float_type:ty, $int_type:ty) => {
        impl FloatToInt<$int_type> for $float_type {
            fn try_into_int(&self) -> Result<$int_type, ConversionError> {
                match self.classify() {
                    FpCategory::Nan | FpCategory::Subnormal => Err(ConversionError::NotIntegerish),
                    FpCategory::Zero => Ok(<$int_type>::default()),
                    FpCategory::Infinite if self.is_sign_positive() => {
                        Err(ConversionError::Overflow)
                    }
                    FpCategory::Infinite => Err(ConversionError::Underflow),
                    FpCategory::Normal => {
                        let truncated_value = self.trunc();
                        const MIN_VALUE: $float_type = <$int_type>::MIN as $float_type;
                        if truncated_value < MIN_VALUE {
                            return Err(ConversionError::Underflow);
                        }
                        const MAX_VALUE: $float_type = <$int_type>::MAX as $float_type;
                        if truncated_value > MAX_VALUE {
                            return Err(ConversionError::Overflow);
                        }
                        if !truncated_value.eq(self) {
                            return Err(ConversionError::NotIntegerish);
                        }
                        return Ok(truncated_value as $int_type);
                    }
                }
            }
        }
    };
}

impl_into_integerish!(f64, isize);
impl_into_integerish!(f64, usize);
impl_into_integerish!(f64, i128);
impl_into_integerish!(f64, u128);
impl_into_integerish!(f64, i64);
impl_into_integerish!(f64, u64);
impl_into_integerish!(f64, i32);
impl_into_integerish!(f64, u32);
impl_into_integerish!(f64, i16);
impl_into_integerish!(f64, u16);
impl_into_integerish!(f64, i8);
impl_into_integerish!(f64, u8);

#[cfg(test)]
mod try_into_int_tests;
