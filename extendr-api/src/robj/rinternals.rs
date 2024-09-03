use crate::*;

///////////////////////////////////////////////////////////////
/// The following impls wrap specific Rinternals.h functions.
///
pub trait Rinternals: Types + Conversions {
    /// Return true if this is the null object.
    fn is_null(&self) -> bool {
        unsafe { Rf_isNull(self.get()).into() }
    }

    /// Return true if this is a symbol.
    fn is_symbol(&self) -> bool {
        unsafe { Rf_isSymbol(self.get()).into() }
    }

    /// Return true if this is a boolean (logical) vector
    fn is_logical(&self) -> bool {
        unsafe { Rf_isLogical(self.get()).into() }
    }

    /// Return true if this is a real (f64) vector.
    fn is_real(&self) -> bool {
        unsafe { Rf_isReal(self.get()).into() }
    }

    /// Return true if this is a complex vector.
    fn is_complex(&self) -> bool {
        unsafe { Rf_isComplex(self.get()).into() }
    }

    /// Return true if this is an expression.
    fn is_expressions(&self) -> bool {
        unsafe { Rf_isExpression(self.get()).into() }
    }

    /// Return true if this is an environment.
    fn is_environment(&self) -> bool {
        unsafe { Rf_isEnvironment(self.get()).into() }
    }

    /// Return true if this is an environment.
    fn is_promise(&self) -> bool {
        self.sexptype() == SEXPTYPE::PROMSXP
    }

    /// Return true if this is a string.
    fn is_string(&self) -> bool {
        unsafe { Rf_isString(self.get()).into() }
    }

    /// Return true if this is an object (ie. has a class attribute).
    fn is_object(&self) -> bool {
        unsafe { Rf_isObject(self.get()).into() }
    }

    /// Return true if this is a S4 object.
    fn is_s4(&self) -> bool {
        unsafe { Rf_isS4(self.get()).into() }
    }

    /// Return true if this is an expression.
    fn is_external_pointer(&self) -> bool {
        self.rtype() == Rtype::ExternalPtr
    }

    /// Get the source ref.
    fn get_current_srcref(val: i32) -> Robj {
        unsafe { Robj::from_sexp(R_GetCurrentSrcref(val as std::ffi::c_int)) }
    }

    /// Get the source filename.
    fn get_src_filename(&self) -> Robj {
        unsafe { Robj::from_sexp(R_GetSrcFilename(self.get())) }
    }

    /// Convert to a string vector.
    fn as_character_vector(&self) -> Robj {
        unsafe { Robj::from_sexp(Rf_asChar(self.get())) }
    }

    /// Convert to vectors of many kinds.
    fn coerce_vector(&self, sexptype: SEXPTYPE) -> Robj {
        single_threaded(|| unsafe { Robj::from_sexp(Rf_coerceVector(self.get(), sexptype)) })
    }

    /// Convert a pairlist (LISTSXP) to a vector list (VECSXP).
    fn pair_to_vector_list(&self) -> Robj {
        single_threaded(|| unsafe { Robj::from_sexp(Rf_PairToVectorList(self.get())) })
    }

    /// Convert a vector list (VECSXP) to a pair list (LISTSXP)
    fn vector_to_pair_list(&self) -> Robj {
        single_threaded(|| unsafe { Robj::from_sexp(Rf_VectorToPairList(self.get())) })
    }

    /// Convert a factor to a string vector.
    fn as_character_factor(&self) -> Robj {
        single_threaded(|| unsafe { Robj::from_sexp(Rf_asCharacterFactor(self.get())) })
    }

    /// Allocate a matrix object.
    fn alloc_matrix(sexptype: SEXPTYPE, rows: i32, cols: i32) -> Robj {
        single_threaded(|| unsafe { Robj::from_sexp(Rf_allocMatrix(sexptype, rows, cols)) })
    }

    /// Do a deep copy of this object.
    /// Note that clone() only adds a reference.
    fn duplicate(&self) -> Robj {
        single_threaded(|| unsafe { Robj::from_sexp(Rf_duplicate(self.get())) })
    }

