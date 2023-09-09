use extendr_api::prelude::*;

// struct contains an inner vector of Option<usize>
#[derive(Debug, Clone)]
pub struct VecUsize(pub Vec<Option<usize>>);

impl AltrepImpl for VecUsize {
    fn length(&self) -> usize {
        self.0.len()
    }
}

// we need to be able to return an Robj of this type so
// we add an empty extendr macro above the impl
#[extendr]
impl VecUsize {}

#[cfg(use_r_altlist)]
impl AltListImpl for VecUsize {
    fn elt(&self, index: usize) -> Robj {
        self.into_robj()
    }
}

#[cfg(use_r_altlist)]
#[extendr]
/// Create an ALTLIST usize vector
///
/// @param robj an integer vector
///
/// The object is `Vec<Option<usize>>` represented as an ALTLIST
fn new_usize(robj: Integers) -> Altrep {
    let x = robj
        .iter()
        .map(|x| match &x {
            _ if x.is_na() => None,
            _ if x.inner() < 0 => None,
            _ => Some(x.inner() as usize),
        })
        .collect();

    // we can't return the object as is, it needs to
    // be converted to an altrep object
    let obj = VecUsize(x);
    // this provides a hidden class to the altrep object for the package extendrtests
    let class = Altrep::make_altlist_class::<VecUsize>("li", "mypkg");

    // create an altrep object from the class
    Altrep::from_state_and_class(obj, class, false)
}

#[cfg(not(use_r_altlist))]
#[extendr]
fn new_usize(robj: Integers) -> Robj {
    extendr_api::nil_value()
}

extendr_module! {
    mod altlist;
    fn new_usize;
}
