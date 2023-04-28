//! Test cases for this were made by @JosiahParry.
//! 
//!
use extendr_api::prelude::*;

// #[extendr]
fn protect_lim2(n: i32) -> List {
    #[derive(Debug)]
    struct PlzBreak(i32);

    let n = n as usize;

    (0..n)
        .into_iter()
        .map(|xi| ExternalPtr::new(PlzBreak(xi as i32)))
        .collect::<List>()
}

// #[extendr]
fn prot_strs(n: i32) -> Strings {
    let n = n as usize;
    (0..n)
        .into_iter()
        .map(|_| Rstr::from_string("val"))
        .collect::<Strings>()
}

#[test]
fn test_from_iterator_collection() {
    test!(
        let s = prot_strs(10_000 * 7);
        println!("Hello: {}", s[54]);
        protect_lim2(10_000 * 7);
    )
}
