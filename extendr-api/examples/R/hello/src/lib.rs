use extendr_api::*;

#[extendr]
fn hello() -> &'static str {
    "hello"
}

// Macro to generate exports
extendr_module! {
    fn hello;
}
