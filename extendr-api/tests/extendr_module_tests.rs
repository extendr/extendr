//! Tests for [`extendr_module`] procedural macro.
//!
use extendr_api::{extendr, extendr_module};

mod root {
    use super::*;

    mod nested_module {
        use super::*;

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
        use adjacent_module;
        fn hello_dummy;
    }
}

mod adjacent_module {
    use super::*;

    #[extendr]
    fn foo() {}

    extendr_module! {
        mod adjacent_module;
        fn foo;
    }
}
