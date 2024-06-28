use prelude::{Rbool, Rcplx, Rfloat, Rint, Scalar};

use super::*;

macro_rules! make_from_iterator {
    ($fn_name : ident, $make_class : ident, $impl : ident, $scalar_type : ident, $prim_type : ty) => {
        pub fn $fn_name<Iter>(iter: Iter) -> Altrep
        where
            Iter: ExactSizeIterator + std::fmt::Debug + Clone + 'static + std::any::Any,
            Iter::Item: Into<$scalar_type>,
        {
            impl<Iter: ExactSizeIterator + std::fmt::Debug + Clone> $impl for Iter
            where
                Iter::Item: Into<$scalar_type>,
            {
                fn elt(&self, index: usize) -> $scalar_type {
                    $scalar_type::from(self.clone().nth(index).unwrap().into())
                }

                fn get_region(&self, index: usize, data: &mut [$scalar_type]) -> usize {
                    let len = self.len();
                    if index > len {
                        0
                    } else {
                        let mut iter = self.clone().skip(index);
                        let num_elems = data.len().min(len - index);
                        let dest = &mut data[0..num_elems];
                        for d in dest.iter_mut() {
                            *d = $scalar_type::from(iter.next().unwrap().into());
                        }
                        num_elems
                    }
                }
            }

            let class = Altrep::$make_class::<Iter>(std::any::type_name::<Iter>(), "extendr");
            let robj: Robj = Altrep::from_state_and_class(iter, class, false).into();
            Altrep { robj }
        }
    };
}

#[derive(PartialEq, Clone)]
pub struct Altrep {
    pub(crate) robj: Robj,
}

/// Rust trait for implementing ALTREP.
/// Implement one or more of these methods to generate an Altrep class.
/// This is likely to be unstable for a while.
pub trait AltrepImpl: Clone + std::fmt::Debug {
    #[cfg(feature = "non-api")]
    /// Constructor that is called when loading an Altrep object from a file.
    unsafe fn unserialize_ex(
        class: Robj,
        state: Robj,
        attributes: Robj,
        obj_flags: i32,
        levels: i32,
    ) -> Robj {
        let res = Self::unserialize(class, state);
        if !res.is_null() {
            single_threaded(|| unsafe {
                let val = res.get();
                SET_ATTRIB(val, attributes.get());
                SET_OBJECT(val, obj_flags);
                SETLEVELS(val, levels);
            })
        }
        res
    }

    /// Simplified constructor that is called when loading an Altrep object from a file.
    fn unserialize(_class: Robj, _state: Robj) -> Robj {
        // We plan to hadle this via Serde by November.
        ().into()
    }

    /// Fetch the state of this object when writing to a file.
    fn serialized_state(_x: SEXP) -> Robj {
        // We plan to hadle this via Serde by November.
        ().into()
    }

    /// Duplicate this object, possibly duplicating attributes.
    /// Currently this manifests the array but preserves the original object.
    fn duplicate_ex(x: SEXP, deep: bool) -> Robj {
        Self::duplicate(x, deep)
    }

    /// Duplicate this object. Called by Rf_duplicate.
    /// Currently this manifests the array but preserves the original object.
    fn duplicate(x: SEXP, _deep: bool) -> Robj {
        Robj::from_sexp(manifest(x))
    }

