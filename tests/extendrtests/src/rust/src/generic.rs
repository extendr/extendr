use extendr_api::prelude::*;

// Tests a 

struct GenericClass {
    a: i32,
}

#[extendr]
impl GenericClass {
    fn new (a : i32) -> Self {
        GenericClass{a: a}
    }
}

impl std::convert::TryFrom<Robj> for GenericClass {
    type Error = &'static str;

    fn try_from(robj: Robj) -> std::result::Result<Self, Self::Error> {
        if let Some(val) = robj.as_integer() {
            Ok(GenericClass{a: val})
        } else {
            Err("Input must be an integer.")
        }
    }
}

#[extendr]
fn vec_generic_class (v : Vec<GenericClass>) -> i32 {
    let mut result = 0;
    for value in v {
       result = result + value.a;
    } 
    result
}

// Macro to generate exports
extendr_module! {
    mod generics;
    fn vec_generic_class;
    impl GenericClass;
}