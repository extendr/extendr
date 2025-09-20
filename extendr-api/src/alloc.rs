//!
//!
//!
use std::alloc;
use std::sync::OnceLock;

use extendr_ffi::{
    CAR, CDR, R_NilValue, R_PreserveObject, R_xlen_t, RAW, Rf_allocVector, Rf_cons, Rf_protect, Rf_unprotect, Rf_xlength, SET_TAG, SETCAR, SETCDR, SEXP, SEXPTYPE
};

thread_local! {
    static PRESERVE_LIST: OnceLock<SEXP> = OnceLock::new();
}
#[inline]
unsafe fn init() -> SEXP {
    let out = Rf_cons(R_NilValue, Rf_cons(R_NilValue, R_NilValue));
    R_PreserveObject(out);
    out
}

#[inline]
unsafe fn get() -> SEXP {
    // One global preserve list across all crates/units.
    PRESERVE_LIST.with(|x| *x.get_or_init(|| unsafe { init() }))
}

#[allow(dead_code)]
#[inline]
unsafe fn count() -> R_xlen_t {
    let head: R_xlen_t = 1;
    let tail: R_xlen_t = 1;
    let list = get();
    Rf_xlength(list) - head - tail
}

#[inline]
unsafe fn insert(x: SEXP) -> SEXP {
    if x == R_NilValue {
        return R_NilValue;
    }

    Rf_protect(x);

    let list = get();

    // head is the list itself; next is the node after head
    let head = list;
    let next = CDR(list);

    // New cell points to current head and next
    let cell = Rf_protect(Rf_cons(head, next));
    SET_TAG(cell, x);

    // Splice cell between head and next
    SETCDR(head, cell);
    SETCAR(next, cell);

    Rf_unprotect(2);

    cell
}

#[inline]
unsafe fn release(cell: SEXP) {
    if cell == R_NilValue {
        return;
    }

    // Neighbors around the cell
    let lhs = CAR(cell);
    let rhs = CDR(cell);

    // Bypass cell
    SETCDR(lhs, rhs);
    SETCAR(rhs, lhs);

    // optional hygiene, although unnecessary
    // SET_TAG(cell, R_NilValue);
    // SETCAR(cell, R_NilValue);
    // SETCDR(cell, R_NilValue);
}

// #[inline]
// unsafe fn print() {
//     let list = get();
//     let fmt = b"%p CAR: %p CDR: %p TAG: %p\n\0".as_ptr() as *const i8;
//     let sep = b"---\n\0".as_ptr() as *const i8;

//     let mut cell = list;
//     while cell != R_NilValue {
//         REprintf(
//             fmt,
//             cell as *mut c_void,
//             CAR(cell) as *mut c_void,
//             CDR(cell) as *mut c_void,
//             TAG(cell) as *mut c_void,
//         );
//         cell = CDR(cell);
//     }
//     REprintf(sep);
// }

// FIXME: `Allocator` does not work if the returned objects are Rust types!
// This means that `#[extendr]`-impl blocks will not work with this allocator!

#[repr(C)]
struct Header {
    cell: SEXP,       // token from insert(sexp)
    offset: u16,      // aligned_ptr - base_data (bytes)
    _pad: u16,        // keep Header size a multiple of 4
}

const HEADER_SIZE: usize = std::mem::size_of::<Header>();

#[derive(Debug)]
pub struct RAllocator;

unsafe impl alloc::GlobalAlloc for RAllocator {
    unsafe fn alloc(&self, layout: alloc::Layout) -> *mut u8 {
        let a = layout.align();
        let n = layout.size();
        if n == 0 { return a as *mut u8; }

        // ensure space for header + worst-case alignment padding
        let total = match n.checked_add(a - 1).and_then(|t| t.checked_add(HEADER_SIZE)) {
            Some(t) => t,
            None => return std::ptr::null_mut(),
        };

        let sexp = Rf_allocVector(SEXPTYPE::RAWSXP, total as isize);
        if sexp.is_null() { return std::ptr::null_mut(); }

        // keep the RAWSXP alive
        let cell = insert(sexp);

        // base of RAWSXP payload, not the SEXP header
        let base = RAW(sexp) as usize;

        // place header immediately before the aligned user pointer
        let start = base + HEADER_SIZE;
        let aligned = (start + (a - 1)) & !(a - 1);

        // write header at aligned - HEADER_SIZE
        let hdr_ptr = (aligned - HEADER_SIZE) as *mut Header;
        std::ptr::write(
            hdr_ptr,
            Header {
                cell,
                offset: (aligned - base) as u16,
                _pad: 0,
            },
        );

        aligned as *mut u8
    }

    unsafe fn dealloc(&self, ptr_user: *mut u8, _layout: alloc::Layout) {
        if ptr_user.is_null() { return; }

        // recover header
        let hdr_ptr = (ptr_user as usize - HEADER_SIZE) as *mut Header;
        let hdr = &*hdr_ptr;

        // drop preserve token
        release(hdr.cell);

        // optional poisoning for debug
        // ptr::write_bytes(hdr_ptr as *mut u8, 0xDD, HEADER_SIZE);
    }
}

