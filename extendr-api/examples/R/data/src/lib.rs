//!
//! Show data extraction and creation methods.
//! See tests/test_data.R for inputs.

use extendr_api::*;

#[extendr]
fn data(input: Robj) -> bool {
    let mut an_integer = 0;
    let mut a_number = 0.;
    let mut a_string = String::new();
    let mut a_bool = false;
    let mut a_list = Vec::new();
    let mut an_integer_array = Vec::new();
    let mut a_number_array = Vec::new();
    let mut a_string_array = Vec::new();
    let mut a_logical_array = Vec::new();

    if let Some(names_and_values) = input.namesAndValues() {
        for (key, value) in names_and_values {
            rprintln!("n={} v={:?}", key, value);
            match key {
                "an_integer" => {
                    if let Some(value) = value.as_integer() {
                        an_integer = value;
                    }
                }
                "a_number" => {
                    if let Some(value) = value.as_numeric() {
                        a_number = value;
                    }
                }
                "a_string" => {
                    if let Some(value) = value.as_str() {
                        a_string = value.to_string();
                    }
                }
                "a_bool" => {
                    if let Some(value) = value.as_bool() {
                        a_bool = value
                    }
                }
                "a_list" => {
                    if let Some(value) = value.list_iter() {
                        a_list = value.collect::<Vec<_>>();
                    }
                }
                "an_integer_array" => {
                    if let Some(value) = value.as_integer_vector() {
                        an_integer_array = value;
                    }
                }
                "a_number_array" => {
                    if let Some(value) = value.as_numeric_vector() {
                        a_number_array = value;
                    }
                }
                "a_string_array" => {
                    if let Some(value) = value.str_iter() {
                        a_string_array = value.map(|s| s.to_string()).collect::<Vec<_>>();
                    }
                }
                "a_logical_array" => {
                    if let Some(value) = value.as_logical_vector() {
                        a_logical_array = value;
                    }
                }
                &_ => (),
            }
        }
    }

    rprintln!("an_integer={:?}", an_integer);
    rprintln!("a_number={:?}", a_number);
    rprintln!("a_string={:?}", a_string);
    rprintln!("a_bool={:?}", a_bool);
    rprintln!("a_list={:?}", a_list);
    rprintln!("an_integer_array={:?}", an_integer_array);
    rprintln!("a_number_array={:?}", a_number_array);
    rprintln!("a_string_array={:?}", a_string_array);
    rprintln!("a_logical_array={:?}", a_logical_array);

    an_integer == 123
        && a_number == 2.5
        && a_string == "hello"
        && a_bool == true
        && a_list == vec![r! {1}, r! {2}, r! {3}]
        && an_integer_array == vec![1, 2, 3]
        && a_number_array == vec![1., 2., 3.]
        && a_logical_array == vec![true, false, true]
}

// Macro to generate exports
extendr_module! {
    mod data;
    fn data;
}
