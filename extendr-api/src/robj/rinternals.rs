use crate::*;
use std::os::raw;

///////////////////////////////////////////////////////////////
/// The following impls wrap specific Rinternals.h functions.
///
impl Robj {
    /// Return true if this is the null object.
    pub fn is_null(&self) -> bool {
        unsafe { Rf_isNull(self.get()) != 0 }
    }

    /// Return true if this is a symbol.
    pub fn is_symbol(&self) -> bool {
        unsafe { Rf_isSymbol(self.get()) != 0 }
    }

    /// Return true if this is a boolean (logical) vector
    pub fn is_logical(&self) -> bool {
        unsafe { Rf_isLogical(self.get()) != 0 }
    }

    /// Return true if this is a real (f64) vector.
    pub fn is_real(&self) -> bool {
        unsafe { Rf_isReal(self.get()) != 0 }
    }

    /// Return true if this is a complex vector.
    pub fn is_complex(&self) -> bool {
        unsafe { Rf_isComplex(self.get()) != 0 }
    }

    /// Return true if this is an expression.
    pub fn is_expr(&self) -> bool {
        unsafe { Rf_isExpression(self.get()) != 0 }
    }

    /// Return true if this is an environment.
    pub fn is_environment(&self) -> bool {
        unsafe { Rf_isEnvironment(self.get()) != 0 }
    }

    /// Return true if this is an environment.
    pub fn is_promise(&self) -> bool {
        self.sexptype() == PROMSXP
    }

    /// Return true if this is a string.
    pub fn is_string(&self) -> bool {
        unsafe { Rf_isString(self.get()) != 0 }
    }

    /// Return true if this is an object.
    pub fn is_object(&self) -> bool {
        unsafe { Rf_isObject(self.get()) != 0 }
    }

    /// Get the source ref.
    pub fn get_current_srcref(val: i32) -> Robj {
        unsafe { new_owned(R_GetCurrentSrcref(val as raw::c_int)) }
    }

    /// Get the source filename.
    pub fn get_src_filename(&self) -> Robj {
        unsafe { new_owned(R_GetSrcFilename(self.get())) }
    }

    /// Convert to a string vector.
    pub fn as_char(&self) -> Robj {
        unsafe { new_owned(Rf_asChar(self.get())) }
    }

    /// Convert to vectors of many kinds.
    pub fn coerce_vector(&self, sexptype: u32) -> Robj {
        single_threaded(|| unsafe { new_owned(Rf_coerceVector(self.get(), sexptype as SEXPTYPE)) })
    }

    /// Convert a pairlist (LISTSXP) to a vector list (VECSXP).
    pub fn pair_to_vector_list(&self) -> Robj {
        single_threaded(|| unsafe { new_owned(Rf_PairToVectorList(self.get())) })
    }

    /// Convert a vector list (VECSXP) to a pair list (LISTSXP)
    pub fn vector_to_pair_list(&self) -> Robj {
        single_threaded(|| unsafe { new_owned(Rf_VectorToPairList(self.get())) })
    }

    /// Convert a factor to a string vector.
    pub fn as_character_factor(&self) -> Robj {
        single_threaded(|| unsafe { new_owned(Rf_asCharacterFactor(self.get())) })
    }

    /// Allocate a matrix object.
    pub fn alloc_matrix(sexptype: SEXPTYPE, rows: i32, cols: i32) -> Robj {
        single_threaded(|| unsafe { new_owned(Rf_allocMatrix(sexptype, rows, cols)) })
    }

    /// Compatible way to duplicate an object. Use obj.clone() instead
    /// for Rust compatibility.
    pub fn duplicate(&self) -> Self {
        single_threaded(|| unsafe { new_owned(Rf_duplicate(self.get())) })
    }

    /// Find a function in an environment ignoring other variables.
    ///
    /// This evaulates promises if they are found.
    ///
    /// See also [global_function()].
    /// ```
    ///    use extendr_api::prelude::*;
    ///    extendr_engine::start_r();
    ///
    ///    let my_fun = base_env().find_function(sym!(ls)).unwrap();
    ///    assert_eq!(my_fun.is_function(), true);
    ///    assert!(base_env().find_function(sym!(qwertyuiop)).is_none());
    /// ```
    pub fn find_function<K: Into<Robj>>(&self, key: K) -> Option<Robj> {
        let key = key.into();
        if !self.is_environment() || !key.is_symbol() {
            return None;
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
            if let Ok(var) = catch_r_error(|| Rf_findFun(key.get(), self.get())) {
                Some(new_borrowed(var))
            } else {
                None
            }
        }
    }

