use extendr_api::*;

#[extendr]
fn hello() -> &'static str {
    "hello"
}

#[extendr]
fn add_ints(x:i32, y:i32) -> i32 {
    x + y
}


/* Doesn't currently work yet.
// Macro to generate exports
extendr_module! {
    mod extendrtests;
    fn hello;
}
*/
