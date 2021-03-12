use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Environment {
    pub(crate) robj: Robj,
}

impl Environment {
    /// Make an R environment object.
    /// ```
    /// use extendr_api::prelude::*;
    /// use std::convert::TryInto;
    /// test! {
    ///     let names_and_values = (0..100).map(|i| (format!("n{}", i), i));
    ///     let mut env = Environment::from_pairs(names_and_values);
    ///     env.set_enclos(global_env().try_into()?);
    ///     assert_eq!(env.len(), 100);
    /// }
    /// ```
    pub fn from_pairs<NV>(names_and_values: NV) -> Self
    where
        NV: IntoIterator,
        NV::Item: SymPair,
    {
        single_threaded(|| {
            let dict_len = 29;
            let robj = call!("new.env", TRUE, global_env(), dict_len).unwrap();
            for nv in names_and_values {
                let (n, v) = nv.sym_pair();
                unsafe { Rf_defineVar(n.get(), v.get(), robj.get()) }
            }
            Environment { robj }
        })
    }

    pub fn enclos(&self) -> Robj {
        unsafe {
            let sexp = self.robj.get();
            new_owned(ENCLOS(sexp))
        }
    }

    pub fn set_enclos(&mut self, parent: Environment) -> &mut Self {
        single_threaded(|| unsafe {
            let sexp = self.robj.get();
            SET_ENCLOS(sexp, parent.robj.get());
        });
        self
    }

    pub fn envflags(&self) -> i32 {
        unsafe {
            let sexp = self.robj.get();
            ENVFLAGS(sexp) as i32
        }
    }

    pub fn set_envflags(&mut self, flags: i32) -> &mut Self {
        unsafe {
            let sexp = self.robj.get();
            SET_ENVFLAGS(sexp, flags.into())
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
                    pairlisttags: frame.as_pairlist_tag_iter().unwrap(),
                    pairlist: frame.as_pairlist_iter().unwrap(),
                }
            } else {
                EnvIter {
                    hash_table: hashtab.as_list().unwrap().values(),
                    pairlist: PairlistValueIter::new(),
                    pairlisttags: PairlistTagIter::new(),
                }
            }
        }
    }

    /// Get the names in an environment.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///    let names_and_values : std::collections::HashMap<_, _> = (0..4).map(|i| (format!("n{}", i), r!(i))).collect();
    ///    let env = Environment::from_pairs(names_and_values);
    ///    assert_eq!(env.names().collect::<Vec<_>>(), vec!["n0", "n1", "n2", "n3"]);
    /// }
    /// ```
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.iter().map(|(k, _)| k)
    }
}

/// Iterator over the names and values of an environment
///
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let names_and_values = (0..100).map(|i| (format!("n{}", i), i));
///     let env = Environment::from_pairs(names_and_values);
///     let robj = r!(env);
///     let names_and_values = robj.as_environment().unwrap().iter().collect::<Vec<_>>();
///     assert_eq!(names_and_values.len(), 100);
///
///     let small_env = new_env_with_capacity(1);
///     small_env.set_local(sym!(x), 1);
///     let names_and_values = small_env.as_environment().unwrap().iter().collect::<Vec<_>>();
///     assert_eq!(names_and_values, vec![("x", r!(1))]);
///
///     let large_env = new_env_with_capacity(1000);
///     large_env.set_local(sym!(x), 1);
///     let names_and_values = large_env.as_environment().unwrap().iter().collect::<Vec<_>>();
///     assert_eq!(names_and_values, vec![("x", r!(1))]);
/// }
///
/// ```
#[derive(Clone)]
pub struct EnvIter {
    hash_table: ListIter,
    pairlist: PairlistValueIter,
    pairlisttags: PairlistTagIter,
}

impl Iterator for EnvIter {
    type Item = (&'static str, Robj);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Environments are a hash table (list) or pair lists (pairlist)
            // Get the first available value from the pair list.
            loop {
                match (self.pairlisttags.next(), self.pairlist.next()) {
                    (Some(key), Some(value)) => {
                        // if the key and value are valid, return a pair.
                        if !key.is_na() && !value.is_unbound_value() {
                            println!("value: {:?}", (&key, &value));
                            return Some((key, value));
                        }
                    }
                    // if the key and value are invalid, move on to the hash table.
                    _ => break,
                }
                // continue pair list loop.
            }

            // Get the first pairlist from the hash table.
            loop {
                if let Some(obj) = self.hash_table.next() {
                    if !obj.is_null() && obj.is_pairlist() {
                        self.pairlisttags = obj.as_pairlist_tag_iter().unwrap();
                        self.pairlist = obj.as_pairlist_iter().unwrap();
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
