use extendr_api::list;
use extendr_api::wrapper::{List, Strings};

fn get_strings() -> Strings {
    Strings::from_values((0..10).map(|i| format!("number {}", i)))
}

fn get_named_list() -> List {
    list!(x = 1, y = "xyz", z = ())
}

fn get_unnamed_list() -> List {
    List::from_values(0..10)
}

// strings: ["number 0", "number 1", "number 2", "number 3", "number 4", "number 5", "number 6", "number 7", "number 8", "number 9"]
// named list: list!(x=1, y=["xyz"], z=())
// unnamed list: list!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9)

fn main() {
    use extendr_api::{test, Result};

    test! {
        let s = get_strings();
        println!("strings: {:?}", s);

        let ln = get_named_list();
        println!("named list: {:?}", ln);

        let lu = get_unnamed_list();
        println!("unnamed list: {:?}", lu);
    }
}
