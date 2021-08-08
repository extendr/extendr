use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Altrep {
    pub(crate) robj: Robj,
}

/// Rust trait for implementing ALTREP.
/// Implement one or more of these methods to generate an Altrep class.
/// Mechanism TBD.
trait AltRepImpl {
    /// Constructor that is called when loading an Altrep object from a file.
    fn unserialize_ex(
        class: Robj,
        state: Robj,
        attributes: Robj,
        obj_flags: i32,
        levels: i32,
    ) -> Robj {
        let res = Self::unserialize(class, state);
        if !res.is_null() {
            unsafe {
                let val = res.get();
                SET_ATTRIB(val, attributes.get());
                SET_OBJECT(val, obj_flags);
                SETLEVELS(val, levels);
            }
        }
        res
    }

    /// Simplified constructor that is called when loading an Altrep object from a file.
    fn unserialize(_class: Robj, _state: Robj) -> Robj {
        ().into()
    }

    /// Fetch the state of this object when writing to a file.
    fn serialized_state(&self) -> Robj {
        ().into()
    }

    /// Duplicate this object, possibly duplicating attributes.
    fn duplicate_ex(&self, _deep: bool) -> Robj {
        ().into()
    }

    /// Duplicate this object. Called by Rf_duplicate.
    fn duplicate(&self, _deep: bool) -> Robj {
        ().into()
    }

    /// Coerce this object into some other type, if possible.
    fn coerce(&self, _ty: RType) -> Robj {
        ().into()
    }

    /// Print the text for .Internal(inspect(obj))
    fn inspect(
        &self,
        _pre: i32,
        _deep: i32,
        _pvec: i32,
        _inspect_subtree: fn(robj: Robj, pre: i32, deep: i32, pvec: i32),
    ) -> bool {
        false
    }

    /// Get the virtual length of the vector.
    /// For example for a compact range, return end - start + 1.
    fn length(&self) -> usize {
        0
    }
}

trait AltVecImpl : AltRepImpl {
    /// Get the data pointer for this vector, possibly expanding the
    /// compact representation into a full R vector.
    fn dataptr(&self, x: SEXP, _writeable: bool) -> *mut u8 {
        unsafe {
            let data2 = R_altrep_data2(x);
            if data2 == R_NilValue || TYPEOF(data2) != INTSXP as i32 {
                Rf_protect(x);
                let len = ALTREP_LENGTH(x);
                let data2 = Rf_allocVector(INTSXP, len as R_xlen_t);
                R_set_altrep_data2(x, data2);
                INTEGER_GET_REGION(x, 0, len as R_xlen_t, INTEGER(data2));
                Rf_unprotect(1);
                INTEGER(data2) as *mut u8
            } else {
                INTEGER(data2) as *mut u8
            }
        }
    }

    /// Get the data pointer for this vector, returning NULL
    /// if the object is unmaterialized.
    fn dataptr_or_null(&self, x: SEXP) -> *const u8 {
        unsafe {
            let data2 = R_altrep_data2(x);
            if data2 == R_NilValue || TYPEOF(data2) != INTSXP as i32 {
                std::ptr::null()
            } else {
                INTEGER(data2) as *const u8
            }
        }
    }

    /// Implement subsetting (eg. x[10:19]) for this Altrep vector.
    fn extract_subset(&self, _indx: Robj, _call: Robj) -> Robj {
        ().into()
    }
}

trait AltIntegerImpl : AltVecImpl {
    /// Get a single element from this vector.
    fn elt(&self, _index: usize) -> i32;

    /// Get a multiple elements from this vector.
    fn get_region(&self, index: usize, data: &mut [i32]) -> usize {
        let len = self.length();
        if index > len {
            0
        } else {
            let num_elems = data.len().min(len - index);
            for i in 0..num_elems {
                data[i] = self.elt(i);
            }
            num_elems
        }
    }

    /// Return true if this vector is sorted.
    fn is_sorted(&self) -> bool {
        false
    }

    /// Return true if this vector does not contain NAs.
    fn no_na(&self) -> bool {
        false
    }

    /// Return the sum of the elements in this vector.
    /// If remove_nas is true, skip and NA values.
    fn sum(&self, _remove_nas: bool) -> Robj {
        ().into()
    }

