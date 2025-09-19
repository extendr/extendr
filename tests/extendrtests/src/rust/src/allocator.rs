use extendr_api::prelude::*;

use extendr_api::ralloc::RAllocator;

// This code enables the `RAllocator` in the R-package.
#[global_allocator]
static GLOBAL: RAllocator = RAllocator;

#[extendr]
fn allocate_rust(capacity: usize) {
    let mut vec = Vec::with_capacity(capacity);
    vec.resize(capacity, 0_f64);
}

#[extendr]
fn allocate_r(capacity: usize) {
    let r_vec = Doubles::new(capacity);
}

extendr_module! {
    mod allocator;
    fn allocate_r;
    fn allocate_rust;
    // fn allocate_r_and_rust;
}