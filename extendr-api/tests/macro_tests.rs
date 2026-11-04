use extendr_api::{list, reprint, reprintln, rprint, rprintln, test, List};

#[test]
fn test_list() {
    test! {
        let l : List = list!();
        assert_eq!(l, List::default());
        let l : List = list!(1);
        assert_eq!(l, List::from_values([1]));
        let l : List = list!(a=1);
        assert_eq!(l, List::from_names_and_values(["a"], [1]).unwrap());
        let l : List = list!(a=1, b=2);
        assert_eq!(l, List::from_names_and_values(["a", "b"], [1, 2]).unwrap());

        assert!(List::from_names_and_values(["a", "b"], [1, 2, 3]).is_err());
    }
}

#[test]
fn test_print_interior_nul() {
    test! {
        rprint!("a\0b");
        rprintln!("a\0b");
        reprint!("a\0b");
        reprintln!("a\0b");
    }
}
