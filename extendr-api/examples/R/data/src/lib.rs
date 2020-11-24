//!
//! Show data extraction and creation methods.
//! See tests/test_data.R for inputs.

use extendr_api::*;


#[extendr]
fn data(input: Robj) -> bool {
    let mut an_integer = 0;
    let mut a_number = 0.;
    let mut a_string = String::new();
    let a_bool = false;
    let a_list = ListIter::new();
    let an_integer_array = Vec::new();
    let a_number_array = Vec::new();
    let a_string_array = StrIter::new();
    //let a_logical_array = Vec::new();

    if let Some(names_and_values) = input.namesAndValues() {
        for (key, value) in names_and_values {
            println!("n={} v={:?}", key, value);
            match key {
                "an_integer" => { an_integer = <i32>::from_robj(&value).unwrap(); }
                "a_number" => { a_number = <f64>::from_robj(&value).unwrap(); }
                "a_string" => { a_string = <String>::from_robj(&value).unwrap(); }
                "a_bool" => { a_bool = <bool>::from_robj(&value).unwrap(); }
                "a_list" => { a_list = <ListIter>::from_robj(&value).unwrap(); }
                "an_integer_array" => { an_integer_array = <Vec<i32>>::from_robj(&value).unwrap(); }
                "a_number_array" => { a_number_array = <Vec<f64>>::from_robj(&value).unwrap(); }
                "a_string_array" => { a_string_array = value.str_iter().unwrap(); }
                //"a_logical_array" => { a_logical_array = Vec<bool>::from_robj(&value).unwrap(); }
            }
        }
    }
    true
}

// Macro to generate exports
extendr_module! {
    mod data;
    fn data;
}
