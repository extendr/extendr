use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Altrep {
    pub(crate) robj: Robj,
}

/// Rust trait for implementing ALTREP.
/// Implement one or more of these methods to generate an Altrep class.
/// Mechanism TBD.
trait AltrepImpl {
    /// Constructor that is called when loading an Altrep object from a file.
    fn unserialize_ex(class: Robj, state: Robj, attributes: Robj, obj_flags: i32, levels: i32) -> Robj {
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
}

trait AltvecImpl: AltrepImpl {
    /// Get the virtual length of the vector.
    /// For example for a compact range, return end - start + 1.
    fn length(&self) -> usize {
        0
    }

    /// Get the data pointer for this vector, possibly expanding the
    /// compact representation into a full R vector.
    /// We may move this into AltIntegerImpl etc. and use a slice.
    fn dataptr(&self, _writeable: bool) -> *mut u8 {
        std::ptr::null_mut()
    }

    /// Get the data pointer for this vector, returning NULL
    /// if the object is 
    /// We may move this into AltIntegerImpl etc. and use a slice.
    fn dataptr_or_null(&self) -> *const u8 {
        std::ptr::null()
    }

    /// Implement subsetting (eg. x[10:19]) for this Altrep vector.
    fn extract_subset(&self, _indx: Robj, _call: Robj) -> Robj {
        ().into()
    }
}

trait AltIntegerImpl: AltvecImpl {
    /// Get a single element from this vector.
    fn elt(&self, _index: usize) -> i32 {
        0
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

trait AltRealImpl: AltvecImpl {
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

trait AltLogicalImpl: AltvecImpl {
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

trait AltRawImpl: AltvecImpl {
    /// Get a single element from this vector.
    fn elt(&self, _index: usize) -> u8 {
        0
    }

    /// Get a multiple elements from this vector.
    fn get_region(&self, _index: usize, _data: &mut [u8]) -> usize {
        0
    }
}

trait AltComplexImpl: AltvecImpl {
    /// Get a single element from this vector.
    fn elt(&self, _index: usize) -> Complex {
        Complex(0.0, 0.0)
    }

    /// Get a multiple elements from this vector.
    fn get_region(&self, _index: usize, _data: &mut [Complex]) -> usize {
        0
    }
}

trait AltStringImpl: AltvecImpl {
    /// Get a single element from this vector.
    fn elt(&self, _index: usize) -> Robj {
        ().into()
    }

    /// Set a single element in this vector.
    fn set_elt(&mut self, _index: usize, _value: Robj) {

    }

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
