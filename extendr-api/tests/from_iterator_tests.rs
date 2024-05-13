//! Test cases for this were made by @JosiahParry.
//!
//!
use extendr_api::prelude::*;

// #[extendr]
fn protect_lim2(n: i32) -> List {
    #[derive(Debug)]
    #[allow(dead_code)]
    struct PlzBreak(i32);

    let n = n as usize;

    (0..n)
        .map(|xi| ExternalPtr::new(PlzBreak(xi as i32)))
        .collect::<List>()
}

// #[extendr]
fn prot_strs(n: i32) -> Strings {
    let n = n as usize;
    (0..n)
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

#[test]
fn test_with_gc_torture_small() {
    test!(
        let x = vec![1, 4, 5, 6];
        R!("gctorture(on = TRUE)")?;
        let list: List = x.into_iter().collect();
        R!("gctorture(on = FALSE)")?;
        assert_eq!(list, list!(1, 4, 5, 6));
    );
}

#[test]
fn test_with_gc_torture_large() {
    test!(
        let x = [0_f64; 150].map(|_|single_threaded(||unsafe {libR_sys::Rf_runif(0., 100.)}));
        R!("gctorture(on = TRUE)")?;
        let list: List = x.into_iter().collect();
        R!("gctorture(on = FALSE)")?;
        assert_eq!(list, List::from_values(x));
    );
}

#[test]
fn test_with_gc_torture_strings() {
    test!(
        let question_quote = ["the","answer","to", "the", "ultimate", "question"];
        R!("gctorture(on = TRUE)")?;
        let qq_r_character_vec: Strings = question_quote.into_iter().collect();
        R!("gctorture(on = FALSE)")?;
        // Strings::from_values is the same as `.collect`.
        let qq_directly = Strings::try_from(&Robj::from(question_quote)).unwrap();
        assert_eq!(qq_r_character_vec, qq_directly);
    );
}
