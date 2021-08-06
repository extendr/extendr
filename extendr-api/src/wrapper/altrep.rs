use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Altrep {
    pub(crate) robj: Robj,
}

/// Rust trait for implementing ALTREP.
trait AltrepImpl {
    fn unserialize_ex(_class: Robj, _state: Robj, _attr: Robj, _objf: i32, _levs: i32) -> Robj {
        ().into()
    }

    fn unserialize(_class: Robj, _state: Robj) -> Robj {
        ().into()
    }

    fn serialized_state(&self) -> Robj {
        ().into()
    }

    fn duplicate_ex(&self, _deep: bool) -> Robj {
        ().into()
    }

    fn duplicate(&self, _deep: bool) -> Robj {
        ().into()
    }

    fn coerce(&self, _ty: RType) -> Robj {
        ().into()
    }

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
    fn length(&self) -> usize {
        0
    }
    fn dataptr(&self, _writeable: bool) -> *mut u8 {
        std::ptr::null_mut()
    }
    fn dataptr_or_null(&self) -> *const u8 {
        std::ptr::null()
    }
    fn extract_subset(&self, _indx: Robj, _call: Robj) -> Robj {
        ().into()
    }
}

trait AltIntegerImpl: AltvecImpl {
    fn elt(&self, _index: usize) -> i32 {
        0
    }
    fn get_region(&self, _index: usize, _data: &mut [i32]) -> usize {
        0
    }
    fn is_sorted(&self) -> bool {
        false
    }
    fn no_na(&self) -> bool {
        false
    }
    fn sum(&self, _remove_nas: bool) -> Robj {
        ().into()
    }
    fn min(&self, _remove_nas: bool) -> Robj {
        ().into()
    }
    fn max(&self, _remove_nas: bool) -> Robj {
        ().into()
    }
}

trait AltRealImpl: AltvecImpl {
    fn elt(&self, _index: usize) -> f64 {
        0.0
    }
    fn get_region(&self, _index: usize, _data: &mut [i32]) -> usize {
        0
    }
    fn is_sorted(&self) -> bool {
        false
    }
    fn no_na(&self) -> bool {
        false
    }
    fn sum(&self, _remove_nas: bool) -> Robj {
        ().into()
    }
    fn min(&self, _remove_nas: bool) -> Robj {
        ().into()
    }
    fn max(&self, _remove_nas: bool) -> Robj {
        ().into()
    }
}

trait AltLogicalImpl: AltvecImpl {
    fn elt(&self, _index: usize) -> Bool {
        NA_LOGICAL
    }
    fn get_region(&self, _index: usize, _data: &mut [i32]) -> usize {
        0
    }
    fn is_sorted(&self) -> bool {
        false
    }
    fn no_na(&self) -> bool {
        false
    }
    fn sum(&self, _remove_nas: bool) -> Robj {
        ().into()
    }
}

trait AltRawImpl: AltvecImpl {
    fn elt(&self, _index: usize) -> u8 {
        0
    }

    fn get_region(&self, _index: usize, _data: &mut [u8]) -> usize {
        0
    }
}

trait AltComplexImpl: AltvecImpl {
    fn altcomplex_elt(&self, _index: usize) -> Complex {
        Complex(0.0, 0.0)
    }

    fn altcomplex_get_region(&self, _index: usize, _data: &mut [Complex]) -> usize {
        0
    }
}

trait AltStringImpl: AltvecImpl {
    fn elt(&self, _index: usize) -> Robj {
        ().into()
    }
    fn set_elt(&self, _index: usize, _value: Robj) {}
    fn is_sorted(&self) -> bool {
        false
    }
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
