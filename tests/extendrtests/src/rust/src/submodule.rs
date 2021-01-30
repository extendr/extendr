use extendr_api::prelude::*;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_submodule() -> &'static str {
    "Hello World!"
}

// Macro to generate exports
extendr_module! {
    mod submodule;
    fn hello_submodule;
}
