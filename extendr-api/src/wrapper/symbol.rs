use super::*;

/// Wrapper for creating symbol objects.
///
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let chr = r!(Symbol::from_string("xyz"));
///     assert_eq!(chr.as_symbol().unwrap().as_str(), "xyz");
/// }
/// ```
///
#[derive(PartialEq, Clone)]
pub struct Symbol {
    pub(crate) robj: Robj,
}

impl Symbol {
    /// Make a symbol object from a string.
    ///
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let chr = r!(Symbol::from_string("xyz"));
    ///     assert_eq!(chr, sym!(xyz));
    /// }
    /// ```
    pub fn from_string<S: AsRef<str>>(val: S) -> Self {
        let val = val.as_ref();
        Symbol {
            robj: Robj::from_sexp(make_symbol(val)),
        }
    }

    // Internal conversion for constant symbols.
    fn from_sexp(sexp: SEXP) -> Symbol {
        unsafe {
            assert!(TYPEOF(sexp) == SEXPTYPE::SYMSXP);
        }
        Symbol {
            robj: Robj::from_sexp(sexp),
        }
    }

    /// Get the string from a symbol object.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     assert_eq!(sym!(xyz).as_symbol().unwrap().as_str(), "xyz");
    /// }
    /// ```
    pub fn as_str(&self) -> &str {
        unsafe {
            let sexp = self.robj.get();
            let printname = PRINTNAME(sexp);
            rstr::charsxp_to_str(printname).unwrap()
        }
    }
}

impl From<&str> for Symbol {
    /// Convert a string to a symbol.
    fn from(name: &str) -> Self {
        Symbol::from_string(name)
    }
}

/// Unbound marker
pub fn unbound_value() -> Symbol {
    unsafe { Symbol::from_sexp(R_UnboundValue) }
}

/// Missing argument marker
pub fn missing_arg() -> Symbol {
    unsafe { Symbol::from_sexp(R_MissingArg) }
}

/// "base"
pub fn base_symbol() -> Symbol {
    unsafe { Symbol::from_sexp(R_BaseSymbol) }
}

/// "{"
pub fn brace_symbol() -> Symbol {
    unsafe { Symbol::from_sexp(R_BraceSymbol) }
}

/// "[["
pub fn bracket_2_symbol() -> Symbol {
    unsafe { Symbol::from_sexp(R_Bracket2Symbol) }
}

/// "["
pub fn bracket_symbol() -> Symbol {
    unsafe { Symbol::from_sexp(R_BracketSymbol) }
}

/// "class"
pub fn class_symbol() -> Symbol {
    unsafe { Symbol::from_sexp(R_ClassSymbol) }
}

/// ".Device"
pub fn device_symbol() -> Symbol {
    unsafe { Symbol::from_sexp(R_DeviceSymbol) }
}

/// "dimnames"
pub fn dimnames_symbol() -> Symbol {
    unsafe { Symbol::from_sexp(R_DimNamesSymbol) }
}

/// "dim"
pub fn dim_symbol() -> Symbol {
    unsafe { Symbol::from_sexp(R_DimSymbol) }
}

/// "$"
pub fn dollar_symbol() -> Symbol {
    unsafe { Symbol::from_sexp(R_DollarSymbol) }
}

/// "..."
pub fn dots_symbol() -> Symbol {
    unsafe { Symbol::from_sexp(R_DotsSymbol) }
}
//     pub fn drop_symbol() -> Symbol { unsafe { Symbol::from_sexp(R_DropSymbol) }}"drop"

/// "::"
pub fn double_colon_symbol() -> Symbol {
    unsafe { Symbol::from_sexp(R_DoubleColonSymbol) }
}

/// ".Last.value"
pub fn lastvalue_symbol() -> Symbol {
    unsafe { Symbol::from_sexp(R_LastvalueSymbol) }
}
/// "levels"
pub fn levels_symbol() -> Symbol {
    unsafe { Symbol::from_sexp(R_LevelsSymbol) }
}
/// "mode"
pub fn mode_symbol() -> Symbol {
    unsafe { Symbol::from_sexp(R_ModeSymbol) }
}
/// "na.rm"
pub fn na_rm_symbol() -> Symbol {
    unsafe { Symbol::from_sexp(R_NaRmSymbol) }
}
/// "name"
pub fn name_symbol() -> Symbol {
    unsafe { Symbol::from_sexp(R_NameSymbol) }
}
/// "names"
pub fn names_symbol() -> Symbol {
    unsafe { Symbol::from_sexp(R_NamesSymbol) }
}
/// _NAMESPACE__."
pub fn namespace_env_symbol() -> Symbol {
    unsafe { Symbol::from_sexp(R_NamespaceEnvSymbol) }
}
/// "package"
pub fn package_symbol() -> Symbol {
    unsafe { Symbol::from_sexp(R_PackageSymbol) }
}
/// "previous"
pub fn previous_symbol() -> Symbol {
    unsafe { Symbol::from_sexp(R_PreviousSymbol) }
}
/// "quote"
pub fn quote_symbol() -> Symbol {
    unsafe { Symbol::from_sexp(R_QuoteSymbol) }
}
/// "row.names"
pub fn row_names_symbol() -> Symbol {
    unsafe { Symbol::from_sexp(R_RowNamesSymbol) }
}
/// ".Random.seed"
pub fn seeds_symbol() -> Symbol {
    unsafe { Symbol::from_sexp(R_SeedsSymbol) }
}
/// "sort.list"
pub fn sort_list_symbol() -> Symbol {
    unsafe { Symbol::from_sexp(R_SortListSymbol) }
}
/// "source"
pub fn source_symbol() -> Symbol {
    unsafe { Symbol::from_sexp(R_SourceSymbol) }
}
/// "spec"
pub fn spec_symbol() -> Symbol {
    unsafe { Symbol::from_sexp(R_SpecSymbol) }
}
/// "tsp"
pub fn tsp_symbol() -> Symbol {
    unsafe { Symbol::from_sexp(R_TspSymbol) }
}
/// ":::"
pub fn triple_colon_symbol() -> Symbol {
    unsafe { Symbol::from_sexp(R_TripleColonSymbol) }
}
/// ".defined"
pub fn dot_defined() -> Symbol {
    unsafe { Symbol::from_sexp(R_dot_defined) }
}
/// ".Method"
pub fn dot_method() -> Symbol {
    unsafe { Symbol::from_sexp(R_dot_Method) }
}
/// "packageName"
pub fn dot_package_name() -> Symbol {
    unsafe { Symbol::from_sexp(R_dot_packageName) }
}

/// ".target"
pub fn dot_target() -> Symbol {
    unsafe { Symbol::from_sexp(R_dot_target) }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate as extendr_api;

    #[test]
    fn test_constant_symbols() {
        test! {
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
        }
    }
}
