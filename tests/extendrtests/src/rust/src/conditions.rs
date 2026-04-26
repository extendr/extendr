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

#[extendr]
fn cnd_abort_with_call(msg: &str, #[extendr(default = "NULL")] call: Option<Environment>) {
    let call = call.unwrap_or_else(Environment::caller);
    abort!(msg, call = call);
}

#[extendr]
fn throw_error_with_percent(msg: &str) {
    extendr_api::throw_r_error(msg);
}

extendr_module! {
    mod conditions;
    fn cnd_warn;
    fn cnd_warn_with_body;
    fn cnd_abort;
    fn cnd_abort_with_body;
    fn cnd_abort_with_call;
    fn throw_error_with_percent;
}
