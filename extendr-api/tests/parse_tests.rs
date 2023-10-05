use extendr_engine::with_r;
use libR_sys::*;

macro_rules! cstr {
    ($s: expr) => {
        std::ffi::CString::new(concat!($s)).unwrap()
    };
}

#[test]
fn test_parse_eval_string() {
    with_r(|| unsafe {
        // let bad_code = cstr!("c(10,42,20)");
        let bad_code = cstr!("c(10,,42,20)");
        // let bad = Rf_protect(R_ParseEvalString(bad_code.as_ptr(), R_GlobalEnv));
        let bad = Rf_protect(R_ParseEvalString(bad_code.as_ptr(), R_NilValue));
        Rf_PrintValue(bad);
        println!("Did we get here?");
        Rf_unprotect(1);
    });
}
