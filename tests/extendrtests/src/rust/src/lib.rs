use extendr_api::*;

#[extendr]
fn hello() -> &'static str {
    "hello"
}

/* Doesn't currently work yet.
// Macro to generate exports
extendr_module! {
    mod extendrtests;
    fn hello;
}
*/