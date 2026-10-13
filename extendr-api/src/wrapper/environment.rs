use super::*;
use extendr_ffi::{
    get_parent_env, get_var_in_frame, R_BaseEnv, R_BindingIsLocked, R_EmptyEnv,
    R_EnvironmentIsLocked, R_GetCurrentEnv, R_GlobalEnv, R_existsVarInFrame, R_lsInternal3,
    R_removeVarFromFrame, Rboolean, Rf_defineVar,
};
#[derive(PartialEq, Clone)]
pub struct Environment {
    pub(crate) robj: Robj,
}

impl Environment {
    /// Create a new, empty environment.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let env = Environment::new_with_parent(Environment::global());
    ///     assert_eq!(env.len(), 0);
    /// }
    /// ```
    pub fn new_with_parent(parent: Environment) -> Self {
        // 14 is a reasonable default.
        Environment::new_with_capacity(parent, 14)
    }

    /// Create a new, empty environment with a reserved size.
    ///
    /// This function will guess the hash table size if required.
    /// Use the Env{} wrapper for more detail.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let env = Environment::new_with_capacity(Environment::global(), 5);
    ///     env.set_local(sym!(a), 1);
    ///     env.set_local(sym!(b), 2);
    ///     assert_eq!(env.len(), 2);
    /// }
    /// ```
    pub fn new_with_capacity(parent: Environment, capacity: usize) -> Self {
        if capacity <= 5 {
            // Unhashed envirnment
            new_env(parent, false, 0)
        } else {
            // Hashed environment for larger hashmaps.
            new_env(parent, true, capacity as i32 * 2 + 1)
        }
    }

    /// Make an R environment object.
    /// ```
    /// use extendr_api::prelude::*;
    /// use std::convert::TryInto;
    /// test! {
    ///     let names_and_values = (0..100).map(|i| (format!("n{}", i), i));
    ///     let mut env = Environment::from_pairs(Environment::global(), names_and_values);
    ///     assert_eq!(env.len(), 100);
    /// }
    /// ```
    #[allow(clippy::wrong_self_convention)]
    pub fn from_pairs<NV>(parent: Environment, names_and_values: NV) -> Self
    where
        NV: IntoIterator,
        NV::Item: SymPair,
    {
        single_threaded(|| {
            let dict_len = 29;
            let env = new_env(parent, true, dict_len);
            for nv in names_and_values {
                let (n, v) = nv.sym_pair();
                if let Some(n) = n {
                    unsafe { Rf_defineVar(n.get(), v.get(), env.get()) }
                }
            }
            env
        })
    }

    /// Get the current evaluation environment.
    pub fn current() -> Self {
        unsafe { Robj::from_sexp(R_GetCurrentEnv()).try_into().unwrap() }
    }

    /// Get the global environment.
    pub fn global() -> Self {
        unsafe { Robj::from_sexp(R_GlobalEnv).try_into().unwrap() }
    }

    /// Get the base environment.
    pub fn base() -> Self {
        unsafe { Robj::from_sexp(R_BaseEnv).try_into().unwrap() }
    }

    /// Get the empty environment.
    pub fn empty() -> Self {
        unsafe { Robj::from_sexp(R_EmptyEnv).try_into().unwrap() }
    }

    /// Get the call associated with this environment.
    /// Mimics rlang's `frame_call()`: evaluates `sys.frames()` in `self`, finds the matching
    /// frame index, then returns `sys.call(i)`.
    /// Returns `None` if not called from a function frame or if the call cannot be determined.
    pub fn call(&self) -> Option<Robj> {
        use crate::robj::Eval;
        // Evaluate sys.frames() in self to get the call stack visible from that frame
        // sys.frames() returns a pairlist of all active frames
        let frames = lang!("sys.frames").eval_with_env(self).ok()?;
        let frames_list = frames.as_pairlist()?;
        // Find the index (1-based) of self in the frames list
        for (i, frame) in frames_list.values().enumerate() {
            let frame_env: Environment = match frame.try_into() {
                Ok(e) => e,
                Err(_) => continue,
            };
            if frame_env == *self {
                let idx = (i + 1) as i32;
                return lang!("sys.call", idx).eval_with_env(self).ok();
            }
        }
        None
    }

    /// Get the caller's environment (i.e. `parent.frame()`).
    pub fn caller() -> Self {
        lang!("parent.frame")
            .eval_with_env(&Environment::current())
            .ok()
            .and_then(|r| r.try_into().ok())
            .unwrap_or_else(Environment::global)
    }

