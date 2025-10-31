/// Checks for a user interrupt in the stack.
///
/// Calls `R_CheckUserInterrupt()`
///
/// If an interrupt is found it will trigger a longjmp.
pub fn check_user_interrupt() {
    unsafe {
        extendr_ffi::R_CheckUserInterrupt();
    }
}

/// Signals an interrupt
///
/// Calls `Rf_onintr()`
pub fn signal_interrupt() {
    unsafe {
        extendr_ffi::Rf_onintr();
    }
}
