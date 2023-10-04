use extendr_macros::extendr;

#[extendr(foo = true)]
fn foo() {}

#[extendr(use_try_from = 1)]
fn foo() {}

#[extendr(r_name = 1)]
fn foo() {}

#[extendr(mod_name = 1)]
fn foo() {}

#[extendr(use_rng = 1)]
fn foo() {}

fn main() {}
