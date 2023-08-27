use crate::prelude::*;

#[test]
fn from_iterator() {
    with_r(|| {
        let vec: Logicals = (0..3).map(|i| (i % 2 == 0).into()).collect();
        assert_eq!(vec, Logicals::from_values([true, false, true]));
    });
}

#[test]
fn iter_mut() {
    with_r(|| {
        let mut vec = Logicals::from_values([true, false, true]);
        vec.iter_mut().for_each(|v| *v = true.into());
        assert_eq!(vec, Logicals::from_values([true, true, true]));
    });
}

// #[test]
// fn iter() {
//     with_r(|| {
//         let mut vec = Logicals::from_values([true, false, true]);
//         assert_eq!(vec.iter().sum::<Rint>(), 3);
//     }
// }

#[test]
fn from_values_short() {
    with_r(|| {
        // Short (<64k) vectors are allocated.
        let vec = Logicals::from_values([true, false, true]);
        assert_eq!(vec.is_altrep(), false);
        assert_eq!(r!(vec.clone()), r!([true, false, true]));
        assert_eq!(vec.elt(1), false);
        let mut dest = [false.into(); 2];
        vec.get_region(1, &mut dest);
        assert_eq!(dest, [false, true]);
    });
}

#[test]
fn from_values_altrep() {
    with_r(|| {
        let vec = Logicals::from_values_altrep((0..1000000000).map(|_| Rbool::from(true)));
        assert_eq!(vec.is_altrep(), true);
        assert_eq!(vec.elt(12345678), true);
        let mut dest = [false.into(); 2];
        vec.get_region(12345678, &mut dest);
        assert_eq!(dest, [true, true]);
    });
}

#[test]
fn new() {
    with_r(|| {
        let vec = Logicals::new(10);
        assert_eq!(vec.is_logical(), true);
        assert_eq!(vec.len(), 10);
    });
}