    /// Return the minimum of the elements in this vector.
    /// If remove_nas is true, skip and NA values.
    fn min(&self, _remove_nas: bool) -> Robj {
        ().into()
    }

    /// Return the maximum of the elements in this vector.
    /// If remove_nas is true, skip and NA values.
    fn max(&self, _remove_nas: bool) -> Robj {
        ().into()
    }
}

trait AltRealImpl {
    /// Get a single element from this vector.
    fn elt(&self, _index: usize) -> f64 {
        0.0
    }

    /// Get a multiple elements from this vector.
    fn get_region(&self, _index: usize, _data: &mut [i32]) -> usize {
        0
    }

    /// Return true if this vector is sorted.
    fn is_sorted(&self) -> bool {
        false
    }

    /// Return true if this vector does not contain NAs.
    fn no_na(&self) -> bool {
        false
    }

    /// Return the sum of the elements in this vector.
    /// If remove_nas is true, skip and NA values.
    fn sum(&self, _remove_nas: bool) -> Robj {
        ().into()
    }

    /// Return the minimum of the elements in this vector.
    /// If remove_nas is true, skip and NA values.
    fn min(&self, _remove_nas: bool) -> Robj {
        ().into()
    }

    /// Return the maximum of the elements in this vector.
    /// If remove_nas is true, skip and NA values.
    fn max(&self, _remove_nas: bool) -> Robj {
        ().into()
    }
}

trait AltLogicalImpl {
    /// Get a single element from this vector.
    fn elt(&self, _index: usize) -> Bool {
        NA_LOGICAL
    }

    /// Get a multiple elements from this vector.
    fn get_region(&self, _index: usize, _data: &mut [i32]) -> usize {
        0
    }

    /// Return true if this vector is sorted.
    fn is_sorted(&self) -> bool {
        false
    }

    /// Return true if this vector does not contain NAs.
    fn no_na(&self) -> bool {
        false
    }

    /// Return the sum of the elements in this vector.
    /// If remove_nas is true, skip and NA values.
    fn sum(&self, _remove_nas: bool) -> Robj {
        ().into()
    }
}

trait AltRawImpl {
    /// Get a single element from this vector.
    fn elt(&self, _index: usize) -> u8 {
        0
    }

    /// Get a multiple elements from this vector.
    fn get_region(&self, _index: usize, _data: &mut [u8]) -> usize {
        0
    }
}

trait AltComplexImpl {
    /// Get a single element from this vector.
    fn elt(&self, _index: usize) -> Complex {
        Complex(0.0, 0.0)
    }

    /// Get a multiple elements from this vector.
    fn get_region(&self, _index: usize, _data: &mut [Complex]) -> usize {
        0
    }
}

trait AltStringImpl {
    /// Get a single element from this vector.
    fn elt(&self, _index: usize) -> Robj {
        ().into()
    }

    /// Set a single element in this vector.
    fn set_elt(&mut self, _index: usize, _value: Robj) {}

    /// Return true if this vector is sorted.
    fn is_sorted(&self) -> bool {
        false
    }

    /// Return true if this vector does not contain NAs.
    fn no_na(&self) -> bool {
        false
    }
}

impl Altrep {
    /// Safely implement R_altrep_data1, R_altrep_data2.
    /// When implementing Altrep classes, this gets the metadata.
    pub fn data(&self) -> (Robj, Robj) {
        unsafe {
            (
                new_owned(R_altrep_data1(self.robj.get())),
                new_owned(R_altrep_data1(self.robj.get())),
            )
        }
    }

    /// Safely (relatively!) implement R_set_altrep_data1, R_set_altrep_data2.
    /// When implementing Altrep classes, this sets the metadata.
    pub fn set_data(&mut self, values: (Robj, Robj)) {
        unsafe {
            R_set_altrep_data1(self.robj.get(), values.0.get());
            R_set_altrep_data2(self.robj.get(), values.1.get());
        }
    }

    /// Safely implement ALTREP_CLASS.
    pub fn class(&self) -> Robj {
        unsafe { new_owned(ALTREP_CLASS(self.robj.get())) }
    }
}

