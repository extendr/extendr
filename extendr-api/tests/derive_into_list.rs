// TODO: On a crate with `proc_macro = true`, Rust doesn't allow
// cross-compilation, so this test cannot be executed on Windows as long as it
// requires corss-compilation.
//
// c.f. https://github.com/extendr/extendr/issues/372
#[cfg(not(target_os = "windows"))]
#[test]
fn test_derive_list() {
    use extendr_api::prelude::*;
    use extendr_macros::{IntoRobj, TryFromRobj};

    test! {
        #[derive(TryFromRobj, IntoRobj, PartialEq, Debug)]
        struct Foo {
            a: u16,
            b: String,
            c: Vec<f64>,
            // This demonstrates the use of wrapper types
            d: List
        }

        // We define the objects "natively", ie the struct in Rust, and the list in R
        let native_rust = Foo {
            a: 5,
            b: String::from("bar"),
            c: vec![1., 2., 3.],
            d: list!(a = 1., b = true)
        };
        let native_r = R!("list(a = 5L, b = 'bar', c = as.double(1:3), d = list(a = 1, b = TRUE))").unwrap();

        // Check the R → Rust conversion using a Robj reference
        let converted_rust_borrow: Foo = (&native_r).try_into().unwrap();
        assert_eq!(&converted_rust_borrow, &native_rust);

        // Check the Rust → R conversion using a struct reference
        let converted_r_borrow: Robj = (&native_rust).into();
        assert!(converted_r_borrow.is_list());
        assert_eq!(&converted_r_borrow, &native_r);

        // Check the Rust → R → Rust round trip using references
        assert_eq!(&native_rust, &Foo::try_from(&converted_r_borrow).unwrap());

        // Check the R → Rust → R round trip using references
        assert_eq!(&native_r, &Robj::from(&converted_rust_borrow));

        // Check the R → Rust conversion with an owned Robj
        let converted_rust_owned: Foo = native_r.try_into().unwrap();
        assert_eq!(&converted_rust_owned, &converted_rust_borrow);

        // Check the Rust → R conversion with an owned struct
        let converted_r_owned: Robj = native_rust.into();
        assert_eq!(&converted_r_borrow, &converted_r_owned);
    }
}
