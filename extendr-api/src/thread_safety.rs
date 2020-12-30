//! Provide limited protection for multithreaded access to the R API.

use std::cell::RefCell;
use std::sync::atomic::{AtomicUsize, Ordering};

static OWNER_THREAD: AtomicUsize = AtomicUsize::new(0);
static NEXT_THREAD_ID: AtomicUsize = AtomicUsize::new(1);

thread_local! {
    static THREAD_ID: RefCell<usize> = RefCell::new(0);
}

// Get an integer 1.. for each thread that calls this.
fn this_thread_id() -> usize {
    THREAD_ID.with(|f| {
        if *f.borrow() == 0 {
            // Initialise with next value.
            *f.borrow_mut() = NEXT_THREAD_ID.fetch_add(1, Ordering::SeqCst);
        }
        *f.borrow()
    })
}

/// Run a function single threaded.
/// Note: This will fail badly if the called function panics or calls RF_error.
///
/// ```
/// use extendr_api::*;
/// use std::sync::atomic::{AtomicUsize, Ordering};
/// static GLOBAL_THREAD_COUNT: AtomicUsize = AtomicUsize::new(0);
///
/// let threads : Vec<_> = (0..100).map(|_| {
///    std::thread::spawn(move|| {
///       single_threaded(|| {
///         // check that we are single threaded.
///         let old_thread_count = GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::AcqRel);
///         assert_eq!(old_thread_count, 0);
///         std::thread::sleep(std::time::Duration::from_millis(1));
///         GLOBAL_THREAD_COUNT.fetch_sub(1, Ordering::AcqRel);
///         // recursive calls are ok.
///         assert_eq!(single_threaded(|| {
///           1
///         }), 1);    
///       })
///    })
/// }).collect();
/// ```
pub fn single_threaded<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let id = this_thread_id();
    let old_id = OWNER_THREAD.load(Ordering::Acquire);

    if old_id == id {
        // println!("{:?} id={} RECURSIVE", std::thread::current().name(), id);
        // recursive call, don't re-lock.
        f()
    } else {
        // println!("{:?} id={}", std::thread::current().name(), id);

        // wait for OWNER_THREAD to become 0 and put us as the owner.
        while OWNER_THREAD
            .compare_exchange(0, id, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }

        // println!("{:?} id={} lock", std::thread::current().name(), id);

        let res = f();

        // println!("{:?} id={} unlock", std::thread::current().name(), id);

        // release the lock.
        OWNER_THREAD.store(0, Ordering::Release);
        res
    }
}

/// This function is used by the wrapper logic to catch
/// panics on return.
pub fn handle_panic<F, R>(err_str: &str, f: F) -> R
where
    F: FnOnce() -> R,
    F: std::panic::UnwindSafe,
{
    match std::panic::catch_unwind(f) {
        Ok(res) => res,
        Err(_) => {
            unsafe {
                libR_sys::Rf_error(err_str.as_ptr() as *const std::os::raw::c_char);
            }
            unreachable!("handle_panic unreachable")
        }
    }
}
