use extendr_api::prelude::*;

#[extendr]
fn cnd_warn(msg: &str) {
    warn!(msg);
}

#[extendr]
fn cnd_warn_with_body(msg: &str, body: Vec<String>) {
    let body: Vec<&str> = body.iter().map(|s| s.as_str()).collect();
    warn!(msg, body.as_slice());
}

#[extendr]
fn cnd_abort(msg: &str) {
    abort!(msg);
}

#[extendr]
fn cnd_abort_with_body(msg: &str, body: Vec<String>) {
    let body = body.iter().map(|s| s.as_str()).collect::<Vec<_>>();
    abort!(msg, body.as_slice());
}

extendr_module! {
    mod conditions;
    fn cnd_warn;
    fn cnd_warn_with_body;
    fn cnd_abort;
    fn cnd_abort_with_body;
}
