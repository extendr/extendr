use extendr_api::prelude::*;
use std::collections::HashSet;

/// Accept a character vector from R and return the unique values.
#[extendr]
fn receive_hashset(values: HashSet<&str>) -> Vec<String> {
    let mut values: Vec<String> = values.into_iter().map(|s| s.to_string()).collect();
    values.sort();
    values
}

extendr_module! {
    mod hashset;
    fn receive_hashset;
}
