//! Provide limited protection for multithreaded access to the R API.

use crate::*;
use std::cell::Cell;
use std::sync::Mutex;

/// A global lock, that should represent the global lock on the R-API.
/// It is not tied to an actual instance of R.
static R_API_LOCK: Mutex<()> = Mutex::new(());

thread_local! {
    static THREAD_HAS_LOCK: Cell<bool> = Cell::new(false);
}

/// Run `f` while ensuring that `f` runs in a single-threaded manner.
///
/// This is intended for single-threaded access of the R's C-API.
/// It is possible to have nested calls of `single_threaded` without deadlocking.
///
/// Note: This will fail badly if the called function `f` panics or calls `Rf_error`.
pub fn single_threaded<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let has_lock = THREAD_HAS_LOCK.with(|x| x.get());

    // acquire R-API lock
    let _guard = if !has_lock {
        Some(R_API_LOCK.lock().unwrap())
    } else {
        None
    };

    // this thread now has the lock
    THREAD_HAS_LOCK.with(|x| x.set(true));

    let result = f();

    // release the R-API lock
    if _guard.is_some() {
        THREAD_HAS_LOCK.with(|x| x.set(false));
    }

    result
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

/// Wrap an R function such as `Rf_findFunction` and convert errors and panics into results.
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