    /// Coerce this object into some other type, if possible.
    fn coerce(_x: SEXP, _ty: Rtype) -> Robj {
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
        single_threaded(|| unsafe {
            let data2 = R_altrep_data2(x);
            if data2 == R_NilValue || TYPEOF(data2) != TYPEOF(x) {
                let data2 = manifest(x);
                R_set_altrep_data2(x, data2);
                DATAPTR(data2) as *mut u8
            } else {
                DATAPTR(data2) as *mut u8
            }
        })
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

    /// Implement subsetting (eg. `x[10:19]`) for this Altrep vector.
    fn extract_subset(_x: Robj, _indx: Robj, _call: Robj) -> Robj {
        // only available in later versions of R.
        // x.extract_subset(indx, call)
        Robj::from(())
    }
}

// Manifest a vector by storing the "elt" values to memory.
// Return the new vector.
fn manifest(x: SEXP) -> SEXP {
    single_threaded(|| unsafe {
        Rf_protect(x);
        let len = XLENGTH(x);
        let data2 = Rf_allocVector(TYPEOF(x), len as R_xlen_t);
        Rf_protect(data2);
        match TYPEOF(x) {
            SEXPTYPE::INTSXP => {
                INTEGER_GET_REGION(x, 0, len as R_xlen_t, INTEGER(data2));
            }
            SEXPTYPE::LGLSXP => {
                LOGICAL_GET_REGION(x, 0, len as R_xlen_t, LOGICAL(data2));
            }
            SEXPTYPE::REALSXP => {
                REAL_GET_REGION(x, 0, len as R_xlen_t, REAL(data2));
            }
            SEXPTYPE::RAWSXP => {
                RAW_GET_REGION(x, 0, len as R_xlen_t, RAW(data2));
            }
            SEXPTYPE::CPLXSXP => {
                COMPLEX_GET_REGION(x, 0, len as R_xlen_t, COMPLEX(data2));
            }
            _ => {
                Rf_unprotect(2);
                panic!("unsupported ALTREP type.")
            }
        };
        Rf_unprotect(2);
        data2
    })
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
                tot += val.inner() as i64;
                min = min.min(val.inner());
                max = max.max(val.inner());
                nas += 1;
            }
        }
        (tot, min, max, len - nas, len)
    }

    /// Get a single element from this vector.
    fn elt(&self, _index: usize) -> Rint;

    /// Get a multiple elements from this vector.
    fn get_region(&self, index: usize, data: &mut [Rint]) -> usize {
        let len = self.length();
        if index > len {
            0
        } else {
            let num_elems = data.len().min(len - index);
            let dest = &mut data[0..num_elems];
            for (i, d) in dest.iter_mut().enumerate() {
                *d = self.elt(i + index);
            }
            num_elems
        }
    }

    /// Return TRUE if this vector is sorted, FALSE if not and Rbool::na() if unknown.
    fn is_sorted(&self) -> Rbool {
        Rbool::na()
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
                tot += val.inner();
                min = min.min(val.inner());
                max = max.max(val.inner());
                nas += 1;
            }
        }
        (tot, min, max, len - nas, len)
    }

    /// Get a single element from this vector.
    fn elt(&self, _index: usize) -> Rfloat;

    /// Get a multiple elements from this vector.
    fn get_region(&self, index: usize, data: &mut [Rfloat]) -> usize {
        let len = self.length();
        if index > len {
            0
        } else {
            let num_elems = data.len().min(len - index);
            let dest = &mut data[0..num_elems];
            for (i, d) in dest.iter_mut().enumerate() {
                *d = self.elt(i + index);
            }
            num_elems
        }
    }

    /// Return TRUE if this vector is sorted, FALSE if not and Rbool::na() if unknown.
    fn is_sorted(&self) -> Rbool {
        Rbool::na()
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
                tot += val.inner() as i64;
                nas += 1;
            }
        }
        (tot, 0, 0, len - nas, len)
    }

    /// Get a single element from this vector.
    fn elt(&self, _index: usize) -> Rbool;

    /// Get a multiple elements from this vector.
    fn get_region(&self, index: usize, data: &mut [Rbool]) -> usize {
        let len = self.length();
        if index > len {
            0
        } else {
            let num_elems = data.len().min(len - index);
            let dest = &mut data[0..num_elems];
            for (i, d) in dest.iter_mut().enumerate() {
                *d = self.elt(i + index);
            }
            num_elems
        }
    }

    /// Return TRUE if this vector is sorted, FALSE if not and Rbool::na() if unknown.
    fn is_sorted(&self) -> Rbool {
        Rbool::na()
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
            Rbool::na().into()
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
            let dest = &mut data[0..num_elems];
            for (i, d) in dest.iter_mut().enumerate() {
                *d = self.elt(i + index);
            }
            num_elems
        }
    }
}

pub trait AltComplexImpl: AltrepImpl {
    /// Get a single element from this vector.
    fn elt(&self, _index: usize) -> Rcplx;

    /// Get a multiple elements from this vector.
    fn get_region(&self, index: usize, data: &mut [Rcplx]) -> usize {
        let len = self.length();
        if index > len {
            0
        } else {
            let num_elems = data.len().min(len - index);
            let dest = &mut data[0..num_elems];
            for (i, d) in dest.iter_mut().enumerate() {
                *d = self.elt(i + index);
            }
            num_elems
        }
    }
}

