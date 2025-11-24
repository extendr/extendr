//! ALTREP support
//!
//! The core functions to support ALTREP in R.
use super::*;

#[allow(non_camel_case_types)]
/// Must return an R value of SEXPTYPE type containing the result of conversion of x into that type, ignoring the attributes.
/// May return a null pointer, making R perform the conversion as usual (see coerceVector). The default method returns a null pointer.
pub type R_altrep_Coerce_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: SEXPTYPE) -> SEXP>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct R_altrep_class_t {
    pub ptr: SEXP,
}

/// Like the Unserialize method above, but must also take care of setting the attributes, the levels field and the object bit on the return value. This is likely impossible to implement within the confines of the R API.
/// The default method calls the Unserialize method, sets the attributes, the object bit, and the levels field, and returns the result.
pub type R_altrep_UnserializeEX_method_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: SEXP,
        arg2: SEXP,
        arg3: SEXP,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
    ) -> SEXP,
>;

/// Must construct and return an ALTREP object of the given class given the data previously prepared by the Serialized_state method above.
/// The default method signals an R-level error.
pub type R_altrep_Unserialize_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: SEXP) -> SEXP>;

/// Must prepare and return a representation of x that can be given to the Unserialize methods below in order to recreate it.
///
/// May return a null pointer, making R materialise the object when saving it. The default method returns a null pointer.
pub type R_altrep_Serialized_state_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP) -> SEXP>;

///  Like the Duplicate method above, but must duplicate the attributes in the same manner.
///  May return a null pointer, in which case R will produce a materialised copy of the object by itself. The default method calls the Duplicate method, duplicates the attributes of x and installs them onto the return value.
pub type R_altrep_DuplicateEX_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: Rboolean) -> SEXP>;

/// Must return a copy (see duplicate) of x that can be altered independently of x, ignoring the attributes. R will duplicate and install the attributes by itself.
///
/// May return a null pointer, in which case R will produce a materialised copy of the object by itself. The default method returns a null pointer.
pub type R_altrep_Duplicate_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: Rboolean) -> SEXP>;

/// This method will be called from .Internal(inspect(...)) after printing the standard SEXP header contents; it can then print (see Rprintf) additional information specific to this ALTREP class. When starting a new line, this method must respect the output indentation pre. Printing the vector elements, if done, should be subject to the recommended vector printing length pvec. The method should call inspect_subtree for child elements, subject to the limits in deep.
/// If the return value is TRUE, R will consider the value inspected and won’t access its contents; otherwise R will then inspect the contents of x as if it was an ordinary value. Either way, R will then take care of inspecting the attributes of x.
/// The default method returns FALSE.
/// Versions: Appeared in 3.5.0.
pub type R_altrep_Inspect_method_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: SEXP,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: SEXP,
                arg2: ::std::os::raw::c_int,
                arg3: ::std::os::raw::c_int,
                arg4: ::std::os::raw::c_int,
            ),
        >,
    ) -> Rboolean,
>;

/// Must return the length of the compact-representation vector, or signal an error. The default method signals an error.
pub type R_altrep_Length_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP) -> R_xlen_t>;

/// Must return a pointer to the start of the buffer of type corresponding to the SEXPTYPE of x and containing XLENGTH(x) elements (see DATAPTR_RO) corresponding to the contents of x. May signal an R error instead.
///  The default method always signals an error.
pub type R_altvec_Dataptr_method_t = ::std::option::Option<
    unsafe extern "C" fn(arg1: SEXP, arg2: Rboolean) -> *mut ::std::os::raw::c_void,
>;

