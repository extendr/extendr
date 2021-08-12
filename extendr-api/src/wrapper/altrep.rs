use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Altrep {
    pub(crate) robj: Robj,
}

/// Rust trait for implementing ALTREP.
/// Implement one or more of these methods to generate an Altrep class.
/// Mechanism TBD.
pub trait AltrepImpl: Clone + std::fmt::Debug {
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
        _deep: bool,
        _pvec: i32, // _inspect_subtree: fn(robj: Robj, pre: i32, deep: i32, pvec: i32),
    ) -> bool {
        rprintln!("{:?}", self);
        true
    }

    /// Get the virtual length of the vector.
    /// For example for a compact range, return end - start + 1.
    fn length(&self) -> usize;

    /// Get the data pointer for this vector, possibly expanding the
    /// compact representation into a full R vector.
    fn dataptr(x: SEXP, _writeable: bool) -> *mut u8 {
        unsafe {
            let data2 = R_altrep_data2(x);
            if data2 == R_NilValue || TYPEOF(data2) != TYPEOF(x) {
                Rf_protect(x);
                let len = XLENGTH_EX(x);
                let data2 = Rf_allocVector(TYPEOF(x) as u32, len as R_xlen_t);
                Rf_protect(data2);
                match TYPEOF(x) as u32 {
                    INTSXP => {
                        INTEGER_GET_REGION(x, 0, len as R_xlen_t, INTEGER(data2));
                    }
                    LGLSXP => {
                        LOGICAL_GET_REGION(x, 0, len as R_xlen_t, LOGICAL(data2));
                    }
                    REALSXP => {
                        REAL_GET_REGION(x, 0, len as R_xlen_t, REAL(data2));
                    }
                    RAWSXP => {
                        RAW_GET_REGION(x, 0, len as R_xlen_t, RAW(data2));
                    }
                    CPLXSXP => {
                        COMPLEX_GET_REGION(x, 0, len as R_xlen_t, COMPLEX(data2));
                    }
                    // STRSXP => { STRING_GET_REGION(x, 0, len as R_xlen_t, INTEGER(data2)); }
                    _ => panic!("unsupported ALTREP type."),
                }
                R_set_altrep_data2(x, data2);
                Rf_unprotect(2);
                DATAPTR(data2) as *mut u8
            } else {
                DATAPTR(data2) as *mut u8
            }
        }
    }

    /// Get the data pointer for this vector, returning NULL
    /// if the object is unmaterialized.
    fn dataptr_or_null(x: SEXP) -> *const u8 {
        unsafe {
            let data2 = R_altrep_data2(x);
            if data2 == R_NilValue || TYPEOF(data2) != TYPEOF(x) {
                std::ptr::null()
            } else {
                DATAPTR(data2) as *const u8
            }
        }
    }

    /// Implement subsetting (eg. x[10:19]) for this Altrep vector.
    fn extract_subset(_x: Robj, _indx: Robj, _call: Robj) -> Robj {
        // only available in later versions of R.
        // x.extract_subset(indx, call)
        Robj::from(())
    }
}

pub trait AltIntegerImpl: AltrepImpl {
    fn tot_min_max_nas(&self) -> (i64, i32, i32, usize, usize) {
        let len = self.length();
        let mut tot = 0;
        let mut nas = 0;
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        for i in 0..len {
            let val = self.elt(i);
            if !val.is_na() {
                tot = tot + val as i64;
                min = min.min(val);
                max = max.max(val);
                nas += 1;
            }
        }
        (tot, min, max, len - nas, len)
    }

    /// Get a single element from this vector.
    fn elt(&self, _index: usize) -> i32;

    /// Get a multiple elements from this vector.
    fn get_region(&self, index: usize, data: &mut [i32]) -> usize {
        let len = self.length();
        if index > len {
            0
        } else {
            let num_elems = data.len().min(len - index);
            for i in index..index + num_elems {
                data[i] = self.elt(i);
            }
            num_elems
        }
    }

    /// Return TRUE if this vector is sorted, FALSE if not and NA_LOGICAL if unknown.
    fn is_sorted(&self) -> Bool {
        NA_LOGICAL
    }

    /// Return true if this vector does not contain NAs.
    fn no_na(&self) -> bool {
        false
    }

    /// Return the sum of the elements in this vector.
    /// If remove_nas is true, skip and NA values.
    fn sum(&self, remove_nas: bool) -> Robj {
        let (tot, _min, _max, nas, _len) = self.tot_min_max_nas();
        if !remove_nas && nas != 0 {
            NA_INTEGER.into()
        } else {
            tot.into()
        }
    }

