use extendr_api::SEXP;
use extendr_api::{prelude::*, UnsafeExternalPtr};

#[extendr]
fn unsafe_externalptr_to_strings(value: UnsafeExternalPtr) -> Strings {
    let sexp = value.addr();
    assert!(!sexp.is_null());
    let robj = unsafe { std::mem::transmute::<_, SEXP>(sexp) };
    let robj = Robj::from_sexp(robj);
    // rprintln!("rtype: {:?}", robj.rtype());
    let raw_robj = robj.as_raw().unwrap();
    Strings::from_values([Rstr::from_string(
        &String::from_utf8_lossy(raw_robj.as_slice()).to_string(),
    )])
}

#[extendr]
fn externalptr_as_raw(value: ExternalPtr<Raw>) -> Strings {
    let raw_robj = value.as_raw().unwrap();
    Strings::from_values([Rstr::from_string(
        &String::from_utf8_lossy(raw_robj.as_slice()).to_string(),
    )])
}

extendr_module! {
    mod unsafe_externalptr;
    fn unsafe_externalptr_to_strings;
    fn externalptr_as_raw;
}
