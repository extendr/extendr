//! Unsafe variant of [ExternalPtr].
//!
//!
//!
//!
//!
//!
use super::*;

#[repr(transparent)]
pub struct UnsafeExternalPtr {
    pub(crate) robj: Robj,
}

impl UnsafeExternalPtr {
    pub fn addr(&self) -> *mut std::ffi::c_void {
        unsafe { R_ExternalPtrAddr(self.robj.get()) }
    }

    /// Get the "tag" of an external pointer. This is the type name in the common case.
    pub fn tag(&self) -> SEXP {
        unsafe { R_ExternalPtrTag(self.robj.get()) }
    }

    /// Get the "protected" field of an external pointer. This is NULL in the common case.
    pub fn protected(&self) -> SEXP {
        unsafe { R_ExternalPtrProtected(self.robj.get()) }
    }
}

impl UnsafeExternalPtr {
    unsafe fn try_into_externalptr<T>(self) -> Result<ExternalPtr<T>>
    where
        T: std::default::Default + 'static,
    {
        // don't need to check if the underlying Robj (SEXP) is an externalptr..

        if self.addr().is_null() {
            return Err(Error::ExpectedExternalNonNullPtr(self.robj));
        }
        // assume the given type is correct
        let addr = self.addr().cast::<T>();
        let value = std::mem::replace(addr.as_mut().unwrap(), T::default());

        Ok(ExternalPtr::new(value))
    }
}

impl TryFrom<&Robj> for UnsafeExternalPtr {
    type Error = Error;

    fn try_from(value: &Robj) -> Result<Self> {
        if !value.is_external_pointer() {
            return Err(Error::ExpectedExternalPtr(value.clone()));
        }
        Ok(UnsafeExternalPtr {
            robj: value.clone(),
        })
    }
}

impl TryFrom<Robj> for UnsafeExternalPtr {
    type Error = Error;

    fn try_from(value: Robj) -> Result<Self> {
        (&value).try_into()
    }
}

// TODO: convenience conversion impls missing, TryFrom<Robj>, TryFrom<Option<Robj>>, etc.