    /// Find a function in an environment ignoring other variables.
    ///
    /// This evaulates promises if they are found.
    ///
    /// See also [global_function()].
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///    let my_fun = base_env().find_function(sym!(ls)).unwrap();
    ///    assert_eq!(my_fun.is_function(), true);
    ///
    ///    // Note: this may crash on some versions of windows which don't support unwinding.
    ///    // assert!(base_env().find_function(sym!(qwertyuiop)).is_none());
    /// }
    /// ```
    fn find_function<K: TryInto<Symbol, Error = Error>>(&self, key: K) -> Result<Robj> {
        let key: Symbol = key.try_into()?;
        if !self.is_environment() {
            return Err(Error::NotFound(key.into()));
        }
        // This may be better:
        // let mut env: Robj = self.into();
        // loop {
        //     if let Some(var) = env.local(&key) {
        //         if let Some(var) = var.eval_promise() {
        //             if var.is_function() {
        //                 break Some(var);
        //             }
        //         }
        //     }
        //     if let Some(parent) = env.parent() {
        //         env = parent;
        //     } else {
        //         break None;
        //     }
        // }
        unsafe {
            let sexp = self.get();
            if let Ok(var) = catch_r_error(|| Rf_findFun(key.get(), sexp)) {
                Ok(Robj::from_sexp(var))
            } else {
                Err(Error::NotFound(key.into()))
            }
        }
    }

    /// Find a variable in an environment.
    ///
    // //TODO: fix me, as this variable is hidden behind non-api as of this writing
    // See also [global_var()].
    ///
    /// Note that many common variables and functions are contained in promises
    /// which must be evaluated and this function may throw an R error.
    ///
    fn find_var<K: TryInto<Symbol, Error = Error>>(&self, key: K) -> Result<Robj> {
        let key: Symbol = key.try_into()?;
        if !self.is_environment() {
            return Err(Error::NotFound(key.into()));
        }
        // Alternative:
        // let mut env: Robj = self.into();
        // loop {
        //     if let Some(var) = env.local(&key) {
        //         println!("v1={:?}", var);
        //         if let Some(var) = var.eval_promise() {
        //             println!("v2={:?}", var);
        //             break Some(var);
        //         }
        //     }
        //     if let Some(parent) = env.parent() {
        //         env = parent;
        //     } else {
        //         break None;
        //     }
        // }
        unsafe {
            let sexp = self.get();
            if let Ok(var) = catch_r_error(|| Rf_findVar(key.get(), sexp)) {
                if var != R_UnboundValue {
                    Ok(Robj::from_sexp(var))
                } else {
                    Err(Error::NotFound(key.into()))
                }
            } else {
                Err(Error::NotFound(key.into()))
            }
        }
    }

    #[cfg(feature = "non-api")]
    /// If this object is a promise, evaluate it, otherwise return the object.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///    let iris_promise = global_env().find_var(sym!(iris)).unwrap();
    ///    let iris_dataframe = iris_promise.eval_promise().unwrap();
    ///    assert_eq!(iris_dataframe.is_frame(), true);
    /// }
    /// ```
    fn eval_promise(&self) -> Result<Robj> {
        if self.is_promise() {
            self.as_promise().unwrap().eval()
        } else {
            Ok(self.as_robj().clone())
        }
    }

    /// Number of columns of a matrix
    fn ncols(&self) -> usize {
        unsafe { Rf_ncols(self.get()) as usize }
    }

    /// Number of rows of a matrix
    fn nrows(&self) -> usize {
        unsafe { Rf_nrows(self.get()) as usize }
    }

    /// Internal function used to implement `#[extendr]` impl
    #[doc(hidden)]
    unsafe fn make_external_ptr<T>(p: *mut T, prot: Robj) -> Robj {
        let type_name: Robj = std::any::type_name::<T>().into();
        Robj::from_sexp(single_threaded(|| {
            R_MakeExternalPtr(
                p as *mut ::std::os::raw::c_void,
                type_name.get(),
                prot.get(),
            )
        }))
    }

    /// Internal function used to implement `#[extendr]` impl
    #[doc(hidden)]
    unsafe fn external_ptr_addr<T>(&self) -> *mut T {
        R_ExternalPtrAddr(self.get()).cast()
    }

