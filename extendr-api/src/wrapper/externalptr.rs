use super::*;
use std::any::Any;
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
///
#[derive(PartialEq, Clone)]
pub struct ExternalPtr<T: Debug + 'static> {
    /// This is the contained Robj.
    pub(crate) robj: Robj,

    /// This is a zero-length object that holds the type of the object.
    marker: std::marker::PhantomData<T>,
}

impl<T: Debug + 'static> robj::GetSexp for ExternalPtr<T> {
    unsafe fn get(&self) -> SEXP {
        self.robj.get()
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
impl<T: Debug + 'static> Length for ExternalPtr<T> {}

/// rtype() and rany()
impl<T: Debug + 'static> Types for ExternalPtr<T> {}

/// as_*()
impl<T: Debug + 'static> Conversions for ExternalPtr<T> {}

/// find_var() etc.
impl<T: Debug + 'static> Rinternals for ExternalPtr<T> {}

/// as_typed_slice_raw() etc.
impl<T: Debug + 'static> Slices for ExternalPtr<T> {}

/// dollar() etc.
impl<T: Debug + 'static> Operators for ExternalPtr<T> {}

impl<T: Debug + 'static> Deref for ExternalPtr<T> {
    type Target = T;

    /// This allows us to treat the Robj as if it is the type T.
    fn deref(&self) -> &Self::Target {
        self.addr()
    }
}

impl<T: Debug + 'static> DerefMut for ExternalPtr<T> {
    /// This allows us to treat the Robj as if it is the mutable type T.
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.addr_mut()
    }
}

struct ExternalData(Box<dyn Any>);

impl<T: Any + Debug> ExternalPtr<T> {
    /// Construct an external pointer object from any type T.
    /// In this case, the R object owns the data and will drop the Rust object
    /// when the last reference is removed via register_c_finalizer.
    ///
    /// An ExternalPtr behaves like a Box except that the information is
    /// tracked by a R object.
    pub fn new(val: T) -> Self {
        use std::ffi::c_void;
        unsafe {
            // This allocates some memory for our object and moves the object into it.
            let v = ExternalData(Box::new(val));

            // This constructs an external pointer to our boxed data.
            // into_raw() converts the box to a malloced pointer.
            let ptr = Box::into_raw(Box::new(v));
            let external_ptr = R_MakeExternalPtr(ptr as *mut c_void, R_NilValue, R_NilValue);

            // ensure that this is protected
            let robj = Robj::from_sexp(external_ptr);

            extern "C" fn finalizer<T: 'static>(x: SEXP) {
                unsafe {
                    let ptr = R_ExternalPtrAddr(x);
                    // Convert the pointer to a box and drop it implictly.
                    // This frees up the memory we have used and calls the "T::drop" method if there is one.
                    drop(Box::from_raw(ptr as *mut ExternalData));

                    // Now set the pointer in ExternalPTR to C `NULL`
                    R_ClearExternalPtr(x);
                }
            }

            // Tell R about our finalizer
            robj.register_c_finalizer(Some(finalizer::<T>));

            // Return an object in a wrapper.
            Self {
                robj,
                marker: std::marker::PhantomData,
            }
        }
    }

    // TODO: make a constructor for references?

    /// Get the "tag" of an external pointer. This is the type name in the common case.
    pub fn tag(&self) -> Robj {
        unsafe { new_owned(R_ExternalPtrTag(self.robj.get())) }
    }

    /// Get the "protected" field of an external pointer. This is NULL in the common case.
    pub fn protected(&self) -> Robj {
        unsafe { new_owned(R_ExternalPtrProtected(self.robj.get())) }
    }

    /// Get the "address" field of an external pointer.
    /// Normally, we will use Deref to do this.
    pub fn addr(&self) -> &T {
        unsafe {
            let ptr = R_ExternalPtrAddr(self.robj.get()) as *const ExternalData;
            let ptr_ref = &*ptr;
            let ptr_ref_downcast = ptr_ref.0.downcast_ref::<T>().unwrap();
            ptr_ref_downcast
        }
    }

    /// Get the "address" field of an external pointer as a mutable reference.
    /// Normally, we will use DerefMut to do this.
    pub fn addr_mut(&mut self) -> &mut T {
        unsafe {
            let ptr = R_ExternalPtrAddr(self.robj.get()) as *mut ExternalData;
            let ptr_ref = &mut *ptr;
            let ptr_ref_downcast = ptr_ref.0.downcast_mut::<T>().unwrap();
            ptr_ref_downcast
        }
    }
}

impl<T: Any + Debug> TryFrom<&Robj> for ExternalPtr<T> {
    type Error = Error;

    fn try_from(robj: &Robj) -> Result<Self> {
        if robj.rtype() != Rtype::ExternalPtr {
            return Err(Error::ExpectedExternalPtr(robj.clone()));
        }
        let is_type = unsafe {
            let external_ptr = R_ExternalPtrAddr(robj.get()) as *const ExternalData;
            let is_type = &*external_ptr;
            let is_type = is_type.0.downcast_ref::<T>();
            is_type.is_some()
        };
        if is_type {
            let res = ExternalPtr::<T> {
                robj: robj.clone(),
                marker: std::marker::PhantomData,
            };
            Ok(res)
        } else {
            Err(Error::ExpectedExternalPtrType(
                robj.clone(),
                std::any::type_name::<T>().into(),
            ))
        }
    }
}

impl<T: Any + Debug> TryFrom<Robj> for ExternalPtr<T> {
    type Error = Error;

    fn try_from(robj: Robj) -> Result<Self> {
        <ExternalPtr<T>>::try_from(&robj)
    }
}

impl<T: Any + Debug> From<ExternalPtr<T>> for Robj {
    fn from(val: ExternalPtr<T>) -> Self {
        val.robj
    }
}

impl<T: Debug + 'static> std::fmt::Debug for ExternalPtr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (&**self as &T).fmt(f)
    }
}
