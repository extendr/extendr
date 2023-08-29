use extendr_api::prelude::*;
use rstest::rstest;

// Tests without NA do not require `test!` macro

#[rstest]
#[case(Rfloat::from(2.0), Rfloat::from(1.0))]
#[case(Rint::from(2), Rint::from(1))]
#[case(Rbool::from(true), Rbool::from(false))]
fn left_gt_right<T, U>(#[case] left: T, #[case] right: T)
where
    T: Scalar<U> + PartialOrd + PartialEq + Copy,
    U: PartialEq + Copy,
{
    assert!(left > right);
}

#[rstest]
#[case(Rfloat::from(2.0), Rfloat::from(1.0))]
#[case(Rfloat::from(2.0), Rfloat::from(2.0))]
#[case(Rint::from(2), Rint::from(1))]
#[case(Rint::from(2), Rint::from(2))]
#[case(Rbool::from(true), Rbool::from(true))]
#[case(Rbool::from(false), Rbool::from(false))]
fn left_gte_right<T, U>(#[case] left: T, #[case] right: T)
where
    T: Scalar<U> + PartialOrd + PartialEq + Copy,
    U: PartialEq + Copy,
{
    assert!(left >= right);
}

#[rstest]
#[case(Rfloat::from(1.0), Rfloat::from(2.0))]
#[case(Rint::from(1), Rint::from(2))]
#[case(Rbool::from(false), Rbool::from(true))]
fn left_lt_right<T, U>(#[case] left: T, #[case] right: T)
where
    T: Scalar<U> + PartialOrd + PartialEq + Copy,
    U: PartialEq + Copy,
{
    assert!(left < right);
}

#[rstest]
#[case(Rfloat::from(1.0), Rfloat::from(2.0))]
#[case(Rfloat::from(2.0), Rfloat::from(2.0))]
#[case(Rint::from(1), Rint::from(2))]
#[case(Rint::from(2), Rint::from(2))]
#[case(Rbool::from(true), Rbool::from(true))]
#[case(Rbool::from(false), Rbool::from(false))]
fn left_lte_right<T, U>(#[case] left: T, #[case] right: T)
where
    T: Scalar<U> + PartialOrd + PartialEq + Copy,
    U: PartialEq + Copy,
{
    assert!(left <= right);
}

#[rstest]
#[case(Rfloat::from(2.0), Rfloat::from(2.0))]
#[case(Rint::from(2), Rint::from(2))]
#[case(Rbool::from(true), Rbool::from(true))]
#[case(Rbool::from(false), Rbool::from(false))]
fn left_eq_right<T, U>(#[case] left: T, #[case] right: T)
where
    T: Scalar<U> + PartialOrd + PartialEq + Copy,
    U: PartialEq + Copy,
{
    assert!(left == right);
    assert!(right == left);
}

#[rstest]
#[case(Rfloat::from(1.0), Rfloat::from(2.0))]
#[case(Rint::from(1), Rint::from(2))]
#[case(Rbool::from(true), Rbool::from(false))]
fn left_neq_right<T, U>(#[case] left: T, #[case] right: T)
where
    T: Scalar<U> + PartialOrd + PartialEq + Copy,
    U: PartialEq + Copy,
{
    assert!(left != right);
    assert!(right != left);
}

// `NA` should be created in `test!` macro block

#[rstest]
#[case(Rfloat::from(1.0))]
#[case(Rint::from(1))]
#[case(Rbool::from(true))]
fn left_gt_or_gte_right_na<T, U>(#[case] left: T)
where
    T: Scalar<U> + PartialOrd + PartialEq + Copy,
    U: PartialEq + Copy,
{
    test! {
        let right = T::na();
        assert_eq!(left > right, false);
        assert_eq!(left >= right, false);
    }
}

#[rstest]
#[case(Rfloat::from(1.0))]
#[case(Rint::from(1))]
#[case(Rbool::from(true))]
fn left_lt_or_lte_right_na<T, U>(#[case] left: T)
where
    T: Scalar<U> + PartialOrd + PartialEq + Copy,
    U: PartialEq + Copy,
{
    test! {
        let right = T::na();
        assert_eq!(left < right, false);
        assert_eq!(left <= right, false);
    }
}

#[rstest]
#[case(Rfloat::from(1.0))]
#[case(Rint::from(1))]
#[case(Rbool::from(true))]
fn left_na_lt_or_lte_right<T, U>(#[case] right: T)
where
    T: Scalar<U> + PartialOrd + PartialEq + Copy,
    U: PartialEq + Copy,
{
    test! {
        let left = T::na();
        assert_eq!(left < right, false);
        assert_eq!(left <= right, false);
    }
}

#[rstest]
#[case(Rfloat::from(1.0))]
#[case(Rint::from(1))]
#[case(Rbool::from(true))]
fn left_na_gt_or_gte_right<T, U>(#[case] right: T)
where
    T: Scalar<U> + PartialOrd + PartialEq + Copy,
    U: PartialEq + Copy,
{
    test! {
        let left = T::na();
        assert_eq!(left > right, false);
        assert_eq!(left >= right, false);
    }
}

#[rstest]
#[case(Rfloat::from(1.0))]
#[case(Rint::from(1))]
#[case(Rbool::from(true))]
#[case(Rbool::from(false))]
fn na_vs_value<T, U>(#[case] value: T)
where
    T: Scalar<U> + PartialOrd + PartialEq + Copy,
    U: PartialEq + Copy,
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
    let mut rints: Vec<Rint> = raw.iter().map(|&x| Rint::from(x)).collect();
    raw.sort();
    rints.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert!(raw.eq(&rints));
}

#[test]
fn collection_sort_rfloat() {
    let mut raw = vec![45.0, 192.0, 87.0, 23.0, 255.0];
    let mut rfloats: Vec<Rfloat> = raw.iter().map(|&x| Rfloat::from(x)).collect();
    raw.sort_by(|a, b| a.partial_cmp(b).unwrap());
    rfloats.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert!(raw.eq(&rfloats));
}

#[rstest]
#[case(vec![45, 192, 87, 23, 255], vec![23, 45, 87, 192, 255], Rint::default())]
#[case(vec![45.0, 192.0, 87.0, 23.0, 255.0], vec![23.0, 45.0, 87.0, 192.0, 255.0], Rfloat::default())]
fn collection_sort<T, U>(#[case] raw: Vec<U>, #[case] ordered: Vec<U>, #[case] _marker: T)
where
    T: Scalar<U> + PartialOrd + PartialEq + Copy + From<U>,
    U: PartialEq + Copy + PartialEq<T>,
{
    let mut scalars: Vec<T> = raw.iter().map(|&x| x.into()).collect();
    scalars.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert!(ordered.eq(&scalars));
}

#[test]
fn collection_sort_bool() {
    let raw = vec![true, false, true, false, true];
    let ordered = vec![false, false, true, true, true];
    let mut scalars: Vec<Rbool> = raw.iter().map(|&x| x.into()).collect();
    scalars.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert!(ordered.eq(&scalars));
}
