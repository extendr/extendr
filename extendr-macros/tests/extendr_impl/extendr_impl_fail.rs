#![allow(unused_imports)]

#[allow(unused_imports)]
use extendr_api::prelude::*;

struct Foo;

#[extendr]
impl Foo {
    fn take_ownership(self) {}
}

struct Foo2;

#[extendr]
impl Foo2 {
    fn take_ownership(self: Self) {}
}

struct Foo3;

#[extendr]
impl Foo3 {
    fn take_ownership(self: Foo3) {}
}
struct Foo4;

#[extendr]
impl Foo4 {
    fn return_ownership(self) -> Self {}
}

struct Foo5;

#[extendr]
impl Foo5 {
    fn take_ownership(self: Self) -> Foo5 {}
}

fn main() {}
