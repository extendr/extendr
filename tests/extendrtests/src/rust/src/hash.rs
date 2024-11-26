use std::collections::HashSet;

use extendr_api::prelude::*;

extendr_module! {
    mod hash;

    fn to_unique_hs_const;
    fn to_unique_hs_str;
    fn to_unique_hs_rstr;
}

#[extendr]
fn to_unique_hs_const(x: Strings) -> usize {
    let mut hs = HashSet::new();
    for word in x.into_iter() {
        hs.insert(word.as_ptr());
    }
    hs.len()
}

#[extendr]
fn to_unique_hs_rstr(x: Strings) -> usize {
    let mut hs = HashSet::new();
    for word in x.into_iter() {
        hs.insert(word);
    }
    hs.len()
}

#[extendr]
fn to_unique_hs_str(r_char_vec: StrIter) -> usize {
    let mut hs = HashSet::new();
    for word in r_char_vec.into_iter() {
        hs.insert(word.as_ptr());
    }
    hs.len()
}
