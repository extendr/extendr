// use extendr_api::prelude::*;
use extendr_api::{list, test, List, Result};

#[test]
fn test_list() {
    test! {
        let l : List = list!();
        assert_eq!(l, List::default());
        let l : List = list!(1);
        assert_eq!(l, List::from_values([1]));
        let l : List = list!(a=1);
        assert_eq!(l, List::from_names_and_values(["a"], [1]));
        let l : List = list!(a=1, b=2);
        assert_eq!(l, List::from_names_and_values(["a", "b"], [1, 2]));
    }
}
