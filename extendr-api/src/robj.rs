//! R object handling.
//!
//! See. https://cran.r-project.org/doc/manuals/R-exts.html
//!
//! Fundamental principals:
//!
//! * Any function that can break the protection mechanism is unsafe.
//! * Users should be able to do almost everything without using libR_sys.
//! * The interface should be friendly to R users without Rust experience.

use libR_sys::*;
use std::os::raw;

use crate::logical::*;
use crate::wrapper::*;
use crate::AnyError;

use ndarray::prelude::*;

/// Wrapper for an R S-expression pointer (SEXP).
///
/// As much as possible we wish to make this object safe (ie. no segfaults).
///
/// If you avoid using unsafe functions it is more likely that you will avoid
/// panics and segfaults. We will take great trouble to ensure that this
/// is true.
///
pub enum Robj {
    /// This object owns the SEXP and must free it.
    Owned(SEXP),

    /// This object references a SEXP such as an input parameter.
    Borrowed(SEXP),

    /// This object references a SEXP owned by libR.
    Sys(SEXP),
}

pub const TRUE: bool = true;
pub const FALSE: bool = false;
pub const NULL: () = ();

impl Clone for Robj {
    fn clone(&self) -> Self {
        self.duplicate()
    }
}

impl Default for Robj {
    fn default() -> Self {
        Robj::from(())
    }
}

pub trait FromRobj<'a>: Sized {
    fn from_robj(_robj: &'a Robj) -> Result<Self, &'static str> {
        Err("unable to convert value from R object")
    }
}

