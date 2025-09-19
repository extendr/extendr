//!
//!
//!
use std::alloc;

// TODO: add this to extendr-ffi, or even feature gate this.
unsafe extern "C" {
    // pub fn vmaxget() -> *mut ::std::os::raw::c_void;
    // pub fn vmaxset(arg1: *const ::std::os::raw::c_void);
    // pub fn R_gc();
    // pub fn R_gc_running() -> ::std::os::raw::c_int;

    /// This function is not thread-safe, see [R-exts: Transient storage allocation](https://cran.r-project.org/doc/manuals/R-exts.html#Transient-storage-allocation-1).
    /// 
    fn R_alloc(nelem: usize, eltsize: usize) -> *mut u8;
    // TODO: use this for 128-bit layouts..
    fn R_allocLD(nelem: usize) -> *mut u128;
}


// FIXME: `Allocator` does not work if the returned objects are Rust types!
// This means that `#[extendr]`-impl blocks will not work with this allocator!


#[derive(Debug)]
pub struct RAllocator;

unsafe impl alloc::GlobalAlloc for RAllocator {
    unsafe fn alloc(&self, layout: alloc::Layout) -> *mut u8 {
        let a = layout.align();
        let n = layout.size();

        if n == 0 {
            return a as *mut u8;
        }

        // Over-allocate so some address in the block satisfies `a`.
        // `a` is guaranteed to be non-zero
        let total = match n.checked_add(a - 1) {
            Some(total) => total,
            None => return std::ptr::null_mut(),
        };

        let base = R_alloc(1, total);
        // FIXME: respond to failure in allocation!
        if base.is_null() {
            return std::ptr::null_mut();
        }

        let addr = base as usize;
        let aligned = (addr + (a - 1)) & !(a - 1);
        aligned as *mut u8
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: alloc::Layout) {
        // no-op: R frees after returning to R
    }
}