    /// Return the minimum of the elements in this vector.
    /// If remove_nas is true, skip and NA values.
    fn min(&self, remove_nas: bool) -> Robj {
        let (_tot, min, _max, nas, len) = self.tot_min_max_nas();
        if !remove_nas && nas != 0 || remove_nas && nas == len {
            NA_INTEGER.into()
        } else {
            min.into()
        }
    }

    /// Return the maximum of the elements in this vector.
    /// If remove_nas is true, skip and NA values.
    fn max(&self, remove_nas: bool) -> Robj {
        let (_tot, _min, max, nas, len) = self.tot_min_max_nas();
        if !remove_nas && nas != 0 || remove_nas && nas == len {
            NA_INTEGER.into()
        } else {
            max.into()
        }
    }
}

pub trait AltRealImpl: AltrepImpl {
    fn tot_min_max_nas(&self) -> (f64, f64, f64, usize, usize) {
        let len = self.length();
        let mut tot = 0.0;
        let mut nas = 0;
        let mut min = f64::MAX;
        let mut max = f64::MIN;
        for i in 0..len {
            let val = self.elt(i);
            if !val.is_na() {
                tot = tot + val;
                min = min.min(val);
                max = max.max(val);
                nas += 1;
            }
        }
        (tot, min, max, len - nas, len)
    }

    /// Get a single element from this vector.
    fn elt(&self, _index: usize) -> f64;

    /// Get a multiple elements from this vector.
    fn get_region(&self, index: usize, data: &mut [f64]) -> usize {
        let len = self.length();
        if index > len {
            0
        } else {
            let num_elems = data.len().min(len - index);
            for i in index..index + num_elems {
                data[i] = self.elt(i);
            }
            num_elems
        }
    }

    /// Return TRUE if this vector is sorted, FALSE if not and NA_LOGICAL if unknown.
    fn is_sorted(&self) -> Bool {
        NA_LOGICAL
    }

    /// Return true if this vector does not contain NAs.
    fn no_na(&self) -> bool {
        false
    }

    /// Return the sum of the elements in this vector.
    /// If remove_nas is true, skip and NA values.
    fn sum(&self, remove_nas: bool) -> Robj {
        let (tot, _min, _max, nas, _len) = self.tot_min_max_nas();
        if !remove_nas && nas != 0 {
            NA_REAL.into()
        } else {
            tot.into()
        }
    }

    /// Return the minimum of the elements in this vector.
    /// If remove_nas is true, skip and NA values.
    fn min(&self, remove_nas: bool) -> Robj {
        let (_tot, min, _max, nas, len) = self.tot_min_max_nas();
        if !remove_nas && nas != 0 || remove_nas && nas == len {
            NA_REAL.into()
        } else {
            min.into()
        }
    }

    /// Return the maximum of the elements in this vector.
    /// If remove_nas is true, skip and NA values.
    fn max(&self, remove_nas: bool) -> Robj {
        let (_tot, _min, max, nas, len) = self.tot_min_max_nas();
        if !remove_nas && nas != 0 || remove_nas && nas == len {
            NA_REAL.into()
        } else {
            max.into()
        }
    }
}

pub trait AltLogicalImpl: AltrepImpl {
    fn tot_min_max_nas(&self) -> (i64, i32, i32, usize, usize) {
        let len = self.length();
        let mut tot = 0;
        let mut nas = 0;
        for i in 0..len {
            let val = self.elt(i);
            if !val.is_na() {
                tot = tot + val.0 as i64;
                nas += 1;
            }
        }
        (tot, 0, 0, len - nas, len)
    }

    /// Get a single element from this vector.
    fn elt(&self, _index: usize) -> Bool;

    /// Get a multiple elements from this vector.
    fn get_region(&self, index: usize, data: &mut [Bool]) -> usize {
        let len = self.length();
        if index > len {
            0
        } else {
            let num_elems = data.len().min(len - index);
            for i in index..index + num_elems {
                data[i] = self.elt(i);
            }
            num_elems
        }
    }

    /// Return TRUE if this vector is sorted, FALSE if not and NA_LOGICAL if unknown.
    fn is_sorted(&self) -> Bool {
        NA_LOGICAL
    }

    /// Return true if this vector does not contain NAs.
    fn no_na(&self) -> bool {
        false
    }

    /// Return the sum of the elements in this vector.
    /// If remove_nas is true, skip and NA values.
    fn sum(&self, remove_nas: bool) -> Robj {
        let (tot, _min, _max, nas, len) = self.tot_min_max_nas();
        if !remove_nas && nas != 0 || remove_nas && nas == len {
            NA_LOGICAL.into()
        } else {
            tot.into()
        }
    }
}