#[macro_export]
macro_rules! impl_new_altinteger {
    ($statetype : ty, $name : expr, $base: expr) => {
        impl From<$statetype> for Altrep {
            fn from(state: $statetype) -> Altrep {
                unsafe {
                    #![allow(non_snake_case)]
                    #![allow(unused_variables)]
                    use std::os::raw::c_int;
                    use std::os::raw::c_void;
                
                    // Get the state for this altrep.
                    // We can bypass the type check as we know what type we have.
                    fn get_state(x: SEXP) -> &'static $statetype {
                        unsafe {
                            let state_ptr = R_ExternalPtrAddr(R_altrep_data1(x));
                            std::mem::transmute(state_ptr)
                        }
                    }

                    // fn get_materialized(x: SEXP) -> Option<&'static mut [i32]> {
                    //     unsafe {
                    //         if TYPEOF(R_altrep_data2(x) == INTSXP {
                                
                    //         }
                    //     }
                    // }

                    unsafe extern "C" fn altrep_UnserializeEX(
                        class: SEXP,
                        state: SEXP,
                        attr: SEXP,
                        objf: c_int,
                        levs: c_int,
                    ) -> SEXP {
                        R_NilValue
                    }

                    unsafe extern "C" fn altrep_Unserialize(class: SEXP, state: SEXP) -> SEXP {
                        R_NilValue
                    }

                    unsafe extern "C" fn altrep_Serialized_state(x: SEXP) -> SEXP {
                        R_NilValue
                    }

                    unsafe extern "C" fn altrep_Coerce(x: SEXP, ty: c_int) -> SEXP {
                        R_NilValue
                    }

                    unsafe extern "C" fn altrep_Duplicate(x: SEXP, deep: Rboolean) -> SEXP {
                        R_NilValue
                    }

                    unsafe extern "C" fn altrep_DuplicateEX(x: SEXP, deep: Rboolean) -> SEXP {
                        R_NilValue
                    }

                    unsafe extern "C" fn altrep_Inspect(
                        x: SEXP,
                        pre: c_int,
                        deep: c_int,
                        pvec: c_int,
                        func: Option<
                            unsafe extern "C" fn(
                                arg1: SEXP,
                                arg2: c_int,
                                arg3: c_int,
                                arg4: c_int,
                            ),
                        >,
                    ) -> Rboolean {
                        0
                    }

                    unsafe extern "C" fn altrep_Length(x: SEXP) -> R_xlen_t {
                        get_state(x).length() as R_xlen_t
                    }

                    unsafe extern "C" fn altvec_Dataptr(
                        x: SEXP,
                        writeable: Rboolean,
                    ) -> *mut c_void {
                        get_state(x).dataptr(x, writeable != 0) as *mut c_void
                    }

                    unsafe extern "C" fn altvec_Dataptr_or_null(x: SEXP) -> *const c_void {
                        get_state(x).dataptr_or_null(x) as *mut c_void
                    }

                    unsafe extern "C" fn altvec_Extract_subset(
                        x: SEXP,
                        indx: SEXP,
                        call: SEXP,
                    ) -> SEXP {
                        R_NilValue
                    }

                    unsafe extern "C" fn altinteger_Elt(x: SEXP, i: R_xlen_t) -> c_int {
                        0
                    }

                    unsafe extern "C" fn altinteger_Get_region(
                        sx: SEXP,
                        i: R_xlen_t,
                        n: R_xlen_t,
                        buf: *mut c_int,
                    ) -> R_xlen_t {
                        0
                    }

                    unsafe extern "C" fn altinteger_Is_sorted(x: SEXP) -> c_int {
                        0
                    }

                    unsafe extern "C" fn altinteger_No_NA(x: SEXP) -> c_int {
                        0
                    }

                    unsafe extern "C" fn altinteger_Sum(x: SEXP, narm: Rboolean) -> SEXP {
                        R_NilValue
                    }

                    unsafe extern "C" fn altinteger_Min(x: SEXP, narm: Rboolean) -> SEXP {
                        R_NilValue
                    }

                    unsafe extern "C" fn altinteger_Max(x: SEXP, narm: Rboolean) -> SEXP {
                        R_NilValue
                    }

                    unsafe extern "C" fn altreal_Elt(x: SEXP, i: R_xlen_t) -> f64 {
                        0.0
                    }

                    unsafe extern "C" fn altreal_Get_region(
                        sx: SEXP,
                        i: R_xlen_t,
                        n: R_xlen_t,
                        buf: *mut f64,
                    ) -> R_xlen_t {
                        0
                    }

                    unsafe extern "C" fn altreal_Is_sorted(x: SEXP) -> c_int {
                        0
                    }

                    unsafe extern "C" fn altreal_No_NA(x: SEXP) -> c_int {
                        0
                    }

                    unsafe extern "C" fn altreal_Sum(x: SEXP, narm: Rboolean) -> SEXP {
                        R_NilValue
                    }

                    unsafe extern "C" fn altreal_Min(x: SEXP, narm: Rboolean) -> SEXP {
                        R_NilValue
                    }

                    unsafe extern "C" fn altreal_Max(x: SEXP, narm: Rboolean) -> SEXP {
                        R_NilValue
                    }

                    unsafe extern "C" fn altlogical_Elt(x: SEXP, i: R_xlen_t) -> c_int {
                        get_state(x).elt(i as usize) as c_int
                    }

                    unsafe extern "C" fn altlogical_Get_region(
                        sx: SEXP,
                        i: R_xlen_t,
                        n: R_xlen_t,
                        buf: *mut c_int,
                    ) -> R_xlen_t {
                        0
                    }

                    unsafe extern "C" fn altlogical_Is_sorted(x: SEXP) -> c_int {
                        0
                    }

                    unsafe extern "C" fn altlogical_No_NA(x: SEXP) -> c_int {
                        0
                    }

                    unsafe extern "C" fn altlogical_Sum(x: SEXP, narm: Rboolean) -> SEXP {
                        R_NilValue
                    }

                    unsafe extern "C" fn altraw_Elt(x: SEXP, i: R_xlen_t) -> Rbyte {
                        0
                    }

                    unsafe extern "C" fn altraw_Get_region(
                        sx: SEXP,
                        i: R_xlen_t,
                        n: R_xlen_t,
                        buf: *mut u8,
                    ) -> R_xlen_t {
                        0
                    }

                    unsafe extern "C" fn altcomplex_Elt(x: SEXP, i: R_xlen_t) -> Rcomplex {
                        Rcomplex { r: 0.0, i: 0.0 }
                    }

                    unsafe extern "C" fn altcomplex_Get_region(
                        sx: SEXP,
                        i: R_xlen_t,
                        n: R_xlen_t,
                        buf: *mut Rcomplex,
                    ) -> R_xlen_t {
                        0
                    }

                    unsafe extern "C" fn altstring_Elt(x: SEXP, i: R_xlen_t) -> SEXP {
                        R_NilValue
                    }

                    unsafe extern "C" fn altstring_Set_elt(x: SEXP, i: R_xlen_t, v: SEXP) {}

                    unsafe extern "C" fn altstring_Is_sorted(x: SEXP) -> c_int {
                        0
                    }

                    unsafe extern "C" fn altstring_No_NA(x: SEXP) -> c_int {
                        0
                    }

                    // TODO: Cache the class.
                    let csname = std::ffi::CString::new($name).unwrap();
                    let csbase = std::ffi::CString::new($base).unwrap();
                    println!("csname={:?}", csname);
                    println!("csbase={:?}", csbase);

                    let class_ptr = R_make_altinteger_class(
                        csname.as_ptr(),
                        csbase.as_ptr(),
                        std::ptr::null_mut(),
                    );
                    println!("class={:?}", class_ptr.ptr);

                    R_set_altrep_UnserializeEX_method(class_ptr, Some(altrep_UnserializeEX));
                    R_set_altrep_Unserialize_method(class_ptr, Some(altrep_Unserialize));
                    R_set_altrep_Serialized_state_method(
                        class_ptr,
                        Some(altrep_Serialized_state),
                    );
                    R_set_altrep_DuplicateEX_method(class_ptr, Some(altrep_DuplicateEX));
                    R_set_altrep_Duplicate_method(class_ptr, Some(altrep_Duplicate));
                    R_set_altrep_Coerce_method(class_ptr, Some(altrep_Coerce));
                    R_set_altrep_Inspect_method(class_ptr, Some(altrep_Inspect));
                    R_set_altrep_Length_method(class_ptr, Some(altrep_Length));

                    R_set_altvec_Dataptr_method(class_ptr, Some(altvec_Dataptr));
                    R_set_altvec_Dataptr_or_null_method(
                        class_ptr,
                        Some(altvec_Dataptr_or_null),
                    );
                    R_set_altvec_Extract_subset_method(class_ptr, Some(altvec_Extract_subset));

                    R_set_altinteger_Elt_method(class_ptr, Some(altinteger_Elt));
                    R_set_altinteger_Get_region_method(class_ptr, Some(altinteger_Get_region));
                    R_set_altinteger_Is_sorted_method(class_ptr, Some(altinteger_Is_sorted));
                    R_set_altinteger_No_NA_method(class_ptr, Some(altinteger_No_NA));
                    R_set_altinteger_Sum_method(class_ptr, Some(altinteger_Sum));
                    R_set_altinteger_Min_method(class_ptr, Some(altinteger_Min));
                    R_set_altinteger_Max_method(class_ptr, Some(altinteger_Max));

                    R_set_altreal_Elt_method(class_ptr, Some(altreal_Elt));
                    R_set_altreal_Get_region_method(class_ptr, Some(altreal_Get_region));
                    R_set_altreal_Is_sorted_method(class_ptr, Some(altreal_Is_sorted));
                    R_set_altreal_No_NA_method(class_ptr, Some(altreal_No_NA));
                    R_set_altreal_Sum_method(class_ptr, Some(altreal_Sum));
                    R_set_altreal_Min_method(class_ptr, Some(altreal_Min));
                    R_set_altreal_Max_method(class_ptr, Some(altreal_Max));

                    R_set_altlogical_Elt_method(class_ptr, Some(altlogical_Elt));
                    R_set_altlogical_Get_region_method(class_ptr, Some(altlogical_Get_region));
                    R_set_altlogical_Is_sorted_method(class_ptr, Some(altlogical_Is_sorted));
                    R_set_altlogical_No_NA_method(class_ptr, Some(altlogical_No_NA));
                    R_set_altlogical_Sum_method(class_ptr, Some(altlogical_Sum));

                    R_set_altraw_Elt_method(class_ptr, Some(altraw_Elt));
                    R_set_altraw_Get_region_method(class_ptr, Some(altraw_Get_region));

                    R_set_altcomplex_Elt_method(class_ptr, Some(altcomplex_Elt));
                    R_set_altcomplex_Get_region_method(class_ptr, Some(altcomplex_Get_region));

                    R_set_altstring_Elt_method(class_ptr, Some(altstring_Elt));
                    R_set_altstring_Set_elt_method(class_ptr, Some(altstring_Set_elt));
                    R_set_altstring_Is_sorted_method(class_ptr, Some(altstring_Is_sorted));
                    R_set_altstring_No_NA_method(class_ptr, Some(altstring_No_NA));

                    let ptr: *mut $statetype = Box::into_raw(Box::new(state));
                    let tag = r!($name);
                    let prot = r!(());
                    let state = R_MakeExternalPtr(ptr as *mut c_void, tag.get(), prot.get());

                    Altrep {
                        robj: new_owned(R_new_altrep(class_ptr, state, R_NilValue)),
                    }
                }
            }
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;
    // use libR_sys::*;


    #[test]
    fn test_altrep() {
        test! {
            #[derive(Debug, Clone)]
            struct MyCompactIntRange {
                start: i32,
                len: i32,
                step: i32,
            }

            impl AltRepImpl for MyCompactIntRange {
                fn length(&self) -> usize {
                    self.len as usize
                }
            }

            impl AltVecImpl for MyCompactIntRange {
            }

            impl AltIntegerImpl for MyCompactIntRange {
                fn elt(&self, index: usize) -> i32 {
                    self.start + self.step * index as i32
                }
            }

            let mystate = MyCompactIntRange { start: 0, len: 10, step: 1 };

            impl_new_altinteger!(MyCompactIntRange, "cir", "mypkg");

            let altrep : Altrep = mystate.into();

            println!("len={}", altrep.len());
            println!("slice={:?}", altrep.as_integer_slice());
        }
    }
}