macro_rules! impl_prim_from_robj {
    ($t: ty) => {
        impl<'a> FromRobj<'a> for $t {
            fn from_robj(robj: &'a Robj) -> Result<Self, &'static str> {
                if let Some(v) = robj.as_i32_slice() {
                    if v.len() == 0 {
                        Err("zero length vector")
                    } else {
                        Ok(v[0] as Self)
                    }
                } else if let Some(v) = robj.as_f64_slice() {
                    if v.len() == 0 {
                        Err("zero length vector")
                    } else {
                        Ok(v[0] as Self)
                    }
                } else {
                    Err("unable to convert R object to primitive")
                }
            }
        }
    };
}

impl_prim_from_robj!(u8);
impl_prim_from_robj!(u16);
impl_prim_from_robj!(u32);
impl_prim_from_robj!(u64);
impl_prim_from_robj!(i8);
impl_prim_from_robj!(i16);
impl_prim_from_robj!(i32);
impl_prim_from_robj!(i64);
impl_prim_from_robj!(f32);
impl_prim_from_robj!(f64);

impl<'a> FromRobj<'a> for &'a str {
    fn from_robj(robj: &'a Robj) -> Result<Self, &'static str> {
        if let Some(s) = robj.as_str() {
            Ok(s)
        } else {
            Err("not a string object")
        }
    }
}

impl<'a> FromRobj<'a> for String {
    fn from_robj(robj: &'a Robj) -> Result<Self, &'static str> {
        if let Some(s) = robj.as_str() {
            Ok(s.to_string())
        } else {
            Err("not a string object")
        }
    }
}

impl<'a> FromRobj<'a> for Vec<i32> {
    fn from_robj(robj: &'a Robj) -> Result<Self, &'static str> {
        if let Some(v) = robj.as_i32_slice() {
            Ok(Vec::from(v))
        } else {
            Err("not an integer or logical vector")
        }
    }
}

impl<'a> FromRobj<'a> for Vec<f64> {
    fn from_robj(robj: &'a Robj) -> Result<Self, &'static str> {
        if let Some(v) = robj.as_f64_slice() {
            Ok(Vec::from(v))
        } else {
            Err("not a floating point vector")
        }
    }
}

/// Input Numeric vector parameter.
/// Note we don't accept mutable R objects as parameters
/// but you can make this behaviour using unsafe code.
impl<'a, T> FromRobj<'a> for ArrayView1<'a, T>
where
    Robj: AsTypedSlice<T>,
{
    fn from_robj(robj: &'a Robj) -> Result<Self, &'static str> {
        if let Some(v) = robj.as_typed_slice() {
            Ok(ArrayView1::<'a, T>::from(v))
        } else {
            Err("not a floating point vector")
        }
    }
}

macro_rules! make_array_view_2 {
    ($type: ty, $fn: tt, $error_str: tt, $($sexp: tt),* ) => {
        impl<'a> FromRobj<'a> for ArrayView2<'a, $type> {
            fn from_robj(robj: &'a Robj) -> Result<Self, &'static str> {
                match robj.sexptype() {
                    $( $sexp )|* => unsafe {
                        let ptr = $fn(robj.get()) as *const $type;
                        let ncols = Rf_ncols(robj.get()) as usize;
                        let nrows = Rf_nrows(robj.get()) as usize;

                        Ok(ArrayView2::from_shape_ptr((nrows, ncols).f(), ptr))
                    },
                    _ => Err($error_str),
                }
            }
        }
    }
}

make_array_view_2!(Bool, INTEGER, "not a logical matrix", LGLSXP);
make_array_view_2!(i32, INTEGER, "not a integer matrix", INTSXP);
make_array_view_2!(f64, REAL, "not a floating point matrix", REALSXP);
make_array_view_2!(u8, RAW, "not a raw matrix", RAWSXP);

/// Pass-through Robj conversion.
impl<'a> FromRobj<'a> for Robj {
    fn from_robj(robj: &'a Robj) -> Result<Self, &'static str> {
        Ok(unsafe { new_borrowed(robj.get()) })
    }
}

impl Robj {
    /// Get a copy of the underlying SEXP.
    /// Note: this is unsafe.
    pub unsafe fn get(&self) -> SEXP {
        match self {
            Robj::Owned(sexp) => *sexp,
            Robj::Borrowed(sexp) => *sexp,
            Robj::Sys(sexp) => *sexp,
        }
    }

    /// Get a copy of the underlying SEXP for mutable types.
    /// This is valid only for owned objects as we are not
    /// permitted to modify parameters or system objects.
    pub unsafe fn get_mut(&mut self) -> Option<SEXP> {
        match self {
            Robj::Owned(sexp) => Some(*sexp),
            Robj::Borrowed(_) => None,
            Robj::Sys(_) => None,
        }
    }

    /// Get the XXXSXP type of the object.
    pub fn sexptype(&self) -> u32 {
        unsafe { TYPEOF(self.get()) as u32 }
    }

    /// Get the extended length of the object.
    pub fn len(&self) -> usize {
        unsafe { Rf_xlength(self.get()) as usize }
    }

    /// Get a read-only reference to the content of an integer or logical vector.
    pub fn as_i32_slice(&self) -> Option<&[i32]> {
        self.as_typed_slice()
    }

    /// Get a read-only reference to the content of an integer or logical vector.
    pub fn as_logical_slice(&self) -> Option<&[Bool]> {
        self.as_typed_slice()
    }

    /// Get a read-only reference to the content of a double vector.
    pub fn as_f64_slice(&self) -> Option<&[f64]> {
        self.as_typed_slice()
    }

    /// Get a read-only reference to the content of an integer or logical vector.
    pub fn as_u8_slice(&self) -> Option<&[u8]> {
        self.as_typed_slice()
    }

    /// Get a read-write reference to the content of an integer or logical vector.
    pub fn as_i32_slice_mut(&mut self) -> Option<&mut [i32]> {
        self.as_typed_slice_mut()
    }

    /// Get a read-write reference to the content of a double vector.
    pub fn as_f64_slice_mut(&mut self) -> Option<&mut [f64]> {
        self.as_typed_slice_mut()
    }

    /// Get a read-write reference to the content of an integer or logical vector.
    pub fn as_u8_slice_mut(&mut self) -> Option<&mut [u8]> {
        self.as_typed_slice_mut()
    }

    /// Get an iterator over a pairlist.
    pub fn pairlist_iter(&self) -> Option<ListIter> {
        match self.sexptype() {
            LISTSXP | LANGSXP | DOTSXP => unsafe {
                Some(ListIter {
                    list_elem: self.get(),
                })
            },
            _ => None,
        }
    }

    /// Get an iterator over an unnamed list.
    pub fn list_iter(&self) -> Option<VecIter> {
        match self.sexptype() {
            VECSXP | EXPRSXP | WEAKREFSXP => unsafe {
                Some(VecIter {
                    vector: self.get(),
                    i: 0,
                    len: self.len(),
                })
            },
            _ => None,
        }
    }

    /// Get an iterator over a string vector.
    pub fn str_iter(&self) -> Option<StrIter> {
        match self.sexptype() {
            STRSXP => unsafe {
                Some(StrIter {
                    vector: self.get(),
                    i: 0,
                    len: self.len(),
                })
            },
            _ => None,
        }
    }

    /// Get a read-only reference to a char, symbol or string type.
    pub fn as_str(&self) -> Option<&str> {
        unsafe {
            match self.sexptype() {
                STRSXP => {
                    if self.len() == 0 {
                        None
                    } else {
                        Some(to_str(R_CHAR(STRING_ELT(self.get(), 0)) as *const u8))
                    }
                }
                CHARSXP => Some(to_str(R_CHAR(self.get()) as *const u8)),
                SYMSXP => Some(to_str(R_CHAR(PRINTNAME(self.get())) as *const u8)),
                _ => None,
            }
        }
    }

    /// Evaluate the expression and return an error or an R object.
    pub fn eval(&self) -> Result<Robj, AnyError> {
        unsafe {
            let mut error: raw::c_int = 0;
            let res = R_tryEval(self.get(), R_GlobalEnv, &mut error as *mut raw::c_int);
            if error != 0 {
                Err(AnyError::from("R eval error"))
            } else {
                Ok(Robj::from(res))
            }
        }
    }

    /// Evaluate the expression and return NULL or an R object.
    pub fn eval_blind(&self) -> Robj {
        unsafe {
            let mut error: raw::c_int = 0;
            let res = R_tryEval(self.get(), R_GlobalEnv, &mut error as *mut raw::c_int);
            if error != 0 {
                Robj::from(())
            } else {
                Robj::from(res)
            }
        }
    }

    /// Parse a string into an R executable object
    pub fn parse(code: &str) -> Result<Robj, AnyError> {
        unsafe {
            use libR_sys::*;
            let mut status = 0_u32;
            let status_ptr = &mut status as *mut u32;
            let code: Robj = code.into();
            let parsed = Robj::from(R_ParseVector(code.get(), -1, status_ptr, R_NilValue));
            match status {
                1 => Ok(parsed),
                _ => Err(AnyError::from("parse_error")),
            }
        }
    }

    /// Parse a string into an R executable object and run it.
    pub fn eval_string(code: &str) -> Result<Robj, AnyError> {
        let expr = Robj::parse(code)?;
        let mut res = Robj::from(());
        if let Some(iter) = expr.list_iter() {
            for lang in iter {
                res = lang.eval()?;
            }
        }
        Ok(res)
    }

    /// Unprotect an object - assumes a transfer of ownership.
    /// This is unsafe because the object pointer may be left dangling.
    pub unsafe fn unprotected(self) -> Robj {
        match self {
            Robj::Owned(sexp) => {
                R_ReleaseObject(sexp);
                Robj::Borrowed(sexp)
            }
            _ => self,
        }
    }

    /// Return true if the object is owned by this wrapper.
    /// If so, it will be released when the wrapper drops.
    pub fn is_owned(&self) -> bool {
        match self {
            Robj::Owned(_) => true,
            _ => false,
        }
    }
}

pub trait AsTypedSlice<T> {
    fn as_typed_slice(&self) -> Option<&[T]> {
        None
    }
    fn as_typed_slice_mut(&mut self) -> Option<&mut [T]> {
        None
    }
}

macro_rules! make_typed_slice {
    ($type: ty, $fn: tt, $($sexp: tt),* ) => {
        impl AsTypedSlice<$type> for Robj {
            fn as_typed_slice(&self) -> Option<&[$type]> {
                match self.sexptype() {
                    $( $sexp )|* => {
                        unsafe {
                            let ptr = $fn(self.get()) as *const $type;
                            Some(std::slice::from_raw_parts(ptr, self.len()))
                        }
                    }
                    _ => None
                }
            }

            fn as_typed_slice_mut(&mut self) -> Option<&mut [$type]> {
                match self.sexptype() {
                    $( $sexp )|* => {
                        unsafe {
                            let ptr = $fn(self.get()) as *mut $type;
                            Some(std::slice::from_raw_parts_mut(ptr, self.len()))
                        }
                    }
                    _ => None
                }
            }
        }
    }
}

make_typed_slice!(Bool, INTEGER, LGLSXP);
make_typed_slice!(i32, INTEGER, INTSXP);
make_typed_slice!(f64, REAL, REALSXP);
make_typed_slice!(u8, RAW, RAWSXP);

///////////////////////////////////////////////////////////////
/// The following impls wrap specific Rinternals.h symbols.
///
#[allow(non_snake_case)]
impl Robj {
    /// The "global" environment
    pub fn globalEnv() -> Robj {
        unsafe { new_sys(R_GlobalEnv) }
    }
    /// An empty environment at the root of the environment tree
    pub fn emptyEnv() -> Robj {
        unsafe { new_sys(R_EmptyEnv) }
    }
    /// The base environment; formerly R_NilValue
    pub fn baseEnv() -> Robj {
        unsafe { new_sys(R_BaseEnv) }
    }
    /// The (fake) namespace for base
    pub fn baseNamespace() -> Robj {
        unsafe { new_sys(R_BaseNamespace) }
    }
    /// for registered namespaces
    pub fn namespaceRegistry() -> Robj {
        unsafe { new_sys(R_NamespaceRegistry) }
    }
    /// Current srcref, for debuggers
    pub fn srcref() -> Robj {
        unsafe { new_sys(R_Srcref) }
    }
    /// The nil object
    pub fn nilValue() -> Robj {
        unsafe { new_sys(R_NilValue) }
    }
    /// Unbound marker
    pub fn unboundValue() -> Robj {
        unsafe { new_sys(R_UnboundValue) }
    }
    /// Missing argument marker
    pub fn missingArg() -> Robj {
        unsafe { new_sys(R_MissingArg) }
    }

    /* Not supported by older R versions.
    /// To be found in BC interp. state (marker)
    pub fn inBCInterpreter() -> Robj { unsafe { new_sys(R_InBCInterpreter) }}
    /// Use current expression (marker)
    pub fn currentExpression() -> Robj { unsafe { new_sys(R_CurrentExpression) }}
    /// character"
    pub fn asCharacterSymbol() -> Robj { unsafe { new_sys(R_AsCharacterSymbol) }}
    */

    /// "base"
    pub fn baseSymbol() -> Robj {
        unsafe { new_sys(R_BaseSymbol) }
    }
    /// "{"
    pub fn braceSymbol() -> Robj {
        unsafe { new_sys(R_BraceSymbol) }
    }
    /// "[["
    pub fn bracket2Symbol() -> Robj {
        unsafe { new_sys(R_Bracket2Symbol) }
    }
    /// "["
    pub fn bracketSymbol() -> Robj {
        unsafe { new_sys(R_BracketSymbol) }
    }
    /// "class"
    pub fn classSymbol() -> Robj {
        unsafe { new_sys(R_ClassSymbol) }
    }
    /// ".Device"
    pub fn deviceSymbol() -> Robj {
        unsafe { new_sys(R_DeviceSymbol) }
    }
    /// "dimnames"
    pub fn dimNamesSymbol() -> Robj {
        unsafe { new_sys(R_DimNamesSymbol) }
    }
    /// "dim"
    pub fn dimSymbol() -> Robj {
        unsafe { new_sys(R_DimSymbol) }
    }
    /// "$"
    pub fn dollarSymbol() -> Robj {
        unsafe { new_sys(R_DollarSymbol) }
    }
    /// "..."
    pub fn dotsSymbol() -> Robj {
        unsafe { new_sys(R_DotsSymbol) }
    }
    ///     pub fn dropSymbol() -> Robj { unsafe { new_sys(R_DropSymbol) }}"drop"
    pub fn doubleColonSymbol() -> Robj {
        unsafe { new_sys(R_DoubleColonSymbol) }
    } //
    /// ".Last.value"
    pub fn lastvalueSymbol() -> Robj {
        unsafe { new_sys(R_LastvalueSymbol) }
    }
    /// "levels"
    pub fn levelsSymbol() -> Robj {
        unsafe { new_sys(R_LevelsSymbol) }
    }
    /// "mode"
    pub fn modeSymbol() -> Robj {
        unsafe { new_sys(R_ModeSymbol) }
    }
    /// "na.rm"
    pub fn naRmSymbol() -> Robj {
        unsafe { new_sys(R_NaRmSymbol) }
    }
    /// "name"
    pub fn nameSymbol() -> Robj {
        unsafe { new_sys(R_NameSymbol) }
    }
    /// "names"
    pub fn namesSymbol() -> Robj {
        unsafe { new_sys(R_NamesSymbol) }
    }
    /// _NAMESPACE__."
    pub fn namespaceEnvSymbol() -> Robj {
        unsafe { new_sys(R_NamespaceEnvSymbol) }
    }
    /// "package"
    pub fn packageSymbol() -> Robj {
        unsafe { new_sys(R_PackageSymbol) }
    }
    /// "previous"
    pub fn previousSymbol() -> Robj {
        unsafe { new_sys(R_PreviousSymbol) }
    }
    /// "quote"
    pub fn quoteSymbol() -> Robj {
        unsafe { new_sys(R_QuoteSymbol) }
    }
    /// "row.names"
    pub fn rowNamesSymbol() -> Robj {
        unsafe { new_sys(R_RowNamesSymbol) }
    }
    /// ".Random.seed"
    pub fn seedsSymbol() -> Robj {
        unsafe { new_sys(R_SeedsSymbol) }
    }
    /// "sort.list"
    pub fn sortListSymbol() -> Robj {
        unsafe { new_sys(R_SortListSymbol) }
    }
    /// "source"
    pub fn sourceSymbol() -> Robj {
        unsafe { new_sys(R_SourceSymbol) }
    }
    /// "spec"
    pub fn specSymbol() -> Robj {
        unsafe { new_sys(R_SpecSymbol) }
    }
    /// "tsp"
    pub fn tspSymbol() -> Robj {
        unsafe { new_sys(R_TspSymbol) }
    }
    /// ":::"
    pub fn tripleColonSymbol() -> Robj {
        unsafe { new_sys(R_TripleColonSymbol) }
    }
    /// ".defined"
    pub fn dot_defined() -> Robj {
        unsafe { new_sys(R_dot_defined) }
    }
    /// ".Method"
    pub fn dot_Method() -> Robj {
        unsafe { new_sys(R_dot_Method) }
    }
    /// "packageName"
    pub fn dot_packageName() -> Robj {
        unsafe { new_sys(R_dot_packageName) }
    } //
    /// ".target"
    pub fn dot_target() -> Robj {
        unsafe { new_sys(R_dot_target) }
    }
    /* fix version issues.
    /// ".Generic"
    pub fn dot_Generic() -> Robj { unsafe { new_sys(R_dot_Generic) }}
    */
    /// NA_STRING as a CHARSXP
    pub fn naString() -> Robj {
        unsafe { new_sys(R_NaString) }
    }
    /// "" as a CHARSXP
    pub fn blankString() -> Robj {
        unsafe { new_sys(R_BlankString) }
    }
    /// as a STRSXP
    pub fn blankScalarString() -> Robj {
        unsafe { new_sys(R_BlankScalarString) }
    }
}

///////////////////////////////////////////////////////////////
/// The following impls wrap specific Rinternals.h functions.
///
#[allow(non_snake_case)]
impl Robj {
    /// Return true if this is the null object.
    pub fn isNull(&self) -> bool {
        unsafe { Rf_isNull(self.get()) != 0 }
    }

    /// Return true if this is a symbol.
    pub fn isSymbol(&self) -> bool {
        unsafe { Rf_isSymbol(self.get()) != 0 }
    }

    /// Return true if this is a boolean (logical) vector
    pub fn isLogical(&self) -> bool {
        unsafe { Rf_isLogical(self.get()) != 0 }
    }

    /// Return true if this is a real (f64) vector.
    pub fn isReal(&self) -> bool {
        unsafe { Rf_isReal(self.get()) != 0 }
    }

    /// Return true if this is a complex vector.
    pub fn isComplex(&self) -> bool {
        unsafe { Rf_isComplex(self.get()) != 0 }
    }

    /// Return true if this is an expression.
    pub fn isExpression(&self) -> bool {
        unsafe { Rf_isExpression(self.get()) != 0 }
    }

    /// Return true if this is an environment.
    pub fn isEnvironment(&self) -> bool {
        unsafe { Rf_isEnvironment(self.get()) != 0 }
    }

    /// Return true if this is a string.
    pub fn isString(&self) -> bool {
        unsafe { Rf_isString(self.get()) != 0 }
    }

    /// Return true if this is an object.
    pub fn isObject(&self) -> bool {
        unsafe { Rf_isObject(self.get()) != 0 }
    }

    /// Get the source ref.
    pub fn getCurrentSrcref(val: i32) -> Robj {
        unsafe { new_owned(R_GetCurrentSrcref(val as raw::c_int)) }
    }

    /// Get the source filename.
    pub fn getSrcFilename(&self) -> Robj {
        unsafe { new_owned(R_GetSrcFilename(self.get())) }
    }

    /// Convert to a string vector.
    pub fn asChar(&self) -> Robj {
        unsafe { new_owned(Rf_asChar(self.get())) }
    }

    /// Convert to vectors of many kinds.
    pub fn coerceVector(&self, sexptype: u32) -> Robj {
        unsafe { new_owned(Rf_coerceVector(self.get(), sexptype as SEXPTYPE)) }
    }

    /// Convert a pairlist (LISTSXP) to a vector list (VECSXP).
    pub fn pairToVectorList(&self) -> Robj {
        unsafe { new_owned(Rf_PairToVectorList(self.get())) }
    }

    /// Convert a vector list (VECSXP) to a pair list (LISTSXP)
    pub fn vectorToPairList(&self) -> Robj {
        unsafe { new_owned(Rf_VectorToPairList(self.get())) }
    }

    /// Assign an integer to each unique string and return a "factor".
    pub fn asCharacterFactor(&self) -> Robj {
        unsafe { new_owned(Rf_asCharacterFactor(self.get())) }
    }

    /// Get a scalar boolean value
    pub fn asLogical(&self) -> bool {
        unsafe { Rf_asLogical(self.get()) != 0 }
    }

    /// Get a scalar 32 bit integer value
    pub fn asInteger(&self) -> i32 {
        unsafe { Rf_asInteger(self.get()) as i32 }
    }

    /// Get a 64 bit double value
    pub fn asReal(&self) -> f64 {
        unsafe { Rf_asReal(self.get()) as f64 }
    }

    /// Allocate a matrix object (see NumericMatrix etc.)
    pub fn allocMatrix(sexptype: SEXPTYPE, rows: i32, cols: i32) -> Robj {
        unsafe { new_owned(Rf_allocMatrix(sexptype, rows, cols)) }
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
    /// for Rust compaitibility.
    pub fn duplicate(&self) -> Self {
        unsafe { new_owned(Rf_duplicate(self.get())) }
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
    SEXP Rf_getAttrib(SEXP, SEXP);
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

    /// Internal function used to implement #[extendr] impl
    pub unsafe fn makeExternalPtr<T>(p: *mut T, tag: Robj, prot: Robj) -> Self {
        new_owned(R_MakeExternalPtr(
            p as *mut ::std::os::raw::c_void,
            tag.get(),
            prot.get(),
        ))
    }

    /// Internal function used to implement #[extendr] impl
    pub unsafe fn externalPtrAddr<T>(&self) -> *mut T {
        R_ExternalPtrAddr(self.get()) as *mut T
    }

    /// Internal function used to implement #[extendr] impl
    pub unsafe fn externalPtrTag(&self) -> Self {
        new_borrowed(R_ExternalPtrTag(self.get()))
    }

    /// Internal function used to implement #[extendr] impl
    pub unsafe fn externalPtrProtected(&self) -> Self {
        new_borrowed(R_ExternalPtrProtected(self.get()))
    }

    pub unsafe fn registerCFinalizer(&self, func: R_CFinalizer_t) {
        R_RegisterCFinalizer(self.get(), func);
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
            if self.isVector() {
                Ok(new_owned(Rf_xlengthgets(self.get(), new_len as R_xlen_t)))
            } else {
                Err(AnyError::from("xlengthgets: Not a vector type"))
            }
        }
    }

    /// Allocated an owned object of a certain type.
    pub fn allocVector(sexptype: u32, len: usize) -> Robj {
        unsafe { new_owned(Rf_allocVector(sexptype, len as R_xlen_t)) }
    }

    /// Return true if two arrays have identical dims.
    pub fn conformable(a: &Robj, b: &Robj) -> bool {
        unsafe { Rf_conformable(a.get(), b.get()) != 0 }
    }

    /// Borrow an element from a list.
    pub fn elt(&self, index: usize) -> Robj {
        unsafe { Robj::from(Rf_elt(self.get(), index as raw::c_int)) }
    }

    //Rboolean Rf_inherits(SEXP, const char *);

    /// Return true if this is an array.
    pub fn isArray(&self) -> bool {
        unsafe { Rf_isArray(self.get()) != 0 }
    }

    /// Return true if this is factor.
    pub fn isFactor(&self) -> bool {
        unsafe { Rf_isFactor(self.get()) != 0 }
    }

    /// Return true if this is a data frame.
    pub fn isFrame(&self) -> bool {
        unsafe { Rf_isFrame(self.get()) != 0 }
    }

    /// Return true if this is a function.
    pub fn isFunction(&self) -> bool {
        unsafe { Rf_isFunction(self.get()) != 0 }
    }

    /// Return true if this is an integer vector.
    pub fn isInteger(&self) -> bool {
        unsafe { Rf_isInteger(self.get()) != 0 }
    }

    /// Return true if this is a language object.
    pub fn isLanguage(&self) -> bool {
        unsafe { Rf_isLanguage(self.get()) != 0 }
    }

    /// Return true if this is a vector list.
    pub fn isList(&self) -> bool {
        unsafe { Rf_isList(self.get()) != 0 }
    }

    /// Return true if this is a matrix.
    pub fn isMatrix(&self) -> bool {
        unsafe { Rf_isMatrix(self.get()) != 0 }
    }

    /// Return true if this is a vector list or null.
    pub fn isNewList(&self) -> bool {
        unsafe { Rf_isNewList(self.get()) != 0 }
    }

    /// Return true if this is a numeric vector but not a factor.
    pub fn isNumber(&self) -> bool {
        unsafe { Rf_isNumber(self.get()) != 0 }
    }

    /// Return true if this is a numeric vector but not a factor or complex.
    pub fn isNumeric(&self) -> bool {
        unsafe { Rf_isNumeric(self.get()) != 0 }
    }

    /// Return true if this is a pairlist.
    pub fn isPairList(&self) -> bool {
        unsafe { Rf_isPairList(self.get()) != 0 }
    }

    /// Return true if this is a primitive function.
    pub fn isPrimitive(&self) -> bool {
        unsafe { Rf_isPrimitive(self.get()) != 0 }
    }

    /// Return true if this is a time series vector (see tsp).
    pub fn isTs(&self) -> bool {
        unsafe { Rf_isTs(self.get()) != 0 }
    }

    /// Return true if this is a user defined binop.
    pub fn isUserBinop(&self) -> bool {
        unsafe { Rf_isUserBinop(self.get()) != 0 }
    }

    /// Return true if this is a valid string.
    pub fn isValidString(&self) -> bool {
        unsafe { Rf_isValidString(self.get()) != 0 }
    }

    /// Return true if this is a valid string.
    pub fn isValidStringF(&self) -> bool {
        unsafe { Rf_isValidStringF(self.get()) != 0 }
    }

    /// Return true if this is a vector.
    pub fn isVector(&self) -> bool {
        unsafe { Rf_isVector(self.get()) != 0 }
    }

    /// Return true if this is an atomic vector.
    pub fn isVectorAtomic(&self) -> bool {
        unsafe { Rf_isVectorAtomic(self.get()) != 0 }
    }

    /// Return true if this is a vector list.
    pub fn isVectorList(&self) -> bool {
        unsafe { Rf_isVectorList(self.get()) != 0 }
    }

    /// Return true if this is can be made into a vector.
    pub fn isVectorizable(&self) -> bool {
        unsafe { Rf_isVectorizable(self.get()) != 0 }
    }

    /// Check an external pointer tag
    /// This may work better by using a symbol cached in a static variable.
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

pub unsafe fn new_owned(sexp: SEXP) -> Robj {
    R_PreserveObject(sexp);
    Robj::Owned(sexp)
}

pub unsafe fn new_borrowed(sexp: SEXP) -> Robj {
    Robj::Borrowed(sexp)
}

pub unsafe fn new_sys(sexp: SEXP) -> Robj {
    Robj::Sys(sexp)
}

/// Compare equality with integer slices.
impl<'a> PartialEq<[i32]> for Robj {
    fn eq(&self, rhs: &[i32]) -> bool {
        self.as_i32_slice() == Some(rhs)
    }
}

/// Compare equality with slices of double.
impl<'a> PartialEq<[f64]> for Robj {
    fn eq(&self, rhs: &[f64]) -> bool {
        self.as_f64_slice() == Some(rhs)
    }
}

/// Compare equality with strings.
impl PartialEq<str> for Robj {
    fn eq(&self, rhs: &str) -> bool {
        self.as_str() == Some(rhs)
    }
}

/// Compare equality with two Robjs.
impl PartialEq<Robj> for Robj {
    fn eq(&self, rhs: &Robj) -> bool {
        if self.sexptype() == rhs.sexptype() && self.len() == rhs.len() {
            unsafe {
                let lsexp = self.get();
                let rsexp = rhs.get();
                match self.sexptype() {
                    NILSXP => true,
                    SYMSXP => PRINTNAME(lsexp) == PRINTNAME(rsexp),
                    LISTSXP | LANGSXP | DOTSXP => self
                        .pairlist_iter()
                        .unwrap()
                        .eq(rhs.pairlist_iter().unwrap()),
                    CLOSXP => false,
                    ENVSXP => false,
                    PROMSXP => false,
                    SPECIALSXP => false,
                    BUILTINSXP => false,
                    CHARSXP => self.as_str() == rhs.as_str(),
                    LGLSXP => self.as_logical_slice() == rhs.as_logical_slice(),
                    INTSXP => self.as_i32_slice() == rhs.as_i32_slice(),
                    REALSXP => self.as_f64_slice() == rhs.as_f64_slice(),
                    CPLXSXP => false,
                    ANYSXP => false,
                    VECSXP | EXPRSXP => self.list_iter().unwrap().eq(rhs.list_iter().unwrap()),
                    STRSXP => self.str_iter().unwrap().eq(rhs.str_iter().unwrap()),
                    BCODESXP => false,
                    EXTPTRSXP => false,
                    WEAKREFSXP => false,
                    RAWSXP => self.as_u8_slice() == rhs.as_u8_slice(),
                    S4SXP => false,
                    NEWSXP => false,
                    FREESXP => false,
                    _ => false,
                }
            }
        } else {
            false
        }
    }
}

/// Implement {:?} formatting.
impl std::fmt::Debug for Robj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.sexptype() {
            NILSXP => write!(f, "NULL"),
            SYMSXP => write!(f, "Symbol({:?})", self.as_str().unwrap()),
            // LISTSXP => false,
            // CLOSXP => false,
            // ENVSXP => false,
            // PROMSXP => false,
            LANGSXP => write!(
                f,
                "Lang({:?})",
                self.pairlist_iter().unwrap().collect::<Vec<Robj>>()
            ),
            // SPECIALSXP => false,
            // BUILTINSXP => false,
            CHARSXP => write!(f, "Character({:?})", self.as_str().unwrap()),
            LGLSXP => {
                let slice = self.as_logical_slice().unwrap();
                if slice.len() == 1 {
                    write!(f, "{}", if slice[0].0 == 0 { "FALSE" } else { "TRUE" })
                } else {
                    write!(f, "&{:?}", slice)
                }
            }
            INTSXP => {
                let slice = self.as_i32_slice().unwrap();
                if slice.len() == 1 {
                    write!(f, "{:?}", slice[0])
                } else {
                    write!(f, "{:?}", self.as_i32_slice().unwrap())
                }
            }
            REALSXP => {
                let slice = self.as_f64_slice().unwrap();
                if slice.len() == 1 {
                    write!(f, "{:?}", slice[0])
                } else {
                    write!(f, "{:?}", slice)
                }
            }
            VECSXP => write!(f, "{:?}", self.list_iter().unwrap().collect::<Vec<_>>()),
            EXPRSXP => write!(
                f,
                "Expr({:?})",
                self.list_iter().unwrap().collect::<Vec<_>>()
            ),
            WEAKREFSXP => write!(
                f,
                "Weakref({:?})",
                self.list_iter().unwrap().collect::<Vec<_>>()
            ),
            // CPLXSXP => false,
            STRSXP => {
                write!(f, "[")?;
                let mut sep = "";
                for obj in self.str_iter().unwrap() {
                    write!(f, "{}{:?}", sep, obj)?;
                    sep = ", ";
                }
                write!(f, "]")
            }
            // DOTSXP => false,
            // ANYSXP => false,
            // VECSXP => false,
            // EXPRSXP => false,
            // BCODESXP => false,
            // EXTPTRSXP => false,
            // WEAKREFSXP => false,
            RAWSXP => {
                let slice = self.as_u8_slice().unwrap();
                if slice.len() == 1 {
                    write!(f, "{}", slice[0])
                } else {
                    write!(f, "{:?}", slice)
                }
            }
            // S4SXP => false,
            // NEWSXP => false,
            // FREESXP => false,
            _ => write!(f, "??"),
        }
    }
}

// Internal utf8 to str conversion.
// Lets not worry about non-ascii/unicode strings for now (or ever).
unsafe fn to_str<'a>(ptr: *const u8) -> &'a str {
    let mut len = 0;
    loop {
        if *ptr.offset(len) == 0 {
            break;
        }
        len += 1;
    }
    let slice = std::slice::from_raw_parts(ptr, len as usize);
    std::str::from_utf8_unchecked(slice)
}

/// Borrow an already protected SEXP
/// Note that the SEXP must outlive the generated object.
impl From<SEXP> for Robj {
    fn from(sexp: SEXP) -> Self {
        unsafe { new_borrowed(sexp) }
    }
}

/// Release any owned objects.
impl Drop for Robj {
    fn drop(&mut self) {
        unsafe {
            match self {
                Robj::Owned(sexp) => R_ReleaseObject(*sexp),
                Robj::Borrowed(_) => (),
                Robj::Sys(_) => (),
            }
        }
    }
}

/// Convert a null to an Robj.
impl From<()> for Robj {
    fn from(_: ()) -> Self {
        // Note: we do not need to protect this.
        unsafe { Robj::Sys(R_NilValue) }
    }
}

/// Convert a boolean to an Robj.
impl From<bool> for Robj {
    fn from(val: bool) -> Self {
        unsafe { new_owned(Rf_ScalarLogical(val as raw::c_int)) }
    }
}

macro_rules! impl_from_int_prim {
    ($t : ty) => {
        impl From<$t> for Robj {
            fn from(val: $t) -> Self {
                unsafe { new_owned(Rf_ScalarInteger(val as raw::c_int)) }
            }
        }
    };
}

impl_from_int_prim!(u8);
impl_from_int_prim!(u16);
impl_from_int_prim!(u32);
impl_from_int_prim!(u64);
impl_from_int_prim!(i8);
impl_from_int_prim!(i16);
impl_from_int_prim!(i32);
impl_from_int_prim!(i64);

macro_rules! impl_from_float_prim {
    ($t : ty) => {
        impl From<$t> for Robj {
            fn from(val: $t) -> Self {
                unsafe { new_owned(Rf_ScalarReal(val as raw::c_double)) }
            }
        }
    };
}

impl_from_float_prim!(f32);
impl_from_float_prim!(f64);

/// Convert a length value to an Robj.
/// Note: This is good only up to 2^53, but that exceeds the address space
/// of current generation computers (8PiB)
impl From<usize> for Robj {
    fn from(val: usize) -> Self {
        unsafe {
            new_owned(if val >= 0x80000000 {
                Rf_ScalarReal(val as raw::c_double)
            } else {
                Rf_ScalarInteger(val as raw::c_int)
            })
        }
    }
}

/// Convert a wrapped string ref to an Robj char object.
impl<'a> From<Character<'a>> for Robj {
    fn from(val: Character) -> Self {
        unsafe {
            new_owned(Rf_mkCharLen(
                val.0.as_ptr() as *const raw::c_char,
                val.0.len() as i32,
            ))
        }
    }
}

/// Convert a wrapped string ref to an Robj language object.
impl<'a> From<Lang<'a>> for Robj {
    fn from(val: Lang<'a>) -> Self {
        unsafe {
            let mut name = Vec::from(val.0.as_bytes());
            name.push(0);
            new_owned(Rf_lang1(Rf_install(name.as_ptr() as *const raw::c_char)))
        }
    }
}

/// Convert a string ref to an Robj string array object.
impl<'a> From<&'a str> for Robj {
    fn from(val: &str) -> Self {
        unsafe {
            let sexp = Rf_allocVector(STRSXP, 1);
            R_PreserveObject(sexp);
            let ssexp = Rf_mkCharLen(val.as_ptr() as *const raw::c_char, val.len() as i32);
            let ptr = STRING_PTR(sexp);
            let slice = std::slice::from_raw_parts_mut(ptr, 1);
            slice[0] = ssexp;
            Robj::Owned(sexp)
        }
    }
}

impl<'a> From<&'a [&str]> for Robj {
    fn from(vals: &'a [&str]) -> Self {
        unsafe {
            let len = vals.len();
            let sexp = Rf_allocVector(STRSXP, len as R_xlen_t);
            R_PreserveObject(sexp);
            for (idx, &v) in vals.iter().enumerate() {
                SET_STRING_ELT(
                    sexp,
                    idx as isize,
                    Rf_mkCharLen(v.as_ptr() as *const raw::c_char, v.len() as i32),
                );
            }
            Robj::Owned(sexp)
        }
    }
}

/// Convert an integer slice to an integer object.
impl<'a> From<&'a [i32]> for Robj {
    fn from(vals: &[i32]) -> Self {
        unsafe {
            let len = vals.len();
            let sexp = Rf_allocVector(INTSXP, len as R_xlen_t);
            R_PreserveObject(sexp);
            let ptr = INTEGER(sexp);
            let slice = std::slice::from_raw_parts_mut(ptr, len);
            for (i, &v) in vals.iter().enumerate() {
                slice[i] = v;
            }
            Robj::Owned(sexp)
        }
    }
}

/// Convert a bool slice to a logical object.
impl From<&[bool]> for Robj {
    fn from(vals: &[bool]) -> Self {
        unsafe {
            let len = vals.len();
            let sexp = Rf_allocVector(LGLSXP, len as R_xlen_t);
            R_PreserveObject(sexp);
            let ptr = LOGICAL(sexp);
            let slice = std::slice::from_raw_parts_mut(ptr, len);
            for (i, &v) in vals.iter().enumerate() {
                slice[i] = v as i32;
            }
            Robj::Owned(sexp)
        }
    }
}

/// Convert a double slice to a numeric object.
impl From<&[f64]> for Robj {
    fn from(vals: &[f64]) -> Self {
        unsafe {
            let len = vals.len();
            let sexp = Rf_allocVector(REALSXP, len as R_xlen_t);
            R_PreserveObject(sexp);
            let ptr = REAL(sexp);
            let slice = std::slice::from_raw_parts_mut(ptr, len);
            for (i, &v) in vals.iter().enumerate() {
                slice[i] = v;
            }
            Robj::Owned(sexp)
        }
    }
}

/// Convert a byte slice to a raw object.
impl From<&[u8]> for Robj {
    fn from(vals: &[u8]) -> Self {
        unsafe {
            let len = vals.len();
            let sexp = Rf_allocVector(RAWSXP, len as R_xlen_t);
            R_PreserveObject(sexp);
            let ptr = RAW(sexp);
            let slice = std::slice::from_raw_parts_mut(ptr, len);
            for (i, &v) in vals.iter().enumerate() {
                slice[i] = v;
            }
            Robj::Owned(sexp)
        }
    }
}

/// Convert vectors of strings to an R object.
impl<T: AsRef<str>> From<Vec<T>> for Robj {
    fn from(vals: Vec<T>) -> Self {
        unsafe {
            // Create a vector an put it on the R_PreciousList
            let sexp = Rf_allocVector(STRSXP, vals.len() as R_xlen_t);
            R_PreserveObject(sexp);

            // populate the slice with character objects.
            // note: a better way would be to steal the allocated buffer from the strings,
            for (i, s) in vals.iter().enumerate() {
                // note that SET_STRING_ELT is more than a store.
                SET_STRING_ELT(sexp, i as R_xlen_t, Rf_mkCharLen(
                    s.as_ref().as_ptr() as *const raw::c_char,
                    s.as_ref().len() as i32,
                ));
            }

            // The sexp is already protected but we need to unprotect it when it dies.
            Robj::Owned(sexp)
        }
    }
}

// Iterator over the objects in a vector or string.
#[derive(Clone)]
pub struct VecIter {
    vector: SEXP,
    i: usize,
    len: usize,
}

impl Iterator for VecIter {
    type Item = Robj;

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.i;
        self.i += 1;
        if i >= self.len {
            return None;
        } else {
            Some(Robj::from(unsafe { VECTOR_ELT(self.vector, i as isize) }))
        }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.i += n;
        self.next()
    }
}

// Iterator over the objects in a vector or string.
#[derive(Clone)]
pub struct ListIter {
    list_elem: SEXP,
}

impl Iterator for ListIter {
    type Item = Robj;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let sexp = self.list_elem;
            if sexp == R_NilValue {
                None
            } else {
                self.list_elem = CDR(sexp);
                Some(new_borrowed(CAR(sexp)))
            }
        }
    }
}

#[derive(Clone)]
pub struct StrIter {
    vector: SEXP,
    i: usize,
    len: usize,
}

impl Iterator for StrIter {
    type Item = &'static str;

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.i;
        self.i += 1;
        if i >= self.len {
            return None;
        } else {
            unsafe {
                let sexp = STRING_ELT(self.vector, i as isize);
                let ptr = R_CHAR(sexp) as *const u8;
                let slice = std::slice::from_raw_parts(ptr, Rf_xlength(sexp) as usize);
                Some(std::str::from_utf8_unchecked(slice))
            }
        }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.i += n;
        self.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::*;

    #[test]
    fn test_debug() {
        // Special values
        assert_eq!(format!("{:?}", Robj::from(NULL)), "NULL");
        assert_eq!(format!("{:?}", Robj::from(TRUE)), "TRUE");
        assert_eq!(format!("{:?}", Robj::from(FALSE)), "FALSE");

        // Scalars
        assert_eq!(format!("{:?}", Robj::from(1)), "1");
        assert_eq!(format!("{:?}", Robj::from(1.)), "1.0");
        assert_eq!(format!("{:?}", Robj::from("hello")), "[\"hello\"]");

        // Vectors
        assert_eq!(format!("{:?}", Robj::from(&[1, 2, 3][..])), "[1, 2, 3]");
        assert_eq!(
            format!("{:?}", Robj::from(&[1., 2., 3.][..])),
            "[1.0, 2.0, 3.0]"
        );
        assert_eq!(
            format!("{:?}", Robj::from(&[1_u8, 2_u8, 3_u8][..])),
            "[1, 2, 3]"
        );

        // Wrappers
        assert_eq!(format!("{:?}", Robj::from(Symbol("x"))), "Symbol(\"x\")");
        assert_eq!(
            format!("{:?}", Robj::from(Character("x"))),
            "Character(\"x\")"
        );
        assert_eq!(
            format!("{:?}", Robj::from(Lang("x"))),
            "Lang([Symbol(\"x\")])"
        );

        // Logical
        assert_eq!(
            format!("{:?}", Robj::from(&[Bool(1), Bool(0)][..])),
            "&[Bool(1), Bool(0)]"
        );
    }

    #[test]
    fn test_from_robj() {
        assert_eq!(<u8>::from_robj(&Robj::from(1)), Ok(1));
        assert_eq!(<u16>::from_robj(&Robj::from(1)), Ok(1));
        assert_eq!(<u32>::from_robj(&Robj::from(1)), Ok(1));
        assert_eq!(<u64>::from_robj(&Robj::from(1)), Ok(1));
        assert_eq!(<i8>::from_robj(&Robj::from(1)), Ok(1));
        assert_eq!(<i16>::from_robj(&Robj::from(1)), Ok(1));
        assert_eq!(<i32>::from_robj(&Robj::from(1)), Ok(1));
        assert_eq!(<i64>::from_robj(&Robj::from(1)), Ok(1));
        assert_eq!(<f32>::from_robj(&Robj::from(1)), Ok(1.));
        assert_eq!(<f64>::from_robj(&Robj::from(1)), Ok(1.));
        assert_eq!(<Vec::<i32>>::from_robj(&Robj::from(1)), Ok(vec![1]));
        assert_eq!(<Vec::<f64>>::from_robj(&Robj::from(1.)), Ok(vec![1.]));
        assert_eq!(
            <ArrayView1<f64>>::from_robj(&Robj::from(1.)),
            Ok(ArrayView1::<f64>::from(&[1.][..]))
        );
        assert_eq!(
            <ArrayView1<i32>>::from_robj(&Robj::from(1)),
            Ok(ArrayView1::<i32>::from(&[1][..]))
        );
        assert_eq!(
            <ArrayView1<Bool>>::from_robj(&Robj::from(true)),
            Ok(ArrayView1::<Bool>::from(&[Bool(1)][..]))
        );
        assert_eq!(
            <ArrayView2<f64>>::from_robj(&Robj::from(1.)),
            Ok(ArrayView2::<f64>::from_shape((1, 1), &[1.][..]).unwrap())
        );
        assert_eq!(
            <ArrayView2<i32>>::from_robj(&Robj::from(1)),
            Ok(ArrayView2::<i32>::from_shape((1, 1), &[1][..]).unwrap())
        );
        assert_eq!(
            <ArrayView2<Bool>>::from_robj(&Robj::from(true)),
            Ok(ArrayView2::<Bool>::from_shape((1, 1), &[Bool(1)][..]).unwrap())
        );

        assert_eq!(
            <ArrayView2<f64>>::from_robj(
                &Robj::eval_string("matrix(c(1, 2, 3, 4, 5, 6, 7, 8), ncol=2, nrow=4, byrow=T)")
                    .unwrap()
            ),
            Ok(ArrayView2::<f64>::from_shape(
                (4, 2),
                &[1f64, 2f64, 3f64, 4f64, 5f64, 6f64, 7f64, 8f64][..]
            )
            .unwrap())
        );

        let hello = Robj::from("hello");
        assert_eq!(<&str>::from_robj(&hello), Ok("hello"));
    }
    #[test]
    fn test_to_robj() {
        assert_eq!(Robj::from(1_u8), Robj::from(1));
        assert_eq!(Robj::from(1_u16), Robj::from(1));
        assert_eq!(Robj::from(1_u32), Robj::from(1));
        assert_eq!(Robj::from(1_u64), Robj::from(1));
        assert_eq!(Robj::from(1_i8), Robj::from(1));
        assert_eq!(Robj::from(1_i16), Robj::from(1));
        assert_eq!(Robj::from(1_i32), Robj::from(1));
        assert_eq!(Robj::from(1_i64), Robj::from(1));
        assert_eq!(Robj::from(1.0_f32), Robj::from(1.));
        assert_eq!(Robj::from(1.0_f64), Robj::from(1.));

        let ab = Robj::from(vec!["a", "b"]);
        let ab2 = Robj::from(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(ab, ab2);
        assert_eq!(format!("{:?}", ab), "[\"a\", \"b\"]");
        assert_eq!(format!("{:?}", ab2), "[\"a\", \"b\"]");
    }

    #[test]
    fn parse_test() -> Result<(), AnyError> {
        start_r();
        let p = Robj::parse("print(1L);print(1L);")?;
        assert_eq!(
            format!("{:?}", p),
            "Expr([Lang([Symbol(\"print\"), 1]), Lang([Symbol(\"print\"), 1])])"
        );

        let p = Robj::eval_string("1L + 1L")?;
        assert_eq!(p, Robj::from(2));
        Ok(())
    }
}
