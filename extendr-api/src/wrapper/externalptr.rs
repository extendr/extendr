//! `ExternalPtr` is a way to leak Rust allocated data to R, forego deallocation
//! to R and its GC strategy.
//!
//! An `ExternalPtr` encompasses three values, an owned pointer to the Rust
//! type, a `tag` and a `prot`. Tag is a helpful naming of the type, but
//! it doesn't offer any solid type-checking capability. And `prot` is meant
//! to be R values, that are supposed to be kept together with the `ExternalPtr`.
//!
//! Neither `tag` nor `prot` are attributes, therefore to use `ExternalPtr` as
//! a class in R, you must decorate it with a class-attribute manually.
//!
//! **Beware**: Equality (by way of `PartialEq`) does not imply equality of value,
//! but equality of pointer. Two objects stored as `ExternalPtr` may be equal
//! in value, but be two distinct entities, with distinct pointers.
//!
//! Instead, rely on `AsRef` to make _by value_ comparison, e.g. to compare
//! for equality of
//! two instances of `ExternalPtr<T>` by value, `a.as_ref() == b.as_ref()`.
//!
use super::*;
use std::fmt::Debug;

/// Wrapper for creating R objects containing any Rust object.
///
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let extptr = ExternalPtr::new(1);
///     assert_eq!(*extptr, 1);
///     let robj : Robj = extptr.into();
///     let extptr2 : ExternalPtr<i32> = robj.try_into().unwrap();
///     assert_eq!(*extptr2, 1);
/// }
/// ```
#[repr(transparent)]
pub struct ExternalPtr<T> {
    /// This is the contained Robj.
    pub(crate) robj: Robj,

    /// This is a zero-length object that holds the type of the object.
    _marker: std::marker::PhantomData<T>,
}

/// Manual implementation of `PartialEq`, because the constraint `T: PartialEq`
/// is not necessary.
impl<T> PartialEq for ExternalPtr<T> {
    fn eq(&self, other: &Self) -> bool {
        self.robj == other.robj && self._marker == other._marker
    }
}

/// Manual implementation of `Clone` trait, because the assumed constraint `T: Clone` is not necessary.
impl<T> Clone for ExternalPtr<T> {
    fn clone(&self) -> Self {
        Self {
            robj: self.robj.clone(),
            _marker: self._marker,
        }
    }
}

impl<T> robj::GetSexp for ExternalPtr<T> {
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

/// len() and is_empty()
impl<T> Length for ExternalPtr<T> {}

/// rtype() and rany()
impl<T> Types for ExternalPtr<T> {}

/// `set_attrib`
impl<T> Attributes for ExternalPtr<T> {}

/// as_*()
impl<T> Conversions for ExternalPtr<T> {}

/// find_var() etc.
impl<T> Rinternals for ExternalPtr<T> {}

/// as_typed_slice_raw() etc.
impl<T> Slices for ExternalPtr<T> {}

/// dollar() etc.
impl<T> Operators for ExternalPtr<T> {}

impl<T> Deref for ExternalPtr<T> {
    type Target = T;

    /// This allows us to treat the Robj as if it is the type T.
    fn deref(&self) -> &Self::Target {
        self.addr()
    }
}

impl<T> DerefMut for ExternalPtr<T> {
    /// This allows us to treat the Robj as if it is the mutable type T.
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.addr_mut()
    }
}

impl<T> ExternalPtr<T> {
    /// Construct an external pointer object from any type T.
    /// In this case, the R object owns the data and will drop the Rust object
    /// when the last reference is removed via register_c_finalizer.
    ///
    /// An ExternalPtr behaves like a Box except that the information is
    /// tracked by a R object.
    pub fn new(val: T) -> Self {
        single_threaded(|| unsafe {
            // This allocates some memory for our object and moves the object into it.
            let boxed = Box::new(val);

            // This constructs an external pointer to our boxed data.
            // into_raw() converts the box to a malloced pointer.
            let robj = Robj::make_external_ptr(Box::into_raw(boxed), Robj::from(()));

            extern "C" fn finalizer<T>(x: SEXP) {
                unsafe {
                    let ptr = R_ExternalPtrAddr(x).cast::<T>();

                    // Free the `tag`, which is the type-name
                    R_SetExternalPtrTag(x, R_NilValue);

                    // Convert the pointer to a box and drop it implictly.
                    // This frees up the memory we have used and calls the "T::drop" method if there is one.
                    drop(Box::from_raw(ptr));

                    // Now set the pointer in ExternalPTR to C `NULL`
                    R_ClearExternalPtr(x);
                }
            }

            // Tell R about our finalizer
            robj.register_c_finalizer(Some(finalizer::<T>));

            // Return an object in a wrapper.
            Self {
                robj,
                _marker: std::marker::PhantomData,
            }
        })
    }

    // TODO: make a constructor for references?

    /// Get the "tag" of an external pointer. This is the type name in the common case.
    pub fn tag(&self) -> Robj {
        unsafe { Robj::from_sexp(R_ExternalPtrTag(self.robj.get())) }
    }

    /// Get the "protected" field of an external pointer. This is NULL in the common case.
    pub fn protected(&self) -> Robj {
        unsafe { Robj::from_sexp(R_ExternalPtrProtected(self.robj.get())) }
    }

