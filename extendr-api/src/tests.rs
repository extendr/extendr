use super::prelude::*;
use crate as extendr_api;

use extendr_macros::extendr;
use extendr_macros::extendr_module;
use extendr_macros::pairlist;

#[extendr]
pub fn inttypes(a: i8, b: u8, c: i16, d: u16, e: i32, f: u32, g: i64, h: u64) {
    assert_eq!(a, 1);
    assert_eq!(b, 2);
    assert_eq!(c, 3);
    assert_eq!(d, 4);
    assert_eq!(e, 5);
    assert_eq!(f, 6);
    assert_eq!(g, 7);
    assert_eq!(h, 8);
}

#[extendr]
pub fn floattypes(a: f32, b: f64) {
    assert_eq!(a, 1.);
    assert_eq!(b, 2.);
}

#[extendr]
pub fn strtypes(a: &str, b: String) {
    assert_eq!(a, "abc");
    assert_eq!(b, "def");
}

#[extendr]
pub fn vectortypes(a: Vec<i32>, b: Vec<f64>) {
    assert_eq!(a, [1, 2, 3]);
    assert_eq!(b, [4., 5., 6.]);
}

#[extendr]
pub fn robjtype(a: Robj) {
    assert_eq!(a, Robj::from(1))
}

#[extendr]
pub fn return_u8() -> u8 {
    123
}

#[extendr]
pub fn return_u16() -> u16 {
    123
}

#[extendr]
pub fn return_u32() -> u32 {
    123
}

#[extendr]
pub fn return_u64() -> u64 {
    123
}

#[extendr]
pub fn return_i8() -> i8 {
    123
}

#[extendr]
pub fn return_i16() -> i16 {
    123
}

#[extendr]
pub fn return_i32() -> i32 {
    123
}

#[extendr]
pub fn return_i64() -> i64 {
    123
}

#[extendr]
pub fn return_f32() -> f32 {
    123.
}

#[extendr]
pub fn return_f64() -> f64 {
    123.
}

#[extendr]
pub fn f64_slice(x: &[f64]) -> &[f64] {
    x
}

#[extendr]
pub fn i32_slice(x: &[i32]) -> &[i32] {
    x
}

#[extendr]
pub fn bool_slice(x: &[Rbool]) -> &[Rbool] {
    x
}

#[extendr]
pub fn f64_iter(x: Doubles) -> Doubles {
    x
}

#[extendr]
pub fn i32_iter(x: Integers) -> Integers {
    x
}

// #[extendr]
// pub fn bool_iter(x: Logicals) -> Logicals {
//     x
// }

#[extendr]
pub fn symbol(x: Symbol) -> Symbol {
    x
}

#[extendr]
pub fn matrix(x: RMatrix<f64>) -> RMatrix<f64> {
    x
}

struct Person {
    pub name: String,
}

