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
mod tests {
    use crate::CanBeNA;
    use crate::conversions::from_integerish::IntoIntegerish;

    #[test]
    fn try_into_integerish_overflow() {
        let value: f64 = 1.000000020000001e200;
        let int_value: Result<i128, _> = value.try_into_integerish();
        dbg! { &int_value };
    }

    #[test]
    fn try_into_integerish_underflow() {
        let value: f64 = -1.000000020000001e200;
        let int_value: Result<i128, _> = value.try_into_integerish();
        dbg! { &int_value };
    }
    
    #[test]
    fn comp_test() {
        let value: f64 = f64::na();
        let int_value: Result<i64, _> = value.try_into_integerish();
        dbg! { &int_value };
    }
}
