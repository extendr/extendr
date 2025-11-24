use extendr_api::prelude::*;

struct MsgOnDrop;

impl Drop for MsgOnDrop {
    fn drop(&mut self) {
        rprintln!("Dropped `MsgOnDrop`!");
    }
}

#[extendr]
fn must_see_drop_msg_r_error() {
    let _a = MsgOnDrop;

    unsafe { Rf_error(c"%s".as_ptr(), c"threw an r error!".as_ptr()) }
}

#[extendr]
fn must_see_drop_msg_panic() {
    let _a = MsgOnDrop;

    panic!()
}

#[extendr]
fn must_see_drop_msg_r_error_heap() {
    let _a = Box::new(MsgOnDrop);

    unsafe { Rf_error(c"%s".as_ptr(), c"threw an r error!".as_ptr()) }
}

#[extendr]
fn must_see_drop_msg_panic_heap() {
    let _a = Box::new(MsgOnDrop);

    panic!()
}

extern "C" {
    pub fn Rf_error(arg1: *const ::std::os::raw::c_char, ...) -> !;

}

extendr_module! {
    mod leak;

    fn must_see_drop_msg_r_error;
    fn must_see_drop_msg_panic;

    fn must_see_drop_msg_r_error_heap;
    fn must_see_drop_msg_panic_heap;
}
