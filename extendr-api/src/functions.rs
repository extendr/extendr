use crate::*;


/// Create a new, empty environment parented on global_env()
///
/// Use the Env{} wrapper for more detail.
/// ```
/// use extendr_api::*;
/// test! {
///     let env = new_env();
///     assert_eq!(env.len(), 0);
/// }
/// ```
pub fn new_env() -> Robj {
    // 14 is a reasonable default.
    new_env_with_capacity(14)
}

/// Create a new, empty environment parented on global_env()
/// with a reserved size.
///
/// This function will guess the hash table size if required.
/// Use the Env{} wrapper for more detail.
/// ```
/// use extendr_api::*;
/// extendr_engine::start_r();
/// let env = new_env_with_capacity(5);
/// env.set_local(sym!(a), 1);
/// env.set_local(sym!(b), 2);
/// assert_eq!(env.len(), 2);
/// ```
pub fn new_env_with_capacity(capacity: usize) -> Robj {
    if capacity <= 5 {
        // Unhashed envirnment
        call!("new.env", FALSE, global_env(), 0).unwrap()
    } else {
        // Hashed environment, the pidgeon hole principle
        // ensures there are empty slots in the hash table.
        call!("new.env", TRUE, global_env(), capacity as i32*2+1).unwrap()
    }
}

/// Get a global variable from global_env() and ancestors.
/// If the result is a promise, evaulate the promise.
///
/// See also [global_var()].
/// ```
/// use extendr_api::*;
/// test! {
///    let iris = global_var(sym!(iris))?;
///    assert_eq!(iris.len(), 5);
/// }
/// ```
pub fn global_var<K : Into<Robj>>(key: K) -> Result<Robj> {
    global_env()
        .find_var(key)
        .ok_or_else(|| Error::NotFound)
        .and_then(|v| v.eval_promise())
}

/// Get a local variable from current_env() and ancestors.
///
/// If the result is a promise, evaulate the promise.
/// The result will come from the calling enviroment
/// of an R function which will enable you to use variables
/// from the caller.
///
/// See also [var!].
/// ```
/// use extendr_api::*;
/// test! {
///    current_env().set_local(sym!(my_var), 1);
///    assert_eq!(local_var(sym!(my_var))?, r!(1));
/// }
/// ```
pub fn local_var<K : Into<Robj>>(key: K) -> Result<Robj> {
    current_env()
        .find_var(key)
        .ok_or_else(|| Error::NotFound)
        .and_then(|v| v.eval_promise())
}

/// Get a global function from global_env() and ancestors.
/// ```
/// use extendr_api::*;
/// test! {
///     let ls = global_function(sym!(ls)).ok_or("ls failed")?;
///     assert_eq!(ls.is_function(), true);
///
///     // Note that the following will throw an R error as iris is not a function.
///     // let iris = global_function(sym!(iris));
/// }
/// ```
pub fn global_function<K : Into<Robj>>(key: K) -> Option<Robj> {
    global_env().find_function(key)
}

/// Find a namespace by name.
///
/// See also [Robj::double_colon].
/// ```
///    use extendr_api::*;
///    extendr_engine::start_r();
///
///    assert_eq!(find_namespace("base").is_some(), true);
///    assert_eq!(find_namespace("stats").is_some(), true);
/// ```
pub fn find_namespace<K : Into<Robj>>(key: K) -> Option<Robj> {
    // single_threaded(|| unsafe { new_borrowed(R_FindNamespace(key.get())) });
    let res = single_threaded(|| call!(".getNamespace", key.into()));
    if let Ok(res) = res {
        Some(res)
    } else {
        None
    }
}

/// The current interpreter environment.
///
/// ```
///    use extendr_api::*;
///    extendr_engine::start_r();
///
///    assert_eq!(current_env(), base_env());
/// ```
pub fn current_env() -> Robj {
    unsafe { new_owned(R_GetCurrentEnv()) }
}

/// The "global" environment
///
/// ```
/// use extendr_api::*;
/// test! {
///     global_env().set_local(sym!(x), "hello");
///     assert_eq!(global_env().local(sym!(x)), Some(r!("hello")));
/// }
/// ```
pub fn global_env() -> Robj {
    unsafe { new_sys(R_GlobalEnv) }
}

/// An empty environment at the root of the environment tree
pub fn empty_env() -> Robj {
    unsafe { new_sys(R_EmptyEnv) }
}

/// The base environment; formerly R_NilValue
///
/// ```
/// use extendr_api::*;
/// test! {
///     global_env().set_local(sym!(x), "hello");
///     assert_eq!(base_env().local(sym!(+)), Some(r!(Primitive("+"))));
/// }
/// ```
pub fn base_env() -> Robj {
    unsafe { new_sys(R_BaseEnv) }
}

/// The namespace for base.
///
/// ```
/// use extendr_api::*;
/// test! {
///    assert_eq!(base_namespace().parent().ok_or("no parent")?, global_env());
/// }
/// ```
pub fn base_namespace() -> Robj {
    unsafe { new_sys(R_BaseNamespace) }
}

/// For registered namespaces.
///
/// ```
///    use extendr_api::*;
///    extendr_engine::start_r();
///
///    assert_eq!(namespace_registry().is_environment(), true);
/// ```
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

/// Extendr test harness.
///
/// See also [test!]
/// ```
/// extendr_api::test(|| {
///   Ok(())
/// })
/// ```
pub fn test<F : FnOnce() -> Result<()>>(f: F) {
    extendr_engine::start_r();

    f().unwrap();
}
