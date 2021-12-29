use extendr_api::prelude::*;
use extendr_macros::IntoRList;

#[test]
fn test_derive_list() {
    test! {
        #[derive(IntoRList)]
        struct Foo {
            a: usize,
            b: String,
            c: Vec<f64>
        }

        let foo = Foo {
            a: 5,
            b: "bar".into(),
            c: vec![1., 2., 3.]
        };

        let list: Robj = foo.into();
        assert!(list.is_list());

        let comparison = R!("list(a = 5, b = 'bar', c = as.double(1:3))").unwrap();
        assert_eq!(list, comparison);
    }
}
