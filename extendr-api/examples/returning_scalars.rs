use extendr_api::scalar::{Rfloat, Rint};
use extendr_api::CanBeNA;

fn get_int() -> Rint {
    Rint::from(1)
}

fn get_na_int() -> Rint {
    Rint::na()
}

fn get_float() -> Rfloat {
    Rfloat::from(1.0)
}

fn get_na_float() -> Rfloat {
    Rfloat::na()
}

fn main() {
    use extendr_api::{test, Result};

    test! {
        let i1 = get_int();
        println!("{:?} {}", i1, i1.is_na());

        let i2 = get_na_int();
        println!("{:?} {}", i2, i2.is_na());

        let f1 = get_float();
        println!("{:?} {}", f1, f1.is_na());

        let f2 = get_na_float();
        println!("{:?} {}", f2, f2.is_na());
    }
}
