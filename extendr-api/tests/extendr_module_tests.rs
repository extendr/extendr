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

/// Regression test: `extendr_module!` must accept item-level `#[cfg(...)]`
/// attributes on `fn` entries, and the generated metadata call must carry the
/// same attribute so that a cfg'd-out function does not leave a dangling
/// reference to a non-existent `meta__` helper.
///
/// `cfg(test)` is always true here and `cfg(not(test))` always false, so the
/// assertions are deterministic without relying on a Cargo feature flag.
mod cfg_gated_module {
    use super::*;

    #[cfg(test)]
    #[extendr]
    fn included_fn() {}

    #[cfg(not(test))]
    #[extendr]
    fn excluded_fn() {}

    extendr_module! {
        mod cfg_gated_module;
        #[cfg(test)]
        fn included_fn;
        #[cfg(not(test))]
        fn excluded_fn;
    }

    #[test]
    fn cfg_gated_fn_entries_respect_attributes() {
        let names: Vec<&str> = get_cfg_gated_module_metadata()
            .functions
            .iter()
            .map(|f| f.rust_name)
            .collect();

        assert!(
            names.contains(&"included_fn"),
            "cfg(test) fn should be present, got {names:?}"
        );
        assert!(
            !names.contains(&"excluded_fn"),
            "cfg(not(test)) fn should be compiled out, got {names:?}"
        );
    }
}