/// Like the Dataptr method above, but the method may return a null pointer to indicate a preference for *_Elt (see REAL_ELT) and *_Get_region (see REAL_GET_REGION) methods instead of full buffer access. The return value will not be used to modify x.
/// The default method always returns a null pointer.
pub type R_altvec_Dataptr_or_null_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP) -> *const ::std::os::raw::c_void>;
/// Must allocate and return a new vector containing the result of subsetting, `x[indx]`, following the usual semantics of the [ operator for numeric subscripts. For values outside the range of x, missing values must be returned (R_NilValue for VECSXP lists, 0 for RAWSXP vectors). May also return a null pointer (see above).
///
/// The default method always returns a null pointer.
/// Versions: Appeared in 3.5.0.
pub type R_altvec_Extract_subset_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: SEXP, arg3: SEXP) -> SEXP>;
/// Must return `x[i]` for the 0-based index i. The default method returns `INTEGER(x)[i]`.
pub type R_altinteger_Elt_method_t = ::std::option::Option<
    unsafe extern "C" fn(arg1: SEXP, arg2: R_xlen_t) -> ::std::os::raw::c_int,
>;
/// Must copy up to n elements, starting at 0-based index i, into `buf[n]`, and return the number of elements copied. The default method defers to INTEGER_ELT and hence the Elt method above.
pub type R_altinteger_Get_region_method_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: SEXP,
        arg2: R_xlen_t,
        arg3: R_xlen_t,
        arg4: *mut ::std::os::raw::c_int,
    ) -> R_xlen_t,
>;
/// Must return one of the sortedness constants: SORTED_DECR, UNKNOWN_SORTEDNESS, SORTED_INCR. See INTEGER_IS_SORTED. The default method always returns UNKNOWN_SORTEDNESS.
pub type R_altinteger_Is_sorted_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP) -> ::std::os::raw::c_int>;
/// Must return 1 if x is known not to store any missing values (see INTEGER_NO_NA), otherwise 0 (including if it’s unknown). The default method always returns 0.
pub type R_altinteger_No_NA_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP) -> ::std::os::raw::c_int>;
/// Must calculate and return the specified summary of the contents of x, ignoring missing values if narm is TRUE. May return a null pointer, in which case R will compute the summary by accessing the elements of the vector.
pub type R_altinteger_Sum_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: Rboolean) -> SEXP>;
/// Must calculate and return the specified summary of the contents of x, ignoring missing values if narm is TRUE. May return a null pointer, in which case R will compute the summary by accessing the elements of the vector.
pub type R_altinteger_Min_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: Rboolean) -> SEXP>;
/// Must calculate and return the specified summary of the contents of x, ignoring missing values if narm is TRUE. May return a null pointer, in which case R will compute the summary by accessing the elements of the vector.
pub type R_altinteger_Max_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: Rboolean) -> SEXP>;
/// Must return `x[i]` for the 0-based index i. The default method returns `REAL(x)[i]`.
pub type R_altreal_Elt_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: R_xlen_t) -> f64>;
/// Must copy up to n elements, starting at 0-based index i, into `buf[n]`, and return the number of elements copied. The default method defers to REAL_ELT and hence the Elt method above.
pub type R_altreal_Get_region_method_t = ::std::option::Option<
    unsafe extern "C" fn(arg1: SEXP, arg2: R_xlen_t, arg3: R_xlen_t, arg4: *mut f64) -> R_xlen_t,
>;
///Must return one of the sortedness constants: SORTED_DECR, UNKNOWN_SORTEDNESS, SORTED_INCR. See REAL_IS_SORTED. The default method always returns UNKNOWN_SORTEDNESS.
pub type R_altreal_Is_sorted_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP) -> ::std::os::raw::c_int>;
/// Must return 1 if x is known not to store any missing values (see REAL_NO_NA), otherwise 0 (including if it’s unknown). The default method always returns 0.
pub type R_altreal_No_NA_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP) -> ::std::os::raw::c_int>;
/// Must calculate and return the specified summary of the contents of x, ignoring missing values if narm is TRUE. May return a null pointer, in which case R will compute the summary by accessing the elements of the vector.
pub type R_altreal_Sum_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: Rboolean) -> SEXP>;
/// Must calculate and return the specified summary of the contents of x, ignoring missing values if narm is TRUE. May return a null pointer, in which case R will compute the summary by accessing the elements of the vector.
pub type R_altreal_Min_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: Rboolean) -> SEXP>;
/// Must calculate and return the specified summary of the contents of x, ignoring missing values if narm is TRUE. May return a null pointer, in which case R will compute the summary by accessing the elements of the vector.
pub type R_altreal_Max_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: Rboolean) -> SEXP>;
/// Must return `x[i]` for the 0-based index i. The default method returns `LOGICAL(x)[i]`.
pub type R_altlogical_Elt_method_t = ::std::option::Option<
    unsafe extern "C" fn(arg1: SEXP, arg2: R_xlen_t) -> ::std::os::raw::c_int,
