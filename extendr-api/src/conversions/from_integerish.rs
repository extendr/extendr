use std::fmt::Debug;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum ConversionError {
    Underflow,
    Overflow,
    NotIntegerish,
}

pub(crate) trait IntoIntegerish<T : Sized> {
    fn try_into_integerish(&self) -> Result<T, ConversionError> where Self: Sized;
}

macro_rules! impl_into_integerish {
    ($float_type:ty, $int_type:ty) => {
        impl IntoIntegerish<$int_type> for $float_type {
            fn try_into_integerish(&self) -> Result<$int_type, ConversionError> {
                if !self.is_normal() {
                    return Err(ConversionError::NotIntegerish);
                }
                
                let truncated_value = self.trunc();
                if truncated_value < <$int_type>::MIN as $float_type {
                    return Err(ConversionError::Underflow);
                }
                if truncated_value > <$int_type>::MAX as $float_type {
                    return Err(ConversionError::Overflow);
                }
                if !truncated_value.eq(self) {
                    return Err(ConversionError::NotIntegerish);
                }
                Ok(truncated_value as $int_type)
            }
        }
    }
}

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

impl_into_integerish!(f32, i128);
impl_into_integerish!(f32, u128);
impl_into_integerish!(f32, i64);
impl_into_integerish!(f32, u64);
impl_into_integerish!(f32, i32);
impl_into_integerish!(f32, u32);
impl_into_integerish!(f32, i16);
impl_into_integerish!(f32, u16);
impl_into_integerish!(f32, i8);
impl_into_integerish!(f32, u8);

#[cfg(test)]
mod try_into_integerish_tests {
    mod f64_source {
        use crate::CanBeNA;
        use crate::conversions::from_integerish::{ConversionError, IntoIntegerish};

        #[test]
        fn large_value_overflow() {
            let value: f64 = 1.000000020000001e200;
            let int_value: Result<i128, _> = value.try_into_integerish();
            assert_eq!(int_value, Err(ConversionError::Overflow))
        }

        #[test]
        fn large_negative_value_underflow() {
            let value: f64 = -1.000000020000001e200;
            let int_value: Result<i128, _> = value.try_into_integerish();
            assert_eq!(int_value, Err(ConversionError::Underflow))
        }

        #[test]
        fn na_not_integerish() {
            let value: f64 = f64::na();
            let int_value: Result<i128, _> = value.try_into_integerish();
            assert_eq!(int_value, Err(ConversionError::NotIntegerish))
        }

        #[test]
        fn fractional_not_integerish() {
            let value: f64 = 1.5;
            let int_value: Result<i128, _> = value.try_into_integerish();
            assert_eq!(int_value, Err(ConversionError::NotIntegerish))
        }

        #[test]
        fn negative_fractional_not_integerish() {
            let value: f64 = -1.5;
            let int_value: Result<i128, _> = value.try_into_integerish();
            assert_eq!(int_value, Err(ConversionError::NotIntegerish))
        }

        #[test]
        fn small_integerish_negative_to_unsigned_underflow() {
            let value: f64 = -1.0;
            let int_value: Result<u128, _> = value.try_into_integerish();
            assert_eq!(int_value, Err(ConversionError::Underflow))
        }

        #[test]
        fn integerish_converts_successfully()
        {
            let value: f64 = 42.0;

            assert_eq!(IntoIntegerish::<i128>::try_into_integerish(&value), Ok(42));
            assert_eq!(IntoIntegerish::<i64>::try_into_integerish(&value), Ok(42));
            assert_eq!(IntoIntegerish::<i32>::try_into_integerish(&value), Ok(42));
            assert_eq!(IntoIntegerish::<i16>::try_into_integerish(&value), Ok(42));
            assert_eq!(IntoIntegerish::<i8>::try_into_integerish(&value), Ok(42));

            assert_eq!(IntoIntegerish::<u128>::try_into_integerish(&value), Ok(42));
            assert_eq!(IntoIntegerish::<u64>::try_into_integerish(&value), Ok(42));
            assert_eq!(IntoIntegerish::<u32>::try_into_integerish(&value), Ok(42));
            assert_eq!(IntoIntegerish::<u16>::try_into_integerish(&value), Ok(42));
            assert_eq!(IntoIntegerish::<u8>::try_into_integerish(&value), Ok(42));
        }
    }

    mod f32_source {
        use crate::CanBeNA;
        use crate::conversions::from_integerish::{ConversionError, IntoIntegerish};

        #[test]
        fn large_value_overflow() {
            let value: f32 = 1.000000020000001e38;
            let int_value: Result<i64, _> = value.try_into_integerish();
            assert_eq!(int_value, Err(ConversionError::Overflow))
        }

        #[test]
        fn large_negative_value_underflow() {
            let value: f32 = -1.000000020000001e38;
            let int_value: Result<i64, _> = value.try_into_integerish();
            assert_eq!(int_value, Err(ConversionError::Underflow))
        }

        #[test]
        fn na_not_integerish() {
            let value: f32 = f64::na() as f32;
            let int_value: Result<i64, _> = value.try_into_integerish();
            assert_eq!(int_value, Err(ConversionError::NotIntegerish))
        }

        #[test]
        fn fractional_not_integerish() {
            let value: f32 = 1.5;
            let int_value: Result<i64, _> = value.try_into_integerish();
            assert_eq!(int_value, Err(ConversionError::NotIntegerish))
        }

        #[test]
        fn small_integerish_negative_to_unsigned_underflow() {
            let value: f32 = -1.0;
            let int_value: Result<u64, _> = value.try_into_integerish();
            assert_eq!(int_value, Err(ConversionError::Underflow))
        }

        #[test]
        fn negative_fractional_not_integerish() {
            let value: f32 = -1.5;
            let int_value: Result<i64, _> = value.try_into_integerish();
            assert_eq!(int_value, Err(ConversionError::NotIntegerish))
        }

        #[test]
        fn integerish_converts_successfully()
        {
            let value: f32 = 42.0;

            assert_eq!(IntoIntegerish::<i128>::try_into_integerish(&value), Ok(42));
            assert_eq!(IntoIntegerish::<i64>::try_into_integerish(&value), Ok(42));
            assert_eq!(IntoIntegerish::<i32>::try_into_integerish(&value), Ok(42));
            assert_eq!(IntoIntegerish::<i16>::try_into_integerish(&value), Ok(42));
            assert_eq!(IntoIntegerish::<i8>::try_into_integerish(&value), Ok(42));

            assert_eq!(IntoIntegerish::<u128>::try_into_integerish(&value), Ok(42));
            assert_eq!(IntoIntegerish::<u64>::try_into_integerish(&value), Ok(42));
            assert_eq!(IntoIntegerish::<u32>::try_into_integerish(&value), Ok(42));
            assert_eq!(IntoIntegerish::<u16>::try_into_integerish(&value), Ok(42));
            assert_eq!(IntoIntegerish::<u8>::try_into_integerish(&value), Ok(42));
        }
    }
}
