use extendr_api::prelude::*;

#[extendr(result = "list")]
fn result_list_ok() -> std::result::Result<i32, String> {
    Ok(123)
}

#[extendr(result = "list")]
fn result_list_err() -> std::result::Result<i32, String> {
    Err("list mode oops".to_string())
}

#[extendr(result = "condition")]
fn result_condition_ok() -> std::result::Result<i32, String> {
    Ok(321)
}

#[extendr(result = "condition")]
fn result_condition_err() -> std::result::Result<i32, String> {
    Err("condition mode oops".to_string())
}

extendr_module! {
    mod result_modes;
    fn result_list_ok;
    fn result_list_err;
    fn result_condition_ok;
    fn result_condition_err;
}
