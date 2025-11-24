//! Provide limited protection for multithreaded access to the R API.
use crate::*;
use extendr_ffi::{
    R_MakeUnwindCont, R_UnwindProtect, Rboolean, Rf_error, Rf_protect, Rf_unprotect,
};
use std::cell::Cell;
use std::sync::Mutex;

/// A global lock, that should represent the global lock on the R-API.
/// It is not tied to an actual instance of R.
static R_API_LOCK: Mutex<()> = Mutex::new(());

thread_local! {
    static THREAD_HAS_LOCK: Cell<bool> = const { Cell::new(false) };
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

static mut R_ERROR_BUF: Option<std::ffi::CString> = None;

pub fn throw_r_error<S: AsRef<str>>(s: S) -> ! {
    let s = s.as_ref();
    unsafe {
        R_ERROR_BUF = Some(std::ffi::CString::new(s).unwrap());
        let ptr = std::ptr::addr_of!(R_ERROR_BUF);
        Rf_error((*ptr).as_ref().unwrap().as_ptr());
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
        if jump != Rboolean::FALSE {
            panic!("R has thrown an error.");
        }
    }

    single_threaded(|| unsafe {
        let fun_ptr = do_call::<F> as *const ();
        let clean_ptr = do_cleanup as *const ();
        let x = false;
        let fun = std::mem::transmute::<
            *const (),
            Option<unsafe extern "C" fn(*mut std::ffi::c_void) -> *mut extendr_ffi::SEXPREC>,
        >(fun_ptr);
        let cleanfun = std::mem::transmute::<
            *const (),
            std::option::Option<unsafe extern "C" fn(*mut std::ffi::c_void, extendr_ffi::Rboolean)>,
        >(clean_ptr);
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

/// This function registers a configurable print panic hook, for use in extendr-based R-packages.
/// If the environment variable `EXTENDR_BACKTRACE` is set to either `true` or `1`,
/// then it displays the entire Rust panic traceback (default hook), otherwise it omits the panic backtrace.
#[no_mangle]
pub extern "C" fn register_extendr_panic_hook() {
    static RUN_ONCE: std::sync::Once = std::sync::Once::new();
    RUN_ONCE.call_once_force(|x| {
        // just ignore repeated calls to this function
        if x.is_poisoned() {
            println!("warning: extendr panic hook info registration was done more than once");
            return;
        }
        let default_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |x| {
            let show_traceback = std::env::var("EXTENDR_BACKTRACE")
                .map(|v| v.eq_ignore_ascii_case("true") || v == "1")
                .unwrap_or(false);
            if show_traceback {
                default_hook(x)
            } else {
                return;
            }
        }));
    });
}
