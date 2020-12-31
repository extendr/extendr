use super::*;
use crate::*;

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

    /* TODO:
    int Rf_asLogical2(SEXP x, int checking, SEXP call, SEXP rho);
    Rcomplex Rf_asComplex(SEXP x);
    void Rf_addMissingVarsToNewEnv(SEXP, SEXP);
    SEXP Rf_alloc3DArray(SEXPTYPE, int, int, int);
    SEXP Rf_allocArray(SEXPTYPE, SEXP);
    SEXP Rf_allocFormalsList2(SEXP sym1, SEXP sym2);
    SEXP Rf_allocFormalsList3(SEXP sym1, SEXP sym2, SEXP sym3);
    SEXP Rf_allocFormalsList4(SEXP sym1, SEXP sym2, SEXP sym3, SEXP sym4);
    SEXP Rf_allocFormalsList5(SEXP sym1, SEXP sym2, SEXP sym3, SEXP sym4, SEXP sym5);
    SEXP Rf_allocFormalsList6(SEXP sym1, SEXP sym2, SEXP sym3, SEXP sym4, SEXP sym5, SEXP sym6);
    SEXP Rf_allocList(int);
    SEXP Rf_allocS4Object(void);
    SEXP Rf_allocSExp(SEXPTYPE);
    SEXP Rf_allocVector3(SEXPTYPE, R_xlen_t, R_allocator_t*);
    R_xlen_t Rf_any_duplicated(SEXP x, Rboolean from_last);
    R_xlen_t Rf_any_duplicated3(SEXP x, SEXP incomp, Rboolean from_last);
    SEXP Rf_applyClosure(SEXP, SEXP, SEXP, SEXP, SEXP);
    SEXP Rf_arraySubscript(int, SEXP, SEXP, SEXP (*)(SEXP,SEXP), SEXP (*)(SEXP, int), SEXP);
    SEXP Rf_classgets(SEXP, SEXP);
    SEXP Rf_cons(SEXP, SEXP);
    SEXP Rf_fixSubset3Args(SEXP, SEXP, SEXP, SEXP*);
    void Rf_copyMatrix(SEXP, SEXP, Rboolean);
    void Rf_copyListMatrix(SEXP, SEXP, Rboolean);
    void Rf_copyMostAttrib(SEXP, SEXP);
    void Rf_copyVector(SEXP, SEXP);
    int Rf_countContexts(int, int);
    SEXP Rf_CreateTag(SEXP);
    void Rf_defineVar(SEXP, SEXP, SEXP);
    SEXP Rf_dimgets(SEXP, SEXP);
    SEXP Rf_dimnamesgets(SEXP, SEXP);
    SEXP Rf_DropDims(SEXP);
    */

    /// Compatible way to duplicate an object. Use obj.clone() instead
    /// for Rust compatibility.
    pub fn duplicate(&self) -> Self {
        single_threaded(|| unsafe { new_owned(Rf_duplicate(self.get())) })
    }

    /*
    SEXP Rf_shallow_duplicate(SEXP);
    SEXP R_duplicate_attr(SEXP);
    SEXP R_shallow_duplicate_attr(SEXP);
    SEXP Rf_lazy_duplicate(SEXP);
    SEXP Rf_duplicated(SEXP, Rboolean);
    Rboolean R_envHasNoSpecialSymbols(SEXP);
    SEXP Rf_eval(SEXP, SEXP);
    SEXP Rf_ExtractSubset(SEXP, SEXP, SEXP);
    SEXP Rf_findFun(SEXP, SEXP);
    SEXP Rf_findFun3(SEXP, SEXP, SEXP);
    void Rf_findFunctionForBody(SEXP);
    SEXP Rf_findVar(SEXP, SEXP);
    SEXP Rf_findVarInFrame(SEXP, SEXP);
    SEXP Rf_findVarInFrame3(SEXP, SEXP, Rboolean);
    */

    /// Find a function in an environment ignoring other variables.
    /// ```
    ///    use extendr_api::*;
    ///    extendr_engine::start_r();
    ///
    ///    R!(my_fun <- function() {});
    ///    let my_fun = global_env().find_function(Symbol("my_fun")).unwrap();
    ///    assert_eq!(my_fun.is_function(), true);
    /// ```
    pub fn find_function<S>(&self, symbol: S) -> Result<Robj, AnyError>
    where
        Robj: From<S>,
    {
        let symbol = Robj::from(symbol);
        if !symbol.is_symbol() {
            return Err("find_fun needs a Symbol. eg. find_fun(Symbol(\"xyz\"))".into());
        }
        if !self.is_environment() {
            return Err("find_fun needs an environment.".into());
        }
        Ok(single_threaded(|| unsafe {
            new_borrowed(Rf_findFun(symbol.get(), self.get()))
        }))
    }

    /*
    SEXP Rf_GetArrayDimnames(SEXP);
    SEXP Rf_GetColNames(SEXP);
    void Rf_GetMatrixDimnames(SEXP, SEXP*, SEXP*, const char**, const char**);
    SEXP Rf_GetOption(SEXP, SEXP);
    SEXP Rf_GetOption1(SEXP);
    int Rf_FixupDigits(SEXP, warn_type);
    int Rf_FixupWidth (SEXP, warn_type);
    int Rf_GetOptionDigits(void);
    int Rf_GetOptionWidth(void);
    SEXP Rf_GetRowNames(SEXP);
    void Rf_gsetVar(SEXP, SEXP, SEXP);
    SEXP Rf_install(const char *);
    SEXP Rf_installChar(SEXP);
    SEXP Rf_installNoTrChar(SEXP);
    SEXP Rf_installTrChar(SEXP);
    SEXP Rf_installDDVAL(int i);
    SEXP Rf_installS3Signature(const char *, const char *);
    Rboolean Rf_isFree(SEXP);
    Rboolean Rf_isOrdered(SEXP);
    Rboolean Rf_isUnmodifiedSpecSym(SEXP sym, SEXP env);
    Rboolean Rf_isUnordered(SEXP);
    Rboolean Rf_isUnsorted(SEXP, Rboolean);
    SEXP Rf_lengthgets(SEXP, R_len_t);
    SEXP Rf_xlengthgets(SEXP, R_xlen_t);
    SEXP R_lsInternal(SEXP, Rboolean);
    SEXP R_lsInternal3(SEXP, Rboolean, Rboolean);
    SEXP Rf_match(SEXP, SEXP, int);
    SEXP Rf_matchE(SEXP, SEXP, int, SEXP);
    SEXP Rf_namesgets(SEXP, SEXP);
    SEXP Rf_mkChar(const char *);
    SEXP Rf_mkCharLen(const char *, int);
    Rboolean Rf_NonNullStringMatch(SEXP, SEXP);
    */

    /// Number of columns of a matrix
    pub fn ncols(&self) -> usize {
        unsafe { Rf_ncols(self.get()) as usize }
    }

    /// Number of rows of a matrix
    pub fn nrows(&self) -> usize {
        unsafe { Rf_nrows(self.get()) as usize }
    }

    /*SEXP Rf_nthcdr(SEXP, int);
    Rboolean Rf_pmatch(SEXP, SEXP, Rboolean);
    Rboolean Rf_psmatch(const char *, const char *, Rboolean);
    void Rf_PrintValue(SEXP);
    void Rf_printwhere(void);
    void Rf_readS3VarsFromFrame(SEXP, SEXP*, SEXP*, SEXP*, SEXP*, SEXP*, SEXP*);
    SEXP Rf_setAttrib(SEXP, SEXP, SEXP);
    void Rf_setSVector(SEXP*, int, SEXP);
    void Rf_setVar(SEXP, SEXP, SEXP);
    SEXP Rf_stringSuffix(SEXP, int);
    SEXPTYPE Rf_str2type(const char *);
    Rboolean Rf_StringBlank(SEXP);
    SEXP Rf_substitute(SEXP,SEXP);
    SEXP Rf_topenv(SEXP, SEXP);
    const char * Rf_translateChar(SEXP);
    const char * Rf_translateChar0(SEXP);
    const char * Rf_translateCharUTF8(SEXP);
    const char * Rf_type2char(SEXPTYPE);
    SEXP Rf_type2rstr(SEXPTYPE);
    SEXP Rf_type2str(SEXPTYPE);
    SEXP Rf_type2str_nowarn(SEXPTYPE);
    SEXP R_GetCurrentEnv();
    Rboolean Rf_isS4(SEXP);
    SEXP Rf_asS4(SEXP, Rboolean, int);
    SEXP Rf_S3Class(SEXP);
    int Rf_isBasicClass(const char *);
    Rboolean R_cycle_detected(SEXP s, SEXP child);
    u32 Rf_getCharCE(SEXP);
    SEXP Rf_mkCharCE(const char *, cetype_t);
    SEXP Rf_mkCharLenCE(const char *, int, cetype_t);
    SEXP R_forceAndCall(SEXP e, int n, SEXP rho);
    */

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

    // SEXP R_ExternalPtrTag(SEXP s);
    // SEXP R_ExternalPtrProtected(SEXP s);
    // void R_ClearExternalPtr(SEXP s);
    // void R_SetExternalPtrAddr(SEXP s, void *p);
    // void R_SetExternalPtrTag(SEXP s, SEXP tag);
    // void R_SetExternalPtrProtected(SEXP s, SEXP p);

    /*
    SEXP R_MakeWeakRef(SEXP key, SEXP val, SEXP fin, Rboolean onexit);
    SEXP R_MakeWeakRefC(SEXP key, SEXP val, R_CFinalizer_t fin, Rboolean onexit);
    SEXP R_WeakRefKey(SEXP w);
    SEXP R_WeakRefValue(SEXP w);
    void R_RunWeakRefFinalizer(SEXP w);
    SEXP R_PromiseExpr(SEXP);
    SEXP R_ClosureExpr(SEXP);
    SEXP R_BytecodeExpr(SEXP e);
    SEXP R_bcEncode(SEXP);
    SEXP R_bcDecode(SEXP);
    void R_registerBC(SEXP, SEXP);
    Rboolean R_checkConstants(Rboolean);
    Rboolean R_BCVersionOK(SEXP);
    void R_RestoreHashCount(SEXP rho);
    Rboolean R_IsPackageEnv(SEXP rho);
    SEXP R_PackageEnvName(SEXP rho);
    SEXP R_FindPackageEnv(SEXP info);
    Rboolean R_IsNamespaceEnv(SEXP rho);
    SEXP R_NamespaceEnvSpec(SEXP rho);
    SEXP R_FindNamespace(SEXP info);
    void R_LockEnvironment(SEXP env, Rboolean bindings);
    Rboolean R_EnvironmentIsLocked(SEXP env);
    void R_LockBinding(SEXP sym, SEXP env);
    void R_unLockBinding(SEXP sym, SEXP env);
    void R_MakeActiveBinding(SEXP sym, SEXP fun, SEXP env);
    Rboolean R_BindingIsLocked(SEXP sym, SEXP env);
    Rboolean R_BindingIsActive(SEXP sym, SEXP env);
    Rboolean R_HasFancyBindings(SEXP rho);
    */

    /// Read-only access to attribute list.
    // fn attrib(&self) -> Robj {
    // unsafe {new_borrowed(ATTRIB(self.get()))}
    // }

    /// Copy a vector and resize it.
    /// See. https://github.com/hadley/r-internals/blob/master/vectors.md
    pub fn xlengthgets(&self, new_len: usize) -> Result<Robj, AnyError> {
        unsafe {
            if self.is_vector() {
                Ok(single_threaded(|| {
                    new_owned(Rf_xlengthgets(self.get(), new_len as R_xlen_t))
                }))
            } else {
                Err(AnyError::from("xlengthgets: Not a vector type"))
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

    /// Borrow an element from a list.
    // pub fn elt(&self, index: usize) -> Robj {
    //     single_threaded(|| unsafe { Robj::from(Rf_elt(self.get(), index as raw::c_int)) })
    // }

    //Rboolean Rf_inherits(SEXP, const char *);

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

    /// Return true if this is NILSXP, LISTSXP, LANGSXP or DOTSXP.
    pub fn is_pair_list(&self) -> bool {
        unsafe { Rf_isPairList(self.get()) != 0 }
    }

    /// Return true if this is a primitive function.
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
}

///
/// ```ignore
///    use extendr_api::*;
///    extendr_engine::start_r();
///
///    println!("{:?}", R!(getNamespace("stats")).unwrap());
///    // assert_eq!(find_namespace("stats").is_some(), true);
///    assert!(false);
/// ```
pub fn find_namespace(name: &str) -> Option<Robj> {
    let name = r!(Symbol(name));
    let res = single_threaded(|| unsafe { new_borrowed(R_FindNamespace(name.get())) });
    Some(res)
}
