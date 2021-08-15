use extendr_api::prelude::*;
use lazy_static::lazy_static;

#[test]
fn test_externalptr() {
    test! {
        let extptr = ExternalPtr::from_val(1);
        assert_eq!(*extptr, 1);
        let robj : Robj = extptr.into();
        let extptr2 : ExternalPtr<i32> = robj.try_into().unwrap();
        assert_eq!(*extptr2, 1);
    }
}

#[test]
fn test_externalptr_drop() {
    test! {
        lazy_static! {
            static ref Z : std::sync::Mutex<bool> = std::sync::Mutex::new(false);
        }

        struct X {
        }

        // Check that drop() is called after the owning object is dropped.
        impl Drop for X {
            fn drop(&mut self) {
                let mut lck = Z.lock().unwrap();
                *lck = true;
            }
        }

        // Create and external pointer to test the drop.
        let extptr = ExternalPtr::from_val(X {});

        // The object should be protected here - drop not called yet.
        R!("gc()").unwrap();
        assert_eq!(*Z.lock().unwrap(), false);

        // Dropping the pointer should allow gc to drop the object.
        drop(extptr);
        R!("gc()").unwrap();
        assert_eq!(*Z.lock().unwrap(), true);
    }
}
