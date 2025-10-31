use extendr_api::{
    prelude::*,
    signal::{check_user_interrupt, signal_interrupt},
};

#[extendr]
fn test_signal() {
    for i in 0..10 {
        // print progress
        rprintln!("iteration {}", i);
        // check for pending interrupts (may longjmp)
        check_user_interrupt();
        // trigger interrupt at iteration 3
        if i == 3 {
            rprintln!("signaling interrupt");
            signal_interrupt();
        }
    }
}

extendr_module! {
    mod signal;
    fn test_signal;
}
