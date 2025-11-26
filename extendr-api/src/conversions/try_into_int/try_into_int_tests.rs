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