#[extendr]
/// impl comment.
impl Person {
    fn new() -> Self {
        Self {
            name: "".to_string(),
        }
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

// see metadata_test for the following comments.

/// comment #1
/// comment #2
/**
    comment #3
    comment #4
**/
#[extendr]
/// aux_func doc comment.
fn aux_func(_person: &Person) {}

// Macro to generate exports
extendr_module! {
    mod my_module;
    fn aux_func;
    impl Person;
}

#[test]
fn export_test() {
    with_r(|| {
        use super::*;
        // Call the exported functions through their generated C wrappers.
        unsafe {
            wrap__inttypes(
                Robj::from(1).get(),
                Robj::from(2).get(),
                Robj::from(3).get(),
                Robj::from(4).get(),
                Robj::from(5).get(),
                Robj::from(6).get(),
                Robj::from(7).get(),
                Robj::from(8).get(),
            );
            wrap__inttypes(
                Robj::from(1.).get(),
                Robj::from(2.).get(),
                Robj::from(3.).get(),
                Robj::from(4.).get(),
                Robj::from(5.).get(),
                Robj::from(6.).get(),
                Robj::from(7.).get(),
                Robj::from(8.).get(),
            );
            wrap__floattypes(Robj::from(1.).get(), Robj::from(2.).get());
            wrap__floattypes(Robj::from(1).get(), Robj::from(2).get());
            wrap__strtypes(Robj::from("abc").get(), Robj::from("def").get());
            wrap__vectortypes(
                Robj::from(&[1, 2, 3] as &[i32]).get(),
                Robj::from(&[4., 5., 6.] as &[f64]).get(),
            );
            wrap__robjtype(Robj::from(1).get());

            // General integer types.
            assert_eq!(Robj::from_sexp(wrap__return_u8()), Robj::from(123_u8));
            assert_eq!(Robj::from_sexp(wrap__return_u16()), Robj::from(123));
            assert_eq!(Robj::from_sexp(wrap__return_u32()), Robj::from(123.));
            assert_eq!(Robj::from_sexp(wrap__return_u64()), Robj::from(123.));
            assert_eq!(Robj::from_sexp(wrap__return_i8()), Robj::from(123));
            assert_eq!(Robj::from_sexp(wrap__return_i16()), Robj::from(123));
            assert_eq!(Robj::from_sexp(wrap__return_i32()), Robj::from(123));
            assert_eq!(Robj::from_sexp(wrap__return_i64()), Robj::from(123.));

            // Floating point types.
            assert_eq!(Robj::from_sexp(wrap__return_f32()), Robj::from(123.));
            assert_eq!(Robj::from_sexp(wrap__return_f64()), Robj::from(123.));
        }
    });
}

#[test]
fn class_wrapper_test() {
    with_r(|| {
        let mut person = Person::new();
        person.set_name("fred");
        let robj = r!(person);
        assert_eq!(robj.check_external_ptr_type::<Person>(), true);
        let person2 = <&Person>::from_robj(&robj).unwrap();
        assert_eq!(person2.name(), "fred");
    });
}

#[test]
fn slice_test() {
    with_r(|| {
        unsafe {
            // #[extendr]
            // pub fn f64_slice(x: &[f64]) -> &[f64] { x }

            let robj = r!([1., 2., 3.]);
            assert_eq!(Robj::from_sexp(wrap__f64_slice(robj.get())), robj);

            // #[extendr]
            // pub fn i32_slice(x: &[i32]) -> &[i32] { x }

            let robj = r!([1, 2, 3]);
            assert_eq!(Robj::from_sexp(wrap__i32_slice(robj.get())), robj);

            // #[extendr]
            // pub fn bool_slice(x: &[Rbool]) -> &[Rbool] { x }

            let robj = r!([TRUE, FALSE, TRUE]);
            assert_eq!(Robj::from_sexp(wrap__bool_slice(robj.get())), robj);

            // #[extendr]
            // pub fn f64_iter(x: Doubles) -> Doubles { x }

            let robj = r!([1., 2., 3.]);
            assert_eq!(Robj::from_sexp(wrap__f64_iter(robj.get())), robj);

            // #[extendr]
            // pub fn i32_iter(x: Integers) -> Integers { x }

            let robj = r!([1, 2, 3]);
            assert_eq!(Robj::from_sexp(wrap__i32_iter(robj.get())), robj);

            // #[extendr]
            // pub fn bool_iter(x: Logicals) -> Logicals { x }

            // TODO: reinstate this test.
            // let robj = r!([TRUE, FALSE, TRUE]);
            // assert_eq!(Robj::from_sexp(wrap__bool_iter(robj.get())), robj);

            // #[extendr]
            // pub fn symbol(x: Symbol) -> Symbol { x }

            let robj = sym!(fred);
            assert_eq!(Robj::from_sexp(wrap__symbol(robj.get())), robj);

            // #[extendr]
            // pub fn matrix(x: Matrix<&[f64]>) -> Matrix<&[f64]> { x }

            let m = RMatrix::new_matrix(1, 2, |r, c| if r == c { 1.0 } else { 0. });
            let robj = r!(m);
            assert_eq!(Robj::from_sexp(wrap__matrix(robj.get())), robj);
        }
    });
}

#[test]
fn r_output_test() {
    // R equivalent
    // > txt_con <- textConnection("test_con", open = "w")
    // > sink(txt_con)
    // > cat("Hello world")
    // > sink()
    // > close(txt_con)
    // > expect_equal(test_con, "Hello world")
    //

    with_r(|| {
        let txt_con = R!(r#"textConnection("test_con", open = "w")"#).unwrap();
        call!("sink", &txt_con).unwrap();
        rprintln!("Hello world %%!"); //%% checks printf formatting is off, yields one % if on
        call!("sink").unwrap();
        call!("close", &txt_con).unwrap();
        let result = R!("test_con").unwrap();
        assert_eq!(result, r!("Hello world %%!"));
    });
}

#[test]
fn test_na_str() {
    assert_ne!(<&str>::na().as_ptr(), "NA".as_ptr());
    assert_eq!(<&str>::na(), "NA");
    assert_eq!("NA".is_na(), false);
    assert_eq!(<&str>::na().is_na(), true);
}

#[test]
fn metadata_test() {
    with_r(|| {
        // Rust interface.
        let metadata = get_my_module_metadata();
        assert_eq!(metadata.functions[0].doc, " comment #1\n comment #2\n\n        comment #3\n        comment #4\n    *\n aux_func doc comment.");
        assert_eq!(metadata.functions[0].rust_name, "aux_func");
        assert_eq!(metadata.functions[0].mod_name, "aux_func");
        assert_eq!(metadata.functions[0].r_name, "aux_func");
        assert_eq!(metadata.functions[0].args[0].name, "_person");
        assert_eq!(metadata.functions[1].rust_name, "get_my_module_metadata");
        assert_eq!(metadata.impls[0].name, "Person");
        assert_eq!(metadata.impls[0].methods.len(), 3);

        // R interface
        let robj = Robj::from_sexp(wrap__get_my_module_metadata());
        let functions = robj.dollar("functions").unwrap();
        let impls = robj.dollar("impls").unwrap();
        assert_eq!(functions.len(), 3);
        assert_eq!(impls.len(), 1);
    });
}

#[test]
fn pairlist_macro_works() {
    with_r(|| {
        assert_eq!(
            pairlist!(1, 2, 3),
            Pairlist::from_pairs(&[("", 1), ("", 2), ("", 3)])
        );
        assert_eq!(
            pairlist!(a = 1, 2, 3),
            Pairlist::from_pairs(&[("a", 1), ("", 2), ("", 3)])
        );
        assert_eq!(
            pairlist!(1, b = 2, 3),
            Pairlist::from_pairs(&[("", 1), ("b", 2), ("", 3)])
        );
        assert_eq!(
            pairlist!(a = 1, b = 2, c = 3),
            Pairlist::from_pairs(&[("a", 1), ("b", 2), ("c", 3)])
        );
        assert_eq!(pairlist!(a = NULL), Pairlist::from_pairs(&[("a", ())]));
        assert_eq!(pairlist!(), Pairlist::from(()));
    });
}

#[test]
fn big_r_macro_works() {
    with_r(|| {
        assert_eq!(R!("1").unwrap(), r!(1.0));
        assert_eq!(R!(r"1").unwrap(), r!(1.0));
        assert_eq!(
            R!(r"
                x <- 1
                x
            ")
            .unwrap(),
            r!(1.0)
        );
        assert_eq!(
            R!(r"
                x <- {{ 1.0 }}
                x
            ")
            .unwrap(),
            r!(1.0)
        );
        assert_eq!(
            R!(r"
                x <- {{ (0..4).collect_robj() }}
                x
            ")
            .unwrap(),
            r!([0, 1, 2, 3])
        );
        assert_eq!(
            R!(r#"
                x <- "hello"
                x
            "#)
            .unwrap(),
            r!("hello")
        );
        assert_eq!(
            Rraw!(
                r"
                x <- {{ 1 }}
                x
            "
            )
            .unwrap(),
            r!(1.0)
        );
    });
}
