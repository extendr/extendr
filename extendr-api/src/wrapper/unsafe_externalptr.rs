//! Unsafe interface to R's `externalptr`
//!
//!
//! R's C-API allows for the passing of owned, thin pointers to foreign objects through its
//! `externalptr` interface. The `extendr-api` provides a type-checked wrapper of this called [ExternalPtr].
//! It is recommended to use that for all purposes of passing Rust types to R, and vice versa.
//!
//! In the case, where a foreign pointer needs to be represented by `extendr`, we provide [`UnsafeExternalPtr`].
//! Type checking of foreign pointers is not possible, thus this wrapper type cannot safely be converted
//! to `[ExternalPtr<T>]`. See [`UnsafeExternalPtr::try_into_externalptr`] for more details.
//!
//!
use super::*;

#[repr(transparent)]
pub struct UnsafeExternalPtr {
    pub(crate) robj: Robj,
}

impl UnsafeExternalPtr {
    /// Returns the opaque pointer stored in this `externalptr`.
    pub fn addr(&self) -> *mut std::ffi::c_void {
        unsafe { R_ExternalPtrAddr(self.robj.get()) }
    }

    /// Get the "tag" of an `externalptr`.
    ///
    /// Usually, it corresponds to the type name.
    pub fn tag(&self) -> SEXP {
        unsafe { R_ExternalPtrTag(self.robj.get()) }
    }

    /// Get the "protected" field of an external pointer. This is R `NULL` in the common case.
    pub fn protected(&self) -> SEXP {
        unsafe { R_ExternalPtrProtected(self.robj.get()) }
    }

    pub fn set_addr(&self, raw: *mut std::ffi::c_void) {
        unsafe { R_SetExternalPtrAddr(self.robj.get(), raw) }
    }

    pub fn set_tag(&self, tag: SEXP) {
        unsafe { R_SetExternalPtrTag(self.robj.get(), tag) }
    }

    pub fn set_protected(&self, protected: SEXP) {
        unsafe { R_SetExternalPtrProtected(self.robj.get(), protected) }
    }
}

impl UnsafeExternalPtr {
    /// Returns a new, owned `externalptr`, with type information corresponding to `T`.
    ///
    unsafe fn try_into_externalptr<T>(self) -> Result<ExternalPtr<T>>
    where
        T: 'static,
    {
        // don't need to check if the underlying Robj (SEXP) is an externalptr..

        if self.addr().is_null() {
            return Err(Error::ExpectedExternalNonNullPtr(self.robj));
        }
        // assume the given type is correct
        let addr = self.addr().cast::<T>();
        // steal the pointer, and amend the type information
        let externalptr = ExternalPtr::from_raw(addr);
        self.set_addr(std::ptr::null_mut());

        Ok(externalptr)
    }
}

impl TryFrom<&Robj> for UnsafeExternalPtr {
    type Error = Error;

    fn try_from(value: &Robj) -> Result<Self> {
        let robj = value.clone();
        if !value.is_external_pointer() {
            return Err(Error::ExpectedExternalPtr(robj));
        }
        Ok(UnsafeExternalPtr { robj })
    }
}

impl TryFrom<Robj> for UnsafeExternalPtr {
    type Error = Error;

    fn try_from(value: Robj) -> Result<Self> {
        (&value).try_into()
    }
}

// TODO: convenience conversion impls missing, TryFrom<Robj>, TryFrom<Option<Robj>>, etc.
