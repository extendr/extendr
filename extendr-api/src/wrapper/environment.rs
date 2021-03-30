use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Environment {
    pub(crate) robj: Robj,
}

impl Environment {
    /// Create a new, empty environment parented on global_env()
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let env = Environment::new(global_env());
    ///     assert_eq!(env.len(), 0);
    /// }
    /// ```
    pub fn new(parent: Environment) -> Self {
        // 14 is a reasonable default.
        Environment::new_with_capacity(parent, 14)
    }

    /// Create a new, empty environment parented on global_env()
    /// with a reserved size.
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
        let robj = if capacity <= 5 {
            // Unhashed envirnment
            call!("new.env", FALSE, parent, 0).unwrap()
        } else {
            // Hashed environment for larger hashmaps.
            call!("new.env", TRUE, parent, capacity as i32 * 2 + 1).unwrap()
        };
        assert!(robj.is_environment());
        Self { robj }
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
    pub fn from_pairs<NV>(parent: Environment, names_and_values: NV) -> Self
    where
        NV: IntoIterator,
        NV::Item: SymPair,
    {
        single_threaded(|| {
            let dict_len = 29;
            let robj = call!("new.env", TRUE, parent, dict_len).unwrap();
            for nv in names_and_values {
                let (n, v) = nv.sym_pair();
                unsafe { Rf_defineVar(n.get(), v.get(), robj.get()) }
            }
            Environment { robj }
        })
    }

    /// Get the enclosing (parent) environment.
    pub fn parent(&self) -> Option<Environment> {
        unsafe {
            let sexp = self.robj.get();
            let robj = new_owned(ENCLOS(sexp));
            robj.try_into().ok()
        }
    }

    /// Set the enclosing (parent) environment.
    pub fn set_parent(&mut self, parent: Environment) -> &mut Self {
        single_threaded(|| unsafe {
            let sexp = self.robj.get();
            SET_ENCLOS(sexp, parent.robj.get());
        });
        self
    }

    /// Get the environment flags.
    pub fn envflags(&self) -> i32 {
        unsafe {
            let sexp = self.robj.get();
            ENVFLAGS(sexp) as i32
        }
    }

    /// Set the environment flags.
    pub fn set_envflags(&mut self, flags: i32) -> &mut Self {
        unsafe {
            let sexp = self.robj.get();
            SET_ENVFLAGS(sexp, flags)
        }
        self
    }

    /// Iterate over an environment.
    pub fn iter(&self) -> EnvIter {
        unsafe {
            let hashtab = new_owned(HASHTAB(self.get()));
            let frame = new_owned(FRAME(self.get()));
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

    /// Set or define a variable in an enviroment.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let env = Environment::new(global_env());
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

    /// Get a variable from an enviroment, but not its ancestors.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let env = Environment::new(global_env());
    ///     env.set_local(sym!(x), "fred");
    ///     assert_eq!(env.local(sym!(x)), Ok(r!("fred")));
    /// }
    /// ```
    pub fn local<K: Into<Robj>>(&self, key: K) -> Result<Robj> {
        let key = key.into();
        if key.is_symbol() {
            unsafe { Ok(new_owned(Rf_findVarInFrame3(self.get(), key.get(), 1))) }
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
            while let Some((key, value)) = self.pairlist.next() {
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