    /// Get the enclosing (parent) environment.
    pub fn parent(&self) -> Option<Environment> {
        unsafe {
            let sexp = self.robj.get();
            let robj = Robj::from_sexp(get_parent_env(sexp));
            robj.try_into().ok()
        }
    }

    #[cfg(feature = "non-api")]
    /// Set the enclosing (parent) environment.
    pub fn set_parent(&mut self, parent: Environment) -> &mut Self {
        single_threaded(|| unsafe {
            let sexp = self.robj.get_mut();
            extendr_ffi::SET_ENCLOS(sexp, parent.robj.get());
        });
        self
    }

    #[cfg(feature = "non-api")]
    /// Get the environment flags.
    pub fn envflags(&self) -> i32 {
        unsafe {
            let sexp = self.robj.get();
            extendr_ffi::ENVFLAGS(sexp)
        }
    }

    #[cfg(feature = "non-api")]
    /// Set the environment flags.
    /// # Safety
    ///
    /// Setting environment flag uses R's C API and is inherently unsafe.
    pub unsafe fn set_envflags(&mut self, flags: i32) -> &mut Self {
        single_threaded(|| unsafe {
            let sexp = self.robj.get_mut();
            extendr_ffi::SET_ENVFLAGS(sexp, flags);
        });
        self
    }

    #[cfg(feature = "non-api")]
    /// Iterate over an environment.
    pub fn iter(&self) -> EnvIter {
        unsafe {
            let hashtab = Robj::from_sexp(extendr_ffi::HASHTAB(self.get()));
            let frame = Robj::from_sexp(extendr_ffi::FRAME(self.get()));
            if hashtab.is_null() && frame.is_pairlist() {
                EnvIter {
                    hash_table: ListIter::new(),
                    pairlist: frame.as_pairlist().unwrap().iter(),
                }
            } else {
                EnvIter {
                    hash_table: hashtab.as_list().unwrap().values(),
                    pairlist: PairlistIter::new(),
                }
            }
        }
    }

