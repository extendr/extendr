//! Provide limited protection for multithreaded access to the R API.

use crate::*;
use std::sync::atomic::{AtomicU32, Ordering};

static OWNER_THREAD: AtomicU32 = AtomicU32::new(0);
static NEXT_THREAD_ID: AtomicU32 = AtomicU32::new(1);

thread_local! {
    static THREAD_ID: u32 = NEXT_THREAD_ID.fetch_add(1, Ordering::SeqCst);
}

// Get an integer 1.. for each thread that calls this.
pub fn this_thread_id() -> u32 {
    THREAD_ID.with(|&v| v)
}

/// Run a function single threaded.
/// Note: This will fail badly if the called function panics or calls RF_error.
///
/// ```
/// use extendr_api::prelude::*;
/// use std::sync::atomic::{AtomicU32, Ordering};
/// static GLOBAL_THREAD_COUNT: AtomicU32 = AtomicU32::new(0);
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

    if old_id != id {
        // wait for OWNER_THREAD to become 0 and put us as the owner.
        while OWNER_THREAD
            .compare_exchange(0, id, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }

    let res = f();

    if old_id != id {
        // release the lock and signal waiting threads.
        OWNER_THREAD.store(0, Ordering::Release);
    }

    res
}

/// This function is used by the wrapper logic to catch
/// panics on return.
///
#[doc(hidden)]
pub fn handle_panic<F, R>(err_str: &str, f: F) -> R
where
    F: FnOnce() -> R,
    F: std::panic::UnwindSafe,
{
    match std::panic::catch_unwind(f) {
        Ok(res) => res,
        Err(_) => {
            let err_str = CString::new(err_str).unwrap();
            unsafe { libR_sys::Rf_error(err_str.as_ptr()) }
        }
    }
}

static mut R_ERROR_BUF: Option<std::ffi::CString> = None;

pub fn throw_r_error<S: AsRef<str>>(s: S) -> ! {
    let s = s.as_ref();
    unsafe {
        R_ERROR_BUF = Some(std::ffi::CString::new(s).unwrap());
        libR_sys::Rf_error(R_ERROR_BUF.as_ref().unwrap().as_ptr());
    };
}

/// Wrap an R function such as Rf_findFunction and convert errors and panics into results.
/// ```ignore
/// use extendr_api::prelude::*;
/// test! {
///    let res = catch_r_error(|| unsafe {
///        throw_r_error("bad things!");
///        std::ptr::null_mut()
///    });
///    assert_eq!(res.is_ok(), false);
/// }
/// ```
pub fn catch_r_error<F>(f: F) -> Result<SEXP>
where
    F: FnOnce() -> SEXP + Copy,
    F: std::panic::UnwindSafe,
{
    use std::os::raw;

    unsafe extern "C" fn do_call<F>(data: *mut raw::c_void) -> SEXP
    where
        F: FnOnce() -> SEXP + Copy,
    {
        let data = data as *const ();
        let f: &F = &*(data as *const F);
        f()
    }

    unsafe extern "C" fn do_cleanup(_: *mut raw::c_void, jump: Rboolean) {
        if jump != 0 {
            panic!("R has thrown an error.");
        }
    }

    single_threaded(|| unsafe {
        let fun_ptr = do_call::<F> as *const ();
        let clean_ptr = do_cleanup as *const ();
        let x = false;
        let fun = std::mem::transmute(fun_ptr);
        let cleanfun = std::mem::transmute(clean_ptr);
        let data = &f as *const _ as _;
        let cleandata = &x as *const _ as _;
        let cont = R_MakeUnwindCont();
        Rf_protect(cont);

        // Note that catch_unwind does not work for 32 bit windows targets.
        let res = match std::panic::catch_unwind(|| {
            R_UnwindProtect(fun, data, cleanfun, cleandata, cont)
        }) {
            Ok(res) => Ok(res),
            Err(_) => Err("Error in protected R code".into()),
        };
        Rf_unprotect(1);
        res
    })
}