>;
/// Must copy up to n elements, starting at 0-based index i, into `buf[n]`, and return the number of elements copied. The default method defers to LOGICAL_ELT and hence the Elt method above.
pub type R_altlogical_Get_region_method_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: SEXP,
        arg2: R_xlen_t,
        arg3: R_xlen_t,
        arg4: *mut ::std::os::raw::c_int,
    ) -> R_xlen_t,
>;
/// Must return one of the sortedness constants: SORTED_DECR, UNKNOWN_SORTEDNESS, SORTED_INCR. See INTEGER_IS_SORTED. The default method always returns UNKNOWN_SORTEDNESS
pub type R_altlogical_Is_sorted_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP) -> ::std::os::raw::c_int>;
/// Must return 1 if x is known not to store any missing values (see LOGICAL_NO_NA), otherwise 0 (including if it’s unknown). The default method always returns 0.
pub type R_altlogical_No_NA_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP) -> ::std::os::raw::c_int>;
/// Must calculate and return the specified summary of the contents of x, ignoring missing values if narm is TRUE. May return a null pointer, in which case R will compute the summary by accessing the elements of the vector.
/// Versions: Appeared in 3.6.0.
pub type R_altlogical_Sum_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: Rboolean) -> SEXP>;
/// Must return `x[i]` for the 0-based index i. The default method returns `RAW(x)[i]`.
pub type R_altraw_Elt_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: R_xlen_t) -> Rbyte>;
/// Must copy up to n elements, starting at 0-based index i, into `buf[n]`, and return the number of elements copied. The default method defers to `RAW_ELT` and hence the Elt method above.
/// Versions: Appeared in 3.6.0.
pub type R_altraw_Get_region_method_t = ::std::option::Option<
    unsafe extern "C" fn(arg1: SEXP, arg2: R_xlen_t, arg3: R_xlen_t, arg4: *mut Rbyte) -> R_xlen_t,
>;
/// Must return `x[i]` for the 0-based index i. The default method returns `COMPLEX(x)[i]`.
pub type R_altcomplex_Elt_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: R_xlen_t) -> Rcomplex>;
/// Must copy up to n elements, starting at 0-based index i, into `buf[n]`, and return the number of elements copied. The default method defers to COMPLEX_ELT and hence the Elt method above.
/// Versions: Appeared in 3.6.0.
pub type R_altcomplex_Get_region_method_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: SEXP,
        arg2: R_xlen_t,
        arg3: R_xlen_t,
        arg4: *mut Rcomplex,
    ) -> R_xlen_t,
>;
/// Must return `x[i]` (for 0-based i) or signal an error. The default method always raises an error.
pub type R_altstring_Elt_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: R_xlen_t) -> SEXP>;
/// Must set `x[i] <- v` (for 0-based i), even if it’s not compatible with the chosen compact representation, or signal an error. The default method always raises an error.
pub type R_altstring_Set_elt_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: R_xlen_t, arg3: SEXP)>;
/// Must return one of the sortedness constants: SORTED_DECR, UNKNOWN_SORTEDNESS, SORTED_INCR. See STRING_IS_SORTED. The default method always returns UNKNOWN_SORTEDNESS.
pub type R_altstring_Is_sorted_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP) -> ::std::os::raw::c_int>;
/// Must return 1 if x is known not to store any missing values (see STRING_NO_NA), otherwise 0 (including if it’s unknown). The default method always returns 0.
pub type R_altstring_No_NA_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP) -> ::std::os::raw::c_int>;
/// Must return `x[[i]]` for the 0-based index i or raise an error.
pub type R_altlist_Elt_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: R_xlen_t) -> SEXP>;
/// Must set `x[[i]] <- v` for the 0-based index i, altering x, or raise an error.
pub type R_altlist_Set_elt_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: R_xlen_t, arg3: SEXP)>;

