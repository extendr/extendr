//!
//! Show data extraction and creation methods.
//! See tests/test_data.R for inputs.

use extendr_api::*;


#[extendr]
fn data(input: Robj) -> bool {
    let names = input.getAttrib(&Robj::namesSymbol());
    for (key, value) in names.str_iter().unwrap().zip(input.list_iter().unwrap()) {
        println!("n={} v={:?}", key, value);
    }
    true
}

// Macro to generate exports
extendr_module! {
    mod data;
    fn data;
}
