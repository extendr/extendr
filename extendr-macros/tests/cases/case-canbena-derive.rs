use extendr_api::prelude::*;
use extendr_macros::CanBeNA;

#[derive(CanBeNA)]
struct TwoFields(Rint, Rint);

#[derive(CanBeNA)]
enum BadEnum {
    A(Rint),
}

fn main() {}
