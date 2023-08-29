use extendr_api::prelude::*;
use extendr_engine::with_r;

#[test]
fn test_altinteger() {
    test! {
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

        // no missing values in the range
        let mystate = MyCompactIntRange { start: 0, len: 10, step: 1, missing_index: usize::MAX };

        let class = Altrep::make_altinteger_class::<MyCompactIntRange>("cir", "mypkg");
        let obj = Altrep::from_state_and_class(mystate, class.clone(), false);

        assert_eq!(obj.len(), 10);
        // assert_eq!(obj.sum(true), r!(45.0));
        assert_eq!(obj.as_robj().as_integer_slice().unwrap(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

        // index 5 is missing
        let mystate_w_missing = MyCompactIntRange { start: 0, len: 10, step: 1, missing_index: 5 };

        let obj_w_missing = Altrep::from_state_and_class(mystate_w_missing, class, false);
        let robj_w_missing = Robj::from(obj_w_missing);
        let integers_w_missing: Integers = robj_w_missing.try_into()?;
        assert_eq!(integers_w_missing.elt(9), Rint::from(9));
        assert!(integers_w_missing.elt(5).is_na());

        // tests for get_region()
        let mut dest = [0.into(); 2];
        integers_w_missing.get_region(2, &mut dest);
        assert_eq!(dest, [2, 3]);
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
            missing_index: usize,  // For testing NA
        }

        impl AltrepImpl for MyCompactRealRange {
            fn length(&self) -> usize {
                self.len
            }
        }

        impl AltRealImpl for MyCompactRealRange {
            fn elt(&self, index: usize) -> Rfloat {
                if index == self.missing_index {
                    Rfloat::na()
                } else {
                    Rfloat::new(self.start + self.step * index as f64)
                }
            }
        }

        // no missing values in the range
        let mystate = MyCompactRealRange { start: 0.0, len: 10, step: 1.0, missing_index: usize::MAX };

        let class = Altrep::make_altreal_class::<MyCompactRealRange>("crr", "mypkg");
        let obj = Altrep::from_state_and_class(mystate, class.clone(), false);

        assert_eq!(obj.len(), 10);
        // assert_eq!(obj.sum(true), r!(45.0));
        assert_eq!(obj.as_robj().as_real_slice().unwrap(), [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);

        // index 5 is missing
        let mystate_w_missing = MyCompactRealRange { start: 0.0, len: 10, step: 1.0, missing_index: 5 };

        let obj_w_missing = Altrep::from_state_and_class(mystate_w_missing, class, false);
        let robj_w_missing = Robj::from(obj_w_missing);
        let doubles_w_missing: Doubles = robj_w_missing.try_into()?;
        assert_eq!(doubles_w_missing.elt(9), Rfloat::from(9.0));

        // TODO: Win32 currently handles NA improperly. Re-enable this when the problem is fixed.
        if cfg!(not(target_arch = "x86")) {
            assert!(doubles_w_missing.elt(5).is_na());
        }

        // tests for get_region()
        let mut dest = [0.0.into(); 2];
        doubles_w_missing.get_region(2, &mut dest);
        assert_eq!(dest, [2.0, 3.0]);
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
                self.len
            }
        }

        impl AltLogicalImpl for IsEven {
            fn elt(&self, index: usize) -> Rbool {
                (index % 2 == 1).into()
            }
        }

        let mystate = IsEven { len: 10 };

        let class = Altrep::make_altlogical_class::<IsEven>("iseven", "mypkg");
        let obj = Altrep::from_state_and_class(mystate, class, false);

        assert_eq!(obj.len(), 10);
        // assert_eq!(obj.sum(true), r!(5.0));
        assert_eq!(obj.as_robj().as_logical_slice().unwrap(), [FALSE, TRUE, FALSE, TRUE, FALSE, TRUE, FALSE, TRUE, FALSE, TRUE]);
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
        assert_eq!(obj.as_robj().as_raw_slice().unwrap(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
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
                self.len
            }
        }

        impl AltComplexImpl for MyCompactComplexRange {
            fn elt(&self, index: usize) -> Rcplx {
                Rcplx::from(c64::new(self.start + self.step * index as f64, self.start + self.step * index as f64))
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
                self.len
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

        assert!(obj.is_altstring());
        assert_eq!(obj.len(), 10);
        assert_eq!(Robj::from(obj), r!(["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]));
    }
}

#[test]
#[cfg(use_r_altlist)]
fn test_altlist() {
    use extendr_api::AltListImpl;
    with_r(|| {
        #[derive(Debug, Clone)]
        pub struct VecUsize(pub Vec<Option<usize>>);

        // need to make the VecUsize object `.into_robj()`-able
        #[extendr]
        impl VecUsize {}

        impl AltrepImpl for VecUsize {
            fn length(&self) -> usize {
                self.0.len()
            }
        }

        impl AltListImpl for VecUsize {
            fn elt(&self, index: usize) -> Robj {
                Self(vec![self.0[index]]).into_robj()
            }
        }

        let vu = VecUsize(vec![Some(1), None, Some(10)]);

        let class = Altrep::make_altlist_class::<VecUsize>("li", "mypkg");
        let obj = Altrep::from_state_and_class(vu, class, false);

        // confirm it is altlist
        assert!(obj.is_altlist());

        // confirm method is accurate
        assert_eq!(obj.len(), 3);

        // convert to a list and test the .elt() method
        let l = List::try_from(obj.into_robj()).unwrap();
        let li = l.elt(1).unwrap();

        assert!(li.inherits("VecUsize"));
    })
}
