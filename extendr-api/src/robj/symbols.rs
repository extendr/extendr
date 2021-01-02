use super::*;

/// The "global" environment
pub fn global_env() -> Robj {
    unsafe { new_sys(R_GlobalEnv) }
}

/// An empty environment at the root of the environment tree
pub fn empty_env() -> Robj {
    unsafe { new_sys(R_EmptyEnv) }
}

/// The base environment; formerly R_NilValue
pub fn base_env() -> Robj {
    unsafe { new_sys(R_BaseEnv) }
}

/// The namespace for base
pub fn base_namespace() -> Robj {
    unsafe { new_sys(R_BaseNamespace) }
}

/// for registered namespaces
pub fn namespace_registry() -> Robj {
    unsafe { new_sys(R_NamespaceRegistry) }
}

/// Current srcref, for debuggers
pub fn srcref() -> Robj {
    unsafe { new_sys(R_Srcref) }
}

/// The nil object
pub fn nil_value() -> Robj {
    unsafe { new_sys(R_NilValue) }
}

/// Unbound marker
pub fn unbound_value() -> Robj {
    unsafe { new_sys(R_UnboundValue) }
}

/// Missing argument marker
pub fn missing_arg() -> Robj {
    unsafe { new_sys(R_MissingArg) }
}

/// "base"
pub fn base_symbol() -> Robj {
    unsafe { new_sys(R_BaseSymbol) }
}

/// "{"
pub fn brace_symbol() -> Robj {
    unsafe { new_sys(R_BraceSymbol) }
}

/// "[["
pub fn bracket_2_symbol() -> Robj {
    unsafe { new_sys(R_Bracket2Symbol) }
}

/// "["
pub fn bracket_symbol() -> Robj {
    unsafe { new_sys(R_BracketSymbol) }
}

/// "class"
pub fn class_symbol() -> Robj {
    unsafe { new_sys(R_ClassSymbol) }
}

/// ".Device"
pub fn device_symbol() -> Robj {
    unsafe { new_sys(R_DeviceSymbol) }
}

/// "dimnames"
pub fn dimnames_symbol() -> Robj {
    unsafe { new_sys(R_DimNamesSymbol) }
}

/// "dim"
pub fn dim_symbol() -> Robj {
    unsafe { new_sys(R_DimSymbol) }
}

/// "$"
pub fn dollar_symbol() -> Robj {
    unsafe { new_sys(R_DollarSymbol) }
}

/// "..."
pub fn dots_symbol() -> Robj {
    unsafe { new_sys(R_DotsSymbol) }
}
//     pub fn drop_symbol() -> Robj { unsafe { new_sys(R_DropSymbol) }}"drop"

/// "::"
pub fn double_colon_symbol() -> Robj {
    unsafe { new_sys(R_DoubleColonSymbol) }
}

/// ".Last.value"
pub fn lastvalue_symbol() -> Robj {
    unsafe { new_sys(R_LastvalueSymbol) }
}
/// "levels"
pub fn levels_symbol() -> Robj {
    unsafe { new_sys(R_LevelsSymbol) }
}
/// "mode"
pub fn mode_symbol() -> Robj {
    unsafe { new_sys(R_ModeSymbol) }
}
/// "na.rm"
pub fn na_rm_symbol() -> Robj {
    unsafe { new_sys(R_NaRmSymbol) }
}
/// "name"
pub fn name_symbol() -> Robj {
    unsafe { new_sys(R_NameSymbol) }
}
/// "names"
pub fn names_symbol() -> Robj {
    unsafe { new_sys(R_NamesSymbol) }
}
/// _NAMESPACE__."
pub fn namespace_env_symbol() -> Robj {
    unsafe { new_sys(R_NamespaceEnvSymbol) }
}
/// "package"
pub fn package_symbol() -> Robj {
    unsafe { new_sys(R_PackageSymbol) }
}
/// "previous"
pub fn previous_symbol() -> Robj {
    unsafe { new_sys(R_PreviousSymbol) }
}
/// "quote"
pub fn quote_symbol() -> Robj {
    unsafe { new_sys(R_QuoteSymbol) }
}
/// "row.names"
pub fn row_names_symbol() -> Robj {
    unsafe { new_sys(R_RowNamesSymbol) }
}
/// ".Random.seed"
pub fn seeds_symbol() -> Robj {
    unsafe { new_sys(R_SeedsSymbol) }
}
/// "sort.list"
pub fn sort_list_symbol() -> Robj {
    unsafe { new_sys(R_SortListSymbol) }
}
/// "source"
pub fn source_symbol() -> Robj {
    unsafe { new_sys(R_SourceSymbol) }
}
/// "spec"
pub fn spec_symbol() -> Robj {
    unsafe { new_sys(R_SpecSymbol) }
}
/// "tsp"
pub fn tsp_symbol() -> Robj {
    unsafe { new_sys(R_TspSymbol) }
}
/// ":::"
pub fn triple_colon_symbol() -> Robj {
    unsafe { new_sys(R_TripleColonSymbol) }
}
/// ".defined"
pub fn dot_defined() -> Robj {
    unsafe { new_sys(R_dot_defined) }
}
/// ".Method"
pub fn dot_method() -> Robj {
    unsafe { new_sys(R_dot_Method) }
}
/// "packageName"
pub fn dot_package_name() -> Robj {
    unsafe { new_sys(R_dot_packageName) }
}

/// ".target"
pub fn dot_target() -> Robj {
    unsafe { new_sys(R_dot_target) }
}

/* fix version issues.
/// ".Generic"
pub fn dot_Generic() -> Robj { unsafe { new_sys(R_dot_Generic) }}
*/

/// NA_STRING as a CHARSXP
pub fn na_string() -> Robj {
    unsafe { new_sys(R_NaString) }
}

/// "" as a CHARSXP
pub fn blank_string() -> Robj {
    unsafe { new_sys(R_BlankString) }
}

/// "" as a STRSXP
pub fn blank_scalar_string() -> Robj {
    unsafe { new_sys(R_BlankScalarString) }
}
