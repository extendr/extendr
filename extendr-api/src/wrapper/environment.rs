use super::*;

#[derive(PartialEq, Clone)]
pub struct Environment {
    pub(crate) robj: Robj,
}

impl Environment {
    /// Create a new, empty environment.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let env = Environment::new_with_parent(global_env());
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
    ///     let env = Environment::new_with_capacity(global_env(), 5);
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
    ///     let mut env = Environment::from_pairs(global_env(), names_and_values);
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

    /// Get the enclosing (parent) environment.
    pub fn parent(&self) -> Option<Environment> {
        unsafe {
            let sexp = self.robj.get();
            let robj = Robj::from_sexp(ENCLOS(sexp));
            robj.try_into().ok()
        }
    }

    /// Set the enclosing (parent) environment.
    pub fn set_parent(&mut self, parent: Environment) -> &mut Self {
        single_threaded(|| unsafe {
            let sexp = self.robj.get_mut();
            SET_ENCLOS(sexp, parent.robj.get());
        });
        self
    }

    /// Get the environment flags.
    pub fn envflags(&self) -> i32 {
        unsafe {
            let sexp = self.robj.get();
            ENVFLAGS(sexp)
        }
    }

    /// Set the environment flags.
    pub fn set_envflags(&mut self, flags: i32) -> &mut Self {
        single_threaded(|| unsafe {
            let sexp = self.robj.get_mut();
            SET_ENVFLAGS(sexp, flags);
        });
        self
    }

    /// Iterate over an environment.
    pub fn iter(&self) -> EnvIter {
        unsafe {
            let hashtab = Robj::from_sexp(HASHTAB(self.get()));
            let frame = Robj::from_sexp(FRAME(self.get()));
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

    /// Get the names in an environment.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///    let names_and_values : std::collections::HashMap<_, _> = (0..4).map(|i| (format!("n{}", i), r!(i))).collect();
    ///    let env = Environment::from_pairs(global_env(), names_and_values);
    ///    assert_eq!(env.names().collect::<Vec<_>>(), vec!["n0", "n1", "n2", "n3"]);
    /// }
    /// ```
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.iter().map(|(k, _)| k)
    }

    /// Set or define a variable in an environment.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let env = Environment::new_with_parent(global_env());
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
    ///     let env = Environment::new_with_parent(global_env());
    ///     env.set_local(sym!(x), "fred");
    ///     assert_eq!(env.local(sym!(x)), Ok(r!("fred")));
    /// }
    /// ```
    pub fn local<K: Into<Robj>>(&self, key: K) -> Result<Robj> {
        let key = key.into();
        if key.is_symbol() {
            unsafe {
                Ok(Robj::from_sexp(Rf_findVarInFrame3(
                    self.get(),
                    key.get(),
                    Rboolean::TRUE,
                )))
            }
        } else {
            Err(Error::NotFound(key))
        }
    }
}

/// Iterator over the names and values of an environment
///
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let names_and_values = (0..100).map(|i| (format!("n{}", i), i));
///     let env = Environment::from_pairs(global_env(), names_and_values);
///     let robj = r!(env);
///     let names_and_values = robj.as_environment().unwrap().iter().collect::<Vec<_>>();
///     assert_eq!(names_and_values.len(), 100);
///
///     let small_env = Environment::new_with_capacity(global_env(), 1);
///     small_env.set_local(sym!(x), 1);
///     let names_and_values = small_env.as_environment().unwrap().iter().collect::<Vec<_>>();
///     assert_eq!(names_and_values, vec![("x", r!(1))]);
///
///     let large_env = Environment::new_with_capacity(global_env(), 1000);
///     large_env.set_local(sym!(x), 1);
///     let names_and_values = large_env.as_environment().unwrap().iter().collect::<Vec<_>>();
///     assert_eq!(names_and_values, vec![("x", r!(1))]);
/// }
///
/// ```
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
                if !key.is_na() && !value.is_unbound_value() {
                    return Some((key, value));
                }
            }

            // Get the first pairlist from the hash table.
            loop {
                if let Some(obj) = self.hash_table.next() {
                    if !obj.is_null() && obj.is_pairlist() {
                        self.pairlist = obj.as_pairlist().unwrap().iter();
                        break;
                    }
                // continue hash table loop.
                } else {
                    // The hash table is empty, end of iteration.
                    return None;
                }
            }
        }
    }
}

impl std::fmt::Debug for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            let sexp = self.get();
            if sexp == R_GlobalEnv {
                write!(f, "global_env()")
            } else if sexp == R_BaseEnv {
                write!(f, "base_env()")
            } else if sexp == R_EmptyEnv {
                write!(f, "empty_env()")
            } else {
                write!(f, "{}", self.deparse().unwrap())
            }
        }
    }
}
