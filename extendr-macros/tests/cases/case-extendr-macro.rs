use extendr_macros::extendr;
use extendr_macros::{IntoRobj, TryFromRobj};

#[extendr(foo = true)]
fn foo() {}

#[extendr(r_name = 1)]
fn foo() {}

#[extendr(mod_name = 1)]
fn foo() {}

#[extendr(use_rng = 1)]
fn foo() {}

// impl -----------------------------------------

struct FooStruct {}

#[extendr]
impl FooStruct {
    fn nonref_self(self) {}
}

impl FooStruct {
    #[extendr]
    fn misplaced_macro(&self) {}
}

#[extendr]
default impl FooStruct {}

#[extendr]
unsafe impl FooStruct {}

#[extendr]
impl<const N: usize> FooStruct {}

struct FooStructWithParam<A> {
    a: A,
}

#[extendr]
impl<A> FooStructWithParam<A> {}

#[extendr]
impl FooStructWithParam<A> where A: usize {}

// derive ---------------------------------------

#[derive(TryFromRobj)]
enum FooEnum1 {
    A,
    B,
    C,
}

#[derive(IntoRobj)]
enum FooEnum2 {
    A,
    B,
    C,
}

fn main() {}
