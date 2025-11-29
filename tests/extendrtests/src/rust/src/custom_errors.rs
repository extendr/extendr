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

#[derive(IntoList, TryFromList)]
struct B(f64);

#[extendr]
#[allow(non_snake_case)]
fn take_and_return_B(mut b: B) -> B {
    b.0 += 1.;
    b
}

extendr_module! {
    mod custom_errors;
    fn custom_error_return;
    fn custom_error_conversion;
    fn take_and_return_B;
}
