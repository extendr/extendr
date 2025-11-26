use extendr_api::prelude::*;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Barrier,
};
use std::thread;
use std::time::Duration;

// A thin wrapper to move an `Robj` across threads for this test only.
// This is intentionally unsafe and exists to demonstrate that `Drop`
// is currently allowed to call into R without respecting the global lock.
#[allow(dead_code)]
struct SendRobj(Robj);
unsafe impl Send for SendRobj {}

/// Ensure R API calls are serialized across threads.
///
/// Dropping an `Robj` triggers `unprotect()` which calls into the R API.
/// This test holds the `single_threaded` lock on one thread while another
/// thread drops an `Robj`. Today the drop proceeds even though the lock
/// is held, meaning the R API is executed from two threads at once.
/// If the lock actually guarded all R calls, `dropped` would stay `false`
/// until the guard is released.
#[test]
fn robj_drop_ignores_single_threaded_lock() {
    extendr_engine::with_r(|| {
        // Build a small R object we can drop on another thread.
        let robj = SendRobj(r!([1, 2, 3]));

        let barrier = Arc::new(Barrier::new(2));
        let dropped = Arc::new(AtomicBool::new(false));

        let handle = {
            let barrier = barrier.clone();
            let dropped = dropped.clone();
            thread::spawn(move || {
                barrier.wait();
                drop(robj);
                dropped.store(true, Ordering::SeqCst);
            })
        };

        single_threaded(|| {
            // Hold the lock while the drop happens on another thread.
            barrier.wait();
            thread::sleep(Duration::from_millis(50));
            assert!(
                !dropped.load(Ordering::SeqCst),
                "Robj drop ran R API code while the global single_threaded lock was held"
            );
        });

        handle.join().unwrap();
    });
}