    /// Get the names in an environment, sorted.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///    let names_and_values : std::collections::HashMap<_, _> = (0..4).map(|i| (format!("n{}", i), r!(i))).collect();
    ///    let env = Environment::from_pairs(Environment::global(), names_and_values);
    ///    assert_eq!(env.names().collect::<Vec<_>>(), vec!["n0", "n1", "n2", "n3"]);
    /// }
    /// ```
    pub fn names(&self) -> impl Iterator<Item = &'static str> {
        let names =
            unsafe { Robj::from_sexp(R_lsInternal3(self.get(), Rboolean::TRUE, Rboolean::TRUE)) };
        names.as_str_iter().into_iter().flatten()
    }

    /// An alias for [`Environment::names`].
    pub fn keys(&self) -> impl Iterator<Item = &'static str> {
        self.names()
    }

    /// Get the values in an environment, ordered by name.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///    let env = Environment::from_pairs(Environment::global(), vec![("b", r!(2)), ("a", r!(1))]);
    ///    assert_eq!(env.values().collect::<Vec<_>>(), vec![r!(1), r!(2)]);
    /// }
    /// ```
    pub fn values(&self) -> impl Iterator<Item = Robj> + '_ {
        self.names()
            .filter_map(move |name| get_in_frame(self, &Symbol::from_string(name)))
    }

    /// Returns `true` if the environment, but not its ancestors, has a binding for `key`.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let env = Environment::new_with_parent(Environment::global());
    ///     env.insert("x", 1)?;
    ///     assert!(env.contains_key("x"));
    ///     assert!(!env.contains_key("y"));
    /// }
    /// ```
    pub fn contains_key<K: AsRef<str>>(&self, key: K) -> bool {
        let key = Symbol::from_string(key);
        unsafe { R_existsVarInFrame(self.get(), key.get()) != Rboolean::FALSE }
    }

    /// Bind `value` to `key` in the environment, returning the previous value if there was one.
    ///
    /// Fails if the binding is locked, or if the environment is locked and `key` is new.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let env = Environment::new_with_parent(Environment::global());
    ///     assert_eq!(env.insert("x", "harry"), Ok(None));
    ///     assert_eq!(env.insert("x", "fred"), Ok(Some(r!("harry"))));
    ///     assert_eq!(env.local(sym!(x)), Ok(r!("fred")));
    /// }
    /// ```
    pub fn insert<K: AsRef<str>, V: Into<Robj>>(&self, key: K, value: V) -> Result<Option<Robj>> {
        let key = Symbol::from_string(key);
        let value = value.into();
        let old = get_in_frame(self, &key);
        unsafe {
            if old.is_some() && R_BindingIsLocked(key.get(), self.get()) != Rboolean::FALSE {
                return Err(Error::Other(format!(
                    "cannot change value of locked binding for '{}'",
                    key.as_str()
                )));
            }
            if old.is_none() && R_EnvironmentIsLocked(self.get()) != Rboolean::FALSE {
                return Err(Error::Other(format!(
                    "cannot add binding of '{}' to a locked environment",
                    key.as_str()
                )));
            }
            Rf_defineVar(key.get(), value.get(), self.get());
        }
        Ok(old)
    }

    /// Remove `key` from the environment, returning its value if there was one.
    ///
    /// Fails if the environment is locked.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let env = Environment::new_with_parent(Environment::global());
    ///     env.insert("x", 1)?;
    ///     assert_eq!(env.remove("x"), Ok(Some(r!(1))));
    ///     assert_eq!(env.remove("x"), Ok(None));
    /// }
    /// ```
    pub fn remove<K: AsRef<str>>(&self, key: K) -> Result<Option<Robj>> {
        let key = Symbol::from_string(key);
        let Some(old) = get_in_frame(self, &key) else {
            return Ok(None);
        };
        unsafe {
            if R_EnvironmentIsLocked(self.get()) != Rboolean::FALSE {
                return Err(Error::Other(format!(
                    "cannot remove '{}' from a locked environment",
                    key.as_str()
                )));
            }
            R_removeVarFromFrame(key.get(), self.get());
        }
        Ok(Some(old))
    }

    /// Set or define a variable in an environment.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let env = Environment::new_with_parent(Environment::global());
    ///     env.set_local(sym!(x), "harry");
    ///     env.set_local(sym!(x), "fred");
    ///     assert_eq!(env.local(sym!(x)), Ok(r!("fred")));
    /// }
    /// ```
    pub fn set_local<K: Into<Robj>, V: Into<Robj>>(&self, key: K, value: V) {
        let key = key.into();
        let value = value.into();
        if key.is_symbol() {
            single_threaded(|| unsafe {
                Rf_defineVar(key.get(), value.get(), self.get());
            })
        }
    }

    /// Get a variable from an environment, but not its ancestors.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let env = Environment::new_with_parent(Environment::global());
    ///     env.set_local(sym!(x), "fred");
    ///     assert_eq!(env.local(sym!(x)), Ok(r!("fred")));
    /// }
    /// ```
    pub fn local<K: Into<Robj>>(&self, key: K) -> Result<Robj> {
        let key = key.into();
        if key.is_symbol() {
            unsafe { Ok(Robj::from_sexp(get_var_in_frame(self.get(), key.get()))) }
        } else {
            Err(Error::NotFound(key))
        }
    }
}

fn get_in_frame(env: &Environment, key: &Symbol) -> Option<Robj> {
    unsafe {
        if R_existsVarInFrame(env.get(), key.get()) == Rboolean::FALSE {
            return None;
        }
        Some(Robj::from_sexp(get_var_in_frame(env.get(), key.get())))
    }
}

/// Iterator over the names and values of an environment
///
#[derive(Clone)]
pub struct EnvIter {
    hash_table: ListIter,
    pairlist: PairlistIter,
}

impl Iterator for EnvIter {
    type Item = (&'static str, Robj);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Environments are a hash table (list) or pair lists (pairlist)
            // Get the first available value from the pair list.
            for (key, value) in &mut self.pairlist {
                // if the key and value are valid, return a pair.
                #[cfg(not(r_4_5))]
                if key.is_na() || value.is_unbound_value() {
                    continue;
                }
                #[cfg(r_4_5)]
                if key.is_na() {
                    continue;
                }
                return Some((key, value));
            }

            // Get the first pairlist from the hash table.
            loop {
                // The hash table is empty, end of iteration.
                let obj = self.hash_table.next()?;
                if !obj.is_null() && obj.is_pairlist() {
                    self.pairlist = obj.as_pairlist().unwrap().iter();
                    break;
                }
                // continue hash table loop.
            }
        }
    }
}

impl std::fmt::Debug for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            let sexp = self.get();
            if sexp == R_GlobalEnv {
                write!(f, "Environment::global()")
            } else if sexp == R_BaseEnv {
                write!(f, "Environment::base()")
            } else if sexp == R_EmptyEnv {
                write!(f, "Environment::empty()")
            } else {
                write!(f, "{}", self.deparse().unwrap())
            }
        }
    }
}
