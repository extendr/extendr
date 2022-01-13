use extendr_api::prelude::*;
use extendr_macros::IntoRList;

#[test]
fn test_derive_list() {
    test! {
        #[derive(IntoRList, PartialEq, Debug)]
        struct Foo {
            a: u16,
            b: String,
            c: Vec<f64>,
            // This demonstrates the use of wrapper types
            d: List
        }

        let native_rust = &Foo {
            a: 5,
            b: String::from("bar"),
            c: vec![1., 2., 3.],
            d: list!(a = 1., b = true)
        };

        // Check the R → Rust conversion
        let native_r = &R!("list(a = 5L, b = 'bar', c = as.double(1:3), d = list(a = 1, b = TRUE))").unwrap();
        let converted_rust: &Foo = &native_r.try_into().unwrap();
        assert_eq!(&converted_rust, &native_rust);

        // Check the Rust → R conversion
        let converted_r: &Robj = &native_rust.into();
        assert!(converted_r.is_list());
        assert_eq!(converted_r, native_r);

        // Check the Rust → R → Rust round trip
        assert_eq!(native_rust, &Foo::try_from(converted_r).unwrap());

        // Check the R → Rust → R round trip
        assert_eq!(native_r, &Robj::from(converted_rust));
    }
}
