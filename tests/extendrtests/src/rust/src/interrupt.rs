use extendr_api::prelude::*;
use std::sync::atomic::{AtomicI32, Ordering};

/// Counts how many `DropGuard`s have been dropped, so R can verify that Rust destructors
/// ran when an interrupt stopped the computation.
static DROPPED: AtomicI32 = AtomicI32::new(0);

struct DropGuard {
    // owns an R object: if this is skipped by a longjmp, the preservation table leaks it
    _robj: Robj,
}

impl Drop for DropGuard {
    fn drop(&mut self) {
        DROPPED.fetch_add(1, Ordering::SeqCst);
    }
}

extern "C" {
    fn raise(sig: std::os::raw::c_int) -> std::os::raw::c_int;
}
const SIGINT: std::os::raw::c_int = 2;

/// Pretend the user pressed Ctrl-C: R's signal handler marks an interrupt as pending.
fn simulate_ctrl_c() {
    unsafe {
        raise(SIGINT);
    }
}

/// Loops `n` times, checking for a user interrupt at every iteration.
/// If `interrupt_at >= 0`, a Ctrl-C is simulated in that iteration.
#[extendr]
fn interrupt_loop(n: i32, interrupt_at: i32) -> i32 {
    let _guard = DropGuard {
        _robj: r!([1.0, 2.0, 3.0]),
    };
    let mut done = 0;
    for i in 0..n {
        if i == interrupt_at {
            simulate_ctrl_c();
        }
        check_user_interrupt();
        done += 1;
    }
    done
}

/// Like `interrupt_loop`, but stops early and returns the partial count instead of
/// signalling the interrupt to R.
#[extendr]
fn interrupt_loop_partial(n: i32, interrupt_at: i32) -> i32 {
    let _guard = DropGuard {
        _robj: r!([1.0, 2.0, 3.0]),
    };
    let mut done = 0;
    for i in 0..n {
        if i == interrupt_at {
            simulate_ctrl_c();
        }
        if interrupt_requested() {
            break;
        }
        done += 1;
    }
    done
}

#[extendr]
fn interrupt_drop_count() -> i32 {
    DROPPED.load(Ordering::SeqCst)
}

extendr_module! {
    mod interrupt;
    fn interrupt_loop;
    fn interrupt_loop_partial;
    fn interrupt_drop_count;
}
