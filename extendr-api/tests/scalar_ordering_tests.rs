use extendr_api::prelude::*;
use rstest::rstest;

// Tests without NA do not require `test!` macro

#[rstest]
#[case(RFloat::from(2.0), RFloat::from(1.0))]
#[case(RInt::from(2), RInt::from(1))]
#[case(RBool::from(true), RBool::from(false))]
fn left_gt_right<T>(#[case] left: T, #[case] right: T)
where
    T: PartialOrd + PartialEq + Copy,
{
    assert!(left > right);
}

#[rstest]
#[case(RFloat::from(2.0), RFloat::from(1.0))]
#[case(RFloat::from(2.0), RFloat::from(2.0))]
#[case(RInt::from(2), RInt::from(1))]
#[case(RInt::from(2), RInt::from(2))]
#[case(RBool::from(true), RBool::from(true))]
#[case(RBool::from(false), RBool::from(false))]
fn left_gte_right<T>(#[case] left: T, #[case] right: T)
where
    T: PartialOrd + PartialEq + Copy,
{
    assert!(left >= right);
}

#[rstest]
#[case(RFloat::from(1.0), RFloat::from(2.0))]
#[case(RInt::from(1), RInt::from(2))]
#[case(RBool::from(false), RBool::from(true))]
fn left_lt_right<T>(#[case] left: T, #[case] right: T)
where
    T: PartialOrd + PartialEq + Copy,
{
    assert!(left < right);
}

#[rstest]
#[case(RFloat::from(1.0), RFloat::from(2.0))]
#[case(RFloat::from(2.0), RFloat::from(2.0))]
#[case(RInt::from(1), RInt::from(2))]
#[case(RInt::from(2), RInt::from(2))]
#[case(RBool::from(true), RBool::from(true))]
#[case(RBool::from(false), RBool::from(false))]
fn left_lte_right<T>(#[case] left: T, #[case] right: T)
where
    T: PartialOrd + PartialEq + Copy,
{
    assert!(left <= right);
}

#[rstest]
#[case(RFloat::from(2.0), RFloat::from(2.0))]
#[case(RInt::from(2), RInt::from(2))]
#[case(RBool::from(true), RBool::from(true))]
#[case(RBool::from(false), RBool::from(false))]
fn left_eq_right<T>(#[case] left: T, #[case] right: T)
where
    T: PartialOrd + PartialEq + Copy,
{
    assert!(left == right);
    assert!(right == left);
}

#[rstest]
#[case(RFloat::from(1.0), RFloat::from(2.0))]
#[case(RInt::from(1), RInt::from(2))]
#[case(RBool::from(true), RBool::from(false))]
fn left_neq_right<T>(#[case] left: T, #[case] right: T)
where
    T: PartialOrd + PartialEq + Copy,
{
    assert!(left != right);
    assert!(right != left);
}

// `NA` should be created in `test!` macro block

#[rstest]
#[case(RFloat::from(1.0))]
#[case(RInt::from(1))]
#[case(RBool::from(true))]
fn left_gt_or_gte_right_na<T>(#[case] left: T)
where
    T: PartialOrd + PartialEq + Copy + CanBeNA,
{
    test! {
        let right = T::na();
        assert_eq!(left > right, false);
        assert_eq!(left >= right, false);
    }
}

#[rstest]
#[case(RFloat::from(1.0))]
#[case(RInt::from(1))]
#[case(RBool::from(true))]
fn left_lt_or_lte_right_na<T>(#[case] left: T)
where
    T: PartialOrd + PartialEq + Copy + CanBeNA,
{
    test! {
        let right = T::na();
        assert_eq!(left < right, false);
        assert_eq!(left <= right, false);
    }
}

#[rstest]
#[case(RFloat::from(1.0))]
#[case(RInt::from(1))]
#[case(RBool::from(true))]
fn left_na_lt_or_lte_right<T>(#[case] right: T)
where
    T: PartialOrd + PartialEq + Copy + CanBeNA,
{
    test! {
        let left = T::na();
        assert_eq!(left < right, false);
        assert_eq!(left <= right, false);
    }
}

#[rstest]
#[case(RFloat::from(1.0))]
#[case(RInt::from(1))]
#[case(RBool::from(true))]
fn left_na_gt_or_gte_right<T>(#[case] right: T)
where
    T: PartialOrd + PartialEq + Copy + CanBeNA,
{
    test! {
        let left = T::na();
        assert_eq!(left > right, false);
        assert_eq!(left >= right, false);
    }
}

#[rstest]
#[case(RFloat::from(1.0))]
#[case(RInt::from(1))]
#[case(RBool::from(true))]
#[case(RBool::from(false))]
fn na_vs_value<T>(#[case] value: T)
where
    T: PartialOrd + PartialEq + Copy + CanBeNA,
{
    test! {
        let na = T::na();
        assert_eq!(value.partial_cmp(&na), None);
        assert_eq!(na.partial_cmp(&value), None);
        assert_eq!(na.partial_cmp(&na), None);
    }
}

#[test]
fn collection_sort_rint() {
    let mut raw = vec![45, 192, 87, 23, 255];
    let mut rints: Vec<RInt> = raw.iter().map(|&x| RInt::from(x)).collect();
    raw.sort();
    rints.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert!(raw.eq(&rints));
}

#[test]
fn collection_sort_rfloat() {
    let mut raw = vec![45.0, 192.0, 87.0, 23.0, 255.0];
    let mut rfloats: Vec<RFloat> = raw.iter().map(|&x| RFloat::from(x)).collect();
    raw.sort_by(|a, b| a.partial_cmp(b).unwrap());
    rfloats.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert!(raw.eq(&rfloats));
}

#[rstest]
#[case(vec![45, 192, 87, 23, 255], vec![23, 45, 87, 192, 255], RInt::default())]
#[case(vec![45.0, 192.0, 87.0, 23.0, 255.0], vec![23.0, 45.0, 87.0, 192.0, 255.0], RFloat::default())]
fn collection_sort<T, U>(#[case] raw: Vec<U>, #[case] ordered: Vec<U>, #[case] _marker: T)
where
    T: PartialOrd + PartialEq + Copy + From<U>,
    U: PartialEq + Copy + PartialEq<T>,
{
    let mut scalars: Vec<T> = raw.iter().map(|&x| x.into()).collect();
    scalars.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert!(ordered.eq(&scalars));
}

#[test]
fn collection_sort_bool() {
    let raw = [true, false, true, false, true];
    let ordered = [false, false, true, true, true];
    let mut scalars: Vec<RBool> = raw.iter().map(|&x| x.into()).collect();
    scalars.sort_by(|a, b| a.partial_cmp(b).unwrap());
    for (xi, yi) in ordered.iter().zip(scalars.iter()) {
        assert_eq!(*xi, yi.to_bool());
    }
}
