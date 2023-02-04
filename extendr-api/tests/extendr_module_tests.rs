//! Tests for [`extendr_module`] procedural macro.
//!
mod root {
    use extendr_api::extendr;
    use extendr_api::extendr_module;
    use extendr_api::GetSexp;

    mod nested_module {
        use extendr_api::extendr;
        use extendr_api::extendr_module;
        use extendr_api::GetSexp;

        #[extendr]
        fn dummy() {}

        extendr_module! {
            mod nested_module;
            fn dummy;
        }
    }

    #[extendr]
    fn hello_dummy() {}

    extendr_module! {
        mod top_level;
        use nested_module;
        fn hello_dummy;
    }
}
