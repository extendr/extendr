use extendr_api::prelude::*;

#[extendr]
fn sum_many_args_same_type(x: &i32, y: &i32, z: &i32) {
    let _result = x + y + z;
}

extendr_module! {
    mod field_errors;

    fn sum_many_args_same_type;
}
