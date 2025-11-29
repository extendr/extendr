//! Provide limited protection for multithreaded access to the R API.
use crate::*;
use extendr_ffi::{
    R_MakeUnwindCont, R_UnwindProtect, Rboolean, Rf_error, Rf_protect, Rf_unprotect,
};
use std::cell::RefCell;
use std::sync::{Mutex, MutexGuard};

static R_API_LOCK: Mutex<()> = Mutex::new(());

struct State {
    depth: u32,
    guard: Option<MutexGuard<'static, ()>>,
}

thread_local! {
    static R_API_STATE: RefCell<State> = const {
        RefCell::new(State { depth: 0, guard: None })
    };
}

struct RApiGuard;

impl Drop for RApiGuard {
    fn drop(&mut self) {
        // RAII exit path (normal return or Rust panic)
        R_API_STATE.with(|cell| {
            let mut st = cell.borrow_mut();
            if st.depth == 0 {
                return;
            }

            st.depth -= 1;
            if st.depth == 0 {
                // dropping the guard here unlocks the mutex
                st.guard = None;
            }
        });
    }
}

/// Run `f` while ensuring single-threaded access to the R C API.
pub fn single_threaded<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    // Enter: take lock on first entry in this thread
    R_API_STATE.with(|cell| {
        let mut st = cell.borrow_mut();
        if st.depth == 0 {
            let g = R_API_LOCK.lock().unwrap_or_else(|p| p.into_inner());
            st.guard = Some(g);
        }
        st.depth += 1;
    });

    let _guard = RApiGuard; // RAII for normal/panic paths

    f()
}

/// Best-effort reset for this thread after a longjmp.
pub fn reset_r_api_for_thread() {
    R_API_STATE.with(|cell| {
        let mut st = cell.borrow_mut();
        st.depth = 0;
        // Setting to None drops the MutexGuard, unlocking the mutex if needed.
        st.guard = None;
    });
}

// Per-thread storage for the last error message
thread_local! {
    // Mutable TLS buffer to keep the error message alive across the R longjmp.
    // Using UnsafeCell avoids RefCell borrow bookkeeping, which would otherwise
    // remain "borrowed" if Rf_error longjmps past the destructor.
    static R_ERROR_BUF: std::cell::UnsafeCell<Option<CString>> = const { std::cell::UnsafeCell::new(None) };
}

static RF_ERROR_FORMAT: &std::ffi::CStr =
    unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"%s\0") };

pub fn throw_r_error<S: AsRef<str>>(s: S) -> ! {
    let msg = s.as_ref();

    let mut cstr_ptr: *const std::os::raw::c_char = std::ptr::null();
    R_ERROR_BUF.with(|slot| unsafe {
        let buf = &mut *slot.get();
        *buf = Some(CString::new(msg).unwrap());
        cstr_ptr = buf.as_ref().unwrap().as_ptr();
    });

    unsafe {
        // Rf_error never returns: it longjmps out through Rust.
        // The CString is never dropped, so the pointer stays valid.
        Rf_error(RF_ERROR_FORMAT.as_ptr(), cstr_ptr)
    }
}

/// Wrap an R function such as `Rf_findFunction` and convert errors and panics into results.
/// ```rust,ignore
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
            }
        }));
    });
}
