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
mod try_into_int_tests {
    use crate as extendr_api;
    use crate::conversions::try_into_int::{ConversionError, FloatToInt};
    use crate::{test, CanBeNA};

    type ConversionResult<T, E> = std::result::Result<T, E>;

    #[test]
    fn test_exact_zero() {
        let value = 0.0;
        let int_value: ConversionResult<i32, _> = value.try_into_int();
        assert_eq!(int_value, Ok(0));
    }

    #[test]
    fn test_exact_negative_zero() {
        let value = -0.0;
        let int_value: ConversionResult<i32, _> = value.try_into_int();
        assert_eq!(int_value, Ok(0));
    }

    #[test]
    fn large_value_overflow() {
        let value: f64 = 1.000000020000001e200;
        let int_value: ConversionResult<i32, _> = value.try_into_int();
        assert_eq!(int_value, Err(ConversionError::Overflow))
    }

    #[test]
    fn large_negative_value_underflow() {
        let value: f64 = -1.000000020000001e200;
        let int_value: ConversionResult<i32, _> = value.try_into_int();
        assert_eq!(int_value, Err(ConversionError::Underflow))
    }

    #[test]
    fn na_not_integerish() {
        // NA-checks are unavailable unless R is set up
        test! {
            let value: f64 = f64::na();
            let int_value: ConversionResult<i32, _> = value.try_into_int();
            assert_eq!(int_value, Err(ConversionError::NotIntegerish));
        }
    }

    #[test]
    fn fractional_not_integerish() {
        let value: f64 = 1.5;
        let int_value: ConversionResult<i32, _> = value.try_into_int();
        assert_eq!(int_value, Err(ConversionError::NotIntegerish))
    }

    #[test]
    fn negative_fractional_not_integerish() {
        let value: f64 = -1.5;
        let int_value: ConversionResult<i32, _> = value.try_into_int();
        assert_eq!(int_value, Err(ConversionError::NotIntegerish))
    }

    #[test]
    fn small_integerish_negative_to_unsigned_underflow() {
        let value: f64 = -1.0;
        let int_value: ConversionResult<u32, _> = value.try_into_int();
        assert_eq!(int_value, Err(ConversionError::Underflow))
    }

    #[test]
    fn integerish_converts_successfully() {
        let value: f64 = 42.0;

        assert_eq!(FloatToInt::<i128>::try_into_int(&value), Ok(42));
        assert_eq!(FloatToInt::<i64>::try_into_int(&value), Ok(42));
        assert_eq!(FloatToInt::<i32>::try_into_int(&value), Ok(42));
        assert_eq!(FloatToInt::<i16>::try_into_int(&value), Ok(42));
        assert_eq!(FloatToInt::<i8>::try_into_int(&value), Ok(42));

        assert_eq!(FloatToInt::<u128>::try_into_int(&value), Ok(42));
        assert_eq!(FloatToInt::<u64>::try_into_int(&value), Ok(42));
        assert_eq!(FloatToInt::<u32>::try_into_int(&value), Ok(42));
        assert_eq!(FloatToInt::<u16>::try_into_int(&value), Ok(42));
        assert_eq!(FloatToInt::<u8>::try_into_int(&value), Ok(42));
    }
}