pub trait AltStringImpl {
    /// Get a single element from this vector.
    fn elt(&self, _index: usize) -> Rstr;

    /// Set a single element in this vector.
    fn set_elt(&mut self, _index: usize, _value: Rstr) {}

    /// Return TRUE if this vector is sorted, FALSE if not and Rbool::na() if unknown.
    fn is_sorted(&self) -> Rbool {
        Rbool::na()
    }

    /// Return true if this vector does not contain NAs.
    fn no_na(&self) -> bool {
        false
    }
}

#[cfg(use_r_altlist)]
pub trait AltListImpl {
    /// Get a single element from this vector
    /// a single element of a list can be any Robj
    fn elt(&self, _index: usize) -> Robj;

    /// Set a single element in this list.
    fn set_elt(&mut self, _index: usize, _value: Robj) {}
}

impl Altrep {
    /// Safely implement R_altrep_data1, R_altrep_data2.
    /// When implementing Altrep classes, this gets the metadata.
    pub fn data(&self) -> (Robj, Robj) {
        unsafe {
            (
                Robj::from_sexp(R_altrep_data1(self.robj.get())),
                Robj::from_sexp(R_altrep_data1(self.robj.get())),
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
        single_threaded(|| unsafe { Robj::from_sexp(ALTREP_CLASS(self.robj.get())) })
    }

    pub fn from_state_and_class<StateType: 'static>(
        state: StateType,
        class: Robj,
        mutable: bool,
    ) -> Altrep {
        single_threaded(|| unsafe {
            use std::os::raw::c_void;

            unsafe extern "C" fn finalizer<StateType: 'static>(x: SEXP) {
                let state = R_ExternalPtrAddr(x);
                let ptr = state as *mut StateType;
                drop(Box::from_raw(ptr));
            }

            let ptr: *mut StateType = Box::into_raw(Box::new(state));
            let tag = R_NilValue;
            let prot = R_NilValue;
            let state = R_MakeExternalPtr(ptr as *mut c_void, tag, prot);

            // Use R_RegisterCFinalizerEx() and set onexit to 1 (TRUE) to invoke
            // the finalizer on a shutdown of the R session as well.
            R_RegisterCFinalizerEx(state, Some(finalizer::<StateType>), Rboolean::TRUE);

            let class_ptr = R_altrep_class_t { ptr: class.get() };
            let sexp = R_new_altrep(class_ptr, state, R_NilValue);

            if !mutable {
                MARK_NOT_MUTABLE(sexp);
            }

            Altrep {
                robj: Robj::from_sexp(sexp),
            }
        })
    }

    /// Return true if the ALTREP object has been manifested (copied into memory).
    pub fn is_manifest(&self) -> bool {
        unsafe { !DATAPTR_OR_NULL(self.get()).is_null() }
    }

