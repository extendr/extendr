use extendr_api::prelude::*;

#[derive(Debug, Clone)]
struct StringInts {
    len: usize,
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
    missing_index: usize, // For testing NA
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
    let mystate = MyCompactIntRange {
        start: 0,
        len: 10,
        step: 1,
        missing_index: usize::MAX,
    };
    let class = Altrep::make_altinteger_class::<MyCompactIntRange>("cir", "mypkg");
    Altrep::from_state_and_class(mystate, class, false)
}

extendr_module! {
    mod altrep;
    fn tst_altstring;
    fn tst_altinteger;
}
