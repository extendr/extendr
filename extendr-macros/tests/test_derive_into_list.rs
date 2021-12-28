use extendr_macros::IntoRList;
use extendr_api::prelude::*;

#[test]
fn test_derive_list(){
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

    let robj: Robj = foo.into();
    dbg!(&robj.class());
}
