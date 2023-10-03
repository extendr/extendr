use extendr_api::prelude::*;
use once_cell::sync::Lazy;

#[test]
fn test_externalptr() {
    test! {
        let extptr = ExternalPtr::new(1);
        assert_eq!(*extptr, 1);
        let robj : Robj = extptr.into();
        let extptr2 : ExternalPtr<i32> = robj.try_into().unwrap();
        assert_eq!(*extptr2, 1);
    }
}

#[test]
fn test_externalptr_drop() {
    test! {
        // This flag will get set when we do the drop.
        static Z : Lazy<std::sync::Mutex<bool>> = Lazy::new(|| std::sync::Mutex::new(false));

        // Dummy structure that will show if we drop correctly.
        #[derive(Debug)]
        struct X {
        }

        // Check that drop() is called after the owning object is dropped.
        impl Drop for X {
            fn drop(&mut self) {
                // Set the flag to show that we have dropped.
                let mut lck = Z.lock().unwrap();
                *lck = true;
            }
        }

        // Create an external pointer to test the drop.
        let extptr = ExternalPtr::new(X {});

        // The object should be protected here - drop not called yet.
        R!("gc()").unwrap();
        assert_eq!(*Z.lock().unwrap(), false);

        // Dropping the pointer should allow gc to drop the object.
        drop(extptr);
        R!("gc()").unwrap();
        assert_eq!(*Z.lock().unwrap(), true);
    }
}

#[test]
fn test_externalptr_deref() {
    test! {
        #[derive(Debug)]
        struct X {
            x: i32,
            y: i32,
        }

        let extptr = ExternalPtr::new(X { x: 1, y: 2});
        assert_eq!(extptr.x, 1);
        assert_eq!(extptr.y, 2);
    }
}
