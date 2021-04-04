use crate::*;

/// Get a global variable from global_env() and ancestors.
/// If the result is a promise, evaulate the promise.
///
/// See also [global_var()].
/// ```
/// use extendr_api::prelude::*;
/// test! {
///    let iris = global_var(sym!(iris))?;
///    assert_eq!(iris.len(), 5);
/// }
/// ```
pub fn global_var<K: Into<Robj>>(key: K) -> Result<Robj> {
    let key = key.into();
    global_env().find_var(key)?.eval_promise()
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
/// use extendr_api::prelude::*;
/// test! {
///    current_env().set_local(sym!(my_var), 1);
///    assert_eq!(local_var(sym!(my_var))?, r!(1));
/// }
/// ```
pub fn local_var<K: Into<Robj>>(key: K) -> Result<Robj> {
    let key = key.into();
    current_env().find_var(key)?.eval_promise()
}

/// Get a global function from global_env() and ancestors.
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let ls = global_function(sym!(ls))?;
///     assert_eq!(ls.is_function(), true);
/// }
/// ```
pub fn global_function<K: Into<Robj>>(key: K) -> Result<Robj> {
    let key = key.into();
    global_env().find_function(key)
}

/// Find a namespace by name.
///
/// See also [Robj::double_colon].
/// ```
/// use extendr_api::prelude::*;
/// test! {
///    assert_eq!(find_namespace("base").is_ok(), true);
///    assert_eq!(find_namespace("stats").is_ok(), true);
/// }
/// ```
pub fn find_namespace<K: Into<Robj>>(key: K) -> Result<Environment> {
    let key = key.into();
    let res = single_threaded(|| call!(".getNamespace", key.clone()));
    if let Ok(res) = res {
        Ok(res.try_into()?)
    } else {
        Err(Error::NamespaceNotFound(key))
    }
}

/// The current interpreter environment.
///
/// ```
/// use extendr_api::prelude::*;
/// test! {
///    assert_eq!(current_env(), base_env());
/// }
/// ```
pub fn current_env() -> Environment {
    unsafe { new_owned(R_GetCurrentEnv()).try_into().unwrap() }
}

/// The "global" environment
///
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     global_env().set_local(sym!(x), "hello");
///     assert_eq!(global_env().local(sym!(x)), Ok(r!("hello")));
/// }
/// ```
pub fn global_env() -> Environment {
    unsafe { new_sys(R_GlobalEnv).try_into().unwrap() }
}

/// An empty environment at the root of the environment tree
pub fn empty_env() -> Environment {
    unsafe { new_sys(R_EmptyEnv).try_into().unwrap() }
}

/// The base environment; formerly R_NilValue
///
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     global_env().set_local(sym!(x), "hello");
///     assert_eq!(base_env().local(sym!(+)), Ok(r!(Primitive::from_str("+"))));
/// }
/// ```
pub fn base_env() -> Environment {
    unsafe { new_sys(R_BaseEnv).try_into().unwrap() }
}

/// The namespace for base.
///
/// ```
/// use extendr_api::prelude::*;
/// test! {
///    assert_eq!(base_namespace().parent().ok_or("no parent")?, global_env());
/// }
/// ```
pub fn base_namespace() -> Environment {
    unsafe { new_sys(R_BaseNamespace).try_into().unwrap() }
}

/// For registered namespaces.
///
/// ```
/// use extendr_api::prelude::*;
/// test! {
///    assert_eq!(namespace_registry().is_environment(), true);
/// }
/// ```
pub fn namespace_registry() -> Environment {
    unsafe { new_sys(R_NamespaceRegistry).try_into().unwrap() }
}

/// Current srcref, for debuggers
pub fn srcref() -> Robj {
    unsafe { new_sys(R_Srcref) }
}

/// The nil object
pub fn nil_value() -> Robj {
    unsafe { new_sys(R_NilValue) }
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

/// Special "NA" string that represents null strings.
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     assert!(na_str().as_ptr() != "NA".as_ptr());
///     assert_eq!(na_str(), "NA");
///     assert_eq!("NA".is_na(), false);
///     assert_eq!(na_str().is_na(), true);
/// }
/// ```
pub fn na_str() -> &'static str {
    unsafe { std::str::from_utf8_unchecked(&[b'N', b'A']) }
}

/// Parse a string into an R executable object
/// ```
/// use extendr_api::prelude::*;
/// test! {
///    let expr = parse("1 + 2").unwrap();
///    assert!(expr.is_expression());
/// }
/// ```
pub fn parse(code: &str) -> Result<Robj> {
    single_threaded(|| unsafe {
        use libR_sys::*;
        let mut status = 0_u32;
        let status_ptr = &mut status as *mut u32;
        let codeobj: Robj = code.into();
        let parsed = new_owned(R_ParseVector(codeobj.get(), -1, status_ptr, R_NilValue));
        match status {
            1 => Ok(parsed),
            _ => Err(Error::ParseError(code.into())),
        }
    })
}

/// Parse a string into an R executable object and run it.
/// Used by the R! macro.
/// ```
/// use extendr_api::prelude::*;
/// test! {
///    let res = eval_string("1 + 2").unwrap();
///    assert_eq!(res, r!(3.));
/// }
/// ```
pub fn eval_string(code: &str) -> Result<Robj> {
    single_threaded(|| {
        let expr = parse(code)?;
        let mut res = Robj::from(());
        if let Some(expr) = expr.as_expression() {
            for lang in expr.values() {
                res = lang.eval()?
            }
        }
        Ok(res)
    })
}

/// Parse a string into an R executable object and run it using
///   parameters param.0, param.1, ...
///
/// Used by the R! macro.
/// ```
/// use extendr_api::prelude::*;
/// test! {
///    let res = eval_string_with_params("param.0", &[&r!(3.)]).unwrap();
///    assert_eq!(res, r!(3.));
/// }
/// ```
pub fn eval_string_with_params(code: &str, values: &[&Robj]) -> Result<Robj> {
    single_threaded(|| {
        let env = Environment::new(global_env());
        for (i, &v) in values.into_iter().enumerate() {
            let key = Symbol::from_string(format!("param.{}", i));
            env.set_local(key, v);
        }

        let expr = parse(code)?;
        let mut res = Robj::from(());
        if let Some(expr) = expr.as_expression() {
            for lang in expr.values() {
                res = lang.eval_with_env(&env)?
            }
        }

        Ok(res)
    })
}

/// Find a function or primitive that may be in a namespace.
/// ```
/// use extendr_api::prelude::*;
/// test! {
///    assert!(find_namespaced_function("+").is_ok());
///    assert!(find_namespaced_function("ls").is_ok());
///    assert!(find_namespaced_function("base::ls").is_ok());
///    assert!(find_namespaced_function("ls")?.is_language());
///    assert!(!find_namespaced_function("basex::ls").is_ok());
/// }
/// ```
pub fn find_namespaced_function(name: &str) -> Result<Language> {
    let mut iter = name.split("::");
    match (iter.next(), iter.next(), iter.next()) {
        (Some(key), None, None) => {
            let gf = global_function(Symbol::from_string(key))?;
            Ok(Language::from_values(&[gf]))
        }
        (Some(ns), Some(key), None) => {
            let namespace = find_namespace(ns)?;
            Ok(Language::from_values(&[
                namespace.local(Symbol::from_string(key))?
            ]))
        }
        _ => Err(Error::NotFound(r!(name))),
    }
}
