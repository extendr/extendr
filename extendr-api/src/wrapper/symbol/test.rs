use extendr_engine::with_r;

use super::*;

#[test]
fn test_constant_symbols() {
    with_r(|| {
        assert!(unbound_value().is_symbol());
        assert!(missing_arg().is_symbol());
        assert!(base_symbol().is_symbol());
        assert!(brace_symbol().is_symbol());
        assert!(bracket_2_symbol().is_symbol());
        assert!(bracket_symbol().is_symbol());
        assert!(class_symbol().is_symbol());
        assert!(device_symbol().is_symbol());
        assert!(dimnames_symbol().is_symbol());
        assert!(dim_symbol().is_symbol());
        assert!(dollar_symbol().is_symbol());
        assert!(dots_symbol().is_symbol());
        assert!(lastvalue_symbol().is_symbol());
        assert!(levels_symbol().is_symbol());
        assert!(mode_symbol().is_symbol());
        assert!(na_rm_symbol().is_symbol());
        assert!(name_symbol().is_symbol());
        assert!(names_symbol().is_symbol());
        assert!(namespace_env_symbol().is_symbol());
        assert!(package_symbol().is_symbol());
        assert!(previous_symbol().is_symbol());
        assert!(quote_symbol().is_symbol());
        assert!(row_names_symbol().is_symbol());
        assert!(seeds_symbol().is_symbol());
        assert!(sort_list_symbol().is_symbol());
        assert!(source_symbol().is_symbol());
        assert!(spec_symbol().is_symbol());
        assert!(tsp_symbol().is_symbol());
        assert!(triple_colon_symbol().is_symbol());
        assert!(dot_defined().is_symbol());
        assert!(dot_method().is_symbol());
        assert!(dot_package_name().is_symbol());
        assert!(dot_target().is_symbol());
    });
}
