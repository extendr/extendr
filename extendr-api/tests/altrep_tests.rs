use extendr_api::prelude::*;

#[test]
fn test_altinteger() {
    test! {
        #[derive(Debug, Clone)]
        struct MyCompactIntRange {
            start: i32,
            len: i32,
            step: i32,
        }

        impl AltrepImpl for MyCompactIntRange {
            fn length(&self) -> usize {
                self.len as usize
            }
        }

        impl AltIntegerImpl for MyCompactIntRange {
            fn elt(&self, index: usize) -> i32 {
                self.start + self.step * index as i32
            }
        }

        let mystate = MyCompactIntRange { start: 0, len: 10, step: 1 };

        let class = Altrep::make_altinteger_class::<MyCompactIntRange>("cir", "mypkg");
        let obj = Altrep::from_state_and_class(mystate, class, false);

        assert_eq!(obj.len(), 10);
        // assert_eq!(obj.sum(true), r!(45.0));
        assert_eq!(obj.as_integer_slice().unwrap(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}

#[test]

fn test_altreal() {
    test! {
        #[derive(Debug, Clone)]
        struct MyCompactRealRange {
            start: f64,
            len: usize,
            step: f64,
        }

        impl AltrepImpl for MyCompactRealRange {
            fn length(&self) -> usize {
                self.len as usize
            }
        }

        impl AltRealImpl for MyCompactRealRange {
            fn elt(&self, index: usize) -> f64 {
                self.start + self.step * index as f64
            }
        }

        let mystate = MyCompactRealRange { start: 0.0, len: 10, step: 1.0 };

        let class = Altrep::make_altreal_class::<MyCompactRealRange>("crr", "mypkg");
        let obj = Altrep::from_state_and_class(mystate, class, false);

        assert_eq!(obj.len(), 10);
        // assert_eq!(obj.sum(true), r!(45.0));
        assert_eq!(obj.as_real_slice().unwrap(), [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
    }
}

#[test]
fn test_altlogical() {
    test! {
        #[derive(Debug, Clone)]
        struct IsEven {
            len: usize,
        }

        impl AltrepImpl for IsEven {
            fn length(&self) -> usize {
                self.len as usize
            }
        }

        impl AltLogicalImpl for IsEven {
            fn elt(&self, index: usize) -> Bool {
                (index % 2 == 1).into()
            }
        }

        let mystate = IsEven { len: 10 };

        let class = Altrep::make_altlogical_class::<IsEven>("iseven", "mypkg");
        let obj = Altrep::from_state_and_class(mystate, class, false);

        assert_eq!(obj.len(), 10);
        // assert_eq!(obj.sum(true), r!(5.0));
        assert_eq!(obj.as_logical_slice().unwrap(), [FALSE, TRUE, FALSE, TRUE, FALSE, TRUE, FALSE, TRUE, FALSE, TRUE]);
    }
}

#[test]
fn test_altraw() {
    test! {
        #[derive(Debug, Clone)]
        struct MyCompactRawRange {
            start: i32,
            len: i32,
            step: i32,
        }

        impl AltrepImpl for MyCompactRawRange {
            fn length(&self) -> usize {
                self.len as usize
            }
        }

        impl AltRawImpl for MyCompactRawRange {
            fn elt(&self, index: usize) -> u8 {
                (self.start + self.step * index as i32) as u8
            }
        }

        let mystate = MyCompactRawRange { start: 0, len: 10, step: 1 };

        let class = Altrep::make_altraw_class::<MyCompactRawRange>("cir", "mypkg");
        let obj = Altrep::from_state_and_class(mystate, class, false);

        assert_eq!(obj.len(), 10);
        assert_eq!(obj.as_raw_slice().unwrap(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}

#[test]
fn test_altcomplex() {
    test! {
        #[derive(Debug, Clone)]
        struct MyCompactComplexRange {
            start: f64,
            len: usize,
            step: f64,
        }

        impl AltrepImpl for MyCompactComplexRange {
            fn length(&self) -> usize {
                self.len as usize
            }
        }

        impl AltComplexImpl for MyCompactComplexRange {
            fn elt(&self, index: usize) -> Cplx {
                Cplx(self.start + self.step * index as f64, self.start + self.step * index as f64)
            }
        }

        let mystate = MyCompactComplexRange { start: 0.0, len: 10, step: 1.0 };

        let class = Altrep::make_altcomplex_class::<MyCompactComplexRange>("ccr", "mypkg");
        let obj = Altrep::from_state_and_class(mystate, class, false);

        assert_eq!(obj.len(), 10);
        //assert_eq!(obj.as_complex_slice().unwrap(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}

#[test]
fn test_altstring() {
    test! {
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

        let mystate = StringInts { len: 10 };

        let class = Altrep::make_altstring_class::<StringInts>("si", "mypkg");
        let obj = Altrep::from_state_and_class(mystate, class, false);

        assert_eq!(obj.len(), 10);
        assert_eq!(Robj::from(obj), r!(["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]));
    }
}
