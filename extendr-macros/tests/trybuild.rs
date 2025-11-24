#[test]
fn test_macro_failures() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/cases/*.rs");
    t.compile_fail("tests/extendr_impl/extendr_impl_fail.rs");
}
