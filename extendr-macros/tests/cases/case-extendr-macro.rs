use extendr_macros::extendr;
use extendr_macros::{IntoRobj, TryFromRobj};

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

#[derive(TryFromRobj)]
enum Foo1 {
    A,
    B,
    C,
}

#[derive(IntoRobj)]
enum Foo2 {
    A,
    B,
    C,
}

fn main() {}
