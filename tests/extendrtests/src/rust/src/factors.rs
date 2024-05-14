use super::*;

// Source: https://github.com/extendr/extendr/issues/740
#[extendr]
fn from_factor_as_integers(x: Robj) {
    rprintln!("x has an Rtype of {:?}", x.rtype());
    rprintln!("{:?}", Integers::try_from(&x));
}

// Source: https://github.com/extendr/extendr/issues/740
#[extendr]
fn from_factor(x: Robj) {
    rprintln!("x has an Rtype of {:?}", x.rtype());
    rprintln!("{:?}", Factors::try_from(&x));
}

extendr_module! {
    mod factors;
    fn from_factor_as_integers;
    fn from_factor;
}
