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

/// A direct representation of R's `externalptr`.
/// This is named "unsafe", as it does not carry type-id information necessary for
/// safe transmuting the stored pointer into typed references (`&T` / `&mut T`, etc.).
///
/// Prefer `[ExternalPtr]` for must use-cases.
#[repr(transparent)]
pub struct UnsafeExternalPtr {
    pub(crate) robj: Robj,
}

impl robj::GetSexp for UnsafeExternalPtr {
    unsafe fn get(&self) -> SEXP {
        self.robj.get()
    }

    unsafe fn get_mut(&mut self) -> SEXP {
        self.robj.get_mut()
    }

    /// Get a reference to a Robj for this type.
    fn as_robj(&self) -> &Robj {
        &self.robj
    }

    /// Get a mutable reference to a Robj for this type.
    fn as_robj_mut(&mut self) -> &mut Robj {
        &mut self.robj
    }
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

    /// Set the stored opaque C pointer, and return the previous contained pointer.
    /// Note that the contained pointer must be dropped manually.
    pub fn set_addr(&self, raw: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
        let previous_addr = self.addr();
        unsafe { R_SetExternalPtrAddr(self.robj.get(), raw) };
        previous_addr
    }

    /// Set the "tag" of the `externalptr`
    pub fn set_tag(&self, tag: SEXP) {
        unsafe { R_SetExternalPtrTag(self.robj.get(), tag) }
    }

    /// Set the "protected" of the `externalptr`
    pub fn set_protected(&self, protected: SEXP) {
        unsafe { R_SetExternalPtrProtected(self.robj.get(), protected) }
    }
}

impl UnsafeExternalPtr {
    /// Returns a new, owned `externalptr`, with type information corresponding to `T`.
    ///
    /// # Safety
    ///
    /// It is on the caller that the given type `T` is indeed the stored pointer in this `externalptr`.
    /// There is no way to ensure that this is the case otherwise, and therefore invoking this method
    /// is deemed unsafe.
    pub unsafe fn try_into_externalptr<T>(self) -> Result<ExternalPtr<T>>
    where
        T: 'static,
    {
        // don't need to check if the underlying Robj (SEXP) is an externalptr..

        if self.addr().is_null() {
            return Err(Error::ExpectedExternalNonNullPtr(self.robj));
        }
        // steal the pointer, and amend the type information
        let addr = self.set_addr(std::ptr::null_mut());
        // assume the given type is correct
        let addr = addr.cast::<T>();
        let externalptr = ExternalPtr::from_raw(addr);

        Ok(externalptr)
    }
}

impl std::fmt::Pointer for UnsafeExternalPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Pointer::fmt(&self.addr(), f)
    }
}

impl Types for UnsafeExternalPtr {} // required for Attributes
impl Length for UnsafeExternalPtr {} // required for Attributes
/// `set_attrib`
impl Attributes for UnsafeExternalPtr {}

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