    /// Find a variable in an environment.
    ///
    /// See also [global_var()].
    ///
    /// Note that many common variables and functions are contained in promises
    /// which must be evaluated and this function may throw an R error.
    /// ```
    ///    use extendr_api::prelude::*;
    ///    extendr_engine::start_r();
    ///
    ///    let iris_dataframe = global_env()
    ///        .find_var(sym!(iris)).unwrap().eval_promise().unwrap();
    ///    assert_eq!(iris_dataframe.is_frame(), true);
    ///    assert_eq!(iris_dataframe.len(), 5);
    ///    assert_eq!(global_env().find_var(sym!(imnotasymbol)), None);
    /// ```
    pub fn find_var<K: Into<Robj>>(&self, key: K) -> Option<Robj> {
        let key = key.into();
        if !self.is_environment() || !key.is_symbol() {
            return None;
        }
        // Alterative:
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
            let var = Rf_findVar(key.get(), self.get());
            if var != R_UnboundValue {
                Some(new_borrowed(var))
            } else {
                None
            }
        }
    }

    /// If this object is a promise, evaluate it, otherwise return the object.
    /// ```
    ///    use extendr_api::prelude::*;
    ///    extendr_engine::start_r();
    ///
    ///    let iris_promise = global_env().find_var(sym!(iris)).unwrap();
    ///    let iris_dataframe = iris_promise.eval_promise().unwrap();
    ///    assert_eq!(iris_dataframe.is_frame(), true);
    /// ```
    pub fn eval_promise(&self) -> Result<Robj> {
        if self.is_promise() {
            let promise = self.as_promise().unwrap();
            if !promise.value.is_unbound_value() {
                Ok(promise.value)
            } else {
                self.eval()
            }
        } else {
            Ok(self.into())
        }
    }

    /// Number of columns of a matrix
    pub fn ncols(&self) -> usize {
        unsafe { Rf_ncols(self.get()) as usize }
    }

    /// Number of rows of a matrix
    pub fn nrows(&self) -> usize {
        unsafe { Rf_nrows(self.get()) as usize }
    }

    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub unsafe fn makeExternalPtr<T>(p: *mut T, tag: Robj, prot: Robj) -> Self {
        Robj::make_external_ptr(p, tag, prot)
    }

    /// Internal function used to implement `#[extendr]` impl
    #[doc(hidden)]
    pub unsafe fn make_external_ptr<T>(p: *mut T, tag: Robj, prot: Robj) -> Self {
        new_owned(single_threaded(|| {
            R_MakeExternalPtr(p as *mut ::std::os::raw::c_void, tag.get(), prot.get())
        }))
    }

    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub unsafe fn externalPtrAddr<T>(&self) -> *mut T {
        R_ExternalPtrAddr(self.get()) as *mut T
    }

    /// Internal function used to implement `#[extendr]` impl
    #[doc(hidden)]
    pub unsafe fn external_ptr_addr<T>(&self) -> *mut T {
        R_ExternalPtrAddr(self.get()) as *mut T
    }

    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub unsafe fn externalPtrTag(&self) -> Self {
        self.external_ptr_tag()
    }

    /// Internal function used to implement `#[extendr]` impl
    #[doc(hidden)]
    pub unsafe fn external_ptr_tag(&self) -> Self {
        new_borrowed(R_ExternalPtrTag(self.get()))
    }

    /// Internal function used to implement `#[extendr]` impl
    #[doc(hidden)]
    pub unsafe fn external_ptr_protected(&self) -> Self {
        new_borrowed(R_ExternalPtrProtected(self.get()))
    }

    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub unsafe fn registerCFinalizer(&self, func: R_CFinalizer_t) {
        single_threaded(|| self.register_c_finalizer(func))
    }

    #[doc(hidden)]
    pub unsafe fn register_c_finalizer(&self, func: R_CFinalizer_t) {
        single_threaded(|| R_RegisterCFinalizer(self.get(), func));
    }

    /// Copy a vector and resize it.
    /// See. https://github.com/hadley/r-internals/blob/master/vectors.md
    pub fn xlengthgets(&self, new_len: usize) -> Result<Robj> {
        unsafe {
            if self.is_vector() {
                Ok(single_threaded(|| {
                    new_owned(Rf_xlengthgets(self.get(), new_len as R_xlen_t))
                }))
            } else {
                Err(Error::NotAVectorType)
            }
        }
    }

    /// Allocated an owned object of a certain type.
    pub fn alloc_vector(sexptype: u32, len: usize) -> Robj {
        single_threaded(|| unsafe { new_owned(Rf_allocVector(sexptype, len as R_xlen_t)) })
    }

    /// Return true if two arrays have identical dims.
    pub fn conformable(a: &Robj, b: &Robj) -> bool {
        single_threaded(|| unsafe { Rf_conformable(a.get(), b.get()) != 0 })
    }

    /// Return true if this is an array.
    pub fn is_array(&self) -> bool {
        unsafe { Rf_isArray(self.get()) != 0 }
    }

    /// Return true if this is factor.
    pub fn is_factor(&self) -> bool {
        unsafe { Rf_isFactor(self.get()) != 0 }
    }

    /// Return true if this is a data frame.
    pub fn is_frame(&self) -> bool {
        unsafe { Rf_isFrame(self.get()) != 0 }
    }

    /// Return true if this is a function.
    pub fn is_function(&self) -> bool {
        unsafe { Rf_isFunction(self.get()) != 0 }
    }

    /// Return true if this is an integer vector (INTSXP) but not a factor.
    pub fn is_integer(&self) -> bool {
        unsafe { Rf_isInteger(self.get()) != 0 }
    }

    /// Return true if this is a language object (LANGSXP).
    pub fn is_language(&self) -> bool {
        unsafe { Rf_isLanguage(self.get()) != 0 }
    }

    /// Return true if this is NILSXP or LISTSXP.
    pub fn is_pairlist(&self) -> bool {
        unsafe { Rf_isList(self.get()) != 0 }
    }

    /// Return true if this is a matrix.
    pub fn is_matrix(&self) -> bool {
        unsafe { Rf_isMatrix(self.get()) != 0 }
    }

    /// Return true if this is NILSXP or VECSXP.
    pub fn is_list(&self) -> bool {
        unsafe { Rf_isNewList(self.get()) != 0 }
    }

    /// Return true if this is INTSXP, LGLSXP or REALSXP but not a factor.
    pub fn is_number(&self) -> bool {
        unsafe { Rf_isNumber(self.get()) != 0 }
    }

    /// Return true if this is a primitive function BUILTINSXP, SPECIALSXP.
    pub fn is_primitive(&self) -> bool {
        unsafe { Rf_isPrimitive(self.get()) != 0 }
    }

    /// Return true if this is a time series vector (see tsp).
    pub fn is_ts(&self) -> bool {
        unsafe { Rf_isTs(self.get()) != 0 }
    }

    /// Return true if this is a user defined binop.
    pub fn is_user_binop(&self) -> bool {
        unsafe { Rf_isUserBinop(self.get()) != 0 }
    }

    /// Return true if this is a valid string.
    pub fn is_valid_string(&self) -> bool {
        unsafe { Rf_isValidString(self.get()) != 0 }
    }

    /// Return true if this is a valid string.
    pub fn is_valid_string_f(&self) -> bool {
        unsafe { Rf_isValidStringF(self.get()) != 0 }
    }

    /// Return true if this is a vector.
    pub fn is_vector(&self) -> bool {
        unsafe { Rf_isVector(self.get()) != 0 }
    }

    /// Return true if this is an atomic vector.
    pub fn is_vector_atomic(&self) -> bool {
        unsafe { Rf_isVectorAtomic(self.get()) != 0 }
    }

    /// Return true if this is a vector list.
    pub fn is_vector_list(&self) -> bool {
        unsafe { Rf_isVectorList(self.get()) != 0 }
    }

    /// Return true if this is can be made into a vector.
    pub fn is_vectorizable(&self) -> bool {
        unsafe { Rf_isVectorizable(self.get()) != 0 }
    }

    /// Check an external pointer tag.
    /// This is used to wrap R objects.
    #[doc(hidden)]
    pub fn check_external_ptr(&self, expected_tag: &str) -> bool {
        if self.sexptype() == libR_sys::EXTPTRSXP {
            let tag = unsafe { self.externalPtrTag() };
            if tag.as_str() == Some(expected_tag) {
                return true;
            }
        }
        false
    }

    pub fn is_missing_arg(&self) -> bool {
        unsafe { self.get() == R_MissingArg }
    }

    pub fn is_unbound_value(&self) -> bool {
        unsafe { self.get() == R_UnboundValue }
    }

    pub fn is_package_env(&self) -> bool {
        unsafe { R_IsPackageEnv(self.get()) != 0 }
    }

    pub fn package_env_name(&self) -> Robj {
        unsafe { new_borrowed(R_PackageEnvName(self.get())) }
    }

    pub fn is_namespace_env(&self) -> bool {
        unsafe { R_IsNamespaceEnv(self.get()) != 0 }
    }

    pub fn namespace_env_spec(&self) -> Robj {
        unsafe { new_borrowed(R_NamespaceEnvSpec(self.get())) }
    }
}
