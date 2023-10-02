use super::*;
use std::any::{Any, TypeId};
use std::fmt::Debug;
use std::marker::PhantomData;

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
    marker: PhantomData<T>,
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

/// Internal type to represent a Rust type tagged it's [`TypeId`].
///
/// This means that [`R_MakeExternalPtr`] returns a tagged pointer, that
/// we can use to test the validity of the type-casting at runtime.
///
#[derive(Debug)]
#[repr(C)]
struct ExternalData<T> {
    type_id: TypeId,
    data: T,
}

impl<T: 'static> ExternalData<T> {
    fn new(data: T) -> Self {
        Self {
            type_id: TypeId::of::<T>(),
            data,
        }
    }
}

impl<T: Any + Debug> ExternalPtr<T> {
    /// Construct an external pointer object from any type T.
    /// In this case, the R object owns the data and will drop the Rust object
    /// when the last reference is removed via register_c_finalizer.
    ///
    /// An ExternalPtr behaves like a Box except that the information is
    /// tracked by a R object.
    pub fn new(val: T) -> Self {
        use std::ffi::c_void;
        // This allocates some memory for our object and moves the object into it.
        let boxed = Box::new(ExternalData::new(val));
        unsafe {
            // This constructs an external pointer to our boxed data.
            // into_raw() converts the box to a malloced pointer.
            let robj = Robj::from_sexp(single_threaded(|| {
                // add `tag` so that in R `.Internal(inspect(x))` would show 
                // type-name.
                let tag = Rf_install(
                    std::ffi::CString::new(std::any::type_name::<T>())
                        .unwrap()
                        .as_ptr(),
                );
                R_MakeExternalPtr(Box::into_raw(boxed) as *mut c_void, tag, R_NilValue)
            }));
            extern "C" fn finalizer<T>(x: SEXP) {
                unsafe {
                    /// Free the `tag` which is the type name
                    R_SetExternalPtrTag(x, R_NilValue);
                    let ptr = R_ExternalPtrAddr(x) as *mut ExternalData<T>;

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
                marker: std::marker::PhantomData,
            }
        }
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
    pub fn addr<'a>(&self) -> &'a T {
        unsafe {
            let ptr = R_ExternalPtrAddr(self.robj.get()) as *const ExternalData<T>;
            &(*ptr).data
        }
    }

    /// Get the "address" field of an external pointer as a mutable reference.
    /// Normally, we will use DerefMut to do this.
    pub fn addr_mut(&mut self) -> &mut T {
        unsafe {
            let ptr = R_ExternalPtrAddr(self.robj.get()) as *mut ExternalData<T>;
            &mut (*ptr).data
        }
    }
}

impl<T: 'static> ExternalPtr<T>
where
    T: Debug,
{
    pub fn try_get_from_robj(robj: &Robj) -> std::result::Result<&T, String> {
        if !robj.is_external_pointer() {
            return Err("expected `ExternalPtr`".into());
        }
        use std::ptr::addr_of;
        let external_ptr = single_threaded(|| unsafe { R_ExternalPtrAddr(robj.get()) });
        let type_id = unsafe { *addr_of!((&*(external_ptr as *const ExternalData<()>)).type_id) };
        if type_id != TypeId::of::<T>() {
            return Err(format!(
                "expected `T` to be of type {}",
                std::any::type_name::<T>()
            ));
        }
        let external_ptr = external_ptr as *const ExternalData<T>;
        Ok(unsafe { &(*external_ptr).data })
    }

    pub fn try_get_mut_from_robj(robj: &mut Robj) -> std::result::Result<&mut T, String> {
        if !robj.is_external_pointer() {
            return Err("expected `ExternalPtr`".into());
        }
        use std::ptr::addr_of;
        let external_ptr = single_threaded(|| unsafe { R_ExternalPtrAddr(robj.get()) });
        let type_id = unsafe { *addr_of!((&*(external_ptr as *const ExternalData<()>)).type_id) };
        if type_id != TypeId::of::<T>() {
            return Err(format!(
                "expected `T` to be of type {}",
                std::any::type_name::<T>()
            ));
        }
        let external_ptr = external_ptr as *mut ExternalData<T>;
        Ok(unsafe { &mut (*external_ptr).data })
    }
}

impl<T: Any + Debug> TryFrom<&Robj> for ExternalPtr<T> {
    type Error = Error;

    fn try_from(robj: &Robj) -> Result<Self> {
        use std::ptr::addr_of;
        if robj.rtype() != Rtype::ExternalPtr {
            return Err(Error::ExpectedExternalPtr(robj.clone()));
        }
        let external_ptr = single_threaded(|| unsafe { R_ExternalPtrAddr(robj.get()) });
        let type_id = unsafe { *addr_of!((&*(external_ptr as *const ExternalData<()>)).type_id) };
        if type_id != TypeId::of::<T>() {
            return Err(Error::ExpectedExternalPtrType(
                robj.clone(),
                format!("expected {}", std::any::type_name::<T>()),
            ));
        }
        Ok(ExternalPtr {
            robj: robj.clone(),
            marker: PhantomData,
        })
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
