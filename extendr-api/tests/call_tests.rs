use extendr_api::prelude::*;

#[test]
fn special_test() {
    test! {
        assert_eq!(call!("`+`", 1, 2)?, r!(3));
        let add = R!("`+`")?;
        assert!(add.is_primitive());
        assert_eq!(add.call(pairlist!(1, 2))?, r!(3));
    }
}
