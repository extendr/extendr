use crate::prelude::*;

#[test]
fn from_iterator() {
    with_r(|| {
        let vec : Integers = (0..3).map(|i| i.into()).collect();
        assert_eq!(vec, Integers::from_values([0, 1, 2]));
    });
}

#[test]
fn iter_mut() {
    with_r(|| {
        let mut vec = Integers::from_values(0..3);
        vec.iter_mut().for_each(|v| *v += 1);
        assert_eq!(vec, Integers::from_values(1..4));
    });
}

#[test]
fn iter() {
    with_r(|| {
        let vec = Integers::from_values(0..3);
        assert_eq!(vec.iter().sum::<Rint>(), 3);
    });
}

#[test]
fn from_values_short() {
    with_r(|| {
        // Short (<64k) vectors are allocated.
        let vec = Integers::from_values((0..3).map(|i| 2-i));
        assert_eq!(vec.is_altrep(), false);
        assert_eq!(r!(vec.clone()), r!([2, 1, 0]));
        assert_eq!(vec.elt(1), 1);
        let mut dest = [0.into(); 2];
        vec.get_region(1, &mut dest);
        assert_eq!(dest, [1, 0]);
    });
}

#[test]
fn from_values_altrep() {
    with_r(|| {
        let vec = Integers::from_values_altrep(0..1000000000);
        assert_eq!(vec.is_altrep(), true);
        assert_eq!(vec.elt(12345678), 12345678);
        let mut dest = [0.into(); 2];
        vec.get_region(12345678, &mut dest);
        assert_eq!(dest, [12345678, 12345679]);
    });
}

#[test]
fn new() {
    with_r(|| {
        let vec = Integers::new(10);
        assert_eq!(vec.is_integer(), true);
        assert_eq!(vec.len(), 10);
    });
}