pub trait AltRawImpl: AltrepImpl {
    /// Get a single element from this vector.
    fn elt(&self, _index: usize) -> u8;

    /// Get a multiple elements from this vector.
    fn get_region(&self, index: usize, data: &mut [u8]) -> usize {
        let len = self.length();
        if index > len {
            0
        } else {
            let num_elems = data.len().min(len - index);
            for i in index..index + num_elems {
                data[i] = self.elt(i);
            }
            num_elems
        }
    }
}

pub trait AltComplexImpl: AltrepImpl {
    /// Get a single element from this vector.
    fn elt(&self, _index: usize) -> Cplx;

    /// Get a multiple elements from this vector.
    fn get_region(&self, index: usize, data: &mut [Cplx]) -> usize {
        let len = self.length();
        if index > len {
            0
        } else {
            let num_elems = data.len().min(len - index);
            for i in index..index + num_elems {
                data[i] = self.elt(i);
            }
            num_elems
        }
    }
}

pub trait AltStringImpl {
    /// Get a single element from this vector.
    fn elt(&self, _index: usize) -> String;

    /// Set a single element in this vector.
    fn set_elt(&mut self, _index: usize, _value: Robj) {}

    /// Return TRUE if this vector is sorted, FALSE if not and NA_LOGICAL if unknown.
    fn is_sorted(&self) -> Bool {
        NA_LOGICAL
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

    pub fn from_state_and_class<StateType: 'static>(state: StateType, class: Robj) -> Altrep {
        single_threaded(|| unsafe {
            use std::os::raw::c_void;

            unsafe extern "C" fn finalizer<StateType: 'static>(x: SEXP) {
                let state = Altrep::get_state_mut::<StateType>(x);
                let ptr = state as *mut StateType;
                Box::from_raw(ptr);
            }

            let ptr: *mut StateType = Box::into_raw(Box::new(state));
            let tag = r!(());
            let prot = r!(());
            let state = R_MakeExternalPtr(ptr as *mut c_void, tag.get(), prot.get());
            R_RegisterCFinalizer(state, Some(finalizer::<StateType>));

            let class_ptr = R_altrep_class_t { ptr: class.get() };

            Altrep {
                robj: new_owned(R_new_altrep(class_ptr, state, R_NilValue)),
            }
        })
    }

    #[allow(dead_code)]
    pub(crate) fn get_state<StateType>(x: SEXP) -> &'static StateType {
        unsafe {
            let state_ptr = R_ExternalPtrAddr(R_altrep_data1(x));
            std::mem::transmute(state_ptr)
        }
    }

    #[allow(dead_code)]
    pub(crate) fn get_state_mut<StateType>(x: SEXP) -> &'static mut StateType {
        unsafe {
            let state_ptr = R_ExternalPtrAddr(R_altrep_data1(x));
            std::mem::transmute(state_ptr)
        }
    }

    pub fn altrep_class<StateType: AltrepImpl + 'static>(
        ty: RType,
        name: &str,
        base: &str,
    ) -> Robj {
        #![allow(non_snake_case)]
        #![allow(unused_variables)]
        use std::os::raw::c_int;
        use std::os::raw::c_void;

        unsafe extern "C" fn altrep_UnserializeEX<StateType: AltrepImpl>(
            class: SEXP,
            state: SEXP,
            attr: SEXP,
            objf: c_int,
            levs: c_int,
        ) -> SEXP {
            <StateType>::unserialize_ex(
                new_owned(class),
                new_owned(state),
                new_owned(attr),
                objf as i32,
                levs as i32,
            )
            .get()
        }

        unsafe extern "C" fn altrep_Unserialize<StateType: AltrepImpl + 'static>(
            class: SEXP,
            state: SEXP,
        ) -> SEXP {
            <StateType>::unserialize(new_owned(class), new_owned(state)).get()
        }

        unsafe extern "C" fn altrep_Serialized_state<StateType: AltrepImpl + 'static>(
            x: SEXP,
        ) -> SEXP {
            Altrep::get_state::<StateType>(x).serialized_state().get()
        }

        unsafe extern "C" fn altrep_Coerce<StateType: AltrepImpl + 'static>(
            x: SEXP,
            ty: c_int,
        ) -> SEXP {
            Altrep::get_state::<StateType>(x)
                .coerce(sxp_to_rtype(ty))
                .get()
        }

        unsafe extern "C" fn altrep_Duplicate<StateType: AltrepImpl + 'static>(
            x: SEXP,
            deep: Rboolean,
        ) -> SEXP {
            Altrep::get_state::<StateType>(x).duplicate(deep == 1).get()
        }

        unsafe extern "C" fn altrep_DuplicateEX<StateType: AltrepImpl + 'static>(
            x: SEXP,
            deep: Rboolean,
        ) -> SEXP {
            Altrep::get_state::<StateType>(x)
                .duplicate_ex(deep == 1)
                .get()
        }

        unsafe extern "C" fn altrep_Inspect<StateType: AltrepImpl + 'static>(
            x: SEXP,
            pre: c_int,
            deep: c_int,
            pvec: c_int,
            func: Option<unsafe extern "C" fn(arg1: SEXP, arg2: c_int, arg3: c_int, arg4: c_int)>,
        ) -> Rboolean {
            if Altrep::get_state::<StateType>(x).inspect(pre, deep == 1, pvec) {
                1
            } else {
                0
            }
        }

        unsafe extern "C" fn altrep_Length<StateType: AltrepImpl + 'static>(x: SEXP) -> R_xlen_t {
            Altrep::get_state::<StateType>(x).length() as R_xlen_t
        }

        unsafe extern "C" fn altvec_Dataptr<StateType: AltrepImpl + 'static>(
            x: SEXP,
            writeable: Rboolean,
        ) -> *mut c_void {
            <StateType>::dataptr(x, writeable != 0) as *mut c_void
        }

        unsafe extern "C" fn altvec_Dataptr_or_null<StateType: AltrepImpl + 'static>(
            x: SEXP,
        ) -> *const c_void {
            <StateType>::dataptr_or_null(x) as *mut c_void
        }

        unsafe extern "C" fn altvec_Extract_subset<StateType: AltrepImpl + 'static>(
            x: SEXP,
            indx: SEXP,
            call: SEXP,
        ) -> SEXP {
            <StateType>::extract_subset(new_owned(x), new_owned(indx), new_owned(call)).get()
        }

        unsafe {
            let csname = std::ffi::CString::new(name).unwrap();
            let csbase = std::ffi::CString::new(base).unwrap();

            let class_ptr = match ty {
                RType::Integer => {
                    R_make_altinteger_class(csname.as_ptr(), csbase.as_ptr(), std::ptr::null_mut())
                }
                RType::Real => {
                    R_make_altreal_class(csname.as_ptr(), csbase.as_ptr(), std::ptr::null_mut())
                }
                RType::Logical => {
                    R_make_altlogical_class(csname.as_ptr(), csbase.as_ptr(), std::ptr::null_mut())
                }
                RType::Raw => {
                    R_make_altraw_class(csname.as_ptr(), csbase.as_ptr(), std::ptr::null_mut())
                }
                RType::Complex => {
                    R_make_altcomplex_class(csname.as_ptr(), csbase.as_ptr(), std::ptr::null_mut())
                }
                RType::String => {
                    R_make_altstring_class(csname.as_ptr(), csbase.as_ptr(), std::ptr::null_mut())
                }
                _ => panic!("expected Altvec compatible type"),
            };

            R_set_altrep_UnserializeEX_method(class_ptr, Some(altrep_UnserializeEX::<StateType>));
            R_set_altrep_Unserialize_method(class_ptr, Some(altrep_Unserialize::<StateType>));
            R_set_altrep_Serialized_state_method(
                class_ptr,
                Some(altrep_Serialized_state::<StateType>),
            );
            R_set_altrep_DuplicateEX_method(class_ptr, Some(altrep_DuplicateEX::<StateType>));
            R_set_altrep_Duplicate_method(class_ptr, Some(altrep_Duplicate::<StateType>));
            R_set_altrep_Coerce_method(class_ptr, Some(altrep_Coerce::<StateType>));
            R_set_altrep_Inspect_method(class_ptr, Some(altrep_Inspect::<StateType>));
            R_set_altrep_Length_method(class_ptr, Some(altrep_Length::<StateType>));

            R_set_altvec_Dataptr_method(class_ptr, Some(altvec_Dataptr::<StateType>));
            R_set_altvec_Dataptr_or_null_method(
                class_ptr,
                Some(altvec_Dataptr_or_null::<StateType>),
            );
            R_set_altvec_Extract_subset_method(class_ptr, Some(altvec_Extract_subset::<StateType>));

            new_owned(class_ptr.ptr)
        }
    }

    pub fn make_altinteger_class<StateType: AltrepImpl + AltIntegerImpl + 'static>(
        name: &str,
        base: &str,
    ) -> Robj {
        #![allow(non_snake_case)]
        use std::os::raw::c_int;

        single_threaded(|| unsafe {
            let class = Altrep::altrep_class::<StateType>(RType::Integer, name, base);
            let class_ptr = R_altrep_class_t { ptr: class.get() };

            unsafe extern "C" fn altinteger_Elt<StateType: AltIntegerImpl + 'static>(
                x: SEXP,
                i: R_xlen_t,
            ) -> c_int {
                Altrep::get_state::<StateType>(x).elt(i as usize) as c_int
            }

            unsafe extern "C" fn altinteger_Get_region<StateType: AltIntegerImpl + 'static>(
                x: SEXP,
                i: R_xlen_t,
                n: R_xlen_t,
                buf: *mut c_int,
            ) -> R_xlen_t {
                let slice = std::slice::from_raw_parts_mut(buf, n as usize);
                Altrep::get_state::<StateType>(x).get_region(i as usize, slice) as R_xlen_t
            }

            unsafe extern "C" fn altinteger_Is_sorted<StateType: AltIntegerImpl + 'static>(
                x: SEXP,
            ) -> c_int {
                Altrep::get_state::<StateType>(x).is_sorted().0 as c_int
            }

            unsafe extern "C" fn altinteger_No_NA<StateType: AltIntegerImpl + 'static>(
                x: SEXP,
            ) -> c_int {
                if Altrep::get_state::<StateType>(x).no_na() {
                    1
                } else {
                    0
                }
            }

            unsafe extern "C" fn altinteger_Sum<StateType: AltIntegerImpl + 'static>(
                x: SEXP,
                narm: Rboolean,
            ) -> SEXP {
                Altrep::get_state::<StateType>(x).sum(narm == 1).get()
            }

            unsafe extern "C" fn altinteger_Min<StateType: AltIntegerImpl + 'static>(
                x: SEXP,
                narm: Rboolean,
            ) -> SEXP {
                Altrep::get_state::<StateType>(x).min(narm == 1).get()
            }

            unsafe extern "C" fn altinteger_Max<StateType: AltIntegerImpl + 'static>(
                x: SEXP,
                narm: Rboolean,
            ) -> SEXP {
                Altrep::get_state::<StateType>(x).max(narm == 1).get()
            }

            R_set_altinteger_Elt_method(class_ptr, Some(altinteger_Elt::<StateType>));
            R_set_altinteger_Get_region_method(class_ptr, Some(altinteger_Get_region::<StateType>));
            R_set_altinteger_Is_sorted_method(class_ptr, Some(altinteger_Is_sorted::<StateType>));
            R_set_altinteger_No_NA_method(class_ptr, Some(altinteger_No_NA::<StateType>));
            R_set_altinteger_Sum_method(class_ptr, Some(altinteger_Sum::<StateType>));
            R_set_altinteger_Min_method(class_ptr, Some(altinteger_Min::<StateType>));
            R_set_altinteger_Max_method(class_ptr, Some(altinteger_Max::<StateType>));

            class
        })
    }

    pub fn make_altreal_class<StateType: AltrepImpl + AltRealImpl + 'static>(
        name: &str,
        base: &str,
    ) -> Robj {
        #![allow(non_snake_case)]
        use std::os::raw::c_int;

        single_threaded(|| unsafe {
            let class = Altrep::altrep_class::<StateType>(RType::Real, name, base);
            let class_ptr = R_altrep_class_t { ptr: class.get() };

            unsafe extern "C" fn altreal_Elt<StateType: AltRealImpl + 'static>(
                x: SEXP,
                i: R_xlen_t,
            ) -> f64 {
                Altrep::get_state::<StateType>(x).elt(i as usize) as f64
            }

            unsafe extern "C" fn altreal_Get_region<StateType: AltRealImpl + 'static>(
                x: SEXP,
                i: R_xlen_t,
                n: R_xlen_t,
                buf: *mut f64,
            ) -> R_xlen_t {
                let slice = std::slice::from_raw_parts_mut(buf, n as usize);
                Altrep::get_state::<StateType>(x).get_region(i as usize, slice) as R_xlen_t
            }

            unsafe extern "C" fn altreal_Is_sorted<StateType: AltRealImpl + 'static>(
                x: SEXP,
            ) -> c_int {
                Altrep::get_state::<StateType>(x).is_sorted().0 as c_int
            }

            unsafe extern "C" fn altreal_No_NA<StateType: AltRealImpl + 'static>(x: SEXP) -> c_int {
                if Altrep::get_state::<StateType>(x).no_na() {
                    1
                } else {
                    0
                }
            }

            unsafe extern "C" fn altreal_Sum<StateType: AltRealImpl + 'static>(
                x: SEXP,
                narm: Rboolean,
            ) -> SEXP {
                Altrep::get_state::<StateType>(x).sum(narm == 1).get()
            }

            unsafe extern "C" fn altreal_Min<StateType: AltRealImpl + 'static>(
                x: SEXP,
                narm: Rboolean,
            ) -> SEXP {
                Altrep::get_state::<StateType>(x).min(narm == 1).get()
            }

            unsafe extern "C" fn altreal_Max<StateType: AltRealImpl + 'static>(
                x: SEXP,
                narm: Rboolean,
            ) -> SEXP {
                Altrep::get_state::<StateType>(x).max(narm == 1).get()
            }

            R_set_altreal_Elt_method(class_ptr, Some(altreal_Elt::<StateType>));
            R_set_altreal_Get_region_method(class_ptr, Some(altreal_Get_region::<StateType>));
            R_set_altreal_Is_sorted_method(class_ptr, Some(altreal_Is_sorted::<StateType>));
            R_set_altreal_No_NA_method(class_ptr, Some(altreal_No_NA::<StateType>));
            R_set_altreal_Sum_method(class_ptr, Some(altreal_Sum::<StateType>));
            R_set_altreal_Min_method(class_ptr, Some(altreal_Min::<StateType>));
            R_set_altreal_Max_method(class_ptr, Some(altreal_Max::<StateType>));
            class
        })
    }

    pub fn make_altlogical_class<StateType: AltrepImpl + AltLogicalImpl + 'static>(
        name: &str,
        base: &str,
    ) -> Robj {
        #![allow(non_snake_case)]
        use std::os::raw::c_int;

        single_threaded(|| unsafe {
            let class = Altrep::altrep_class::<StateType>(RType::Logical, name, base);
            let class_ptr = R_altrep_class_t { ptr: class.get() };

            unsafe extern "C" fn altlogical_Elt<StateType: AltLogicalImpl + 'static>(
                x: SEXP,
                i: R_xlen_t,
            ) -> c_int {
                Altrep::get_state::<StateType>(x).elt(i as usize).0 as c_int
            }

            unsafe extern "C" fn altlogical_Get_region<StateType: AltLogicalImpl + 'static>(
                x: SEXP,
                i: R_xlen_t,
                n: R_xlen_t,
                buf: *mut c_int,
            ) -> R_xlen_t {
                let slice = std::slice::from_raw_parts_mut(buf as *mut Bool, n as usize);
                Altrep::get_state::<StateType>(x).get_region(i as usize, slice) as R_xlen_t
            }

            unsafe extern "C" fn altlogical_Is_sorted<StateType: AltLogicalImpl + 'static>(
                x: SEXP,
            ) -> c_int {
                Altrep::get_state::<StateType>(x).is_sorted().0 as c_int
            }

            unsafe extern "C" fn altlogical_No_NA<StateType: AltLogicalImpl + 'static>(
                x: SEXP,
            ) -> c_int {
                if Altrep::get_state::<StateType>(x).no_na() {
                    1
                } else {
                    0
                }
            }

            unsafe extern "C" fn altlogical_Sum<StateType: AltLogicalImpl + 'static>(
                x: SEXP,
                narm: Rboolean,
            ) -> SEXP {
                Altrep::get_state::<StateType>(x).sum(narm == 1).get()
            }

            R_set_altlogical_Elt_method(class_ptr, Some(altlogical_Elt::<StateType>));
            R_set_altlogical_Get_region_method(class_ptr, Some(altlogical_Get_region::<StateType>));
            R_set_altlogical_Is_sorted_method(class_ptr, Some(altlogical_Is_sorted::<StateType>));
            R_set_altlogical_No_NA_method(class_ptr, Some(altlogical_No_NA::<StateType>));
            R_set_altlogical_Sum_method(class_ptr, Some(altlogical_Sum::<StateType>));

            class
        })
    }

    pub fn make_altraw_class<StateType: AltrepImpl + AltRawImpl + 'static>(
        name: &str,
        base: &str,
    ) -> Robj {
        #![allow(non_snake_case)]

        single_threaded(|| unsafe {
            let class = Altrep::altrep_class::<StateType>(RType::Raw, name, base);
            let class_ptr = R_altrep_class_t { ptr: class.get() };

            unsafe extern "C" fn altraw_Elt<StateType: AltRawImpl + 'static>(
                x: SEXP,
                i: R_xlen_t,
            ) -> Rbyte {
                Altrep::get_state::<StateType>(x).elt(i as usize) as Rbyte
            }

            unsafe extern "C" fn altraw_Get_region<StateType: AltRawImpl + 'static>(
                x: SEXP,
                i: R_xlen_t,
                n: R_xlen_t,
                buf: *mut u8,
            ) -> R_xlen_t {
                let slice = std::slice::from_raw_parts_mut(buf, n as usize);
                Altrep::get_state::<StateType>(x).get_region(i as usize, slice) as R_xlen_t
            }

            R_set_altraw_Elt_method(class_ptr, Some(altraw_Elt::<StateType>));
            R_set_altraw_Get_region_method(class_ptr, Some(altraw_Get_region::<StateType>));

            class
        })
    }

    pub fn make_altcomplex_class<StateType: AltrepImpl + AltComplexImpl + 'static>(
        name: &str,
        base: &str,
    ) -> Robj {
        #![allow(non_snake_case)]

        single_threaded(|| unsafe {
            let class = Altrep::altrep_class::<StateType>(RType::Complex, name, base);
            let class_ptr = R_altrep_class_t { ptr: class.get() };

            unsafe extern "C" fn altcomplex_Elt<StateType: AltComplexImpl + 'static>(
                x: SEXP,
                i: R_xlen_t,
            ) -> Rcomplex {
                std::mem::transmute(Altrep::get_state::<StateType>(x).elt(i as usize))
            }

            unsafe extern "C" fn altcomplex_Get_region<StateType: AltComplexImpl + 'static>(
                x: SEXP,
                i: R_xlen_t,
                n: R_xlen_t,
                buf: *mut Rcomplex,
            ) -> R_xlen_t {
                let slice = std::slice::from_raw_parts_mut(buf as *mut Cplx, n as usize);
                Altrep::get_state::<StateType>(x).get_region(i as usize, slice) as R_xlen_t
            }

            R_set_altcomplex_Elt_method(class_ptr, Some(altcomplex_Elt::<StateType>));
            R_set_altcomplex_Get_region_method(class_ptr, Some(altcomplex_Get_region::<StateType>));

            class
        })
    }

    pub fn make_altstring_class<StateType: AltrepImpl + AltStringImpl + 'static>(
        name: &str,
        base: &str,
    ) -> Robj {
        #![allow(non_snake_case)]
        use std::os::raw::c_char;
        use std::os::raw::c_int;

        single_threaded(|| unsafe {
            let class = Altrep::altrep_class::<StateType>(RType::String, name, base);
            let class_ptr = R_altrep_class_t { ptr: class.get() };

            unsafe extern "C" fn altstring_Elt<StateType: AltStringImpl + 'static>(
                x: SEXP,
                i: R_xlen_t,
            ) -> SEXP {
                let s = Altrep::get_state::<StateType>(x).elt(i as usize);
                Rf_mkCharLen(s.as_ptr() as *mut c_char, s.len() as c_int)
            }

            unsafe extern "C" fn altstring_Set_elt<StateType: AltStringImpl + 'static>(
                x: SEXP,
                i: R_xlen_t,
                v: SEXP,
            ) {
                Altrep::get_state_mut::<StateType>(x).set_elt(i as usize, new_owned(v))
            }

            unsafe extern "C" fn altstring_Is_sorted<StateType: AltStringImpl + 'static>(
                x: SEXP,
            ) -> c_int {
                Altrep::get_state::<StateType>(x).is_sorted().0 as c_int
            }

            unsafe extern "C" fn altstring_No_NA<StateType: AltStringImpl + 'static>(
                x: SEXP,
            ) -> c_int {
                if Altrep::get_state::<StateType>(x).no_na() {
                    1
                } else {
                    0
                }
            }

            R_set_altstring_Elt_method(class_ptr, Some(altstring_Elt::<StateType>));
            R_set_altstring_Set_elt_method(class_ptr, Some(altstring_Set_elt::<StateType>));
            R_set_altstring_Is_sorted_method(class_ptr, Some(altstring_Is_sorted::<StateType>));
            R_set_altstring_No_NA_method(class_ptr, Some(altstring_No_NA::<StateType>));

            class
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_altinteger() {
        test! {
            #[derive(Debug, Clone)]
            struct MyCompactIntRange {
                start: i32,
                len: i32,
                step: i32,
            }

            impl AltrepImpl for MyCompactIntRange {
                fn length(&self) -> usize {
                    self.len as usize
                }
            }

            impl AltIntegerImpl for MyCompactIntRange {
                fn elt(&self, index: usize) -> i32 {
                    self.start + self.step * index as i32
                }
            }

            let mystate = MyCompactIntRange { start: 0, len: 10, step: 1 };

            let class = Altrep::make_altinteger_class::<MyCompactIntRange>("cir", "mypkg");
            let obj = Altrep::from_state_and_class(mystate, class);

            assert_eq!(obj.len(), 10);
            // assert_eq!(obj.sum(true), r!(45.0));
            assert_eq!(obj.as_integer_slice().unwrap(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        }
    }

    #[test]

    fn test_altreal() {
        test! {
            #[derive(Debug, Clone)]
            struct MyCompactRealRange {
                start: f64,
                len: usize,
                step: f64,
            }

            impl AltrepImpl for MyCompactRealRange {
                fn length(&self) -> usize {
                    self.len as usize
                }
            }

            impl AltRealImpl for MyCompactRealRange {
                fn elt(&self, index: usize) -> f64 {
                    self.start + self.step * index as f64
                }
            }

            let mystate = MyCompactRealRange { start: 0.0, len: 10, step: 1.0 };

            let class = Altrep::make_altreal_class::<MyCompactRealRange>("crr", "mypkg");
            let obj = Altrep::from_state_and_class(mystate, class);

            assert_eq!(obj.len(), 10);
            // assert_eq!(obj.sum(true), r!(45.0));
            assert_eq!(obj.as_real_slice().unwrap(), [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
        }
    }

    #[test]
    fn test_altlogical() {
        test! {
            #[derive(Debug, Clone)]
            struct IsEven {
                len: usize,
            }

            impl AltrepImpl for IsEven {
                fn length(&self) -> usize {
                    self.len as usize
                }
            }

            impl AltLogicalImpl for IsEven {
                fn elt(&self, index: usize) -> Bool {
                    (index % 2 == 1).into()
                }
            }

            let mystate = IsEven { len: 10 };

            let class = Altrep::make_altlogical_class::<IsEven>("iseven", "mypkg");
            let obj = Altrep::from_state_and_class(mystate, class);

            assert_eq!(obj.len(), 10);
            // assert_eq!(obj.sum(true), r!(5.0));
            assert_eq!(obj.as_logical_slice().unwrap(), [FALSE, TRUE, FALSE, TRUE, FALSE, TRUE, FALSE, TRUE, FALSE, TRUE]);
        }
    }

    #[test]
    fn test_altraw() {
        test! {
            #[derive(Debug, Clone)]
            struct MyCompactRawRange {
                start: i32,
                len: i32,
                step: i32,
            }

            impl AltrepImpl for MyCompactRawRange {
                fn length(&self) -> usize {
                    self.len as usize
                }
            }

            impl AltRawImpl for MyCompactRawRange {
                fn elt(&self, index: usize) -> u8 {
                    (self.start + self.step * index as i32) as u8
                }
            }

            let mystate = MyCompactRawRange { start: 0, len: 10, step: 1 };

            let class = Altrep::make_altraw_class::<MyCompactRawRange>("cir", "mypkg");
            let obj = Altrep::from_state_and_class(mystate, class);

            assert_eq!(obj.len(), 10);
            assert_eq!(obj.as_raw_slice().unwrap(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        }
    }

    #[test]
    fn test_altcomplex() {
        test! {
            #[derive(Debug, Clone)]
            struct MyCompactComplexRange {
                start: f64,
                len: usize,
                step: f64,
            }

            impl AltrepImpl for MyCompactComplexRange {
                fn length(&self) -> usize {
                    self.len as usize
                }
            }

            impl AltComplexImpl for MyCompactComplexRange {
                fn elt(&self, index: usize) -> Cplx {
                    Cplx(self.start + self.step * index as f64, self.start + self.step * index as f64)
                }
            }

            let mystate = MyCompactComplexRange { start: 0.0, len: 10, step: 1.0 };

            let class = Altrep::make_altcomplex_class::<MyCompactComplexRange>("ccr", "mypkg");
            let obj = Altrep::from_state_and_class(mystate, class);

            assert_eq!(obj.len(), 10);
            //assert_eq!(obj.as_complex_slice().unwrap(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        }
    }

    #[test]
    fn test_altstring() {
        test! {
            #[derive(Debug, Clone)]
            struct StringInts {
                len: usize
            }

            impl AltrepImpl for StringInts {
                fn length(&self) -> usize {
                    self.len as usize
                }
            }

            impl AltStringImpl for StringInts {
                fn elt(&self, index: usize) -> String {
                    format!("{}", index).into()
                }
            }

            let mystate = StringInts { len: 10 };

            let class = Altrep::make_altstring_class::<StringInts>("si", "mypkg");
            let obj = Altrep::from_state_and_class(mystate, class);

            assert_eq!(obj.len(), 10);
            assert_eq!(Robj::from(obj), r!(["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]));
        }
    }
}