extern "C" {
    /// Returns a nonzero value if x is a “compact representation” (ALTREP) value. Access to the data pointers of ALTREP values may be problematic and should be replaced with access to individual elements (see REAL_ELT) or vector subregions (see REAL_GET_REGION). See also: <https://svn.r-project.org/R/branches/ALTREP/ALTREP.html>
    pub fn ALTREP(x: SEXP) -> ::std::os::raw::c_int;
    /// Returns the ALTREP class object containing the methods set up for use on ax. The return value is of unspecified SEXPTYPE. Behaviour is undefined if ax is not an ALTREP object.
    pub fn ALTREP_CLASS(x: SEXP) -> SEXP;
    /// Returns one of the two ALTREP instance variables belonging to ax. Behaviour is undefined if ax is not an ALTREP object.
    pub fn R_altrep_data1(x: SEXP) -> SEXP;
    /// Returns one of the two ALTREP instance variables belonging to ax. Behaviour is undefined if ax is not an ALTREP object.
    pub fn R_altrep_data2(x: SEXP) -> SEXP;
    /// Allocates, constructs and returns a compact-representation value. It will have the TYPEOF() of the corresponding class and will use the methods in aclass and the instance variables to respond to calls such as XLENGTH().
    pub fn R_new_altrep(aclass: R_altrep_class_t, data1: SEXP, data2: SEXP) -> SEXP;
    /// Allocates, registers and returns an ALTREP class with the given names.
    /// In addition to the methods given here, compact representation string vectors must implement the common “altrep” methods (see R_set_altrep_..._method) and the common “altvec” methods (see R_set_altvec_..._method).
    pub fn R_make_altstring_class(
        cname: *const ::std::os::raw::c_char,
        pname: *const ::std::os::raw::c_char,
        info: *mut DllInfo,
    ) -> R_altrep_class_t;
    /// Allocates, registers and returns an ALTREP class with the given names.

    /// In addition to the methods given here, compact representation integer vectors must implement the common “altrep” methods (see R_set_altrep_..._method) and the common “altvec” methods (see R_set_altvec_..._method).
    pub fn R_make_altinteger_class(
        cname: *const ::std::os::raw::c_char,
        pname: *const ::std::os::raw::c_char,
        info: *mut DllInfo,
    ) -> R_altrep_class_t;
    /// Allocates, registers and returns an ALTREP class with the given names.

    /// In addition to the methods given here, compact representation integer vectors must implement the common “altrep” methods (see R_set_altrep_..._method) and the common “altvec” methods (see R_set_altvec_..._method).
    pub fn R_make_altreal_class(
        cname: *const ::std::os::raw::c_char,
        pname: *const ::std::os::raw::c_char,
        info: *mut DllInfo,
    ) -> R_altrep_class_t;
    /// Allocates, registers and returns an ALTREP class with the given names.

    /// In addition to the methods given here, compact representation integer vectors must implement the common “altrep” methods (see R_set_altrep_..._method) and the common “altvec” methods (see R_set_altvec_..._method).
    pub fn R_make_altlogical_class(
        cname: *const ::std::os::raw::c_char,
        pname: *const ::std::os::raw::c_char,
        info: *mut DllInfo,
    ) -> R_altrep_class_t;
    /// Allocates, registers and returns an ALTREP class with the given names.

    /// In addition to the methods given here, compact representation integer vectors must implement the common “altrep” methods (see R_set_altrep_..._method) and the common “altvec” methods (see R_set_altvec_..._method).
    pub fn R_make_altraw_class(
        cname: *const ::std::os::raw::c_char,
        pname: *const ::std::os::raw::c_char,
        info: *mut DllInfo,
    ) -> R_altrep_class_t;
    /// Allocates, registers and returns an ALTREP class with the given names.

    /// In addition to the methods given here, compact representation integer vectors must implement the common “altrep” methods (see R_set_altrep_..._method) and the common “altvec” methods (see R_set_altvec_..._method).
    pub fn R_make_altcomplex_class(
        cname: *const ::std::os::raw::c_char,
        pname: *const ::std::os::raw::c_char,
        info: *mut DllInfo,
    ) -> R_altrep_class_t;
    /// Allocates, registers and returns an ALTREP class with the given names.

    /// In addition to the methods given here, compact representation integer vectors must implement the common “altrep” methods (see R_set_altrep_..._method) and the common “altvec” methods (see R_set_altvec_..._method).
    pub fn R_make_altlist_class(
        cname: *const ::std::os::raw::c_char,
        pname: *const ::std::os::raw::c_char,
        info: *mut DllInfo,
    ) -> R_altrep_class_t;
    /// Returns a nonzero value if x is an ALTREP value whose class pointer is class.
    pub fn R_altrep_inherits(x: SEXP, arg1: R_altrep_class_t) -> Rboolean;
    /// Sets the function pointer inside the ALTREP class object. Currently, these methods are common to all ALTREP classes. There is no facility for unregistering ALTREP classes, so once a function pointer is set from a package shared library, it must not be unloaded.
    pub fn R_set_altrep_UnserializeEX_method(
        cls: R_altrep_class_t,
        fun: R_altrep_UnserializeEX_method_t,
    );
    /// Sets the function pointer inside the ALTREP class object. Currently, these methods are common to all ALTREP classes. There is no facility for unregistering ALTREP classes, so once a function pointer is set from a package shared library, it must not be unloaded.
    pub fn R_set_altrep_Unserialize_method(
        cls: R_altrep_class_t,
        fun: R_altrep_Unserialize_method_t,
    );
    /// Sets the function pointer inside the ALTREP class object. Currently, these methods are common to all ALTREP classes. There is no facility for unregistering ALTREP classes, so once a function pointer is set from a package shared library, it must not be unloaded.
    pub fn R_set_altrep_Serialized_state_method(
        cls: R_altrep_class_t,
        fun: R_altrep_Serialized_state_method_t,
    );
    /// Sets the corresponding ALTREP instance variable of ax to v, sharing it without duplication. Behaviour is undefined if ax is not an ALTREP object.
    /// Versions: Appeared in 3.5.0.
    pub fn R_set_altrep_data1(x: SEXP, v: SEXP);
    /// Sets the corresponding ALTREP instance variable of ax to v, sharing it without duplication. Behaviour is undefined if ax is not an ALTREP object.
    /// Versions: Appeared in 3.5.0.
    pub fn R_set_altrep_data2(x: SEXP, v: SEXP);
    ///Sets the function pointer inside the ALTREP class object. Currently, these methods are common to all ALTREP classes. There is no facility for unregistering ALTREP classes, so once a function pointer is set from a package shared library, it must not be unloaded.
    pub fn R_set_altrep_DuplicateEX_method(
        cls: R_altrep_class_t,
        fun: R_altrep_DuplicateEX_method_t,
    );
    /// Sets the function pointer inside the ALTREP class object. Currently, these methods are common to all ALTREP classes. There is no facility for unregistering ALTREP classes, so once a function pointer is set from a package shared library, it must not be unloaded.
    pub fn R_set_altrep_Duplicate_method(cls: R_altrep_class_t, fun: R_altrep_Duplicate_method_t);
    /// Sets the function pointer inside the ALTREP class object. Currently, these methods are common to all ALTREP classes. There is no facility for unregistering ALTREP classes, so once a function pointer is set from a package shared library, it must not be unloaded.
    pub fn R_set_altrep_Coerce_method(cls: R_altrep_class_t, fun: R_altrep_Coerce_method_t);
    /// Sets the function pointer inside the ALTREP class object. Currently, these methods are common to all ALTREP classes. There is no facility for unregistering ALTREP classes, so once a function pointer is set from a package shared library, it must not be unloaded.
    pub fn R_set_altrep_Inspect_method(cls: R_altrep_class_t, fun: R_altrep_Inspect_method_t);
    /// Sets the function pointer inside the ALTREP class object. Currently, these methods are common to all ALTREP classes. There is no facility for unregistering ALTREP classes, so once a function pointer is set from a package shared library, it must not be unloaded.
    pub fn R_set_altrep_Length_method(cls: R_altrep_class_t, fun: R_altrep_Length_method_t);
    /// Sets the function pointer inside the ALTREP class object. Currently, all ALTREP classes are vector classes. There is no facility for unregistering ALTREP classes, so once a function pointer is set from a package shared library, it must not be unloaded.
    pub fn R_set_altvec_Dataptr_method(cls: R_altrep_class_t, fun: R_altvec_Dataptr_method_t);
    /// Sets the function pointer inside the ALTREP class object. Currently, all ALTREP classes are vector classes. There is no facility for unregistering ALTREP classes, so once a function pointer is set from a package shared library, it must not be unloaded.
    pub fn R_set_altvec_Dataptr_or_null_method(
        cls: R_altrep_class_t,
        fun: R_altvec_Dataptr_or_null_method_t,
    );
    /// Sets the function pointer inside the ALTREP class object. Currently, all ALTREP classes are vector classes. There is no facility for unregistering ALTREP classes, so once a function pointer is set from a package shared library, it must not be unloaded.
    pub fn R_set_altvec_Extract_subset_method(
        cls: R_altrep_class_t,
        fun: R_altvec_Extract_subset_method_t,
    );
    /// Sets the function pointer inside the ALTREP class object. Behaviour is undefined if cls is not a compact integer vector class. There is no facility for unregistering ALTREP classes, so once a function pointer is set from a package shared library, it must not be unloaded.
    pub fn R_set_altinteger_Elt_method(cls: R_altrep_class_t, fun: R_altinteger_Elt_method_t);
    /// Sets the function pointer inside the ALTREP class object. Behaviour is undefined if cls is not a compact integer vector class. There is no facility for unregistering ALTREP classes, so once a function pointer is set from a package shared library, it must not be unloaded.
    pub fn R_set_altinteger_Get_region_method(
        cls: R_altrep_class_t,
        fun: R_altinteger_Get_region_method_t,
    );
    /// Sets the function pointer inside the ALTREP class object. Behaviour is undefined if cls is not a compact integer vector class. There is no facility for unregistering ALTREP classes, so once a function pointer is set from a package shared library, it must not be unloaded.
    pub fn R_set_altinteger_Is_sorted_method(
        cls: R_altrep_class_t,
        fun: R_altinteger_Is_sorted_method_t,
    );
    /// Sets the function pointer inside the ALTREP class object. Behaviour is undefined if cls is not a compact integer vector class. There is no facility for unregistering ALTREP classes, so once a function pointer is set from a package shared library, it must not be unloaded.
    pub fn R_set_altinteger_No_NA_method(cls: R_altrep_class_t, fun: R_altinteger_No_NA_method_t);
    /// Sets the function pointer inside the ALTREP class object. Behaviour is undefined if cls is not a compact integer vector class. There is no facility for unregistering ALTREP classes, so once a function pointer is set from a package shared library, it must not be unloaded.
    pub fn R_set_altinteger_Sum_method(cls: R_altrep_class_t, fun: R_altinteger_Sum_method_t);
    /// Sets the function pointer inside the ALTREP class object. Behaviour is undefined if cls is not a compact integer vector class. There is no facility for unregistering ALTREP classes, so once a function pointer is set from a package shared library, it must not be unloaded.
    pub fn R_set_altinteger_Min_method(cls: R_altrep_class_t, fun: R_altinteger_Min_method_t);
    /// Sets the function pointer inside the ALTREP class object. Behaviour is undefined if cls is not a compact integer vector class. There is no facility for unregistering ALTREP classes, so once a function pointer is set from a package shared library, it must not be unloaded.
    pub fn R_set_altinteger_Max_method(cls: R_altrep_class_t, fun: R_altinteger_Max_method_t);
    /// Sets the function pointer inside the ALTREP class object. Behaviour is undefined if cls is not a compact real vector class. There is no facility for unregistering ALTREP classes, so once a function pointer is set from a package shared library, it must not be unloaded.
    pub fn R_set_altreal_Elt_method(cls: R_altrep_class_t, fun: R_altreal_Elt_method_t);
    /// Sets the function pointer inside the ALTREP class object. Behaviour is undefined if cls is not a compact real vector class. There is no facility for unregistering ALTREP classes, so once a function pointer is set from a package shared library, it must not be unloaded.
    pub fn R_set_altreal_Get_region_method(
        cls: R_altrep_class_t,
        fun: R_altreal_Get_region_method_t,
    );
    /// Sets the function pointer inside the ALTREP class object. Behaviour is undefined if cls is not a compact real vector class. There is no facility for unregistering ALTREP classes, so once a function pointer is set from a package shared library, it must not be unloaded.
    pub fn R_set_altreal_Is_sorted_method(cls: R_altrep_class_t, fun: R_altreal_Is_sorted_method_t);
    /// Sets the function pointer inside the ALTREP class object. Behaviour is undefined if cls is not a compact real vector class. There is no facility for unregistering ALTREP classes, so once a function pointer is set from a package shared library, it must not be unloaded.
    pub fn R_set_altreal_No_NA_method(cls: R_altrep_class_t, fun: R_altreal_No_NA_method_t);
    /// Sets the function pointer inside the ALTREP class object. Behaviour is undefined if cls is not a compact real vector class. There is no facility for unregistering ALTREP classes, so once a function pointer is set from a package shared library, it must not be unloaded.
    pub fn R_set_altreal_Sum_method(cls: R_altrep_class_t, fun: R_altreal_Sum_method_t);
    /// Sets the function pointer inside the ALTREP class object. Behaviour is undefined if cls is not a compact real vector class. There is no facility for unregistering ALTREP classes, so once a function pointer is set from a package shared library, it must not be unloaded.
    pub fn R_set_altreal_Min_method(cls: R_altrep_class_t, fun: R_altreal_Min_method_t);
    /// Sets the function pointer inside the ALTREP class object. Behaviour is undefined if cls is not a compact real vector class. There is no facility for unregistering ALTREP classes, so once a function pointer is set from a package shared library, it must not be unloaded.
    pub fn R_set_altreal_Max_method(cls: R_altrep_class_t, fun: R_altreal_Max_method_t);
    /// Sets the function pointer inside the ALTREP class object. Behaviour is undefined if cls is not a compact integer vector class. There is no facility for unregistering ALTREP classes, so once a function pointer is set from a package shared library, it must not be unloaded.
    pub fn R_set_altlogical_Elt_method(cls: R_altrep_class_t, fun: R_altlogical_Elt_method_t);
    /// Sets the function pointer inside the ALTREP class object. Behaviour is undefined if cls is not a compact integer vector class. There is no facility for unregistering ALTREP classes, so once a function pointer is set from a package shared library, it must not be unloaded.
    pub fn R_set_altlogical_Get_region_method(
        cls: R_altrep_class_t,
        fun: R_altlogical_Get_region_method_t,
    );
    /// Sets the function pointer inside the ALTREP class object. Behaviour is undefined if cls is not a compact integer vector class. There is no facility for unregistering ALTREP classes, so once a function pointer is set from a package shared library, it must not be unloaded.
    pub fn R_set_altlogical_Is_sorted_method(
        cls: R_altrep_class_t,
        fun: R_altlogical_Is_sorted_method_t,
    );
    /// Sets the function pointer inside the ALTREP class object. Behaviour is undefined if cls is not a compact integer vector class. There is no facility for unregistering ALTREP classes, so once a function pointer is set from a package shared library, it must not be unloaded.
    pub fn R_set_altlogical_No_NA_method(cls: R_altrep_class_t, fun: R_altlogical_No_NA_method_t);
    /// Sets the function pointer inside the ALTREP class object. Behaviour is undefined if cls is not a compact integer vector class. There is no facility for unregistering ALTREP classes, so once a function pointer is set from a package shared library, it must not be unloaded.
    pub fn R_set_altlogical_Sum_method(cls: R_altrep_class_t, fun: R_altlogical_Sum_method_t);
    /// Sets the function pointer inside the ALTREP class object. Behaviour is undefined if cls is not a compact complex vector class. There is no facility for unregistering ALTREP classes, so once a function pointer is set from a package shared library, it must not be unloaded.
    pub fn R_set_altraw_Elt_method(cls: R_altrep_class_t, fun: R_altraw_Elt_method_t);
    /// Sets the function pointer inside the ALTREP class object. Behaviour is undefined if cls is not a compact complex vector class. There is no facility for unregistering ALTREP classes, so once a function pointer is set from a package shared library, it must not be unloaded.
    pub fn R_set_altraw_Get_region_method(cls: R_altrep_class_t, fun: R_altraw_Get_region_method_t);
    /// Sets the function pointer inside the ALTREP class object. Behaviour is undefined if cls is not a compact complex vector class. There is no facility for unregistering ALTREP classes, so once a function pointer is set from a package shared library, it must not be unloaded.
    pub fn R_set_altcomplex_Elt_method(cls: R_altrep_class_t, fun: R_altcomplex_Elt_method_t);
    /// Sets the function pointer inside the ALTREP class object. Behaviour is undefined if cls is not a compact complex vector class. There is no facility for unregistering ALTREP classes, so once a function pointer is set from a package shared library, it must not be unloaded.
    pub fn R_set_altcomplex_Get_region_method(
        cls: R_altrep_class_t,
        fun: R_altcomplex_Get_region_method_t,
    );
    /// Sets the function pointer inside the ALTREP class object. Behaviour is undefined if cls is not a compact string representation class. There is no facility for unregistering ALTREP classes, so once a function pointer is set from a package shared library, it must not be unloaded.
    pub fn R_set_altstring_Elt_method(cls: R_altrep_class_t, fun: R_altstring_Elt_method_t);
    /// Sets the function pointer inside the ALTREP class object. Behaviour is undefined if cls is not a compact string representation class. There is no facility for unregistering ALTREP classes, so once a function pointer is set from a package shared library, it must not be unloaded.
    pub fn R_set_altstring_Set_elt_method(cls: R_altrep_class_t, fun: R_altstring_Set_elt_method_t);
    /// Sets the function pointer inside the ALTREP class object. Behaviour is undefined if cls is not a compact string representation class. There is no facility for unregistering ALTREP classes, so once a function pointer is set from a package shared library, it must not be unloaded.
    pub fn R_set_altstring_Is_sorted_method(
        cls: R_altrep_class_t,
        fun: R_altstring_Is_sorted_method_t,
    );
    /// Sets the function pointer inside the ALTREP class object. Behaviour is undefined if cls is not a compact string representation class. There is no facility for unregistering ALTREP classes, so once a function pointer is set from a package shared library, it must not be unloaded.
    pub fn R_set_altstring_No_NA_method(cls: R_altrep_class_t, fun: R_altstring_No_NA_method_t);
    /// Sets the function pointer inside the ALTREP class object. Behaviour is undefined if cls is not a compact complex vector class. There is no facility for unregistering ALTREP classes, so once a function pointer is set from a package shared library, it must not be unloaded.
    pub fn R_set_altlist_Elt_method(cls: R_altrep_class_t, fun: R_altlist_Elt_method_t);
    /// Sets the function pointer inside the ALTREP class object. Behaviour is undefined if cls is not a compact complex vector class. There is no facility for unregistering ALTREP classes, so once a function pointer is set from a package shared library, it must not be unloaded.
    pub fn R_set_altlist_Set_elt_method(cls: R_altrep_class_t, fun: R_altlist_Set_elt_method_t);
}
