use extendr_api::prelude::*;

#[extendr]
fn custom_error_return() -> std::result::Result<(), std::io::Error> {
    Ok(())
}

struct A;

impl TryFrom<Robj> for A {
    type Error = std::io::Error;

    fn try_from(_value: Robj) -> std::result::Result<Self, Self::Error> {
        Ok(A)
    }
}

#[extendr]
fn custom_error_conversion(_val: A) -> std::result::Result<(), std::io::Error> {
    Ok(())
}

extendr_module! {
    mod custom_errors;
    fn custom_error_return;
    fn custom_error_conversion;
}
