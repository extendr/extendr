use super::*;
use std::any::Any;

/// Wrapper for creating R objects containing Any Rust object.
///
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let extptr = ExternalPtr::from_val(1);
///     assert_eq!(*extptr, 1);
///     let robj : Robj = extptr.into();
///     let extptr2 : ExternalPtr<i32> = robj.try_into().unwrap();
///     assert_eq!(*extptr2, 1);
/// }
/// ```
///
#[derive(Debug, PartialEq, Clone)]
pub struct ExternalPtr<T> {
    pub(crate) robj: Robj,
    marker: std::marker::PhantomData<T>,
}

impl<T> Deref for ExternalPtr<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.robj.external_ptr_addr::<T>() }
    }
}

impl<T: Any> ExternalPtr<T> {
    pub fn from_val(val: T) -> Self {
        unsafe {
            let type_name = std::any::type_name::<T>();
            let boxed = Box::new(val);
            let robj = Robj::make_external_ptr(Box::into_raw(boxed), r!(type_name), r!(()));

            extern "C" fn finalizer<T>(x: SEXP) {
                unsafe {
                    let ptr = R_ExternalPtrAddr(x) as *mut T;
                    Box::from_raw(ptr);
                }
            }
            robj.register_c_finalizer(Some(finalizer::<T>));
            Self {
                robj,
                marker: std::marker::PhantomData,
            }
        }
    }

    pub fn external_ptr_tag(&self) -> Robj {
        unsafe { new_owned(R_ExternalPtrTag(self.robj.get())) }
    }

    pub fn external_ptr_protected(&self) -> Robj {
        unsafe { new_owned(R_ExternalPtrProtected(self.robj.get())) }
    }
}

impl<T: Any> TryFrom<Robj> for ExternalPtr<T> {
    type Error = Error;

    fn try_from(robj: Robj) -> Result<Self> {
        if robj.rtype() != RType::ExternalPtr {
            return Err(Error::ExpectedExternalPtr(robj));
        }

        let res = ExternalPtr::<T> {
            robj,
            marker: std::marker::PhantomData,
        };

        let type_name = std::any::type_name::<T>();
        if res.external_ptr_tag().as_str() != Some(type_name) {
            return Err(Error::ExpectedExternalPtrType(res.robj, type_name.into()));
        }

        Ok(res)
    }
}

impl<T: Any> From<ExternalPtr<T>> for Robj {
    fn from(val: ExternalPtr<T>) -> Self {
        val.robj
    }
}