    /// Get the "address" field of an external pointer.
    /// Normally, we will use Deref to do this.
    ///
    /// ## Panics
    ///
    /// When the underlying pointer is C `NULL`.
    pub fn addr(&self) -> &T {
        self.try_addr().unwrap()
    }

    /// Get the "address" field of an external pointer as a mutable reference.
    /// Normally, we will use DerefMut to do this.
    ///
    /// ## Panics
    ///
    /// When the underlying pointer is C `NULL`.
    pub fn addr_mut(&mut self) -> &mut T {
        self.try_addr_mut().unwrap()
    }
    /// Get the "address" field of an external pointer.
    /// Normally, we will use Deref to do this.
    ///
    /// ## Panics
    ///
    /// When the underlying pointer is C `NULL`.
    pub fn try_addr(&self) -> Result<&T> {
        unsafe {
            R_ExternalPtrAddr(self.robj.get())
                .cast::<T>()
                .cast_const()
                .as_ref()
                .ok_or_else(|| Error::ExpectedExternalNonNullPtr(self.robj.clone()))
        }
    }

    /// Get the "address" field of an external pointer as a mutable reference.
    /// Normally, we will use DerefMut to do this.
    ///
    /// ## Panics
    ///
    /// When the underlying pointer is C `NULL`.
    pub fn try_addr_mut(&mut self) -> Result<&mut T> {
        unsafe {
            R_ExternalPtrAddr(self.robj.get_mut())
                .cast::<T>()
                .as_mut()
                .ok_or_else(|| Error::ExpectedExternalNonNullPtr(self.robj.clone()))
        }
    }
}

impl<T> TryFrom<&Robj> for &ExternalPtr<T> {
    type Error = Error;

    fn try_from(value: &Robj) -> Result<Self> {
        if !value.is_external_pointer() {
            return Err(Error::ExpectedExternalPtr(value.clone()));
        }
        unsafe { Ok(std::mem::transmute(value)) }
    }
}

impl<T> TryFrom<&mut Robj> for &mut ExternalPtr<T> {
    type Error = Error;

    fn try_from(value: &mut Robj) -> Result<Self> {
        if !value.is_external_pointer() {
            return Err(Error::ExpectedExternalPtr(value.clone()));
        }
        unsafe { Ok(std::mem::transmute(value)) }
    }
}

impl<T> TryFrom<&Robj> for ExternalPtr<T> {
    type Error = Error;

    fn try_from(robj: &Robj) -> Result<Self> {
        let result: &Self = robj.try_into()?;
        Ok(result.clone())
    }
}

impl<T> TryFrom<Robj> for ExternalPtr<T> {
    type Error = Error;

    fn try_from(robj: Robj) -> Result<Self> {
        <ExternalPtr<T>>::try_from(&robj)
    }
}

impl<T> From<ExternalPtr<T>> for Robj {
    fn from(val: ExternalPtr<T>) -> Self {
        val.robj
    }
}

impl<T> From<Option<ExternalPtr<T>>> for Robj {
    fn from(value: Option<ExternalPtr<T>>) -> Self {
        match value {
            None => nil_value(),
            Some(value) => value.into(),
        }
    }
}

impl<T: Debug> std::fmt::Debug for ExternalPtr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (&**self as &T).fmt(f)
    }
}

impl<T> AsRef<T> for ExternalPtr<T> {
    fn as_ref(&self) -> &T {
        self.addr()
    }
}

impl<T> AsMut<T> for ExternalPtr<T> {
    fn as_mut(&mut self) -> &mut T {
        self.addr_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use extendr_engine::with_r;

    #[derive(Debug)]
    struct BareWrapper(i32);

    #[test]
    fn externalptr_is_ptr() {
        with_r(|| {
            let a = BareWrapper(42);
            let b = BareWrapper(42);
            assert_eq!(a.0, b.0);

            let a_ptr = std::ptr::addr_of!(a);
            let b_ptr = std::ptr::addr_of!(b);
            let a_externalptr = ExternalPtr::new(a);
            let b_externalptr = ExternalPtr::new(b);

            assert_ne!(
                a_ptr, b_ptr,
                "pointers has to be equal by address, not value"
            );

            assert_ne!(
                a_externalptr.robj, b_externalptr.robj,
                "R only knows about the pointer, and not the pointee"
            );
            assert_ne!(
                a_externalptr, b_externalptr,
                "ExternalPtr acts exactly like a pointer"
            );
            assert_ne!(&a_externalptr, &b_externalptr,);
        });
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    struct Wrapper(i32);

    #[test]
    fn compare_externalptr_pointee() {
        with_r(|| {
            let a = Wrapper(42);
            let b = Wrapper(42);
            let a_externalptr = ExternalPtr::new(a);
            let b_externalptr = ExternalPtr::new(b);
            assert_eq!(a_externalptr.as_ref(), b_externalptr.as_ref());

            // let's test more use of `PartialOrd` on `T`
            let a_externalptr = ExternalPtr::new(Wrapper(50));
            let b_externalptr = ExternalPtr::new(Wrapper(60));
            assert!(a_externalptr.as_ref() <= b_externalptr.as_ref());
            assert_eq!(
                a_externalptr.as_ref().max(b_externalptr.as_ref()),
                &Wrapper(60)
            )
        });
    }
}
