use extendr_api::prelude::*;
use extendr_api::AltListImpl;

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

impl AltListImpl for VecUsize {
    fn elt(&self, index: usize) -> Robj {
        self.into_robj()
    }
}


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




#[derive(Debug, Clone)]
struct StringInts {
    len: usize
}

impl AltrepImpl for StringInts {
    fn length(&self) -> usize {
        self.len as usize
    }
}

impl AltStringImpl for StringInts {
    fn elt(&self, index: usize) -> Rstr {
        format!("{}", index).into()
    }
}


#[extendr]
/// Test ALTSTRING representation
fn tst_altstring() -> Altrep {
    let mystate = StringInts { len: 10 };
    let class = Altrep::make_altstring_class::<StringInts>("si", "mypkg");
    Altrep::from_state_and_class(mystate, class, false)
}


#[derive(Debug, Clone)]
struct MyCompactIntRange {
    start: i32,
    len: i32,
    step: i32,
    missing_index: usize,  // For testing NA
}

impl AltrepImpl for MyCompactIntRange {
    fn length(&self) -> usize {
        self.len as usize
    }
}

impl AltIntegerImpl for MyCompactIntRange {
    fn elt(&self, index: usize) -> Rint {
        if index == self.missing_index {
            Rint::na()
        } else {
            Rint::new(self.start + self.step * index as i32)
        }
    }
}


/// Test ALTINTEGER support
#[extendr]
fn tst_altinteger() -> Altrep {
    let mystate = MyCompactIntRange { start: 0, len: 10, step: 1, missing_index: usize::MAX };
    let class = Altrep::make_altinteger_class::<MyCompactIntRange>("cir", "mypkg");
    Altrep::from_state_and_class(mystate, class, false)
}

extendr_module! {
    mod altrep;
    fn new_usize;
    fn tst_altstring;
    fn tst_altinteger;
}
