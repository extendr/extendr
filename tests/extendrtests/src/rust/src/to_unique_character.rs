use extendr_api::prelude::*;

#[extendr]
fn to_unique_rstr(r_char_vec: Strings) -> usize {
    let capacity = r_char_vec.len();
    let mut seen: Vec<Rstr> = Vec::with_capacity(capacity);
    let mut n_unique = 0;
    for word in r_char_vec.into_iter().cloned() {
        if !seen.contains(&word) {
            n_unique += 1;
            seen.push(word);
        }
    }
    n_unique
}

#[extendr]
fn to_unique_str(r_char_vec: StrIter) -> usize {
    let capacity = r_char_vec.len();
    let mut seen: Vec<&str> = Vec::with_capacity(capacity);
    let mut n_unique = 0;
    for word in r_char_vec {
        if !seen.contains(&word) {
            n_unique += 1;
            seen.push(word);
        }
    }
    n_unique
}

extendr_module! {
    mod to_unique_character;
    fn to_unique_rstr;
    fn to_unique_str;
}