    #[allow(dead_code)]
    pub(crate) fn get_state<StateType>(x: SEXP) -> &'static StateType {
        unsafe {
            let state_ptr = R_ExternalPtrAddr(R_altrep_data1(x));
            &*(state_ptr as *const StateType)
        }
    }

    #[allow(dead_code)]
    pub(crate) fn get_state_mut<StateType>(x: SEXP) -> &'static mut StateType {
        unsafe {
            let state_ptr = R_ExternalPtrAddr(R_altrep_data1(x));
            &mut *(state_ptr as *mut StateType)
        }
    }

    fn altrep_class<StateType: AltrepImpl + 'static>(ty: Rtype, name: &str, base: &str) -> Robj {
        #![allow(non_snake_case)]
        #![allow(unused_variables)]
        use std::os::raw::c_int;
        use std::os::raw::c_void;

        #[cfg(feature = "non-api")]
        unsafe extern "C" fn altrep_UnserializeEX<StateType: AltrepImpl>(
            class: SEXP,
            state: SEXP,
            attr: SEXP,
            objf: c_int,
            levs: c_int,
        ) -> SEXP {
            <StateType>::unserialize_ex(
                Robj::from_sexp(class),
                Robj::from_sexp(state),
                Robj::from_sexp(attr),
                objf as i32,
                levs as i32,
            )
            .get()
        }

        unsafe extern "C" fn altrep_Unserialize<StateType: AltrepImpl + 'static>(
            class: SEXP,
            state: SEXP,
        ) -> SEXP {
            <StateType>::unserialize(Robj::from_sexp(class), Robj::from_sexp(state)).get()
        }

        unsafe extern "C" fn altrep_Serialized_state<StateType: AltrepImpl + 'static>(
            x: SEXP,
        ) -> SEXP {
            <StateType>::serialized_state(x).get()
        }

        unsafe extern "C" fn altrep_Coerce<StateType: AltrepImpl + 'static>(
            x: SEXP,
            ty: SEXPTYPE,
        ) -> SEXP {
            <StateType>::coerce(x, sxp_to_rtype(ty)).get()
        }

        unsafe extern "C" fn altrep_Duplicate<StateType: AltrepImpl + 'static>(
            x: SEXP,
            deep: Rboolean,
        ) -> SEXP {
            <StateType>::duplicate(x, deep == Rboolean::TRUE).get()
        }

        unsafe extern "C" fn altrep_DuplicateEX<StateType: AltrepImpl + 'static>(
            x: SEXP,
            deep: Rboolean,
        ) -> SEXP {
            <StateType>::duplicate_ex(x, deep == Rboolean::TRUE).get()
        }

        unsafe extern "C" fn altrep_Inspect<StateType: AltrepImpl + 'static>(
            x: SEXP,
            pre: c_int,
            deep: c_int,
            pvec: c_int,
            func: Option<unsafe extern "C" fn(arg1: SEXP, arg2: c_int, arg3: c_int, arg4: c_int)>,
        ) -> Rboolean {
            Altrep::get_state::<StateType>(x)
                .inspect(pre, deep == 1, pvec)
                .into()
        }

        unsafe extern "C" fn altrep_Length<StateType: AltrepImpl + 'static>(x: SEXP) -> R_xlen_t {
            Altrep::get_state::<StateType>(x).length() as R_xlen_t
        }

        unsafe extern "C" fn altvec_Dataptr<StateType: AltrepImpl + 'static>(
            x: SEXP,
            writeable: Rboolean,
        ) -> *mut c_void {
            <StateType>::dataptr(x, writeable != Rboolean::FALSE) as *mut c_void
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
            <StateType>::extract_subset(
                Robj::from_sexp(x),
                Robj::from_sexp(indx),
                Robj::from_sexp(call),
            )
            .get()
        }

        unsafe {
            let csname = std::ffi::CString::new(name).unwrap();
            let csbase = std::ffi::CString::new(base).unwrap();

            let class_ptr = match ty {
                Rtype::Integers => {
                    R_make_altinteger_class(csname.as_ptr(), csbase.as_ptr(), std::ptr::null_mut())
                }
                Rtype::Doubles => {
                    R_make_altreal_class(csname.as_ptr(), csbase.as_ptr(), std::ptr::null_mut())
                }
                Rtype::Logicals => {
                    R_make_altlogical_class(csname.as_ptr(), csbase.as_ptr(), std::ptr::null_mut())
                }
                Rtype::Raw => {
                    R_make_altraw_class(csname.as_ptr(), csbase.as_ptr(), std::ptr::null_mut())
                }
                Rtype::Complexes => {
                    R_make_altcomplex_class(csname.as_ptr(), csbase.as_ptr(), std::ptr::null_mut())
                }
                Rtype::Strings => {
                    R_make_altstring_class(csname.as_ptr(), csbase.as_ptr(), std::ptr::null_mut())
                }
                #[cfg(use_r_altlist)]
                Rtype::List => {
                    R_make_altlist_class(csname.as_ptr(), csbase.as_ptr(), std::ptr::null_mut())
                }
                _ => panic!("expected Altvec compatible type"),
            };

            #[cfg(feature = "non-api")]
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

            Robj::from_sexp(class_ptr.ptr)
        }
    }

    /// Make an integer ALTREP class that can be used to make vectors.
    pub fn make_altinteger_class<StateType: AltrepImpl + AltIntegerImpl + 'static>(
        name: &str,
        base: &str,
    ) -> Robj {
        #![allow(non_snake_case)]
        use std::os::raw::c_int;

        single_threaded(|| unsafe {
            let class = Altrep::altrep_class::<StateType>(Rtype::Integers, name, base);
            let class_ptr = R_altrep_class_t { ptr: class.get() };

            unsafe extern "C" fn altinteger_Elt<StateType: AltIntegerImpl + 'static>(
                x: SEXP,
                i: R_xlen_t,
            ) -> c_int {
                Altrep::get_state::<StateType>(x).elt(i as usize).inner() as c_int
            }

            unsafe extern "C" fn altinteger_Get_region<StateType: AltIntegerImpl + 'static>(
                x: SEXP,
                i: R_xlen_t,
                n: R_xlen_t,
                buf: *mut c_int,
            ) -> R_xlen_t {
                let slice = std::slice::from_raw_parts_mut(buf as *mut Rint, n as usize);
                Altrep::get_state::<StateType>(x).get_region(i as usize, slice) as R_xlen_t
            }

            unsafe extern "C" fn altinteger_Is_sorted<StateType: AltIntegerImpl + 'static>(
                x: SEXP,
            ) -> c_int {
                Altrep::get_state::<StateType>(x).is_sorted().inner() as c_int
            }

            unsafe extern "C" fn altinteger_No_NA<StateType: AltIntegerImpl + 'static>(
                x: SEXP,
            ) -> c_int {
                i32::from(Altrep::get_state::<StateType>(x).no_na())
            }

            unsafe extern "C" fn altinteger_Sum<StateType: AltIntegerImpl + 'static>(
                x: SEXP,
                narm: Rboolean,
            ) -> SEXP {
                Altrep::get_state::<StateType>(x)
                    .sum(narm == Rboolean::TRUE)
                    .get()
            }

            unsafe extern "C" fn altinteger_Min<StateType: AltIntegerImpl + 'static>(
                x: SEXP,
                narm: Rboolean,
            ) -> SEXP {
                Altrep::get_state::<StateType>(x)
                    .min(narm == Rboolean::TRUE)
                    .get()
            }

            unsafe extern "C" fn altinteger_Max<StateType: AltIntegerImpl + 'static>(
                x: SEXP,
                narm: Rboolean,
            ) -> SEXP {
                Altrep::get_state::<StateType>(x)
                    .max(narm == Rboolean::TRUE)
                    .get()
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

    /// Make a real ALTREP class that can be used to make vectors.
    pub fn make_altreal_class<StateType: AltrepImpl + AltRealImpl + 'static>(
        name: &str,
        base: &str,
    ) -> Robj {
        #![allow(non_snake_case)]
        use std::os::raw::c_int;

        single_threaded(|| unsafe {
            let class = Altrep::altrep_class::<StateType>(Rtype::Doubles, name, base);
            let class_ptr = R_altrep_class_t { ptr: class.get() };

            unsafe extern "C" fn altreal_Elt<StateType: AltRealImpl + 'static>(
                x: SEXP,
                i: R_xlen_t,
            ) -> f64 {
                Altrep::get_state::<StateType>(x).elt(i as usize).inner()
            }

            unsafe extern "C" fn altreal_Get_region<StateType: AltRealImpl + 'static>(
                x: SEXP,
                i: R_xlen_t,
                n: R_xlen_t,
                buf: *mut f64,
            ) -> R_xlen_t {
                let slice = std::slice::from_raw_parts_mut(buf as *mut Rfloat, n as usize);
                Altrep::get_state::<StateType>(x).get_region(i as usize, slice) as R_xlen_t
            }

            unsafe extern "C" fn altreal_Is_sorted<StateType: AltRealImpl + 'static>(
                x: SEXP,
            ) -> c_int {
                Altrep::get_state::<StateType>(x).is_sorted().inner() as c_int
            }

            unsafe extern "C" fn altreal_No_NA<StateType: AltRealImpl + 'static>(x: SEXP) -> c_int {
                i32::from(Altrep::get_state::<StateType>(x).no_na())
            }

            unsafe extern "C" fn altreal_Sum<StateType: AltRealImpl + 'static>(
                x: SEXP,
                narm: Rboolean,
            ) -> SEXP {
                Altrep::get_state::<StateType>(x)
                    .sum(narm == Rboolean::TRUE)
                    .get()
            }

            unsafe extern "C" fn altreal_Min<StateType: AltRealImpl + 'static>(
                x: SEXP,
                narm: Rboolean,
            ) -> SEXP {
                Altrep::get_state::<StateType>(x)
                    .min(narm == Rboolean::TRUE)
                    .get()
            }

            unsafe extern "C" fn altreal_Max<StateType: AltRealImpl + 'static>(
                x: SEXP,
                narm: Rboolean,
            ) -> SEXP {
                Altrep::get_state::<StateType>(x)
                    .max(narm == Rboolean::TRUE)
                    .get()
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

    /// Make a logical ALTREP class that can be used to make vectors.
    pub fn make_altlogical_class<StateType: AltrepImpl + AltLogicalImpl + 'static>(
        name: &str,
        base: &str,
    ) -> Robj {
        #![allow(non_snake_case)]
        use std::os::raw::c_int;

        single_threaded(|| unsafe {
            let class = Altrep::altrep_class::<StateType>(Rtype::Logicals, name, base);
            let class_ptr = R_altrep_class_t { ptr: class.get() };

            unsafe extern "C" fn altlogical_Elt<StateType: AltLogicalImpl + 'static>(
                x: SEXP,
                i: R_xlen_t,
            ) -> c_int {
                Altrep::get_state::<StateType>(x).elt(i as usize).inner() as c_int
            }

            unsafe extern "C" fn altlogical_Get_region<StateType: AltLogicalImpl + 'static>(
                x: SEXP,
                i: R_xlen_t,
                n: R_xlen_t,
                buf: *mut c_int,
            ) -> R_xlen_t {
                let slice = std::slice::from_raw_parts_mut(buf as *mut Rbool, n as usize);
                Altrep::get_state::<StateType>(x).get_region(i as usize, slice) as R_xlen_t
            }

            unsafe extern "C" fn altlogical_Is_sorted<StateType: AltLogicalImpl + 'static>(
                x: SEXP,
            ) -> c_int {
                Altrep::get_state::<StateType>(x).is_sorted().inner() as c_int
            }

            unsafe extern "C" fn altlogical_No_NA<StateType: AltLogicalImpl + 'static>(
                x: SEXP,
            ) -> c_int {
                i32::from(Altrep::get_state::<StateType>(x).no_na())
            }

            unsafe extern "C" fn altlogical_Sum<StateType: AltLogicalImpl + 'static>(
                x: SEXP,
                narm: Rboolean,
            ) -> SEXP {
                Altrep::get_state::<StateType>(x)
                    .sum(narm == Rboolean::TRUE)
                    .get()
            }

            R_set_altlogical_Elt_method(class_ptr, Some(altlogical_Elt::<StateType>));
            R_set_altlogical_Get_region_method(class_ptr, Some(altlogical_Get_region::<StateType>));
            R_set_altlogical_Is_sorted_method(class_ptr, Some(altlogical_Is_sorted::<StateType>));
            R_set_altlogical_No_NA_method(class_ptr, Some(altlogical_No_NA::<StateType>));
            R_set_altlogical_Sum_method(class_ptr, Some(altlogical_Sum::<StateType>));

            class
        })
    }

    /// Make a raw ALTREP class that can be used to make vectors.
    pub fn make_altraw_class<StateType: AltrepImpl + AltRawImpl + 'static>(
        name: &str,
        base: &str,
    ) -> Robj {
        #![allow(non_snake_case)]

        single_threaded(|| unsafe {
            let class = Altrep::altrep_class::<StateType>(Rtype::Raw, name, base);
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

    /// Make a complex ALTREP class that can be used to make vectors.
    pub fn make_altcomplex_class<StateType: AltrepImpl + AltComplexImpl + 'static>(
        name: &str,
        base: &str,
    ) -> Robj {
        #![allow(non_snake_case)]

        single_threaded(|| unsafe {
            let class = Altrep::altrep_class::<StateType>(Rtype::Complexes, name, base);
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
                let slice = std::slice::from_raw_parts_mut(buf as *mut Rcplx, n as usize);
                Altrep::get_state::<StateType>(x).get_region(i as usize, slice) as R_xlen_t
            }

            R_set_altcomplex_Elt_method(class_ptr, Some(altcomplex_Elt::<StateType>));
            R_set_altcomplex_Get_region_method(class_ptr, Some(altcomplex_Get_region::<StateType>));

            class
        })
    }

    /// Make a string ALTREP class that can be used to make vectors.
    pub fn make_altstring_class<StateType: AltrepImpl + AltStringImpl + 'static>(
        name: &str,
        base: &str,
    ) -> Robj {
        #![allow(non_snake_case)]
        use std::os::raw::c_int;

        single_threaded(|| unsafe {
            let class = Altrep::altrep_class::<StateType>(Rtype::Strings, name, base);
            let class_ptr = R_altrep_class_t { ptr: class.get() };

            unsafe extern "C" fn altstring_Elt<StateType: AltStringImpl + 'static>(
                x: SEXP,
                i: R_xlen_t,
            ) -> SEXP {
                Altrep::get_state::<StateType>(x).elt(i as usize).get()
            }

            unsafe extern "C" fn altstring_Set_elt<StateType: AltStringImpl + 'static>(
                x: SEXP,
                i: R_xlen_t,
                v: SEXP,
            ) {
                Altrep::get_state_mut::<StateType>(x)
                    .set_elt(i as usize, Robj::from_sexp(v).try_into().unwrap())
            }

            unsafe extern "C" fn altstring_Is_sorted<StateType: AltStringImpl + 'static>(
                x: SEXP,
            ) -> c_int {
                Altrep::get_state::<StateType>(x).is_sorted().inner() as c_int
            }

            unsafe extern "C" fn altstring_No_NA<StateType: AltStringImpl + 'static>(
                x: SEXP,
            ) -> c_int {
                i32::from(Altrep::get_state::<StateType>(x).no_na())
            }

            R_set_altstring_Elt_method(class_ptr, Some(altstring_Elt::<StateType>));
            R_set_altstring_Set_elt_method(class_ptr, Some(altstring_Set_elt::<StateType>));
            R_set_altstring_Is_sorted_method(class_ptr, Some(altstring_Is_sorted::<StateType>));
            R_set_altstring_No_NA_method(class_ptr, Some(altstring_No_NA::<StateType>));

            class
        })
    }

    #[cfg(use_r_altlist)]
    pub fn make_altlist_class<StateType: AltrepImpl + AltListImpl + 'static>(
        name: &str,
        base: &str,
    ) -> Robj {
        #![allow(non_snake_case)]

        single_threaded(|| unsafe {
            let class = Altrep::altrep_class::<StateType>(Rtype::List, name, base);
            let class_ptr = R_altrep_class_t { ptr: class.get() };

            unsafe extern "C" fn altlist_Elt<StateType: AltListImpl + 'static>(
                x: SEXP,
                i: R_xlen_t,
            ) -> SEXP {
                Altrep::get_state::<StateType>(x).elt(i as usize).get()
            }

            unsafe extern "C" fn altlist_Set_elt<StateType: AltListImpl + 'static>(
                x: SEXP,
                i: R_xlen_t,
                v: SEXP,
            ) {
                Altrep::get_state_mut::<StateType>(x).set_elt(i as usize, Robj::from_sexp(v))
            }

            R_set_altlist_Elt_method(class_ptr, Some(altlist_Elt::<StateType>));
            R_set_altlist_Set_elt_method(class_ptr, Some(altlist_Set_elt::<StateType>));
            class
        })
    }

    make_from_iterator!(
        make_altinteger_from_iterator,
        make_altinteger_class,
        AltIntegerImpl,
        Rint,
        i32
    );
    make_from_iterator!(
        make_altlogical_from_iterator,
        make_altlogical_class,
        AltLogicalImpl,
        Rbool,
        i32
    );
    make_from_iterator!(
        make_altreal_from_iterator,
        make_altreal_class,
        AltRealImpl,
        Rfloat,
        f64
    );
    make_from_iterator!(
        make_altcomplex_from_iterator,
        make_altcomplex_class,
        AltComplexImpl,
        Rcplx,
        c64
    );
}

impl<Iter: ExactSizeIterator + std::fmt::Debug + Clone> AltrepImpl for Iter {
    fn length(&self) -> usize {
        self.len()
    }
}