    /// Internal function used to implement `#[extendr]` impl
    #[doc(hidden)]
    unsafe fn external_ptr_tag(&self) -> Robj {
        Robj::from_sexp(R_ExternalPtrTag(self.get()))
    }

    /// Internal function used to implement `#[extendr]` impl
    #[doc(hidden)]
    unsafe fn external_ptr_protected(&self) -> Robj {
        Robj::from_sexp(R_ExternalPtrProtected(self.get()))
    }

    #[doc(hidden)]
    unsafe fn register_c_finalizer(&self, func: R_CFinalizer_t) {
        // Use R_RegisterCFinalizerEx() and set onexit to 1 (TRUE) to invoke the
        // finalizer on a shutdown of the R session as well.
        single_threaded(|| R_RegisterCFinalizerEx(self.get(), func, Rboolean::TRUE));
    }

    /// Copy a vector and resize it.
    /// See. <https://github.com/hadley/r-internals/blob/master/vectors.md>
    fn xlengthgets(&self, new_len: usize) -> Result<Robj> {
        unsafe {
            if self.is_vector() {
                Ok(single_threaded(|| {
                    Robj::from_sexp(Rf_xlengthgets(self.get(), new_len as R_xlen_t))
                }))
            } else {
                Err(Error::ExpectedVector(self.as_robj().clone()))
            }
        }
    }

    /// Allocated an owned object of a certain type.
    fn alloc_vector(sexptype: SEXPTYPE, len: usize) -> Robj {
        single_threaded(|| unsafe { Robj::from_sexp(Rf_allocVector(sexptype, len as R_xlen_t)) })
    }

    /// Return true if two arrays have identical dims.
    fn conformable(a: &Robj, b: &Robj) -> bool {
        single_threaded(|| unsafe { Rf_conformable(a.get(), b.get()).into() })
    }

    /// Return true if this is an array.
    fn is_array(&self) -> bool {
        unsafe { Rf_isArray(self.get()).into() }
    }

    /// Return true if this is factor.
    fn is_factor(&self) -> bool {
        unsafe { Rf_isFactor(self.get()).into() }
    }

    /// Return true if this is a data frame.
    fn is_frame(&self) -> bool {
        unsafe { Rf_isFrame(self.get()).into() }
    }

    /// Return true if this is a function or a primitive (CLOSXP, BUILTINSXP or SPECIALSXP)
    fn is_function(&self) -> bool {
        unsafe { Rf_isFunction(self.get()).into() }
    }

    /// Return true if this is an integer vector (INTSXP) but not a factor.
    fn is_integer(&self) -> bool {
        unsafe { Rf_isInteger(self.get()).into() }
    }

    /// Return true if this is a language object (LANGSXP).
    fn is_language(&self) -> bool {
        unsafe { Rf_isLanguage(self.get()).into() }
    }

    /// Return true if this is NILSXP or LISTSXP.
    fn is_pairlist(&self) -> bool {
        unsafe { Rf_isList(self.get()).into() }
    }

    /// Return true if this is a matrix.
    fn is_matrix(&self) -> bool {
        unsafe { Rf_isMatrix(self.get()).into() }
    }

    /// Return true if this is NILSXP or VECSXP.
    fn is_list(&self) -> bool {
        unsafe { Rf_isNewList(self.get()).into() }
    }

    /// Return true if this is INTSXP, LGLSXP or REALSXP but not a factor.
    fn is_number(&self) -> bool {
        unsafe { Rf_isNumber(self.get()).into() }
    }

    /// Return true if this is a primitive function BUILTINSXP, SPECIALSXP.
    fn is_primitive(&self) -> bool {
        unsafe { Rf_isPrimitive(self.get()).into() }
    }

    /// Return true if this is a time series vector (see tsp).
    fn is_ts(&self) -> bool {
        unsafe { Rf_isTs(self.get()).into() }
    }

    /// Return true if this is a user defined binop.
    fn is_user_binop(&self) -> bool {
        unsafe { Rf_isUserBinop(self.get()).into() }
    }

    #[cfg(feature = "non-api")]
    /// Return true if this is a valid string.
    fn is_valid_string(&self) -> bool {
        unsafe { Rf_isValidString(self.get()).into() }
    }

    #[cfg(feature = "non-api")]
    /// Return true if this is a valid string.
    fn is_valid_string_f(&self) -> bool {
        unsafe { Rf_isValidStringF(self.get()).into() }
    }

