use extendr_api::{list, test, List};

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
fn test_macros_without_prelude() {
    use extendr_api::{Attributes, Length, Robj, Symbol};
    test! {
        assert_eq!(extendr_api::sym!(wombat), Robj::from(Symbol::from_string("wombat")));
        assert_eq!(extendr_api::lang!("c").len(), 1);
        assert_eq!(extendr_api::lang!("c", 1, b = 2).len(), 3);
        assert!(extendr_api::data_frame!().inherits("data.frame"));
        assert!(extendr_api::data_frame!(x = 1).inherits("data.frame"));
        assert!(extendr_api::factor!(["a", "b"]).inherits("factor"));
        extendr_api::rprint!("");
        extendr_api::rprintln!();
        extendr_api::reprint!("");
        extendr_api::reprintln!();
    }
}
