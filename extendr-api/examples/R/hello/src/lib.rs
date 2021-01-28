use extendr_api::prelude::*;

#[extendr]
fn hello() -> &'static str {
    "hello"
}

// Macro to generate exports
extendr_module! {
    mod hello;
    fn hello;
}