    /// Return true if this is a vector.
    fn is_vector(&self) -> bool {
        unsafe { Rf_isVector(self.get()).into() }
    }

    /// Return true if this is an atomic vector.
    fn is_vector_atomic(&self) -> bool {
        unsafe { Rf_isVectorAtomic(self.get()).into() }
    }

    /// Return true if this is a vector list.
    fn is_vector_list(&self) -> bool {
        unsafe { Rf_isVectorList(self.get()).into() }
    }

    /// Return true if this is can be made into a vector.
    fn is_vectorizable(&self) -> bool {
        unsafe { Rf_isVectorizable(self.get()).into() }
    }

    /// Return true if this is RAWSXP.
    fn is_raw(&self) -> bool {
        self.rtype() == Rtype::Raw
    }

    /// Return true if this is CHARSXP.
    fn is_char(&self) -> bool {
        self.rtype() == Rtype::Rstr
    }

    /// Check an external pointer tag.
    /// This is used to wrap R objects.
    #[doc(hidden)]
    fn check_external_ptr_type<T>(&self) -> bool {
        if self.sexptype() == SEXPTYPE::EXTPTRSXP {
            let tag = unsafe { self.external_ptr_tag() };
            if tag.as_str() == Some(std::any::type_name::<T>()) {
                return true;
            }
        }
        false
    }

    fn is_missing_arg(&self) -> bool {
        unsafe { self.get() == R_MissingArg }
    }

    fn is_unbound_value(&self) -> bool {
        unsafe { self.get() == R_UnboundValue }
    }

    fn is_package_env(&self) -> bool {
        unsafe { R_IsPackageEnv(self.get()).into() }
    }

    fn package_env_name(&self) -> Robj {
        unsafe { Robj::from_sexp(R_PackageEnvName(self.get())) }
    }

    fn is_namespace_env(&self) -> bool {
        unsafe { R_IsNamespaceEnv(self.get()).into() }
    }

    fn namespace_env_spec(&self) -> Robj {
        unsafe { Robj::from_sexp(R_NamespaceEnvSpec(self.get())) }
    }

    /// Returns `true` if this is an ALTREP object.
    fn is_altrep(&self) -> bool {
        unsafe { ALTREP(self.get()) != 0 }
    }

    /// Returns `true` if this is an integer ALTREP object.
    fn is_altinteger(&self) -> bool {
        unsafe { ALTREP(self.get()) != 0 && TYPEOF(self.get()) == SEXPTYPE::INTSXP }
    }

    /// Returns `true` if this is an real ALTREP object.
    fn is_altreal(&self) -> bool {
        unsafe { ALTREP(self.get()) != 0 && TYPEOF(self.get()) == SEXPTYPE::REALSXP }
    }

    /// Returns `true` if this is an logical ALTREP object.
    fn is_altlogical(&self) -> bool {
        unsafe { ALTREP(self.get()) != 0 && TYPEOF(self.get()) == SEXPTYPE::LGLSXP }
    }

    /// Returns `true` if this is a raw ALTREP object.
    fn is_altraw(&self) -> bool {
        unsafe { ALTREP(self.get()) != 0 && TYPEOF(self.get()) == SEXPTYPE::RAWSXP }
    }

    /// Returns `true` if this is an integer ALTREP object.
    fn is_altstring(&self) -> bool {
        unsafe { ALTREP(self.get()) != 0 && TYPEOF(self.get()) == SEXPTYPE::STRSXP }
    }

    /// Returns `true` if this is an integer ALTREP object.
    #[cfg(use_r_altlist)]
    fn is_altlist(&self) -> bool {
        unsafe { ALTREP(self.get()) != 0 && TYPEOF(self.get()) == SEXPTYPE::VECSXP }
    }

    /// Generate a text representation of this object.
    fn deparse(&self) -> Result<String> {
        use crate as extendr_api;
        let strings: Strings = call!("deparse", self.as_robj())?.try_into()?;
        if strings.len() == 1 {
            Ok(String::from(strings.elt(0).as_str()))
        } else {
            Ok(strings
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(""))
        }
    }
}

impl Rinternals for Robj {}
