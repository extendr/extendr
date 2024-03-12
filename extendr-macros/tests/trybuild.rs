#[test]
fn test_macro_failures() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/cases/*.rs");
}
